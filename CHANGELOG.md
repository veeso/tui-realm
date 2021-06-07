# Changelog

- [Changelog](#changelog)
  - [0.4.0](#040)
  - [0.3.2](#032)
  - [0.3.1](#031)
  - [0.3.0](#030)
  - [0.2.2](#022)
  - [0.2.1](#021)
  - [0.1.0](#010)

---

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
