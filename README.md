# tui-realm

<p align="center">
  <img src="docs/images/tui-realm.svg" width="256" height="256" />
</p>

<p align="center">~ A tui-rs framework inspired by Elm and React ~</p>
<p align="center">
  <a href="/docs/get-started.md" target="_blank">Get started</a>
  ·
  <a href="https://github.com/veeso/tui-realm-stdlib" target="_blank">Standard Library</a>
  ·
  <a href="https://docs.rs/tuirealm" target="_blank">Documentation</a>
</p>

<p align="center">Developed by <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Current version: 1.1.0 (21/11/2021)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/tui-realm/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/tui-realm.svg"
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
  <a href="https://www.buymeacoffee.com/veeso">
    <img
      src="https://img.shields.io/badge/Donate-BuyMeACoffee-yellow.svg"
      alt="Buy me a coffee"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/tui-realm/actions"
    ><img
      src="https://github.com/veeso/tui-realm/workflows/Crossterm/badge.svg"
      alt="Crossterm CI"
  /></a>
  <a href="https://github.com/veeso/tui-realm/actions"
    ><img
      src="https://github.com/veeso/tui-realm/workflows/Crossterm%20%28Windows%29/badge.svg"
      alt="Crossterm CI (Windows)"
  /></a>
  <a href="https://github.com/veeso/tui-realm/actions"
    ><img
      src="https://github.com/veeso/tui-realm/workflows/Termion/badge.svg"
      alt="Termion CI"
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
  - [Get started 🏁](#get-started-)
    - [Add tui-realm to your Cargo.toml 🦀](#add-tui-realm-to-your-cargotoml-)
    - [Create a tui-realm application 🪂](#create-a-tui-realm-application-)
    - [Run examples 🔍](#run-examples-)
  - [Standard components library 🎨](#standard-components-library-)
  - [Community components 🏘️](#community-components-️)
  - [Guides 🎓](#guides-)
  - [Documentation 📚](#documentation-)
  - [Apps using tui-realm 🚀](#apps-using-tui-realm-)
  - [Legacy API 📜](#legacy-api-)
  - [Support the developer ☕](#support-the-developer-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [License 📃](#license-)

---

## About tui-realm 👑

tui-realm is a **framework** for [tui](https://github.com/fdehau/tui-rs) to simplify the implementation of terminal user interfaces adding the possibility to work with re-usable components with properties and states, as you'd do in React. But that's not all: the components communicate with the ui engine via a system based on **Messages** and **Events**, providing you with the possibility to implement `update` routines as happens in Elm. In addition, the components are organized inside the **View**, which manages mounting/umounting, focus and event forwarding for you.

And that's also explains the reason of the name: Realm stands for React and Elm.

tui-realm also comes with a standard library of components, which can be added to your dependencies, that you may find very useful. Don't worry, they are optional if you don't want to use them 😉, just follow the guide in [get started](#get-started-).

![Demo](/docs/images/demo.gif)

See tui-realm in action in the [Example](#run-examples) or if you want to read more about tui-realm start reading the official guide [HERE](docs/en/get-started.md).

## Features 🎁

- ⌨️ **Event-driven**
- ⚛️ Based on **React** and **Elm**
- 🍲 **Boilerplate** code
- 🚀 Quick-setup
- 🎯 Single **focus** and **states** management
- 🙂 Easy to learn
- 🤖 Adaptable to any use case

---

## Get started 🏁

> ⚠️ Warning: currently tui-realm supports these backends: crossterm, termion

### Add tui-realm to your Cargo.toml 🦀

If you want the default features, just add tuirealm 1.x version:

```toml
tuirealm = "^1.1.0"
```

otherwise you can specify the features you want to add:

```toml
tuirealm = { version = "^1.1.0", default-features = false, features = [ "derive", "with-termion" ] }
```

Supported features are:

- `derive` (*default*): add the `#[derive(MockComponent)]` proc macro to automatically implement `MockComponent` for `Component`. [Read more](https://github.com/veeso/tuirealm_derive).
- `with-crossterm` (*default*): use [crossterm](https://github.com/crossterm-rs/crossterm) as backend for tui.
- `with-termion`: use [termion](https://github.com/redox-os/termion) as backend for tui.

> ⚠️ You can enable only one backend at the time and at least one must be enabled in order to build.  
> ❗ You don't need tui as a dependency, since you can access to tui types via `use tuirealm::tui::`

### Create a tui-realm application 🪂

View how to implement a tui-realm application in the [related guide](/docs/get-started.md).

### Run examples 🔍

Still confused about how tui-realm works? Don't worry, try with the examples:

- [demo](/examples/demo.rs): a simple application which shows how tui-realm works

    ```sh
    cargo run --example demo
    ```

---

## Standard components library 🎨

Tui-realm comes with an optional standard library of components I thought would have been useful for most of the applications.
If you want to use it, just add the [tui-realm-stdlib](https://github.com/veeso/tui-realm-stdlib) to your `Cargo.toml` dependencies.

## Community components 🏘️

These components are not included in tui-realm, but have been developed by other users. I like advertising other's contents, so here you can find a list of components you may find useful for your next tui-realm project 💜.

- [tui-realm-treeview](https://github.com/veeso/tui-realm-treeview) A treeview component developed by [@veeso](https://github.com/veeso)

Want to add yours? Open an issue using the `New app/component` template 😄

---

## Guides 🎓

- [Get Started Guide](/docs/en/get-started.md)
- [Advanced concepts](/docs/en/advanced.md)
- [Migrating from tui-realm 0.x to 1.x](/docs/en/migrating-legacy.md)

---

## Documentation 📚

The developer documentation can be found on Rust Docs at <https://docs.rs/tuirealm>

---

## Apps using tui-realm 🚀

- [termusic](https://github.com/tramhao/termusic)
- [termscp](https://github.com/veeso/termscp)
- [tuifeed](https://github.com/veeso/tuifeed)

Want to add yours? Open an issue using the `New app/component` template 😄

## Legacy API 📜

Looking for the old ugly tui-realm API? You can find it [here](https://github.com/veeso/tui-realm/tree/legacy)

---

## Support the developer ☕

If you like tui-realm and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![Buy-me-a-coffee](https://img.shields.io/badge/-buy_me_a%C2%A0coffee-gray?style=for-the-badge&logo=buy-me-a-coffee)](https://www.buymeacoffee.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve tui-realm, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View tui-realm's changelog [HERE](CHANGELOG.md)

---

## License 📃

tui-realm is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
