# 0223-session-retention-archive-then-delete

Status: accepted
Date: 2026-10-17
Area: persistence
Scope: The daemon can retire old sessions automatically: archive finished ones after an idle window, then delete ones that stayed archived past a second window.

## Decision

Retention is a two-stage, opt-in policy configured in `[retention]`:

- **Auto-archive.** A finished session (Done or Errored) whose last activity is older than `auto_archive_after_days` is archived through the ordinary archive operation. Sessions that are running, paused, pending, or awaiting input are never auto-archived.
- **Delete after archive.** A session that has stayed archived longer than `delete_archived_after_days` is deleted through the ordinary delete operation. Archive is the grace period: restarting the session un-archives it and takes it out of the sweep, and archiving it again restarts its clock.
- **The clock is when the session was archived**, recorded at archive time. A session archived before this record existed starts its clock the first time the sweep sees it, so turning retention on never deletes anything immediately.
- **Every rule is off by default.** Unset windows mean "never". Upgrading never removes data.
- **Uncommitted work is protected by default.** With `protect_dirty_worktrees` (default on), a session whose worktree has uncommitted or untracked changes is kept and logged instead of deleted. Committed work is not at risk: deleting a session removes its worktree but not its branch.
- **Scope.** Only top-level user sessions are swept. Subagents and native mirrors follow their parent through the existing cascade; the minibuffer is never touched. Sessions in projects named in `exclude_projects` are skipped.
- **`dry_run`** logs what the sweep would archive or delete and changes nothing.
- The sweep runs shortly after daemon start and then hourly, reading the live config on each pass, so a config reload takes effect without a restart.

## Reason

Archive keeps transcripts and worktrees forever, so disk use and list clutter grow without bound unless someone deletes by hand. Deleting directly after inactivity would be irreversible with no warning; routing through archive first gives a visible, reversible stage where the user can still restore a session before it is destroyed.

## Consequences

- Supersedes the "no auto-archive policies, or archive retention/expiry" non-goal in [0126-session-archive](0126-session-archive.md).
- The sweep must keep using the single archive and delete entry points so the cascade ([0052](0052-session-removal-cascades-to-subagents.md)), adapter teardown, and client broadcasts stay identical to a manual action.
- Archived subagents whose parent stays active are not swept; they leave with their parent.
- Hourly granularity: a session is retired up to an hour after its window expires.

## Non-Goals

- No per-session "keep forever" pin.
- No trash or undo after delete.
- No client UI for retention settings; it is config-only.
