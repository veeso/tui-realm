# Migrating from tui-realm 3.x

- [Migrating from tui-realm 3.x](#migrating-from-tui-realm-3x)
  - [Introduction](#introduction)

---

## Introduction

This document is a work in progress until 4.0 is released, listing the essential changes as they are introduced.

### Replaced `TextSpan` with ratatui equivalent

The previous `tuirealm::props::TextSpan` has been replaced with `ratatui::text::{Span, Line, Text}`.

Because of the new types, new `AttrValue` and `PropValue` variants have been introduced: `TextSpan`, `TextLine` and `Text`.
