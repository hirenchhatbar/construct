# 0222-one-chord-jumps-to-the-next-session-that-wants-you

Status: accepted
Date: 2026-10-17
Area: ux
Scope: `C-x C-a` moves the list selection to the next row wearing the attention dot, wrapping, in both clients.

## Decision

`C-x C-a` selects the next row in the session list that wears the attention dot, in list order after the current selection, wrapping from the last row to the first. There is one direction only; no "previous" chord.

- **Same stops as the wants-you tally.** A row qualifies exactly when it paints the dot: archived rows and harness-native subagent mirrors never qualify. A collapsed project or parent whose hidden descendant wants attention qualifies as itself; the chord selects it and does not expand it.
- **Repeating the chord walks the queue.** Selecting a session clears its dot, so the next press moves on. With nothing left to visit, the selection stays put and the client says so.
- **Focus does not move.** Pressed from a focused session view, the view follows the new selection so the user can answer in place. The chord is `C-x`-prefixed so it escapes a focused child PTY like every other `C-x` chord.
- **Shared spelling.** The TUI's emacs and vim profiles and the web UI bind the same chord to the same action (spec 0150).

## Reason

Users work inside one session while others finish, error, or ask for approval. The dot and the wants-you tally showed *that* something wanted them but reaching it took the mouse. One wrapping chord the user can press with Ctrl held is enough: the queue drains as it is visited, so a reverse direction adds a binding without adding reach.

`C-x C-n` would be the obvious spelling but already reorders the selection; `C-x C-a` ("attention") was unbound in every keymap.

## Consequences

- The qualifying-row rule must stay in step with whatever decides the dot and the wants-you tally; a chord that stops on rows without a dot, or skips rows with one, breaks the user's read of the list.
- Sessions hidden under a collapsed row are reached in two steps: the chord lands on the collapsed row, the user expands it.

## Non-Goals

- A "previous attention" chord.
- Auto-expanding collapsed projects or parents, which would mutate persisted collapse state as a side effect of navigation.
