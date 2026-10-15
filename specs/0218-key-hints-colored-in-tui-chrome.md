# 0218-key-hints-colored-in-tui-chrome

Status: accepted
Date: 2026-10-15
Area: tui
Scope: Bracketed key hints in TUI chrome get their own theme color so keys stand out from the prose around them.

## Decision

Every bracketed key hint in TUI chrome (`[C-x z]`, `[Esc]`, the `[r]` of `[r]ename`) is painted in the theme's `key_hint` color, brackets included. Only the bracketed key is colored; the label after it keeps its surrounding style. Both keymap profiles (emacs and vim) follow the same rule.

`key_hint` is a theme color like any other: every built-in palette defines it, and `theme.toml` `[colors]` can override it.

Exceptions:

- A highlighted or hovered row keeps its highlight colors for the key too, so contrast on the highlight background is never worse than the row's own text.
- Bars with their own background (the modeline) keep the bar's colors; a single key color cannot contrast with every theme's bar.
- Buttons that happen to be bracketed (`[next step]`, `[p Publish]`, `[start the interactive tour]`) are not keys and keep their button styling.

## Reason

Hints read as plain prose otherwise, and users scanning for "which key does this" had to parse every line. A dedicated hue lets the eye jump straight to the keys in footers, dialogs, the minibuffer placeholder, help, and the tour.

## Consequences

- Coloring only applies to text construct writes itself. Anything that can carry user, agent, or daemon data (session titles, model names, file names, error notes, transcripts, tool blocks, agent-authored panel links) stays uncolored, because brackets there are not keys.
- New chrome hints should use the bracket form so they pick up the color automatically.
- Adding a palette means choosing a `key_hint` that differs in hue from `text`/`dim` and reads on that palette's background.

## Non-Goals

- Coloring keys inside session transcripts or harness output.
- Making key hints clickable where they were not already.
