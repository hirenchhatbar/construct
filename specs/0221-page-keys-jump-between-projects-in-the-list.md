# 0221-page-keys-jump-between-projects-in-the-list

Status: accepted
Date: 2026-10-16
Area: tui
Scope: `C-PageUp` / `C-PageDown` move the session-list selection between project headers from any pane.

## Decision

`C-PageDown` selects the next project header and `C-PageUp` the previous one, wrapping at the ends. Sessions, operators and archive rows are skipped. The keys work the same whether the session list or a session pane has focus, including a session whose terminal is capturing keystrokes.

Bare `PageUp` / `PageDown` keep paging the active view everywhere. With no project in the list, the Ctrl variants fall through to their normal route.

## Reason

The list jump keys stop on live sessions as well as project headers (spec 0219), so getting from one project to the next takes one press per session in between. A key that only stops on projects makes project-to-project movement one keystroke.

Bare page keys were tried first, scoped to list focus. They behaved differently depending on which pane held focus, which was confusing. One modified pair with one meaning everywhere avoids that.

## Consequences

- `C-PageUp` / `C-PageDown` never reach a child terminal while the list has a project.
- `C-PageUp` from a session inside a project lands on that project's header first, the same way the list jump keys move backwards.
- Collapsed projects are still stops: their header stays in the list.
- Terminals that claim Ctrl+PageUp/PageDown for tab switching won't deliver these keys.

## Non-Goals

- Jumping straight to the Nth project by number.
