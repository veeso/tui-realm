# Changelog

- [Changelog](#changelog)
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

## 0.5.0

Released on ??

- New components:
  - Added `BarChart` component
  - Added `Canvas` component
  - Added `Chart` component
  - Added `Select` component
  - Added `Sparkline` component
  - Added `Table` component
- Component changes:
  - **Label**
    - Label now supports text alignment `with_text_alignment()`
  - **Paragraph**:
    - paragraph will now use `TuiParagraph` to render instead of `List`.
    - paragraph now supports text alignment `with_text_alignment()`
    - paragraph now supports wrap with trim `with_trim()`
  - **Span**
    - Span now supports text alignment `with_text_alignment()`
- `PropValue` can now also be:
  - `Alignment` (`props::Alignment`)
  - `Dataset`
  - `Shape`
  - `Style`
- ❗ Breaking changes ❗
  - ❗ Renamed `Canvas` to `Frame` ❗
  - ❗ Renamed `Table` to `List` ❗
  - ❗ Removed `ScrollTable`; Use `List` with `scrollable(true)` instead ❗

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
