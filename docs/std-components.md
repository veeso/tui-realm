# Standard components library

- [Standard components library](#standard-components-library)
  - [Introduction](#introduction)
  - [BarChart](#barchart)
  - [Checkbox](#checkbox)
  - [Input](#input)
  - [Label](#label)
  - [Paragraph](#paragraph)
  - [Progress bar](#progress-bar)
  - [Radio](#radio)
  - [Select](#select)
  - [Sparkline](#sparkline)
  - [ScrollTable](#scrolltable)
  - [Span](#span)
  - [Table](#table)
  - [Textarea](#textarea)
  - [Utilities](#utilities)
  - [What's next](#whats-next)

---

## Introduction

This document describes all the components provided by the standard components library of tui-realm. For each component is described what the component is, the messages it handles after an event or an update and the properties which you can set for it.

Each component is described by 4 parts:

- events: the events and the associated messages for each component.
- update: the message returned after an update.
- state: the state returned by the component in the payload.
- properties: what each prop builder method provides.

---

## BarChart

A chart with bars. The bar chart can work both in "active" and "disabled" mode.

When in disabled mode, the chart won't be interactive, so you won't be able to move through data using keys.
If you have more data than the maximum amount of bars that can be displayed, you'll have to update data to display the remaining entries

While in active mode (default) you can put as many entries as you wish. You can move with arrows and END/HOME keys

**Events**:

| Event                | Message         | Behaviour                                      |
|----------------------|-----------------|------------------------------------------------|
| `KeyCode::Right`     | `None`          | Move the cursor right                          |
| `KeyCode::Left`      | `None`          | Move the cursor left                           |
| `KeyCode::End`       | `None`          | Move "cursor" to the end of chart              |
| `KeyCode::Home`      | `None`          | Move "cursor" to the first entry of the chart  |
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

## Checkbox

A checkbox group. Provides the possibility to select between multiple options, when `get_state` is invoked returns a vector of index; each index represents the index of the item selected.

**Events**:

| Event                | Message         | Behaviour                                      |
|----------------------|-----------------|------------------------------------------------|
| `KeyCode::Right`     | `None`          | Increment the selected choice index by 1       |
| `KeyCode::Left`      | `None`          | Decrement the selected choice index by 1       |
| `KeyCode::Char(' ')` | `OnChange`      | Check or uncheck the item at the current index |
| `KeyCode::Enter`     | `OnSubmit`      | Just returns the selection                     |
| `KeyCode::Char(_)`   | `OnKey`         |                                                |

**Update**: `Msg::OnChange` if the selection changed, `Msg::None` otherwise.

**State**: the state returned is a `VecOfUsize` containing the indexes of the selected item in the checkbox group.

**Properties**:

- `with_color`: foreground color
- `with_inverted_colors`: color used when item is at current index
- `with_borders`: set borders properties for component
- `with_options`: set checkbox options and title
- `with_value`: set selected by-default items by their index

---

## Input

An input text. Provides the possiblity to input a text with the possibility to set the input length and the input type (number, password, text). It also allows to use arrows to move the cursor inside of the input box. When `get_state` is invoked, returns the current content of the input as String or as Number based on the current input type.

**Events**:

| Event                | Message           | Behaviour                                            |
|----------------------|-------------------|------------------------------------------------------|
| `KeyCode::Backspace` | `OnChange | None` | Remove previous character in input                   |
| `KeyCode::Delete`    | `OnChange | None` | Delete next character in input                       |
| `KeyCode::Enter`     | `OnSubmit`        | Submit input                                         |
| `KeyCode::Left`      | `None`            | Move cursor left                                     |
| `KeyCode::Right`     | `None`            | Move cursor right                                    |
| `KeyCode::End`       | `None`            | Move cursor at the end of input                      |
| `KeyCode::Home`      | `None`            | Move cursor at the beginning of input                |
| `KeyCode::Char(_)`   | `OnChange | None` | Push character, if allowed by method, into the input |

**Update**: `Msg::OnChange` if the value changed, `Msg::None` otherwise.

**State**: the state returned is a `Str` or an `Unsigned` based on the selected input type containing the current value of the input.

**Properties**:

- `with_foreground`: foreground color
- `with_background`: background color
- `with_borders`: set borders properties for component
- `with_label`: set input label
- `with_input`: set the input type
- `with_input_len`: set the maximum input length
- `with_value`: set initial value for the input

---

## Label

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

---

## Paragraph

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
- `with_texts`: set block title and paragraph text

---

## Progress bar

A progress bar. The progress bar provides the possibility to show the current progress and to show a label above it.

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
- `with_texts`: set block title and progress bar label

---

## Radio

A radio button group. Provides the possibility to select a single option in a group of options. When `get_state` is invoked returns the index of the selected option as Unsigned.

**Events**:

| Event                | Message         | Behaviour                                        |
|----------------------|-----------------|--------------------------------------------------|
| `KeyCode::Right`     | `OnChange`      | Change the selected option to current item index |
| `KeyCode::Left`      | `OnChange`      | Change the selected option to current item index |
| `KeyCode::Enter`     | `OnSubmit`      | Just returns the index of the selected item      |
| `KeyCode::Char(_)`   | `OnKey`         |                                                |

**Update**: `Msg::OnChange` if the choice changed, `Msg::None` otherwise.

**State**: the state returned is an `Unsigned` containing the index of the selected item in the radio group.

**Properties**:

- `with_color`: foreground color
- `with_inverted_colors`: color used when item is at current index
- `with_borders`: set borders properties for component
- `with_options`: set radio options and title
- `with_value`: set default selected item by its index

---

## Select

A select like in HTML. Provides the possibility to select a single option in a group of options. When `get_state` is invoked returns the index of the selected option as Unsigned, but only if the selection tab is closed. Returns `Payload::None` otherwise. The tab can be opened with `<ENTER>`; once opened you can move with arrows to select the entry. To close the form, you need to press `<ENTER>` again. Once the tab is closed, a `Msg::OnSubmit` is raised with the selected index.
If the component loses focus, the selection tab is automatically closed
This component should have a variable size in the layout to be displayed properly. Please view the example: `examples/select.rs`.

**Events**:

| Event                | Message                  | Behaviour                        |
|----------------------|--------------------------|----------------------------------|
| `KeyCode::Up`        | `OnChange` | `None`      | Move select up, if tab is open   |
| `KeyCode::Down`      | `OnChange` | `None`      | Move select down, if tab is open |
| `KeyCode::Enter`     | `OnSubmit` | `None`      | Open or close the select tab     |
| `KeyCode::Char(_)`   | `OnKey`         |                                                |

**Update**: `Msg::OnChange` if the choice changed, `Msg::None` otherwise.

**State**: the state returned is an `Unsigned` containing the index of the selected item in the radio group.

**Properties**:

- `with_foreground`: foreground color
- `with_background`: background color
- `with_highlighted_color`: a different color for highlighted entry; `foreground` otherwise
- `with_highlighted_symbol`: cursor for highlighted entry in selection tab.
- `with_borders`: set borders properties for component
- `with_options`: set radio options and title
- `with_value`: set default selected item by its index

---

## Sparkline

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

## ScrollTable

a table with the possibility to scroll text with arrows. In order to scroll, the component must be active.

**Events**:

| Event               | Message | Behaviour                 |
|---------------------|---------|---------------------------|
| `KeyCode::Down`     | `OnKey` | Move cursor down          |
| `KeyCode::Up`       | `OnKey` | Move cursor up            |
| `KeyCode::PageDown` | `OnKey` | Move cursor down by 8     |
| `KeyCode::PageUp`   | `OnKey` | Move cursor up by 8       |
| `KeyCode::End`      | `OnKey` | Move cursor to last item  |
| `KeyCode::Home`     | `OnKey` | Move cursor to first item |
| `KeyCode::Char(_)`  | `OnKey` | Return pressed key        |

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
- `with_table`: set block title and table entries

---

## Span

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
- `with_spans`: set block title and paragraph text

---

## Table

A table, but without the possibility to interact with it and without scrolling.

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
- `with_table`: set block title and table entries

## Textarea

A textarea is like a paragraph, but has the possibility to scroll the text.

**Events**:

| Event               | Message | Behaviour                 |
|---------------------|---------|---------------------------|
| `KeyCode::Down`     | `OnKey` | Move cursor down          |
| `KeyCode::Up`       | `OnKey` | Move cursor up            |
| `KeyCode::PageDown` | `OnKey` | Move cursor down by 8     |
| `KeyCode::PageUp`   | `OnKey` | Move cursor up by 8       |
| `KeyCode::End`      | `OnKey` | Move cursor to last item  |
| `KeyCode::Home`     | `OnKey` | Move cursor to first item |
| `KeyCode::Char(_)`  | `OnKey` | Return pressed key        |

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
- `with_texts`: set block title and and text spans

---

## Utilities

The standard components library also exports the `utils` module, which provides these very handy functions:

- **wrap_spans**: Creates span lines from text spans, in order to wrap lines
- **use_or_default_styles**: use colors and modifiers of the text spans if not `Color::Reset` or `Modifiers::empty()`, otherwise use the properties defined the `Props`.
- **get_block**: creates the block for the widget. If focus is true, the colors are applied, otherwise `Color::Reset`.

## What's next

You might now be interested in learning how to implement your custom components! If so, check out the guide [here](new-components.md)!
