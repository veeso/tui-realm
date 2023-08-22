# tui-realm-stdlib

<p align="center">
  <img src="/docs/images/tui-realm.svg" width="256" height="256" />
</p>

<p align="center">~ A tui-rs framework inspired by Elm and React ~</p>
<p align="center">
  <a href="https://github.com/veeso/tuirealm_derive" target="_blank">tui-realm derive</a>
  ·
  <a href="https://github.com/veeso/tui-realm" target="_blank">tui-realm</a>
  ·
  <a href="https://docs.rs/tui-realm-stdlib" target="_blank">Documentation</a>
</p>

<p align="center">Developed by <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Current version: 1.3.0 (22/08/2023)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/tui-realm-stdlib/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/tui-realm-stdlib.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/tui-realm-stdlib"
    ><img
      src="https://img.shields.io/crates/d/tui-realm-stdlib.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/tui-realm-stdlib"
    ><img
      src="https://img.shields.io/crates/v/tui-realm-stdlib.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/tui-realm-stdlib/actions"
    ><img
      src="https://github.com/veeso/tui-realm-stdlib/workflows/Build/badge.svg"
      alt="CI"
  /></a>
  <a href="https://docs.rs/tui-realm-stdlib"
    ><img
      src="https://docs.rs/tui-realm-stdlib/badge.svg"
      alt="Docs"
  /></a>
</p>

---

- [tui-realm-stdlib](#tui-realm-stdlib)
  - [About tui-realm-stdlib 👑](#about-tui-realm-stdlib-)
  - [Get started 🏁](#get-started-)
    - [Add tui-realm to your Cargo.toml 🦀](#add-tui-realm-to-your-cargotoml-)
  - [Support the developer ☕](#support-the-developer-)
  - [Components 🎨](#components-)
    - [Utilities](#utilities)
  - [Documentation 📚](#documentation-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [License 📃](#license-)

---

## About tui-realm-stdlib 👑

tui-realm-stdlib is the standard component library for [tui-realm](https://github.com/veeso/tui-realm).

It provides several **Mock Components** for your tui-realm applications. Probably all the components you need are here 😉

---

## Get started 🏁

### Add tui-realm to your Cargo.toml 🦀

```toml
tui-realm-stdlib = "^1.3.0"
```

or if you're not using the default **crossterm backend**, specify another backend in the cargo entry:

```toml
tui-realm-stdlib = { version = "^1.3.0", default-features = false, features = [ "ratatui", "termion" ] }
```

Latest version of tui-realm-stdlib requires **tui-realm 1.9.0** or higher

```toml
tuirealm = "^1.9.0"
```

---

## Support the developer ☕

If you like tui-realm and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Components 🎨

All the components implemented in the standard library can be viewed in the [components wiki](/docs/components.md).

---

### Utilities

The standard components library also exports the `utils` module, which provides these very handy functions:

- **wrap_spans**: Creates span lines from text spans, in order to wrap lines
- **use_or_default_styles**: use colors and modifiers of the text spans if not `Color::Reset` or `Modifiers::empty()`, otherwise use the properties defined the `Props`.
- **get_block**: creates the block for the widget. If focus is true, the colors are applied, otherwise `Color::Reset`.

---

## Documentation 📚

The developer documentation can be found on Rust Docs at <https://docs.rs/tui-realm-stdlib>

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
