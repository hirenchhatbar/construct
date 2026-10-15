# 0220-key-hints-trail-pane-and-dialog-titles

Status: accepted
Date: 2026-10-15
Area: tui
Scope: Panes and dialogs advertise their non-obvious keys right after the title on their top border.

## Decision

A pane or dialog whose keys are not already listed in its own body shows them on its top border, directly after the title, in the bracketed form of spec 0218 (keys in `key_hint`, prose dim). The hint is shown whether or not the surface has focus.

- Keys that go through the keymap (e.g. the session list's jump keys) are read from the live keymap, so the hint always matches what is bound in the active profile. A chord bound twice only to tolerate how terminals report Shift (`}` and `S-}`) is shown once.
- Keys a dialog handles itself (Esc, Enter, arrows) are written as fixed text; they are the same in every profile.
- A surface that already lists its keys in a footer or header does not repeat them in the title.

When a title is too narrow for everything, the hint wins over secondary title content (such as the session list's fleet tallies) and is dropped only when even the title plus hint would collide with a control.

## Reason

Navigation keys like the session list's project/session jump were only discoverable through the help popup. Putting them where the eye already lands, the title, makes each surface self-describing.

## Consequences

- New dialogs without a key footer should add a title hint.
- Keymap-driven hints must be looked up, never hardcoded, so a profile switch or a future rebinding can't leave them stale.

## Non-Goals

- Listing every key a surface accepts; the hint covers the ones a user would not guess.
- Hints inside session transcripts or harness output.
