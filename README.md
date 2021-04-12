# tui-realm

[![License: MIT](https://img.shields.io/badge/License-MIT-teal.svg)](https://opensource.org/licenses/MIT) [![Stars](https://img.shields.io/github/stars/veeso/tui-realm.svg)](https://github.com/veeso/tui-realm) [![Downloads](https://img.shields.io/crates/d/tui-realm.svg)](https://crates.io/crates/tui-realm) [![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/crates/tui-realm) [![Docs](https://docs.rs/tui-realm/badge.svg)](https://docs.rs/tui-realm)  

[![Build](https://github.com/veeso/tui-realm/workflows/Linux/badge.svg)](https://github.com/veeso/tui-realm/actions) [![Build](https://github.com/veeso/tui-realm/workflows/MacOS/badge.svg)](https://github.com/veeso/tui-realm/actions) [![Build](https://github.com/veeso/tui-realm/workflows/Windows/badge.svg)](https://github.com/veeso/tui-realm/actions) [![codecov](https://codecov.io/gh/veeso/tui-realm/branch/main/graph/badge.svg?token=au67l7nQah)](https://codecov.io/gh/veeso/tui-realm)

Developed by Christian Visintin  
Current version: 0.1.0 FIXME: (??/04/2021)

---

- [tui-realm](#tui-realm)
  - [About tui-realm 👑](#about-tui-realm-)
    - [Why tui-realm 🤔](#why-tui-realm-)
  - [Get started 🏁](#get-started-)
  - [Documentation 📚](#documentation-)
  - [About other backends](#about-other-backends)
  - [Known issues 🧻](#known-issues-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [Buy me a coffee ☕](#buy-me-a-coffee-)
  - [License 📃](#license-)

---

## About tui-realm 👑

tui-realm is a **framework** for [tui](https://github.com/fdehau/tui-rs) which provides a layer to simplify the implementation of terminal user interfaces adding the possibility to work with re-usable component with properties and state, as you'd do in React; but that's not all: the input events are handled through a system based on **Messages**, providing you with the possibility to implement `update` functions as happens in Elm.

And that's also explains the reason of the name: Realm stands for React and Elm.

### Why tui-realm 🤔

Personally I didn't start this project from scratch. I've just decided to make a library out of the already existing code in [termscp](https://github.com/veeso/termscp), which I had just finished at the time I started this project. I thought this library could have come handy for somebody.

You might be wondering now how much is this project influenced by the development of termscp. Well, a lot actually, I won't deny this, so don't expect this library to always try to fit the community needs, I'm just providing you with a tool I've made for myself, but that I wanted to share with the community.

---

## Get started 🏁

⚠ Warning: tui-realm works only with crossterm as backend ⚠

TODO: fill

---

## Documentation 📚

The developer documentation can be found on Rust Docs at <https://docs.rs/tui-realm>

---

## About other backends

TODO: fill

---

## Known issues 🧻

TODO: fill

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve tui-realm, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View tui-realm's changelog [HERE](CHANGELOG.md)

---

## Buy me a coffee ☕

If you like tui-realm and you're grateful for the work I've done, please consider a little donation 🥳

[![Buy-me-a-coffee](https://img.buymeacoffee.com/button-api/?text=Buy%20me%20a%20coffee&emoji=&slug=veeso&button_colour=404040&font_colour=ffffff&font_family=Comic&outline_colour=ffffff&coffee_colour=FFDD00)](https://www.buymeacoffee.com/veeso)

---

## License 📃

tui-realm is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
