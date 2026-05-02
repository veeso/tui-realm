## 4.1.0

Released on 2026-05-02

### Features

- **tuirealm:** add `Attribute::HighlightStyleUnfocused`
- **stdlib:** handle `Attribute::HighlightStyleUnfocused` in `CommonHighlight`
- **stdlib:** change components to allow `HighlightStyleUnfocused` properly
- **treeview:** add `highlight_style_inactive` to set highlight style when unfocused

### Bug Fixes

- **stdlib:** fix `Table` not respecting row's line style

## 4.0.0

Released on 2026-04-18

### ⚠ Breaking Changes

- The MSRV has changed to be 1.88
- **tuirealm:** remove Attribute Value `TextSpan`(String), introduce `TextSpan`, `TextLine` and `Text`
  > `TextSpan` has been replaced with `TextSpan(Span)`, `TextLine(Line)` and `Text(Text)` from ratatui instead of a single String.
- **tuirealm:** replace title tuple with struct
  > Titles are now a `Title` struct instead of a tuple of `(String, Alignment)`. It also uses ratatui's `Line` now.
- **tuirealm:** remove top-level reexports
  > Only allow module-level imports, to remove import confusion and potentially different names. Also helps to clarify what each type belong to.
- **tuirealm:** change `Props` to align with common collection expectations
  > For example `get` now returns a reference instead of a clone. This practically replaces the old `get` with the old `get_ref`.
- **tuirealm:** remove `PropPayload` & State "Tup3" and "Tup4" variants
  > As they were bloating the `PropPayload` size and were practically unused.
- **tuirealm:** rename `PropPayload` & State "One" to "Single" and "Tup2" to "Pair"
  > As now `Tup3` and `Tup4` were gone, `Single` and `Pair` better represent the variants on a glance.
- **tuirealm:** change `Component::on` parameter `Event` to be a reference
  > `Component::on` parameter `Event` is now a reference. This allows for less cloning.
- **tuirealm:** rename `Component`(old) to `AppComponent`, `MockComponent` to `Component`
  > The relationship between `Component`(old) and `MockComponent` were always somewhat confusing, especially because `MockComponent` was the main functionality.
- **tuirealm:** change `Component::query` to return `QueryResult` (mainly making use of `AttrValueRef`)
  > This cannot just be `&AttrValue` as that would require storing attributes directly as `AttrValue`, which would make things like `CommonProps` infeasable or very annoying.
- **tuirealm:**: change Attribtue `Alignment` to be split into `AlignmentHorizontal` and `AlignmentVertical`
  > To align with ratatui's split in 0.30
- **tuirealm:** rename Attribute `FocusStyle` to `UnfocusedStyle`
  > To represent what it actually always meant.
- **tuirealm:** rename Attribute `HighlightColor` to `HighlightStyle`
  > To align with current usage.
- **tuirealm:** move Application poll timeout and configuration to `PollStrategy` variants
  > As not all strategies need the same options. It also makes it more clear on a glance where poll actually happens and allows different timings when necessary.
- **tuirealm:** change `Port` return values to be `PortResult`(`Result<T, PortError>`)
  > This allows better error propagation with actual messages, making it clearer what the issue is. Also allows errors to be intermittend or permanent.
- **tuirealm:** change `Application::poll` return type to be `PollError`(via `ApplicationError` wrapper)
  > Better explain what failed.
- **tuirealm:** remove `TerminalBridge`
  > Ever since other backend other than `crossterm` were supported, `TerminalBridge` did not work correctly. All functionality that may have been used via `TerminalBridge` (like panic handling) has been moved to be on the backends themself.
- **tuirealm:** replace `PollStrategy::UpTo` with `PollStrategy::UpToNoWait`
- **tuirealm:** remove `Update` trait
  > The trait was never required in any bounds, so it has been removed. This allows the `update` function to be customized.
- **tuirealm:** rename `CmdResult` variant `None` to `NoChange`
  > To help explain "on a glance" what this one dones.
- **stdlib:** remove `utils::get_title_or_center`
  > `utils::get_title_or_center` is now obsolote thanks to the `Title` struct having a default. It was also causing titles to be present when there shouldnt be.
