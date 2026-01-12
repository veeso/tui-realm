# Migrating from tui-realm 3.x

- [Migrating from tui-realm 3.x](#migrating-from-tui-realm-3x)
  - [Introduction](#introduction)

---

## Introduction

This document is a work in progress until 4.0 is released, listing the essential changes as they are introduced.

### ratatui 0.30

`ratatui` has been upgraded to 0.30, for all their breaking changes, read their [Blockpost](https://ratatui.rs/highlights/v030).

### Replaced `TextSpan` with ratatui equivalent

The previous `tuirealm::props::TextSpan` has been replaced with `ratatui::text::{Span, Line, Text}`.

Because of the new types, new `AttrValue` and `PropValue` variants have been introduced: `TextSpan`, `TextLine` and `Text`.

### Replace `(String, Alignment)` Titles with proper struct

The previous `(String, Alignment)` Tuple has been replaced with a more feature-full `Title` struct.

Due to the title now using `Line` under the hood, it is now possible to style individual characters in the title.

### Removal of `PropPayload` and `State` `Tup3` and `Tup4` variants

The `3` and `4` tuple variants of `PropPayload` and `State` have been removed as they were blowing up the size of the respective enum without practically anyone using it.

If multiple types are still necessary, consider either using `PropPayload::Vec` or for more descriptive fields use a custom struct in `PropPayload::Any`.

### Rename `PropPayload` and `State` variant `Tup2` to `Pair`

As other tuple variants are now removed, it is more descriptive to rename `Tup2` to `Pair`.

### Rename `PropPayload` and `State` variant `One` to `Single`

With `Tup2` now being renamed to `Pair` and considering the other variants, `Single` aligns better with the naming scheme than `One`.

### Removal of Dataset related value

`Dataset` is practically only required for `tui_realm_stdlib::components::Chart`, and even then does not need to be stored in `Props`, so it can be easily
moved to be carried over `PropPayload::Any`.

### `Component::on` parameter `Event` is now a reference

With 4.0, `Component::on`'s `Event` parameter is now a reference. This allowed us to remove clones in-between that had always been done
but now it is up to the user if a clone is actually necessary.

### `termion` backend / adapter changes

The `termion` backend adapter has been refactored to better fit-in with how `termion` works.

This effectively means that `new` does not exist anymore, but more specific new functions now exist:
- `new_raw`
- `new_alternate_raw`
- `new_mouse_alternate_raw`
- `new_mouse_raw`

Additionally, `TerminalBridge::new_termion` and `init_termion` have been removed, instead use `TerminalBridge::new_init_termion` instead.
