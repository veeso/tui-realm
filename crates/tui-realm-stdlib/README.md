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
  - [Buy me a coffee ☕](#buy-me-a-coffee-)
  - [Components 🎨](#components-)
    - [BarChart](#barchart)
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
    - [Sparkline](#sparkline)
    - [Span](#span)
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

## Buy me a coffee ☕

If you like tui-realm-stdlib and you're grateful for the work I've done, please consider a little donation 🥳

[![Buy-me-a-coffee](https://img.buymeacoffee.com/button-api/?text=Buy%20me%20a%20coffee&emoji=&slug=veeso&button_colour=404040&font_colour=ffffff&font_family=Comic&outline_colour=ffffff&coffee_colour=FFDD00)](https://www.buymeacoffee.com/veeso)

---

## Components 🎨

### BarChart

A chart with bars. The bar chart can work both in "active" and "disabled" mode.

When in disabled mode, the chart won't be interactive, so you won't be able to move through data using keys.
If you have more data than the maximum amount of bars that can be displayed, you'll have to update data to display the remaining entries

While in active mode (default) you can put as many entries as you wish. You can move with arrows and END/HOME keys

**Events**:

| Event                | Message         | Behaviour                                      |
|----------------------|-----------------|------------------------------------------------|
| `KeyCode::Right`     | `None`          | Move the cursor right                          |
| `KeyCode::Left`      | `None`          | Move the cursor left                           |
| `GoTo(End)`       | `None`          | Move "cursor" to the end of chart              |
| `GoTo(Begin)`      | `None`          | Move "cursor" to the first entry of the chart  |
| `KeyCode::Char(_)`   | `OnKey`         |                                                |

**State**: `None`.

**Properties**:

- `disabled`: Sets the chart in disabled mode
- `with_foreground`: foreground color
- `with_background`: background color
- `with_title`: title for chart
- `with_label_style`: Sets the style for data labels
- `with_max_bars`: maximum amount of bars to display. If not provided, will be the maximum allowed by the area width.
- `with_bar_style`: sets style for bars
- `with_bar_gap`: sets gap for bars
- `with_value_style`: Sets style for values
- `with_data`: set data for chart. Is a vec of tuple of labels and u64
- `push_record_back`: Just push the provided record to the back of data (end)
- `push_record_front`: Just push the provided record to the front of data (begin)
- `pop_record_front`: Pops the first element of data
- `pop_record_back`: Pops the last element of data

### Chart

A chart displayed on a cartesian axis system. Can work both in "active" and "disabled" mode.

When in disabled mode, the chart won't be interactive, so you won't be able to move through data using keys.
If you have more data than the maximum amount of bars that can be displayed, you'll have to update data to display the remaining entries

While in active mode (default) you can put as many entries as you wish. You can move with arrows and END/HOME keys

**Events**:

| Event                | Message         | Behaviour                                      |
|----------------------|-----------------|------------------------------------------------|
| `KeyCode::Right`     | `None`          | Move the cursor right                          |
| `KeyCode::Left`      | `None`          | Move the cursor left                           |
| `GoTo(End)`       | `None`          | Move "cursor" to the end of chart              |
| `GoTo(Begin)`      | `None`          | Move "cursor" to the first entry of the chart  |
| `KeyCode::Char(_)`   | `OnKey`         |                                                |

**State**: `None`.

**Properties**:

- `disabled`: Sets the chart in disabled mode
- `with_foreground`: foreground color
- `with_background`: background color
- `with_title`: title for chart
- `with_label_style`: Sets the style for data labels
- `with_*_bounds`: Something regarding the viewport; view tui-rs documentation (which doesn't exist actually). I don't know how it works actually.
- `with_*_labels`: Set labels for provided axis
- `with_*_style`: Set style for provided axis
- `with_*_title`: Set title for axis
- `with_data`: set data for chart. Is a vec of `Dataset`
- `push_record`: Just push the provided record to the back of data (end)
- `pop_record_front`: Pops the first element of data
- `pop_record_back`: Pops the last element of data

### Checkbox

A checkbox group. Provides the possibility to select between multiple options, when `get_state` is invoked returns a vector of index; each index represents the index of the item selected.

**Events**:

| Event                | Message         | Behaviour                                      |
|----------------------|-----------------|------------------------------------------------|
| `KeyCode::Right`     | `None`          | Increment the selected choice index by 1       |
| `KeyCode::Left`      | `None`          | Decrement the selected choice index by 1       |
| `KeyCode::Char(' ')` | `Changed`      | Check or uncheck the item at the current index |
| `KeyCode::Enter`     | `Submit`      | Just returns the selection                     |
| `KeyCode::Char(_)`   | `OnKey`         |                                                |

**Update**: `Msg::Changed` if the selection changed, `Msg::None` otherwise.

**State**: the state returned is a `VecOfUsize` containing the indexes of the selected item in the checkbox group.

**Properties**:

- `with_color`: foreground color
- `with_inverted_colors`: color used when item is at current index
- `with_borders`: set borders properties for component
- `with_options`: set checkbox options
- `with_title`: set checkbox title
- `with_value`: set selected by-default items by their index
- `rewind`: if true, when moving beyond limits of component, the choice will be rewinded, instead of remaining the same

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

**Events**:

| Event                | Message            | Behaviour          |
|----------------------|--------------------|--------------------|
| `KeyCode::Char(_)`   | `OnKey`            | Return pressed key |

**Update**: None

**State**: None

**Properties**:

- `with_foreground`: set foreground color
- `with_background`: set background color
- `bold`: set text bold
- `italic`: set text italic
- `rapid_blink`: set rapid blink for text
- `reversed`: reverses colors
- `slow_blink` set slow blink for test
- `strikethrough`: set strikethrough for text
- `underlined`: set underlined text
- `with_text`: set label text
- `with_text_alignment`: set text alignment

---

### Line gauge

A line indicating progress. The progress bar provides the possibility to show the current progress and to show a label above it.

**Events**:

| Event                | Message            | Behaviour          |
|----------------------|--------------------|--------------------|
| `KeyCode::Char(_)`   | `OnKey`            | Return pressed key |

**Update**: None

**State**: None

**Properties**:

- `with_progbar_color`: set progress bar color
- `with_background`: set background color
- `with_progress`: set progress. **WARNING**: must be in range 0.0,1.0
- `with_borders`: set border properties
- `with_label`: set progress bar label
- `with_title`: set progress bar title
- `with_line_normal`: use default line
- `with_line_rounded`: use roundeed line
- `with_line_tick`: use thick line
- `with_line_doubled` use double line

---

### List

a list of rows with the possibility to scroll text with arrows. In order to scroll, the component must be active.

**Events**:

Events will be reported only when set as `Scrollable`

| Event               | Message | Behaviour                 |
|---------------------|---------|---------------------------|
| `Move(Down)`     | `OnKey` | Move cursor down          |
| `Move(Up)`       | `OnKey` | Move cursor up            |
| `Scroll(Down)` | `OnKey` | Move cursor down by 8     |
| `Scroll(Up)`   | `OnKey` | Move cursor up by 8       |
| `GoTo(End)`      | `OnKey` | Move cursor to last item  |
| `GoTo(Begin)`     | `OnKey` | Move cursor to first item |
| `KeyCode::Char(_)`  | `OnKey` | Return pressed key        |

**Update**: None

**State**: If `scrollable`, returns current list index, otherwise None

**Properties**:

- `with_foreground`: set foreground color
- `with_background`: set background color
- `scrollable`: mark the list as scrollable
- `bold`: set text bold
- `italic`: set text italic
- `rapid_blink`: set rapid blink for text
- `reversed`: reverses colors
- `slow_blink` set slow blink for test
- `strikethrough`: set strikethrough for text
- `underlined`: set underlined text
- `with_borders`: set border properties
- `with_rows`: set table entries
- `with_title`: set block title

---

### Paragraph

A text paragraph. Like in HTML this has to be considered a block element, and supports multi-line texts with different styles. The text is automatically wrapped.

**Events**:

| Event                | Message            | Behaviour          |
|----------------------|--------------------|--------------------|
| `KeyCode::Char(_)`   | `OnKey`            | Return pressed key |

**Update**: None

**State**: None

**Properties**:

- `with_foreground`: set foreground color
- `with_background`: set background color
- `bold`: set text bold
- `italic`: set text italic
- `rapid_blink`: set rapid blink for text
- `reversed`: reverses colors
- `slow_blink` set slow blink for test
- `strikethrough`: set strikethrough for text
- `underlined`: set underlined text
- `with_borders`: set border properties
- `with_texts`: set paragraph text
- `with_title` set paragraph title
- `with_text_alignment`: set text alignment
- `with_trim`: select whether to trim rows when wrapping

---

### Progress bar

A progress bar or basically a gauge. The progress bar provides the possibility to show the current progress and to show a label above it.

**Events**:

| Event                | Message            | Behaviour          |
|----------------------|--------------------|--------------------|
| `KeyCode::Char(_)`   | `OnKey`            | Return pressed key |

**Update**: None

**State**: None

**Properties**:

- `with_progbar_color`: set progress bar color
- `with_background`: set background color
- `with_progress`: set progress. **WARNING**: must be in range 0.0,1.0
- `with_borders`: set border properties
- `with_label`: set progress bar label
- `with_title`: set progress bar title

---

### Radio

A radio button group. Provides the possibility to select a single option in a group of options. When `get_state` is invoked returns the index of the selected option as Unsigned.

**Events**:

| Event                | Message         | Behaviour                                        |
|----------------------|-----------------|--------------------------------------------------|
| `KeyCode::Right`     | `Changed`      | Change the selected option to current item index |
| `KeyCode::Left`      | `Changed`      | Change the selected option to current item index |
| `KeyCode::Enter`     | `Submit`      | Just returns the index of the selected item      |
| `KeyCode::Char(_)`   | `OnKey`         |                                                |

**Update**: `Msg::Changed` if the choice changed, `Msg::None` otherwise.

**State**: the state returned is an `Unsigned` containing the index of the selected item in the radio group.

**Properties**:

- `with_color`: foreground color
- `with_inverted_colors`: color used when item is at current index
- `with_borders`: set borders properties for component
- `with_options`: set radio options
- `with_title`: set radio title
- `with_value`: set default selected item by its index
- `rewind`: if true, when moving beyond limits of component, the choice will be rewinded, instead of remaining the same

---

### Select

A select like in HTML. Provides the possibility to select a single option in a group of options. When `get_state` is invoked returns the index of the selected option as Unsigned, but only if the selection tab is closed. Returns `Payload::None` otherwise. The tab can be opened with `<ENTER>`; once opened you can move with arrows to select the entry. To close the form, you need to press `<ENTER>` again. Once the tab is closed, a `Msg::Submit` is raised with the selected index.
If the component loses focus, the selection tab is automatically closed
This component should have a variable size in the layout to be displayed properly. Please view the example: `examples/select.rs`.

**Events**:

| Command              | Result                | Behaviour                        |
|----------------------|--------------------------|----------------------------------|
| `Move(Up)`           | `Changed` | `None`      | Move select up, if tab is open   |
| `Move(Down)`         | `Changed` | `None`      | Move select down, if tab is open |
| `KeyCode::Enter`     | `Submit` | `None`      | Open or close the select tab     |
| `KeyCode::Char(_)`   | `OnKey`         |                                                |

**Update**: `Msg::Changed` if the choice changed, `Msg::None` otherwise.

**State**: the state returned is an `Unsigned` containing the index of the selected item in the radio group.

**Properties**:

- `with_foreground`: foreground color
- `with_background`: background color
- `with_highlighted_color`: a different color for highlighted entry; `foreground` otherwise
- `with_highlighted_symbol`: cursor for highlighted entry in selection tab.
- `with_borders`: set borders properties for component
- `with_options`: set select options
- `with_title`: set select title
- `with_value`: set default selected item by its index
- `rewind`: if true, when moving beyond limits of component, the choice will be rewinded, instead of remaining the same

---

### Sparkline

A sparkline chart.

**Events**:

| Event                | Message         | Behaviour                                      |
|----------------------|-----------------|------------------------------------------------|
| `KeyCode::Char(_)`   | `OnKey`         |                                                |

**State**: `None`.

**Properties**:

- `with_foreground`: foreground color
- `with_background`: background color
- `with_label`: label for sparkline
- `with_max_entries`: maximum amount of entries to display. If not provided, will be the maximum allowed by the area width.
- `with_data`: set data for sparkline. Is a vec of u64
- `push_record_back`: Just push the provided record to the back of data (end)
- `push_record_front`: Just push the provided record to the front of data (begin)
- `pop_record_front`: Pops the first element of data
- `pop_record_back`: Pops the last element of data

---

### Span

A span is an in-line component which supports text with different styles.

**Events**:

| Event                | Message            | Behaviour          |
|----------------------|--------------------|--------------------|
| `KeyCode::Char(_)`   | `OnKey`            | Return pressed key |

**Update**: None

**State**: None

**Properties**:

- `with_foreground`: set foreground color
- `with_background`: set background color
- `bold`: set text bold
- `italic`: set text italic
- `rapid_blink`: set rapid blink for text
- `reversed`: reverses colors
- `slow_blink` set slow blink for test
- `strikethrough`: set strikethrough for text
- `underlined`: set underlined text
- `with_borders`: set border properties
- `with_spans`: set paragraph text
- `with_title` set block title
- `with_text_alignment`: set text alignment

---

### Table

a table of rows with the possibility to scroll text with arrows. In order to scroll, the component must be active.

**Events**:

Events will be reported only when set as `Scrollable`

| Event         | Message          | Behaviour                 |
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
- `Custom("col-spacing", Size)`: column spacing
- `FocusStyle(Style)`: inactive style
- `Foreground(Color)`: set foreground color
- `Height(Size)`: set row height
- `HighlightedColor(Color)`: set highlighted color
- `HighlightedStr(String)`: set highlighted string
- `Scroll(Flag)`: set whether is scrollable
- `ScrollStep(Length)`: set scroll step
- `Text(Payload(Vec(String)))`: set table headers
- `TextProps(TextModifiers)`: set text modifiers
- `Title(Title)`: set block title
- `Value(Table)`: set table
- `Width(Payload(Vec(U16)))`: set col widths

---

### Textarea

A textarea is like a paragraph, but has the possibility to scroll the text.

**Events**:

| Cmd                 | Result | Behaviour                 |
|---------------------|--------|---------------------------|
| `Move(Down)`        | `None` | Move cursor down          |
| `Move(Up)`          | `None` | Move cursor up            |
| `Scroll(Down)`      | `None` | Move cursor down by 8     |
| `Scroll(Up)`        | `None` | Move cursor up by 8       |
| `GoTo(End)`         | `None` | Move cursor to last item  |
| `GoTo(Begin)`       | `None` | Move cursor to first item |

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
