# Changelog

- [Changelog](#changelog)
  - [1.3.1](#131)
  - [1.3.0](#130)
  - [1.2.0](#120)
  - [1.1.7](#117)
  - [1.1.6](#116)
  - [1.1.5](#115)
  - [1.1.4](#114)
  - [1.1.3](#113)
  - [1.1.2](#112)
  - [1.1.0](#110)
  - [1.0.3](#103)
  - [1.0.2](#102)
  - [1.0.1](#101)
  - [1.0.0](#100)
  - [0.6.4](#064)
  - [0.6.3](#063)
  - [0.6.2](#062)
  - [0.6.1](#061)
  - [0.6.0](#060)
  - [0.5.0](#050)

---


## 1.3.1

Released on 06/01/2024

- [Issue 20](https://github.com/veeso/tui-realm-stdlib/pull/20)
- [Issue 18](https://github.com/veeso/tui-realm-stdlib/pull/18)

## 1.3.0

Released on 22/08/2023

- Added support for `ratatui`
  - to enable `ratatui` set feature `ratatui` (enabled by default)
  - to enable `tui` set feature `tui`.
- Deprecated features `with-crossterm`, `with-termion`

## 1.2.0

Released on 17/10/2022

- Added support for shape `Label` in canvas
- Added `Marker` to canvas props

## 1.1.7

Released on 03/08/2022

- Fixed bar chart (Credit [@tpoliaw](https://github.com/tpoliaw))
- Fixed `Select` border style not applied (Credit [@tramhao](https://github.com/tramhao))
- Updated `textwrap` to `0.15`

## 1.1.6

Released on 30/01/2022

- [Issue 6](https://github.com/veeso/tui-realm-stdlib/issues/6): Implemented `Cmd::Cancel` for `Select` and close tab on tab.
  - Command `Cancel`, when pressed will:
    - close the select tab
    - restore the select value TO THE VALUE SET BEFORE OPENING THE TAB.
  - When the select component loses the focus, the tab will be always closed and the previous value will be restored.

## 1.1.5

Released on 08/12/2021

- Added `Value` property for `Table` and `List` to set current line when component is scrollable

## 1.1.4

Released on 28/11/2021

- Fixed highlighted color rendered also when not selected in lists and tables

## 1.1.3

Released on 28/11/2021 **Yanked**

- Fixed a glitch which made list items always highlighted

## 1.1.2

Released on 28/11/2021 **Yanked**

- Solved issue for highlighted items: when highlighted text where rendered with default foreground, instead of with terminal background color

## 1.1.0

Released on 22/11/2021

- Compatibility with tui-realm 1.1.0

## 1.0.3

Released on 15/11/2021

- Made states fields and methods public

## 1.0.2

Released on 15/11/2021

- `get_block` (utils) must be public

## 1.0.1

Released on 14/11/2021

- All `states` for components are now accessible

## 1.0.0

Released on 13/11/2021

- New components 🎉
  - Container
  - Phantom
  - Spinner
- Migrated components to tui-realm 1.0.0

## 0.6.4

Released on 05/11/2021

- Bugfix
  - [Issue 3](https://github.com/veeso/tui-realm-stdlib/issues/4): textarea eliding some characters at the end of area if not ASCII chars.

## 0.6.3

Released on 04/10/2021

- Input: Escape input if is KEY + (ALT or CTRL), but NOT if is CTRL+ALT+KEY or CTRL+ALT+SHIFT+KEY

## 0.6.2

Released on 31/08/2021

- Bugfix
  - [Issue 3](https://github.com/veeso/tui-realm-stdlib/issues/3): Table/list lose focus when updating and component is scrollable

## 0.6.1

Released on 27/08/2021

- New components 🎉
  - ProgressLine component (aka tui-rs LineGauge)
- Table/list state:
  - If table or list is set as `scrollable`, then `get_state` will return the index of the selected entry.
- ❗ Breaking changes ❗
  - Removed `public` access to components module. Just access to components and to props builder from `tui_realm_stdlib::COMPONENT_NAME`

## 0.6.0

Released on 03/08/2021

- Compatibility with `tui-realm 0.6.0`
- ❗ Breaking changes ❗
  - from now on `with_title` functions takes both title text and alignment

## 0.5.0

Released on 31/07/2021

- New components:
  - Added `BarChart` component
  - Added `Canvas` component
  - Added `Chart` component
  - Added `Select` component
  - Added `Sparkline` component
  - Added `Table` component
- Component changes:
  - **Checkbox**
    - `with_options` now takes only options
    - added `with_title` to set the title
    - `rewind` property
  - **Label**
    - Label now supports text alignment `with_text_alignment()`
  - **List**
    - From now on `with_rows` takes only the table
    - Added `with_title`
  - **Paragraph**
    - paragraph will now use `TuiParagraph` to render instead of `List`.
    - paragraph now supports text alignment `with_text_alignment()`
    - paragraph now supports wrap with trim `with_trim()`
    - Added `with_title` to set title
    - From now on `with_texts` only sets the texts for the paragraph
  - **ProgressBar**
    - added `with_title` to set the title
    - added `with_label` to set the label
    - removed `with_texts`
  - **Radio**
    - `with_options` now takes only options
    - added `with_title` to set the title
    - `rewind` property
  - **Select**
    - `with_options` now takes only options
    - `rewind` property
    - added `with_title` to set the title
  - **Span**
    - Span now supports text alignment `with_text_alignment()`
  - **Textarea**
    - From now on `with_texts` only sets the texts for the paragraph
    - Added `with_title` to set title
- ❗ Breaking changes ❗
  - ❗ Removed `TextSpanBuilder`, you can just use the same methods on `TextSpan` when creating it ❗
  - ❗ Renamed `Table` to `List` ❗
  - ❗ Removed `ScrollTable`; Use `List` with `scrollable(true)` instead ❗
