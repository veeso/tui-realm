# tui-realm-stdlib

<p align="center">
  <img src="docs/images/tui-realm.svg" width="256" height="256" />
</p>

[![License: MIT](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/licenses/MIT) [![Stars](https://img.shields.io/github/stars/veeso/tui-realm-stdlib.svg)](https://github.com/veeso/tui-realm-stdlib) [![Downloads](https://img.shields.io/crates/d/tui-realm-stdlib.svg)](https://crates.io/crates/tui-realm-stdlib) [![Crates.io](https://img.shields.io/badge/crates.io-v0.6.1-orange.svg)](https://crates.io/crates/tui-realm-stdlib) [![Docs](https://docs.rs/tui-realm-stdlib/badge.svg)](https://docs.rs/tui-realm-stdlib)  

[![Build](https://github.com/veeso/tui-realm-stdlib/workflows/Linux/badge.svg)](https://github.com/veeso/tui-realm-stdlib/actions) [![Build](https://github.com/veeso/tui-realm-stdlib/workflows/MacOS/badge.svg)](https://github.com/veeso/tui-realm-stdlib/actions) [![Build](https://github.com/veeso/tui-realm-stdlib/workflows/Windows/badge.svg)](https://github.com/veeso/tui-realm-stdlib/actions) [![Coverage Status](https://coveralls.io/repos/github/veeso/tui-realm-stdlib/badge.svg?branch=main)](https://coveralls.io/github/veeso/tui-realm-stdlib?branch=main)

Developed by Christian Visintin  
Current version: 1.0.0 (FIXME: 03/08/2021)

---

- [tui-realm-stdlib](#tui-realm-stdlib)
  - [About tui-realm-stdlib 👑](#about-tui-realm-stdlib-)
  - [Get started 🏁](#get-started-)
    - [Add tui-realm to your Cargo.toml 🦀](#add-tui-realm-to-your-cargotoml-)
  - [Examples](#examples)
  - [Support the developer ☕](#support-the-developer-)
  - [Components 🎨](#components-)
    - [BarChart](#barchart)
    - [Canvas](#canvas)
    - [Chart](#chart)
    - [Checkbox](#checkbox)
    - [Input](#input)
    - [Label](#label)
    - [Line gauge](#line-gauge)
    - [List](#list)
    - [Paragraph](#paragraph)
    - [Progress bar](#progress-bar)
    - [Radio](#radio)
    - [Select](#select)
    - [Span](#span)
    - [Sparkline](#sparkline)
    - [Table](#table)
    - [Textarea](#textarea)
    - [Utilities](#utilities)
  - [Documentation 📚](#documentation-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [License 📃](#license-)

---

## About tui-realm-stdlib 👑

tui-realm-stdlib is the standard component library for [tui-realm](https://github.com/veeso/tui-realm).

---

## Get started 🏁

⚠ Warning: tui-realm works only with **crossterm** as backend ⚠  

### Add tui-realm to your Cargo.toml 🦀

```toml
tui-realm-stdlib = "0.6.1"
```

Since this library requires `crossterm` too, you'll also need to add it to your Cargo.toml

```toml
crossterm = "0.20.0"
```

Latest version of tui-realm-stdlib requires **tui-realm 0.6.0**

```toml
tuirealm = "0.6.0"
```

## Examples

Want to have a demo of components? Try with examples

```sh
cargo run --example component-name
```

such as

```sh
cargo run --example select
```

---

## Support the developer ☕

If you like tui-realm and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![Buy-me-a-coffee](https://img.shields.io/badge/-buy_me_a%C2%A0coffee-gray?style=for-the-badge&logo=buy-me-a-coffee)](https://www.buymeacoffee.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Components 🎨

> ❗ Props with name preceeded by `$` are constants that can be found in `tui_realm_stdlib::props::`

### BarChart

A chart with bars. The bar chart can work both in "active" and "disabled" mode.

When in disabled mode, the chart won't be interactive, so you won't be able to move through data using keys.
If you have more data than the maximum amount of bars that can be displayed, you'll have to update data to display the remaining entries

While in active mode (default) you can put as many entries as you wish. You can move with arrows and END/HOME keys

**Commands**:

| Cmd               | CmdResult       | Behaviour                                      |
|-------------------|-----------------|------------------------------------------------|
| `Move(Right)`     | `None`          | Move the cursor right                          |
| `Move(Left)`      | `None`          | Move the cursor left                           |
| `GoTo(End)`       | `None`          | Move "cursor" to the end of chart              |
| `GoTo(Begin)`     | `None`          | Move "cursor" to the first entry of the chart  |

**State**: `None`.

**Properties**:

- `Background(Color)`: background color
- `Custom($BAR_CHART_BARS_GAP, Size)`: sets gap for bars
- `Custom($BAR_CHART_BARS_STYLE, Style)`: sets style for bars
- `Custom($BAR_CHART_LABEL_STYLE, Style)`: Sets the style for data labels
- `Custom($BAR_CHART_MAX_BARS, Length)`: maximum amount of bars to display. If not provided, will be the maximum allowed by the area width.
- `Custom($BAR_CHART_VALUES_STYLE, Style)`: Sets style for values
- `Dataset(Payload(LinkedList(Tup2(String, U64))))`: set data for chart. Is a vec of tuple of labels and u64
- `Disabled(Flag)`: Sets the chart in disabled mode
- `Foreground(Color)`: foreground color
- `Title(Title)`: title for chart
- `Width(Size)`: Define bar width

### Canvas

Canvas component can be used to draw shapes on the terminal.

**Commands**: None

**State**: None

**Properties**:

- `Background(Color)`: background color
- `Borders(Borders)`: set borders properties for component
- `Custom($CANVAS_X_BOUNDS, Payload(Tup2(F64, F64)))`: Something regarding the viewport; view tui-rs documentation (which doesn't exist actually). I don't know how it works actually.
- `Custom($CANVAS_Y_BOUNDS, Payload(Tup2(F64, F64)))`: Something regarding the viewport; view tui-rs documentation (which doesn't exist actually). I don't know how it works actually.
- `Foreground(Color)`: foreground color
- `Shape(Payload(Vec(Shape)))`: set shapes for canvas.
- `Title(Title)`: title for chart

### Chart

A chart displayed on a cartesian axis system. Can work both in "active" and "disabled" mode.

When in disabled mode, the chart won't be interactive, so you won't be able to move through data using keys.
If you have more data than the maximum amount of bars that can be displayed, you'll have to update data to display the remaining entries

While in active mode (default) you can put as many entries as you wish. You can move with arrows and END/HOME keys

**Commands**:

| Cmd               | CmdResult       | Behaviour                                      |
|-------------------|-----------------|------------------------------------------------|
| `GoTo(Begin)`     | `None`          | Move "cursor" to the first entry of the chart  |
| `GoTo(End)`       | `None`          | Move "cursor" to the end of chart              |
| `Move(Left)`      | `None`          | Move the cursor left                           |
| `Move(Right)`     | `None`          | Move the cursor right                          |

**State**: `None`.

**Properties**:

- `Background(Color)`: background color
- `Borders(Borders)`: set borders properties for component
- `Custom($CHART_X_BOUNDS, Payload(Tup2(F64, F64)))`: Something regarding the viewport; view tui-rs documentation (which doesn't exist actually). I don't know how it works actually.
- `Custom($CHART_X_LABELS, Payload(Vec(String)))`: Set labels for x axis
- `Custom($CHART_X_STYLE, Style)`: Set style for x axis
- `Custom($CHART_X_TITLE, String)`: Set title for x axis
- `Custom($CHART_Y_BOUNDS, Payload(Tup2(F64, F64)))`: Something regarding the viewport; view tui-rs documentation (which doesn't exist actually). I don't know how it works actually.
- `Custom($CHART_Y_LABELS, Payload(Vec(String)))`: Set labels for y axis
- `Custom($CHART_Y_STYLE, Style)`: Set style for x axis
- `Custom($CHART_Y_TITLE, String)`: Set title for x axis
- `Dataset(Payload(Vec(Dataset)))`: set data for chart. Is a vec of `Dataset`
- `Disabled(Flag)`: Sets the chart in disabled mode
- `FocusStyle(Style)`: inactive style
- `Foreground(Color)`: foreground color
- `Title(Title)`: title for chart

### Checkbox

A checkbox group. Provides the possibility to select between multiple options, when `get_state` is invoked returns a vector of index; each index represents the index of the item selected.

**Commands**:

| Cmd           | CmdResult       | Behaviour                                      |
|---------------|-----------------|------------------------------------------------|
| `Move(Left)`  | `None`          | Decrement the selected choice index by 1       |
| `Move(Right)` | `None`          | Increment the selected choice index by 1       |
| `Submit`      | `Submit`        | Just returns the selection                     |
| `Toggle`      | `Changed`       | Check or uncheck the item at the current index |

**State**: the state returned is `Vec(Usize)` containing the indexes of the selected item in the checkbox group.

**Properties**:

- `Background(Color)`: color used when item is at current index
- `Borders(Borders)`: set borders properties for component
- `Content(Payload(Vec(String)))`: set checkbox options
- `FocusStyle(Style)`: inactive style
- `Foreground(Color)`: foreground color
- `Rewind(Flag)`: if true, when moving beyond limits of component, the choice will be rewinded, instead of remaining the same
- `Title(Title)`: set checkbox title
- `Value(Payload(Vec(Usize)))`: set selected by-default items by their index

---

### Input

An input text. Provides the possiblity to input a text with the possibility to set the input length and the input type (number, password, text, ...). It also allows to use arrows to move the cursor inside of the input box. When `state` is invoked, returns the current content of the input as String or as Number based on the current input type.

**Commands**:

| Command              | Result            | Behaviour                                            |
|----------------------|-------------------|------------------------------------------------------|
| `Cancel`             | `Changed | None`  | Delete next character in input                       |
| `Delete`             | `Changed | None`  | Remove previous character in input                   |
| `GoTo(Begin)`        | `None`            | Move cursor at the end of input                      |
| `GoTo(End)`          | `None`            | Move cursor at the beginning of input                |
| `Move(Left)`         | `None`            | Move cursor left                                     |
| `Move(Right)`        | `None`            | Move cursor right                                    |
| `Submit`             | `Submit | None`   | Submit input                                         |
| `Type(ch)`           | `Changed | None`  | Push character, if allowed by method, into the input |

**State**: the state returned is a `State::One(StateValue::String)` if the input is valid, `State::None` otherwise.

**Properties**:

- `Background(Color)`: background color
- `Borders(Borders)`: set borders properties for component
- `Display(Flag)`: if False component is hidden
- `FocusStyle(Style)`: style for when component is not active
- `Foreground(Color)`: foreground color
- `InputLength(Length)`: set the maximum input length
- `InputType(InputType)`: set the input type
- `Title(Title)`: set input box title
- `Value(String)`: set value for the input

---

### Label

A text label. Provides the possibility to display a simple text, with the possibility to set modifiers and colors.

**Commands**: None

**State**: None

**Properties**:

- `Alignment(Alignment)`: set text alignment
- `Background(Color)`: set background color
- `Foreground(Color)`: set foreground color
- `Text(String)`: set label text
- `TextProps(TextModifiers)`: set text modifiers

---

### Line gauge

A line indicating progress. The progress bar provides the possibility to show the current progress and to show a label above it.

**Commands**: None

**State**: None

**Properties**:

- `Background(Color)`: set background color
- `Borders(Borders)`: set border properties
- `Foreground(Color)`: set progress bar color
- `Text(String)`: set progress bar label
- `TextProps(TextModifiers)`: set text modifiers
- `Title(Title)`: set progress bar title
- `Value(Payload(One(F64)))`: set progress. **WARNING**: must be in range 0.0,1.0
- `Style(Payload(One(U8)))` defines line gauge style. Allowed values are:
  - `$LINE_GAUGE_STYLE_NORMAL`
  - `$LINE_GAUGE_STYLE_DOUBLE`
  - `$LINE_GAUGE_STYLE_ROUND`
  - `$LINE_GAUGE_STYLE_THICK`

---

### List

a list of rows with the possibility to scroll text with arrows. In order to scroll, the component must be active.

**Commands**:

Events will be reported only when set as `Scrollable`

| Cmd              | CmdResult | Behaviour               |
|------------------|---------|---------------------------|
| `GoTo(Begin)`    | `OnKey` | Move cursor to first item |
| `GoTo(End)`      | `OnKey` | Move cursor to last item  |
| `Move(Down)`     | `OnKey` | Move cursor down          |
| `Move(Up)`       | `OnKey` | Move cursor up            |
| `Scroll(Down)`   | `OnKey` | Move cursor down by 8     |
| `Scroll(Up)`     | `OnKey` | Move cursor up by 8       |

**State**: If `scrollable`, returns current list index as `State(One(Usize))`, otherwise None

**Properties**:

- `Background(Color)`: set background color
- `Borders(Borders)`: set border properties
- `Content(Table)`: set entries as a table
- `FocusStyle(Style)`: inactive style
- `Foreground(Color)`: set foreground color
- `Rewind(Flag)`: rewind list if boundaries are reached
- `Scroll(Flag)`: set whether list is scrollable (interactive)
- `ScrollStep(Length)`: set scroll step
- `TextProps(TextModifiers)`: set text modifiers
- `Title(Title)`: set block title

---

### Paragraph

A text paragraph. Like in HTML this has to be considered a block element, and supports multi-line texts with different styles. The text is automatically wrapped.

**Commands**: None

**State**: None

**Properties**:

- `Alignment(Alignment)`: set text alignment
- `Background(Color)`: set background color
- `Borders(Borders)`: set border properties
- `Foreground(Color)`: set foreground color
- `HighlightedColor(Color)`: a different color for highlighted entry; `foreground` otherwise
- `HighlightedStr(String)`: cursor for highlighted entry in selection tab.
- `Text(Payload(Vec(TextSpan)))`: set paragraph text
- `TextProps(TextModifiers)`: set text modifiers
- `TextWrap(Flag)`: select whether to trim rows when wrapping
- `Title(Title)` set paragraph title

---

### Progress bar

A progress bar or basically a gauge. The progress bar provides the possibility to show the current progress and to show a label above it.

**Commands**: None

**State**: None

**Properties**:

- `Background(Color)`: set background color
- `Borders(Borders)`: set border properties
- `Foreground(Color)`: set progress bar color
- `Text(String)`: set progress bar label
- `TextProps(TextModifiers)`: set text modifiers
- `Title(Title)`: set progress bar title
- `Value(Payload(One(F64)))`: set progress. **WARNING**: must be in range 0.0,1.0

---

### Radio

A radio button group. Provides the possibility to select a single option in a group of options. When `get_state` is invoked returns the index of the selected option as Unsigned.

**Commands**:

| Cmd                  | CmdResult       | Behaviour                                        |
|----------------------|-----------------|--------------------------------------------------|
| `Move(Left)`         | `Changed`       | Change the selected option to current item index |
| `Move(Right)`        | `Changed`       | Change the selected option to current item index |
| `Submit`             | `Submit`        | Just returns the index of the selected item      |

**State**: the state returned is `One(Usize)` containing the index of the selected item in the radio group.

**Properties**:

- `Background(Color)`: color used when item is at current index
- `Borders(Borders)`: set borders properties for component
- `Content(Payload(Vec(String)))`: set radio options
- `FocusStyle(Style)`: inactive style
- `Foreground(Color)`: foreground color
- `Rewind(Flag)`: if true, when moving beyond limits of component, the choice will be rewinded, instead of remaining the same
- `Title(Title)`: set radio title
- `Value(Payload(One(Usize)))`: set default selected item by its index

---

### Select

A select like in HTML. Provides the possibility to select a single option in a group of options. When `state` is invoked returns the index of the selected option as Unsigned, but only if the selection tab is closed. Returns `State::None` otherwise. The tab can be opened with `Cmd::Submit`; once opened you can move with arrows to select the entry. To close the form, you need to press `Cmd::Submit` again. Once the tab is closed, a `CmdResult::Submit` is raised with the selected index.
If the component loses focus, the selection tab is automatically closed
This component should have a variable size in the layout to be displayed properly. Please view the example: `examples/select.rs`.

**Commands**:

| Command      | Result             | Behaviour                                                      |
|--------------|--------------------|----------------------------------------------------------------|
| `Move(Down)` | `Changed` | `None` | Move select down, if tab is open                               |
| `Move(Up)`   | `Changed` | `None` | Move select up, if tab is open                                 |
| `Submit`     | `Submit` | `None`  | Open or close the select tab; Returns state if tab gets closed |

**State**: the state returned is `One(Usize)` containing the index of the selected item in the radio group.

**Properties**:

- `Background(Color)`: background color
- `Borders(Borders)`: set borders properties for component
- `Content(Payload(Vec(String)))`: set select options
- `FocusStyle(Style)`: inactive style
- `Foreground(Color)`: foreground color
- `HighlightedColor(Color)`: a different color for highlighted entry; `foreground` otherwise
- `HighlightedStr(String)`: cursor for highlighted entry in selection tab.
- `Rewind(Flag)`: if true, when moving beyond limits of component, the choice will be rewinded, instead of remaining the same
- `Title(Title)`: set select title
- `Value(Payload(One(Usize)))`: set default selected item by its index

---

### Span

A span is an in-line component which supports text with different styles.

**Commands**: None

**State**: None

**Properties**:

- `Alignment(Alignment)`: set text alignment
- `Background(Color)`: set background color
- `Foreground(Color)`: set foreground color
- `Text(Payload(Vec(TextSpan)))` set text spans
- `TextProps(TextModifiers)`: set text modifiers

---

### Sparkline

A sparkline chart.

**Commands**: None

**State**: `None`.

**Properties**:

- `Background(Color)`: background color
- `Dataset(Payload(Vec(U64)))`: set data for sparkline. Is a vec of u64
- `Foreground(Color)`: foreground color
- `Title(Title)`: label for sparkline
- `Width(Length)`: maximum amount of entries to display. If not provided, will be the maximum allowed by the area width.

---

### Table

a table of rows with the possibility to scroll text with arrows. In order to scroll, the component must be active.

**Commands**:

Events will be reported only when set as `Scrollable`

| Cmd           | CmdResult        | Behaviour                 |
|---------------|------------------|---------------------------|
| `GoTo(Begin)` | `Changed | None` | Move cursor to first item |
| `GoTo(End)`   | `Changed | None` | Move cursor to last item  |
| `Move(Down)`  | `Changed | None` | Move cursor down          |
| `Move(Up)`    | `Changed | None` | Move cursor up            |
| `Scroll(Down)`| `Changed | None` | Move cursor down by 8     |
| `Scroll(Up)`  | `Changed | None` | Move cursor up by 8       |

**State**: If `scrollable`, returns current `One(Usize(index))`, otherwise None

**Properties**:

- `Background(Color)`: set background color
- `Borders(Borders)`: set border properties
- `Content(Table)`: set table
- `Custom($TABLE_COLUMN_SPACING, Size)`: column spacing
- `FocusStyle(Style)`: inactive style
- `Foreground(Color)`: set foreground color
- `Height(Size)`: set row height
- `HighlightedColor(Color)`: set highlighted color
- `HighlightedStr(String)`: set highlighted string
- `Rewind(Flag)`: rewind list if boundaries are reached
- `Scroll(Flag)`: set whether is scrollable
- `ScrollStep(Length)`: set scroll step
- `Text(Payload(Vec(String)))`: set table headers
- `TextProps(TextModifiers)`: set text modifiers
- `Title(Title)`: set block title
- `Width(Payload(Vec(U16)))`: set col widths

---

### Textarea

A textarea is like a paragraph, but has the possibility to scroll the text.

**Commands**:

| Cmd                 | Result | Behaviour                 |
|---------------------|--------|---------------------------|
| `GoTo(Begin)`       | `None` | Move cursor to first item |
| `GoTo(End)`         | `None` | Move cursor to last item  |
| `Move(Down)`        | `None` | Move cursor down          |
| `Move(Up)`          | `None` | Move cursor up            |
| `Scroll(Down)`      | `None` | Move cursor down by 8     |
| `Scroll(Up)`        | `None` | Move cursor up by 8       |

**Properties**:

- `Background(Color)`: set background color
- `Borders(Borders)`: set border properties
- `FocusStyle(Style)`: inactive style
- `Foreground(Color)`: set foreground color
- `HighlightedStr(String)`: set highlighted string
- `ScrollStep(Length)`: set scroll step
- `Text(Payload(Vec(TextSpan)))`: set text spans
- `TextProps(TextModifiers)`: set text modifiers
- `Title(Title)`: set block title

**State**: None

---

### Utilities

The standard components library also exports the `utils` module, which provides these very handy functions:

- **wrap_spans**: Creates span lines from text spans, in order to wrap lines
- **use_or_default_styles**: use colors and modifiers of the text spans if not `Color::Reset` or `Modifiers::empty()`, otherwise use the properties defined the `Props`.
- **get_block**: creates the block for the widget. If focus is true, the colors are applied, otherwise `Color::Reset`.

---

## Documentation 📚

The developer documentation can be found on Rust Docs at <https://docs.rs/tuirealm>

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve tui-realm-stdlib, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View tui-realm's changelog [HERE](CHANGELOG.md)

---

## License 📃

tui-realm-stdlib is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
