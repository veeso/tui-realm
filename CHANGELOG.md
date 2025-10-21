# Changelog

- [Changelog](#changelog)
  - [2.0.1](#201)
  - [2.0.0](#200)
  - [1.0.0](#100)

---

## next

Unreleased

- Feat: Add field-level attribute `#[component]`.
- Feat: Support Tuple structs. The component has to be the 0th field.
- Fix: support container-level `#[component = "field"]` syntax (in addition to the previous only `#[component("field")]`).
- **MSRV**: 1.85.1

## 2.0.1

Released on 08/06/2025

- Fix: Added support for structs with lifetime and generics
- It is now possible to specify the component to apply the `#derive(MockComponent)` to, using the `#[component("ComponentName")]` attribute.
- **MSRV**: 1.64

## 2.0.0

Released on 12/10/2024

- Changes to code to be compatible with ratatui

## 1.0.0

Released on 13/11/2021

First release
