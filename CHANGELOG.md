# Changelog

- [Changelog](#changelog)
  - [3.2.0](#320)
  - [3.1.0](#310)
  - [3.0.1](#301)
  - [3.0.0](#300)
  - [2.2.0](#220)
  - [2.1.0](#210)
  - [2.0.3](#203)
  - [2.0.2](#202)
  - [2.0.1](#201)
  - [2.0.0](#200)
  - [1.9.2](#192)
  - [1.9.1](#191)
  - [1.9.0](#190)
  - [1.8.0](#180)
  - [1.7.1](#171)
  - [1.7.0](#170)
  - [1.6.0](#160)
  - [1.5.0](#150)
  - [1.4.2](#142)
  - [1.4.1](#141)
  - [1.4.0](#140)
  - [1.3.0](#130)
  - [1.2.1](#121)
  - [1.2.0](#120)
  - [1.1.2](#112)
  - [1.1.0](#110)
  - [1.0.1](#101)
  - [1.0.0](#100)
  - [0.6.0](#060)
  - [0.5.1](#051)
  - [0.5.0](#050)
  - [0.4.3](#043)
  - [0.4.2](#042)
  - [0.4.1](#041)
  - [0.4.0](#040)
  - [0.3.2](#032)
  - [0.3.1](#031)
  - [0.3.0](#030)
  - [0.2.2](#022)
  - [0.2.1](#021)
  - [0.1.0](#010)

---

## next

Unreleased

- Add `PropPayload::Any` variant using `dyn Any`, allowing arbitrary data via `MockComponent::query` and `MockComponent::attr`.

## 3.2.0

Released on 10/11/2025

- Fix `Application::active` to not change focus if the given id is the same as the current focus.
  - This also fixes `Application::remount` unsetting focus, if the remounted component-id had focus.
- Add `get_mut` function for `Props`.
- Add `as_*_mut` functions for `AttrValue`, `PropPayload` & `PropValue`.

## 3.1.0

Released on 26/08/2025

- Remove `PartialOrd` bound for `UserEvent`.
- Add `Send` bound for `UserEvent` to trait `Poll`, as was already required for adding it to `SyncPort`.
- Remove unnecessary bounds on Input Event Listeners.
- Improve documentation for `PollAsync` and `Poll`.
- Add `PollStrategy::UpToNoWait` which is practially the same as `PollStrategy::UpTo`, just that it does not block again after the first event.
- Add `PollStrategy::BlockCollectUpTo` to block until there is at least one event available, then collect up to `n` events, if available without blocking again.
- Allow sharing `SubClause`s in `Sub` by passing in a instance of `Arc<SubClause>`.

## 3.0.1

Released on 09/06/2025

- Fix `subclause_and_not!` macro, which was creating a `not(AndMany...)` instead of `AndMany(not(...), not(...), ...)`.

## 3.0.0

Released on 21/05/2025

- Add `SubClause::AndMany` for easier adding of many clauses that would otherwise be nested `SubClause::And`.
- Add `SubClause::OrMany` for easier adding of many clauses that would otherwise be nested `SubClause::Or`.
- Change the `subclause_` macros to make use of the new `SubClause::*Many` variants.
- Change the `subclause_` macros to be able to end with a `,` without error.
- Change the `subclause_and_not` to only have one `Not` instead of for every case.
- Remove `+ Sync` bound on `PollAsync` trait and `U`(Event) bound and all related functions.
- Fix bug that `EventListener::stop` did not actually stop any async tasks.
- Add function `EventListenerCfg::async_tick` to switch the ticker from Sync-Port to Async-Ports. Note that `EventListenerCfg::tick_interval` is still necessary to enable any ticker.
- Dont start the Sync-Port worker if there are no sync ports (note that `async_tick(false)`(the default) also counts as a sync port).
- Fix accidental always inclusion of `crossterm` dependency, even if `crossterm` feature was disabled. (since [2.2.0](#220))
- Change `PollAsync::poll` to take `&mut self` instead of `&self`.
- Add async `crossterm` input listener `CrosstermAsyncStream` as a alternative to the sync `CrosstermInputListener`.
- [Issue 94](https://github.com/veeso/tui-realm/issues/94) / [PR 94](https://github.com/veeso/tui-realm/issues/97): improve **Async Ports** to make full use of it being async by running all async ports on the runtime instead of blocking on them like a Sync-port.

  ```rust
  let event_listener = EventListenerCfg::default()
        .crossterm_input_listener(Duration::from_millis(10), 3)
        .with_handle(tokio::runtime::Handle::current())
        .add_async_port(
            Box::new(AsyncPort::new()),
            Duration::from_millis(1000),
            1,
        );
  ```

## 2.2.0

Released on 15/05/2025

- Added new `SubEventClause::Discriminant`, which works as `SubEventClause::User`, but only checks the discriminant of the enum, instead of the whole enum. (e.g. `Foo::Bar(2)` has the same discriminant as `Foo::Bar(20)`, while when using `SubEventClause::User`, it would be different)
- Rust edition `2024`

## 2.1.0

Released on 12/02/2025

- [feat(core::props::AttrValue): add "as_*" function](https://github.com/veeso/tui-realm/pull/91)
- [feat(core::props::Props): add function "get_ref"](https://github.com/veeso/tui-realm/pull/89)
- [Add as_ functions for PropPayload and PropValue](https://github.com/veeso/tui-realm/pull/88)
- [style(core::props::value::PropValue): consistent docs](https://github.com/veeso/tui-realm/pull/87)
- ratatui 0.29

## 2.0.3

Released on 14/10/2024

- Fixed: macros were not usable from external crates since the `tuirealm::` namespace of the recursive macro was not specified

## 2.0.2

Released on 14/10/2024

- Added `subclause_and_not!`

## 2.0.1

Released on 13/10/2024

- Fixed docs not building

## 2.0.0

Released on 13/10/2024

- Dropped support for `tui-rs`. Tui-rs was deprecated a long time ago, so it doesn't really makes sense to keep supporting it.
- Added new methods for `TerminalBridge`
  - `init`: Initialize a terminal with reasonable defaults for most applications.
    - Raw mode is enabled
    - Alternate screen buffer enabled
    - A panic hook is installed that restores the terminal before panicking. Ensure that this method is called after any other panic hooks that may be installed to ensure that the terminal is.
  - `restore`: Restore the terminal to its original state
  - `set_panic_hook`: Sets a panic hook that restores the terminal before panicking.
  - Added `draw` to `TerminalBridge`
- `CmdResult::Custom(&'static str)` changed to `CmdResult::Custom(&'static str, State)`
- Added new `subclause_and!(Id::Foo, Id::Bar, Id::Baz)` and `subclause_or!(Id::Foo, Id::Bar, Id::Baz)` macros.
- Removed `InputListener`. Now use `CrosstermInputListener` or `TermionInputListener`.
- Added Event handling for Mouse Events
  - Added `Mouse` in `SubEventClause`.
- Bump `ratatui` version to `0.28`
- Dont enable `MouseCapture` by default
- Add function `enable_mouse_capture` and `disable_mouse_capture` to `TerminalBridge`
- **Max poll for ports**:
  - Add `Port::set_max_poll` to set the amount a `Port` is polled in a single `Port::should_poll`.
  - Add `EventListenerCfg::port` to add a manually constructed `Port`
  - Previous `EventListenerCfg::port` has been renamed to `EventListenerCfg::add_port`

Huge thanks to [hasezoey](https://github.com/hasezoey) for the contributions.

## 1.9.2

Released on 04/03/2023

- Bump `ratatui` to `0.26`

## 1.9.1

Relesed on 19/10/2023

- Fixed duplicated key events on Windows
- Update dependencies:
  - bitflags 2
  - termion 2

## 1.9.0

Released on 22/08/2023

- Bump `crossterm` to `0.27`
- Added support for `ratatui`
  - to enable `ratatui` set feature `ratatui` (enabled by default)
  - to enable `tui` set feature `tui`.
- Deprecated features `with-crossterm`, `with-termion`

## 1.8.0

Released on 14/08/2022

- Bump `crossterm` to `0.25`
- Bump `tui` to `0.19`
- Added events (supported by crossterm)

    ```rs
    FocusGained,
    /// Window focus lost
    FocusLost,
    /// Clipboard content pasted
    Paste(String),
    ```

- Added new key events (supported by crossterm)

    ```rust
    /// Caps lock pressed
    CapsLock,
    /// Scroll lock pressed
    ScrollLock,
    /// Num lock pressed
    NumLock,
    /// Print screen key
    PrintScreen,
    /// Pause key
    Pause,
    /// Menu key
    Menu,
    /// keypad begin
    KeypadBegin,
    /// Media key
    Media(MediaKeyCode),
    ```

## 1.7.1

Released on 03/08/2022

- Fixed [issue 40](https://github.com/veeso/tui-realm/issues/40)

## 1.7.0

Released on 23/06/2022

- Added `unwrap()` methods to `State` and `StaateValue`
- Added `StateValue::None`

## 1.6.0

Released on 29/04/2022

- Updated `crossterm` to `0.23`
- Updated `tui` to `0.18`
- Fixed build issue on nightly toolchain

## 1.5.0

Released on 06/03/2022

- Updated `tui` to `0.17`
- Added **Injectors**
  - Properties injectors are trait objects, which must implement the `Injector` trait, which can provide some property (defined as a tuple of `Attribute` and `AttrValue`) for components when they're mounted.
  - Read more in [advanced concepts](/docs/en/advanced.md#properties-injectors)

## 1.4.2

Released on 04/01/2022

- Added `focus()` method to `Application` which returns a reference to id of the current active component in the `View`

## 1.4.1

Released on 27/12/2021

- Fixed serialization for key events
- Removed serde for `Event`

## 1.4.0

Released on 24/12/2021

- Added `serialize` feature: once this feature is enabled, the `Serialize` and the `Deserialize` traits will be available for certain entities:
  - `Key`, `KeyEvent` and `KeyModifiers`: this will give you the possibility to use these entities in a serialized data for configuring keybindings.
  - `Event`: implemented in order to provide a Port with commands from an external source.

## 1.3.0

Released on 28/11/2021

- Added `lock_ports()` and `unlock_ports()` to pause event listener.
  - Once `lock_ports()` is called, **Ports** won't be polled as long as `unlock_ports()` is not called.
  - by default ports are **Unlocked**

## 1.2.1

Released on 27/11/2021

- `TextSpan` `From` trait implementation, now accepts `AsRef<str>`

## 1.2.0

Released on 25/11/2021

> Yet, another update 🙄

- Application API:
  - Added `lock_subs()` and `unlock_subs()` methods.
    - Once `lock_subs()` is called, events won't be anymore propagated to subscriptions as long as `unlock_subs()` is not called.
    - by default events **will be propagated** to subs.
- Sub Clause:
  - Added `Id` to `HasAttr` and `HasState` sub clauses.
  - Added new `IsMounted(Id)` sub clause

## 1.1.2

Released on 23/11/2021

> ❗ There's no 1.1.1 version. Since I don't like it as a version number, I decided to skip it

- Application API changes:
  - Removed `sanitize` since `View` is no more accessible
  - Added `umount_all` method which umounts all components from `View` and active subscriptions

## 1.1.0

Released on 21/11/2021

- `tick()` will now return a `Vec<Msg>`. There's no need to pass an `Update` trait anymore
  - The reasons behind this is that it was too annoying to handle the model in a separate structure which could not render the Ui.
- Exposed `PollStrategy` at root level

## 1.0.1

Released on 20/11/2021

- Improved performance for crossterm listener

## 1.0.0

Released on 13/11/2021

- New API; view docs

## 0.6.0

Released on 03/08/2021

- ❗ Compatibility with tui `0.16` and crossterm `0.20` ❗
  - You can now set the block title alignment
    - Added `title` to `Props`
    - `BlockTitle` type in `Props`, which is made up of `text` and `alignment`. Use this instead of setting title in `own` map
  - 🔴 A really bad new in `Msg` matching 😭

      in crossterm `0.20` to solve an issue they removed the `#[derive(Eq, PartialEq)]` from `KeyEvent`.
      This has now caused an issue when matching against `OnKey` events:

      ```txt
      error: to use a constant of type `KeyEvent` in a pattern, `KeyEvent` must be annotated with `#[derive(PartialEq, Eq)]`
      ```

      To solve this issue you must from now on use a guard match to match keys:

      ```rust
      fn update(model: &mut Model, view: &mut View, msg: Option<(String, Msg)>) -> Option<(String, Msg)> {
          let ref_msg: Option<(&str, &Msg)> = msg.as_ref().map(|(s, msg)| (s.as_str(), msg));
          match ref_msg {
              None => None, // Exit after None
              Some(msg) => match msg {
                  (COMPONENT_COUNTER1, key) if key == &MSG_KEY_TAB => {
                      view.active(COMPONENT_COUNTER2);
                      None
                  }
                  (COMPONENT_COUNTER2, key) if key == &MSG_KEY_TAB => {
                      view.active(COMPONENT_COUNTER1);
                      None
                  }
                  (_, key) if key == &MSG_KEY_ESC => {
                      // Quit on esc
                      model.quit();
                      None
                  }
                  _ => None,
              },
          }
      }
      ```

## 0.5.1

Released on 31/07/2021

- Bugfix:
  - Expose `get_data` from `Dataset`

## 0.5.0

Released on 31/07/2021

- New **PropValue** values:
  - `Alignment`
  - `Dataset`
  - `Shape`
  - `Style`
  - `Table`
  - `TextSpan`
- Added `unwrap_{type}()` helpers for `PropPayload` and `PropValue`
- ❗ Breaking changes ❗
  - Removed `Color` from `PropValue`, use `palette` instead ❗
  - ❗ Removed `TextParts` from `Props`, use `own` properties instead ❗
  - ❗ Removed `TextSpanBuilder`, you can just use the same methods on `TextSpan` when creating it ❗
  - ❗ Renamed `Canvas` to `Frame` ❗
  - ❗ Moved standard library to [tui-realm-stdlib](https://crates.io/crates/tui-realm-stdlib) ❗
  - ❗ Removed `with-components` feature ❗

## 0.4.3

Released on 23/06/2021

- Fixed TextArea not scrolling properly
- Added `with_highlight_str` and `with_max_scroll_step` to TextArea

## 0.4.2

Releasaed on 11/06/2021

- Hotfix for 0.4.1: preserve styles on scroll table; `with_highlight_color` method.

## 0.4.1

Released on 11/06/2021

- Fixed scrolltable not handling focus
- Added `with_highlight_str` and `with_max_scroll_step` to scrolltable

## 0.4.0

Released on 07/06/2021

- Another **Prop API Update**
  - Removed `input_len` and `input_type` from properties. Use `own` instead with new `PropValue`
  - Added `Color` and `InputType` to `PropValue`
  - Removed `value` from `Props`
  - Added `own`: key-value storage (`HashMap<&'static str, PropPayload>`) to store any value into properties.
- Dependencies:
  - `textwrap` 0.14.0

## 0.3.2

Released on 04/06/2021

- Updated `Linked` in `PropPayload` and `Payload` with a `LinkedList`

## 0.3.1

Released on 02/06/2021

- Fixed input cursor for UTF8 (multi-bytes characters) ([issue 5](https://github.com/veeso/tui-realm/issues/5))
- Added `Update` trait to ease update implementation

## 0.3.0

Released on 15/05/2021

- Changed `PropValue` API to be similiar to the `Msg` API. Now there are both `PropPayload` and `PropValue` as happens with `Payload` and `Value`
- Fixed index behaviour for checkbox and radio on update

## 0.2.2

Released on 03/05/2021

- Bumped `tui-rs` to `0.15.0`

## 0.2.1

Released on 02/05/2021

- Updated Payload API with `Value`

## 0.1.0

Released on 20/04/2021

- First release