- **stdlib:** adjust all components to align with the new & changed attributes
- **stdlib:** change drawing to be consistent across all components
  > Now all components handle common things like stlye and blocks practically the same, thanks to `CommonProps`.
- **stdlib:** adjust all component functions and attributes to use better types
  > This effectively means that things like `highlight_str` now take `Line` instead of `String` and similar function.
- **stdlib:** rename component `ProgressBar` to `Gauge`
  > To align with ratatui naming, making it easier to differeniate between `LineGauge` and `Gauge` (especially because `LineGuage` was a better fit for most progress bars).
- **stdlib:** rename `highlighted_*` functions to `highlight_*`
  > To align with common naming convention.
- **textarea:** remove feature and support for clipboard pasting
  > The crate used was outdated and unmaintained. Clipboard pasting should be done in the application / event handler anyway
- **textarea:** add function `paste` to add more than one characters at a time
  > This also is only one undo/redo action.

### Added

- **tuirealm:** add Application `get_component(_mut)` function
  > Now individual components can be accessed to be modified directly via rust's `Any`.
- **tuirealm:** add `termwiz` as a backend
  > To match what ratatui supports.
- **tuirealm:** allow to set custom backend terminal options on creation
  > Now things like `Inline` viewports are properly supported.
- **tuirealm:** add `TestTerminal` backend for drawing & sending events for testing without actual backing terminal
- **tuirealm:** add `testing::render_to_string` for easy testing of component draw output
  > It is recommend to be used together with something like [`insta`](https://crates.io/crates/insta)
- **tuirealm:** add Attribute `AlignmentHorizontal` and `AlignmentVertical`
  > Mainly for use with Attribute Value with the same names.
- **tuirealm:** add Attribute `AlwaysActive`
  > This allows components to always draw as if focused. Example being Headers that are only movable through keyboard shortcuts, or static `Paragraph`'s.
- **tuirealm:** add Attribute Value `Marker`
  > Which contains ratatui's `Marker` type. For example used in Canvas.
- **tuirealm:** add `CmdResult` variant `Visual`
  > To indicate something changed that needs a redraw, but not actualy observable `State` change.
- **tuirealm:** add missing `PropValue` `*_color` and `*_table` function
- **tuirealm:** add `AttrValueRef`, `PropPayloadRef` and `PropValueRef`
  > Those are also now used for `Component::query` return types.
- **tuirealm:** change `Event`'s `as_keyboard`, `as_mouse` and `as_user` function to be public
  > To allow easy casting of events.
- **stdlib:** allow setting custom Filled and Unfilled line styles for `LineGauge` via `::line_style`
- **stdlib:** add option to force manual stepping for `Spinner` component.
  > By default the `Spinner` always takes a step when drawn, which can be inconsistent if there is a variance in amount of draws causing it to speed up or slow down.
- **stdlib:** due to the commonization of common draw logic, all components now have `style` and `modifiers`

### Changed

- **tuirealm:** update to ratatui 0.30
- **tuirealm:** remove `PropBoundExt` trait
  > This has become unnecessary and hence was removed.
- **tuirealm:** dont enable `Mouse` mode when calling `*_alternate_screen` in `crossterm` backend
  > This should not have been the default in the first place. `Mouse` mode can still be changed via `*_mouse_capture`.
- **stdlib:** make use of `CmdResult::Invalid` instead of `CmdResult::None`

### Fixed

- A bunch of issues with inconsistent drawing style

### Performance

- **stdlib:** common properties are now stored within `CommonProps`
  > Instead of in `Props`, removing the need to default or panic; also removes some indirection.

### Documentation

- Documentation like `get-stated` and `advanced` has been updated to be compatible with tuirelam 4.0
  > Guides had been very outdated even in 3.x

### Miscellaneous

- tuirealm related crates have been merged into a monorepo
  > allowing for easier development of `derive`, `tuirealm` and `stdlib` (among some other extra first-party crates)

---

Additionally, also read [the 4.0 migration guide](crates/tuirealm/docs/en/migrating-4.0.md).
