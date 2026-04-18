# tui-realm

<p align="center">
  <img src="crates/tuirealm/docs/images/tui-realm.svg" alt="logo" width="256" height="256" />
</p>

<p align="center">~ A ratatui framework inspired by Elm and React ~</p>
<p align="center">
  <a href="crates/tuirealm/docs/en/get-started.md" target="_blank">Get started</a>
  ·
  <a href="https://github.com/veeso/tui-realm/tree/main/crates/tuirealm-stdlib" target="_blank">Standard Library</a>
  ·
  <a href="https://docs.rs/tuirealm" target="_blank">Documentation</a>
</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/tui-realm/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/tui-realm.svg?style=plain"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/tuirealm"
    ><img
      src="https://img.shields.io/crates/d/tuirealm.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/tuirealm"
    ><img
      src="https://img.shields.io/crates/v/tuirealm.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
<a href="https://github.com/veeso/tui-realm/actions/workflows/tests.yml"
    ><img
      src="https://github.com/veeso/tui-realm/actions/workflows/tests.yml/badge.svg"
      alt="CI"
  /></a>
  <a href="https://coveralls.io/github/veeso/tui-realm"
    ><img
      src="https://coveralls.io/repos/github/veeso/tui-realm/badge.svg"
      alt="Coveralls"
  /></a>
  <a href="https://docs.rs/tuirealm"
    ><img
      src="https://docs.rs/tuirealm/badge.svg"
      alt="Docs"
  /></a>
</p>

---

- [tui-realm](#tui-realm)
  - [About tui-realm 👑](#about-tui-realm-)
  - [Features 🎁](#features-)
  - [Get Started](#get-started)
  - [Changelogs](#changelogs)

---

## About tui-realm 👑

`tui-realm` is a **framework** for **[ratatui](https://github.com/ratatui-org/ratatui)** to simplify the implementation of terminal user interfaces adding the possibility to work with re-usable components with properties and states, as you'd do in React. But that's not all: the components communicate with the ui engine via a system based on **Messages** and **Events**, providing you with the possibility to implement `update` routines as happens in Elm. In addition, the components are organized inside the **View**, which manages mounting/umounting, focus and event forwarding for you.

And that also explains the reason of the name: **Realm stands for React and Elm**.

tui-realm also comes with a standard library of components, which can be added to your dependencies, that you may find very useful.

![Demo](crates/tuirealm/docs/images/demo.gif)

See tui-realm in action in either the [core examples](crates/tuirealm/examples/) or in the [stdlib examples](crates/tuirealm-stdlib/examples/).

## Features 🎁

- ⌨️ **Event-driven**
- ⚛️ Based on **React** and **Elm**
- 🍲 **Boilerplate** code
- 🚀 Quick-setup
- 🎯 Single **focus** and **states** management
- 🙂 Easy to learn
- 🤖 Adaptable to any use case

---

## Get Started

This is a monorepo of the following crates:

- [tuirealm](crates/tuirealm/): The core crate containing all basic functionality
- [tui-realm-stdlib](crates/tuirealm-stdlib/): The standard library, which provides convenience wrappers for standard ratatui widgets
- [tuirealm_derive](crates/tuirealm-derive/): A helper derive library to derive `Component` for components which just pass those functions to a underlying component
- [tui-realm-treeview](crates/tuirealm-treeview/): A Tree Component implementation
- [tui-realm-textarea](crates/tuirealm-textarea/): A Text Area Component implementation

## Changelogs

See [CHANGELOG.md](CHANGELOG.md).
