# Components 🎨

- [Components 🎨](#components-)
  - [Quick introduction](#quick-introduction)
  - [BarChart](#barchart)
  - [Canvas](#canvas)
  - [Chart](#chart)
  - [Checkbox](#checkbox)
  - [Container](#container)
  - [Input](#input)
  - [Label](#label)
  - [Line gauge](#line-gauge)
  - [List](#list)
  - [Paragraph](#paragraph)
  - [Phantom](#phantom)
  - [Progress bar](#progress-bar)
  - [Radio](#radio)
  - [Select](#select)
  - [Span](#span)
  - [Sparkline](#sparkline)
  - [Spinner](#spinner)
  - [Table](#table)
  - [Textarea](#textarea)

---

## Quick introduction

This document contains the reference and the example image for all the components exposed in the standard library.

For each component you'll find its **command API**, the **Properties** it can handle and the **State** it'll return.

Aside of this, keep in mind that **Every component** will also reserve these two properties, as specified in the tui-realm documentation:

- `Attribute::Display(AttrValue::Flag)`: if `False` the component WON'T be rendered.
- `Attribute::Focus(AttrValue::Flag)`: indicates whether the component is **active** or not. This property is always **AUTOMATICALLY** handled by the **View**.

This library also uses a few *Custom* attributes. These custom attributes are obviously of kind `Attribute::Custom` and their keys are defined in this document with a `$` before their names. You can access to these values directly from the library module `tui_realm_stdlib::props::$KEY_NAME`.

## BarChart

![bar_chart](/docs/images/components/bar_chart.gif)

> ✨ Check me out ✨  
> `cargo run --example bar-chart`

A chart with bars. The bar chart can work both in "active" and "disabled" mode.

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

## Canvas

![canvas](/docs/images/components/canvas.gif)

> ✨ Check me out ✨  
> `cargo run --example canvas`

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

## Chart

![chart](/docs/images/components/chart.gif)

> ✨ Check me out ✨  
> `cargo run --example chart`

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

## Checkbox

![checkbox](/docs/images/components/checkbox.gif)

> ✨ Check me out ✨  
> `cargo run --example checkbox`

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

## Container

![container](/docs/images/components/container.gif)

> ✨ Check me out ✨  
> `cargo run --example container`

A container works a div. It is just an empty container which can contain other components.
You can mount children in it using the `children()` method on the constructor or with the `children` property when implementing the `Component`.
By default all **Commands** are forwarded to all children and a **Batch** of **Command result** is returned, but you can obviously implement it as you want overriding the `perform()` method in the **Component**.
While for `attr()` it will apply the properties for all the children by default. You can override this behaviour.

**Commands**: depends on children

**State**: `None`

**Properties**:

- `Background(Color)`: default background color
- `Borders(Borders)`: set borders properties for container
- `Foreground(Color)`: default foreground color
- `Layout(Layout)`: set the layout to use to render children. **Children will be rendered in order by index** (so `constraints[0] => children[0]`, ...)
- `Title(Title)`: set title for div

---

## Input

![input](/docs/images/components/input.gif)

> ✨ Check me out ✨  
> `cargo run --example input`

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
- `Custom($INPUT_INVALID_STYLE, Style)`: Set style to apply to component when input is invalid
- `Custom($INPUT_PLACEHOLDER, String)`: Set a placeholder to display when the input is empty
- `Custom($INPUT_PLACEHOLDER_STYLE, Style)`: Set style for placeholder text
- `Display(Flag)`: if False component is hidden
- `FocusStyle(Style)`: style for when component is not active
- `Foreground(Color)`: foreground color
- `InputLength(Length)`: set the maximum input length
- `InputType(InputType)`: set the input type
- `Title(Title)`: set input box title
- `Value(String)`: set value for the input

---

## Label

![label](/docs/images/components/label.gif)

> ✨ Check me out ✨  
> `cargo run --example label`

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

## Line gauge

![line_gauge](/docs/images/components/line_gauge.gif)

> ✨ Check me out ✨  
> `cargo run --example line_gauge`

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

## List

![list](/docs/images/components/list.gif)

> ✨ Check me out ✨  
> `cargo run --example list`

a list of rows with the possibility to scroll text with arrows. In order to scroll, the component must be active.

**Commands**:

Events will be reported only when set as `Scrollable`

| Cmd              | CmdResult        | Behaviour                 |
|------------------|------------------|---------------------------|
| `GoTo(Begin)`    | `Changed | None` | Move cursor to first item |
| `GoTo(End)`      | `OnKey | None`   | Move cursor to last item  |
| `Move(Down)`     | `OnKey | None`   | Move cursor down          |
| `Move(Up)`       | `OnKey | None`   | Move cursor up            |
| `Scroll(Down)`   | `OnKey | None`   | Move cursor down by 8     |
| `Scroll(Up)`     | `OnKey | None`   | Move cursor up by 8       |

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

## Paragraph

![paragraph](/docs/images/components/paragraph.gif)

> ✨ Check me out ✨  
> `cargo run --example paragraph`

A text paragraph. Like in HTML this has to be considered a block element, and supports multi-line texts with different styles. The text is automatically wrapped.

**Commands**: None

**State**: None

**Properties**:

- `Alignment(Alignment)`: set text alignment
- `Background(Color)`: set background color
- `Borders(Borders)`: set border properties
- `Foreground(Color)`: set foreground color
- `Text(Payload(Vec(TextSpan)))`: set paragraph text
- `TextProps(TextModifiers)`: set text modifiers
- `TextWrap(Flag)`: select whether to trim rows when wrapping
- `Title(Title)` set paragraph title

---

## Phantom

Phantom is a component which doesn't render and has no property. It is sole purpose is to be a global listener for some kinds of events.
This component suits well to work as a subscriber for some global events (such as an `ESC` key to terminate).

**Commands**: None

**State**: None

**Properties**: None

---

## Progress bar

![progress_bar](/docs/images/components/progress_bar.gif)

> ✨ Check me out ✨  
> `cargo run --example progress_bar`

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

## Radio

![radio](/docs/images/components/radio.gif)

> ✨ Check me out ✨  
> `cargo run --example radio`

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

## Select

![select](/docs/images/components/select.gif)

> ✨ Check me out ✨  
> `cargo run --example select`

A select like in HTML. Provides the possibility to select a single option in a group of options. When `state` is invoked returns the index of the selected option as Unsigned, but only if the selection tab is closed. Returns `State::None` otherwise. The tab can be opened with `Cmd::Submit`; once opened you can move with arrows to select the entry. To close the form, you need to press `Cmd::Submit` again. Once the tab is closed, a `CmdResult::Submit` is raised with the selected index.
If the component loses focus, the selection tab is automatically closed
This component should have a variable size in the layout to be displayed properly. Please view the example: `examples/select.rs`.

**Commands**:

| Command      | Result             | Behaviour                                                      |
|--------------|--------------------|----------------------------------------------------------------|
| `Move(Down)` | `Changed` | `None` | Move select down, if tab is open                               |
| `Move(Up)`   | `Changed` | `None` | Move select up, if tab is open                                 |
| `Submit`     | `Submit` | `None`  | Open or close the select tab; Returns state if tab gets closed |

**State**: the state returned is `One(Usize)` containing the index of the selected item in the radio group. This state is returned only when the select is closed; otherwise `None` is returned

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

## Span

> ✨ Check me out ✨  
> `cargo run --example span`

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

## Sparkline

![sparkline](/docs/images/components/sparkline.gif)

> ✨ Check me out ✨  
> `cargo run --example sparkline`

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

## Spinner

![spinner](/docs/images/components/spinner.gif)

> ✨ Check me out ✨  
> `cargo run --example spinner`

A spinner is a spinner indicating a loading. It has a sequence of char to iterate over and on each `view()` call the step is increased by one.
So for example the sequence may be `"⣾⣽⣻⢿⡿⣟⣯⣷"`, so at first view `⣾` will be rendered, on the 2nd step `⣽`, etc.

**Commands**: None

**State**: None

**Properties**:

- `Background(Color)`: set background color
- `Foreground(Color)`: set foreground color
- `Text(String)` set the spinner sequence. Each char of the string represents a step

---

## Table

![table](/docs/images/components/table.gif)

> ✨ Check me out ✨  
> `cargo run --example table`

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

## Textarea

![textarea](/docs/images/components/textarea.gif)

> ✨ Check me out ✨  
> `cargo run --example textarea`

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
