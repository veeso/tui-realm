# Changelog

- [Changelog](#changelog)
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
