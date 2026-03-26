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

### Attribute `Alignment` has been split into horizontal and vertical

To align with ratatui `0.30` change of renaming `Alignment` to `HorizontalAlignment`, `tui-realm` renames the old Attribute `Alignment` to `AlignmentHorizontal`
and adds a new attribute named `AlignmentVertical` for `VerticalAlignment`.

### Removal of Dataset related value

`Dataset` is practically only required for `tui_realm_stdlib::components::Chart`, and even then does not need to be stored in `Props`, so it can be easily
moved to be carried over `PropPayload::Any`.

### Removal of `Props::get`(old) and `Props::get_or`

`Props::get` has been removed in favor of `Props::get_ref`, to align with STD types like `Vec::get`'s return type.
This also makes cloning explicit to the user.

`Props::get_or` has been removed as it relied on `Props::get` and couldnt reasonably be converted to use `Props::get_ref`.

### Rename of `Props::get_ref` to `Props::get`

Due to `Props::get`(old) having been removed and to better align with STD types like `Vec::get`, `::get_ref` has been renamed to just `::get`.

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

### Removal of `PropBoundExt`

The function `as_any` and `as_any_mut` are now directly implemented on `dyn PropBound`, not requiring another trait to be imported.

### Change of `Poll` return types

With 4.0, `Poll::poll` and `PollAsync::poll` return types changed from `ListenerResult` to `PortResult`.
Due to this change the new `PortError` allows passing more context on what happend. It also support specifying if the Error is Intermittent(should poll again) or Permanent(should stop the port).

This is due to `ListenerError`'s variants being meant to be mostly internal.
`ListenerResult` has also been changed to be non-public.

### Separate Error types for `poll`

In addition to the [`*Poll::poll` return type changes](#change-of-poll-return-types), `ApplicationError` got a specific `Poll` variant over it being combined with Listener start / stop errors.

### Change `PollStrategy::UpToNoWait` to be `PollStrategy::UpTo`

The old `PollStrategy::UpTo` has been removed and replaced with previously known `PollStrategy::UpToNoWait`.

This has been done as the behavior of "wait TIMEOUT for each N, if there was a event available" is not something that is useful to event-driven tui applications.
(With the new `UpTo`, previously known as `UpToNoWait` will "wait TIMEOUT once, collect event, afterwards collect up to N-1 amount (without blocking again), as long as there are events")

### Poll timeout moved to be in `PollStrategy`

The timeout that was previously stored on the `EventListener(Cfg|Builder)` has been moved to be stored on the `PollStrategy` instead.
This has been done due to some strategies not using a timeout alltogether, and some have different meanings for the duration specified.

### Removal of `TerminalBridge`

The wrapper `TerminalBridge` has been removed as it did not provide any benefit over using the backends directly, or using the trait directly.

Panic handlers and restore have been implemented on the backends themself now, where necessary & possible.
For individual notes, see `Restore` and `On Panic` sections on the backends themself.

### Removal of the `Update` trait

To be consistent with other "external" functions like `view`, it has been decided to remove the `Update` trait as it was never actually required as a bounds anywhere.

This makes it consistent with other functions like `view` which did not have a trait previously.
This allows for customization of how the `update` function is called, for example if you dont ever returns a message for recursive processing, it can now be omitted.

Migration is as simple as changing `impl Update for Model` to `impl Model` and potentially changing the visibility to `pub fn`.

### Export cleanup: module-qualified imports required

Root-level re-exports have been removed from `tuirealm` and `tui-realm-stdlib`. You must now import types through their module paths:

```rust
// Before (3.x)
use tuirealm::{Application, Component, MockComponent, Event, State, Frame};

// After (4.0)
use tuirealm::application::Application;
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::Event;
use tuirealm::state::State;
use tuirealm::ratatui::Frame;
```

For `tui-realm-stdlib`, component types are now under `components`:

```rust
// Before
use tui_realm_stdlib::Input;

// After
use tui_realm_stdlib::components::Input;
```

### `MockComponent` renamed to `Component`

The `Component` trait (event handling) has been renamed to `AppComponent`.
The `MockComponent` trait (rendering, state, props) has been renamed to `Component`.
The derive macro `#[derive(MockComponent)]` is now `#[derive(Component)]` to match the new `Component` name.

```rust
// Before (3.x)
use tuirealm::{MockComponent, Component};

#[derive(MockComponent)]
struct MyWidget { component: Input }

impl Component<Msg, UserEvent> for MyWidget {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> { ... }
}

// After (4.0)
use tuirealm::component::{AppComponent, Component};  // traits

#[derive(Component)]
struct MyWidget { component: Input }

impl AppComponent<Msg, UserEvent> for MyWidget {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Msg> { ... }
}
```

### Change `Component::query` to return Borrowed content

`Component::query` has been changed to allow for and prefer borrowed content, but still allow owned content to be returned.
This allowed the consumer to decide when a clone is actually necessary, for practically anything other than `PropPayload::Any`.
