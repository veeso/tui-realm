# Changelog

- [Changelog](#changelog)
  - [1.0.0](#100)
  - [0.3.0](#030)
  - [0.2.1](#021)
  - [0.2.0](#020)
  - [0.1.1](#011)
  - [0.1.0](#010)

---

## 1.0.0

Released on 14/11/2021

- Migrated component to tui-realm 1.x
- Total refactoring; using orange-trees as engine

## 0.3.0

Released on 12/08/2021

- tui-realm 0.6.0 compatibility
- added `alignment` to `with_title`

## 0.2.1

Released on 02/08/2021

- tui-realm 0.5.1 compatibility

## 0.2.0

Released on 07/06/2021

- **Keep state property**:
  - Possibility to keep active the selected node after an update
- **With node property**:
  - Set the id of the default active node in the properties
- **PAGE_UP** and **PAGE_DOWN** keys
  - Advance by remaining siblings forward or backward
  - You can set a maximum amount of steps with `with_steps` in props
- Dependencies:
  - `tui-realm` updated to `0.4.0`

## 0.1.1

Released on 07/06/2021

- Added `root_mut`
- Added `query_mut`

## 0.1.0

Released on 06/06/2021

- First release
