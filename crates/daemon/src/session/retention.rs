//! Periodic retention sweep (spec 0223): archive finished sessions that sat
//! idle too long, and delete sessions that stayed archived too long.

use super::*;
use crate::config::RetentionConfig;
use construct_protocol::SessionKind;
use std::collections::HashSet;
use std::path::Path;

const SWEEP_INTERVAL: Duration = Duration::from_secs(60 * 60);
/// Let boot-time resume settle before the first pass.
const FIRST_SWEEP_DELAY: Duration = Duration::from_secs(60);
const DAY_MS: i64 = 24 * 60 * 60 * 1000;

#[derive(Debug, PartialEq, Eq)]
enum Verdict {
    Keep,
    Archive,
    Delete,
}

/// Pure decision for one session, separated from IO so it is unit-testable.
fn verdict(
    cfg: &RetentionConfig,
    s: &SessionSummary,
    archived_at_ms: Option<i64>,
    now_ms: i64,
) -> Verdict {
    let older_than = |since_ms: i64, days: u64| {
        now_ms.saturating_sub(since_ms) >= (days as i64).saturating_mul(DAY_MS)
    };
    if s.archived {
        match (cfg.delete_archived_after_days, archived_at_ms) {
            (Some(days), Some(at)) if older_than(at, days) => Verdict::Delete,
            _ => Verdict::Keep,
        }
    } else {
        let idle_since = s.last_event_at.unwrap_or(s.created_at).timestamp_millis();
        match cfg.auto_archive_after_days {
            Some(days) if s.state.is_terminal() && older_than(idle_since, days) => Verdict::Archive,
            _ => Verdict::Keep,
        }
    }
}

impl SessionManager {
    pub fn spawn_retention_sweep(self: &Arc<Self>) {
        let mgr = self.clone();
        tokio::spawn(async move {
            let mut tick = tokio::time::interval_at(
                tokio::time::Instant::now() + FIRST_SWEEP_DELAY,
                SWEEP_INTERVAL,
            );
            loop {
                tick.tick().await;
                mgr.retention_sweep().await;
            }
        });
    }

    async fn retention_sweep(&self) {
        // Read per pass so a config reload applies without a restart.
        let cfg = self.config().retention.clone();
        if cfg.auto_archive_after_days.is_none() && cfg.delete_archived_after_days.is_none() {
            return;
        }
        let excluded: HashSet<String> = self
            .list_groups()
            .await
            .into_iter()
            .filter(|g| cfg.exclude_projects.contains(&g.name))
            .map(|g| g.id)
            .collect();
        let now_ms = Utc::now().timestamp_millis();
        for s in self.list().await {
            // Subagents and native mirrors follow their parent (spec 0052);
            // the minibuffer is never retired.
            if s.kind != SessionKind::User
                || s.parent_session_id.is_some()
                || s.native_subagent.is_some()
                || s.group_id.as_ref().is_some_and(|g| excluded.contains(g))
            {
                continue;
            }
            let mut archived_at = self.storage.archived_at_ms(&s.id);
            if s.archived && archived_at.is_none() {
                // Archived before retention existed: start its clock now
                // rather than guess, so enabling the feature deletes nothing
                // immediately.
                let _ = self.storage.set_archived_at_ms(&s.id, now_ms);
                archived_at = Some(now_ms);
            }
            match verdict(&cfg, &s, archived_at, now_ms) {
                Verdict::Keep => {}
                Verdict::Archive if cfg.dry_run => {
                    tracing::info!(session = %s.id, "retention dry run: would archive");
                }
                Verdict::Archive => match self.archive(&s.id).await {
                    Ok(()) => tracing::info!(session = %s.id, "retention: archived"),
                    Err(e) => {
                        tracing::warn!(session = %s.id, error = %e, "retention archive failed")
                    }
                },
                Verdict::Delete => {
                    if cfg.protect_dirty_worktrees {
                        if let Some(wt) = s.worktree.as_deref() {
                            if worktree::has_uncommitted_changes(Path::new(wt)).await {
                                tracing::info!(session = %s.id, worktree = wt, "retention: kept, worktree has uncommitted changes");
                                continue;
                            }
                        }
                    }
                    if cfg.dry_run {
                        tracing::info!(session = %s.id, "retention dry run: would delete");
                        continue;
                    }
                    match self.delete(&s.id).await {
                        Ok(()) => tracing::info!(session = %s.id, "retention: deleted"),
                        Err(e) => {
                            tracing::warn!(session = %s.id, error = %e, "retention delete failed")
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn summary(state: &str, archived: bool, last_event_at: &str) -> SessionSummary {
        serde_json::from_value(serde_json::json!({
            "id": "s1",
            "harness": "shell",
            "cwd": "/tmp",
            "state": state,
            "created_at": "2026-01-01T00:00:00Z",
            "last_event_at": last_event_at,
            "archived": archived,
        }))
        .expect("summary")
    }

    #[test]
    fn verdicts() {
        let cfg = RetentionConfig {
            auto_archive_after_days: Some(7),
            delete_archived_after_days: Some(30),
            ..RetentionConfig::default()
        };
        let day = DAY_MS;
        let last = "2026-01-01T00:00:00Z";
        let t0 = chrono::DateTime::parse_from_rfc3339(last)
            .unwrap()
            .timestamp_millis();

        // Finished and idle past the window: archive; before it: keep.
        assert_eq!(
            verdict(&cfg, &summary("done", false, last), None, t0 + 7 * day),
            Verdict::Archive
        );
        assert_eq!(
            verdict(&cfg, &summary("errored", false, last), None, t0 + 6 * day),
            Verdict::Keep
        );
        // Never archive a session that is not finished.
        assert_eq!(
            verdict(
                &cfg,
                &summary("awaiting_input", false, last),
                None,
                t0 + 99 * day
            ),
            Verdict::Keep
        );
        // Archived: the archive stamp decides, not activity.
        assert_eq!(
            verdict(&cfg, &summary("done", true, last), Some(t0), t0 + 30 * day),
            Verdict::Delete
        );
        assert_eq!(
            verdict(&cfg, &summary("done", true, last), Some(t0), t0 + 29 * day),
            Verdict::Keep
        );
        assert_eq!(
            verdict(&cfg, &summary("done", true, last), None, t0 + 99 * day),
            Verdict::Keep
        );
        // Rules off: nothing happens.
        let off = RetentionConfig::default();
        assert_eq!(
            verdict(&off, &summary("done", false, last), None, t0 + 999 * day),
            Verdict::Keep
        );
        assert_eq!(
            verdict(&off, &summary("done", true, last), Some(t0), t0 + 999 * day),
            Verdict::Keep
        );
    }
}
