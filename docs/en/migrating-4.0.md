# Migrating from tui-realm 3.x

- [Migrating from tui-realm 3.x](#migrating-from-tui-realm-3x)
  - [Introduction](#introduction)

---

## Introduction

This document is a work in progress until 4.0 is released, listing the essential changes as they are introduced.

### Replaced `TextSpan` with ratatui equivalent

The previous `tuirealm::props::TextSpan` has been replaced with `ratatui::text::{Span, Line, Text}`.

Because of the new types, new `AttrValue` and `PropValue` variants have been introduced: `TextSpan`, `TextLine` and `Text`.

### Replace `(String, Alignment)` Titles with proper struct

The previous `(String, Alignment)` Tuple has been replaced with a more feature-full `Title` struct.

Due to the title now using `Line` under the hood, it is now possible to style individual characters in the title.

### Removal of `PropPayload::Tup*` variants

The tuple variants of `PropPayload` have been removed as they were blowing up the size of `PropPayload` in general.

If multiple types are still necessary, consider either using `PropPayload::Vec` or for more descriptive fields use a custom struct in `PropPayload::Any`.

### `Component::on` parameter `Event` is now a reference

With 4.0, `Component::on`'s `Event` parameter is now a reference. This allowed us to remove clones in-between that had always been done
but now it is up to the user if a clone is actually necessary.
