# Changelog

- [Changelog](#changelog)
  - [1.0.0](#100)
  - [0.6.1](#061)
  - [0.6.0](#060)
  - [0.5.0](#050)

---

## 1.0.0

Released on FIXME:

- ...

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
