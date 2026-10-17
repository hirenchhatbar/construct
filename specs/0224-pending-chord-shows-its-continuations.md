# 0224-pending-chord-shows-its-continuations

Status: accepted
Date: 2026-10-17
Area: ux
Scope: A half-typed chord shows every key that can follow it, in both the TUI and the web UI.

## Decision

While a chord is pending (`C-x`, vim's `C-w`, `g`, `[`, …), the client lists every key that can complete or extend it, each with a short description of its action. A key that opens a longer chord is listed with `…` instead of an action.

- **Delayed, then sticky.** The list appears only after the chord has sat half-typed for about a second, so a practiced chord never flashes it. Once up, it follows the chord deeper with no new wait, until the chord fires or is cancelled. Both clients use the same delay.
- **A shown list doesn't time out.** Where a client cancels idle chords (the web UI), it stops doing so once the list is up, so the chord waits for a key or Escape while the user reads.
- **Generated from the live keymap.** The list is derived from the same binding table that resolves the chord, so it can never advertise a key that is not bound. Shifted twins of a character key are listed once.
- **TUI: floats over the bottom of the screen.** It covers rows instead of growing the footer, so a prefix never slides the main block (spec 0112). It is display-only and lays out in columns; when it doesn't fit, the last cell says how many more there are.
- **Web UI: the pending-chord echo carries the list.** Chords with no web surface stay listed but dimmed: the binding exists, it only answers with a notice.

## Reason

`C-x` alone leads to dozens of actions. Echoing just the typed prefix told users a chord was armed but not what they could do with it, so they had to leave for the help popup or memorize the table.

## Consequences

- Every action reachable by a chord needs a short description. The TUI and the web UI word their shared `C-x` chords the same way (spec 0150).
- Adding a binding adds its hint automatically; no separate hint list to maintain.
- The panel covers whatever sits at the bottom of the screen while it is up.

## Non-Goals

- Not a replacement for the help popup, which explains bindings in context.
- No clicking a hint to run it.
