# 0219-list-jump-keys-stop-on-projects-and-sessions

Status: accepted
Date: 2026-10-15
Area: tui
Scope: The session-list jump keys (`}` / `{` and `]]` / `[[` in vim, `M-n` / `M-p` in emacs) stop on project headers and live sessions alike.

## Decision

The list jump keys move the selection to the next / previous **project header or live session**, wrapping at the ends. They skip operator rows, "N archived" disclosure rows, and archived sessions.

The row-by-row keys (`j` / `k`, `C-n` / `C-p`, arrows) keep stopping on every materialized row.

## Reason

Jumping only between project headers made moving to a session a two-step motion (jump to the header, then step down). Skipping the rows that are rarely a destination (operators, archive disclosures, archived sessions) gives one pair of keys that reaches both projects and the sessions in them.

## Consequences

- Collapsed projects hide their members, so jumps over a collapsed project still go header to header.
- Landing on a project header still shows its dashboard; the jump does not pass through it silently.
- Operators and archive rows remain reachable only through the row-by-row keys or the mouse.
