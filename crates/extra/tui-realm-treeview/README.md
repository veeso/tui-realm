# tui-realm-treeview

<p align="center">
  <img src="/docs/images/tui-realm-treeview.svg" width="256" height="256" />
</p>

<p align="center">~ Treeview component for tui-realm ~</p>
<p align="center">
  <a href="https://github.com/veeso/orange-trees" target="_blank">orange trees</a>
  ·
  <a href="https://github.com/veeso/tui-realm" target="_blank">tui-realm</a>
  ·
  <a href="https://docs.rs/tui-realm-treeview" target="_blank">Documentation</a>
</p>

<p align="center">Developed by <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Current version: 1.0.0 (FIXME:/10/2021)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/tui-realm-treeview/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/tui-realm-treeview.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/tui-realm-treeview"
    ><img
      src="https://img.shields.io/crates/d/tui-realm-treeview.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/tui-realm-treeview"
    ><img
      src="https://img.shields.io/crates/v/tui-realm-treeview.svg"
      alt="Latest version"
  /></a>
  <a href="https://www.buymeacoffee.com/veeso">
    <img
      src="https://img.shields.io/badge/Donate-BuyMeACoffee-yellow.svg"
      alt="Buy me a coffee"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/tui-realm-treeview/actions"
    ><img
      src="https://github.com/veeso/tui-realm-treeview/workflows/Build/badge.svg"
      alt="CI"
  /></a>
  <a href="https://coveralls.io/github/veeso/tui-realm-treeview"
    ><img
      src="https://coveralls.io/repos/github/veeso/tui-realm-treeview/badge.svg"
      alt="Coveralls"
  /></a>
  <a href="https://docs.rs/tui-realm-treeview"
    ><img
      src="https://docs.rs/tui-realm-treeview/badge.svg"
      alt="Docs"
  /></a>
</p>

---

- [tui-realm-treeview](#tui-realm-treeview)
  - [About tui-realm-treeview 🌲](#about-tui-realm-treeview-)
  - [Get started 🏁](#get-started-)
    - [Add tui-realm-treeview to your Cargo.toml 🦀](#add-tui-realm-treeview-to-your-cargotoml-)
    - [Use the treeview component](#use-the-treeview-component)
    - [About performance](#about-performance)
  - [Component](#component)
    - [Updating the tree](#updating-the-tree)
  - [Documentation 📚](#documentation-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [Support the developer ☕](#support-the-developer-)
  - [License 📃](#license-)

---

## About tui-realm-treeview 🌲

tui-realm-treeview is an implementation of a **treeview component** for [tui-realm](https://github.com/veeso/tui-realm).
It uses the [Orange trees](https://github.com/veeso/orange-trees) engine for implementing trees.

![Demo](docs/images/demo.gif)

---

## Get started 🏁

### Add tui-realm-treeview to your Cargo.toml 🦀

```toml
tui-realm-treeview = "^1.0.0"
```

Or if you don't use **Crossterm**, define the backend as you do with tui-realm:

```toml
tui-realm-treeview = { version = "^1.0.0", default-features = false, features = [ "with-termion" ] }
```

### Use the treeview component

View how to use the treeview-component following the [example](examples/demo.rs). The example contains a simple file explorer using a tree view, the depth is set to 3.

```sh
cargo run --example demo
```

- Press `ENTER` to expand the selected directory
- Press `BACKSPACE` to go to upper directory
- Move up and down with `UP/DOWN` arrow keys
- Advance by up to 8 entries with `PGUP/PGDOWN`
- Open directories with `RIGHT`
- Close directories with `LEFT`
- Change window between input field and treeview with `TAB`
- Press `ESC` to quit

### About performance

In this library there is a consistent use of recursion, and since rust is not functional, this might lead to stack overflows when dealing with huge trees. In addition consider that each level of depth added, will slow down the application exponentially.

Best practices:

- Except when dealing with small trees, always set a depth for the tree
- For file systems, depth 3 should be fine for each directory, then expand each directory on demand as implemented in the example

---

## Component

**Commands**:

| Cmd                       | Result           | Behaviour                                            |
|---------------------------|------------------|------------------------------------------------------|
| `Custom($TREE_CMD_CLOSE)` | `None`           | Close selected node                                  |
| `Custom($TREE_CMD_OPEN)`  | `None`           | Open selected node                                   |
| `GoTo(Begin)`             | `Changed | None` | Move cursor to the top of the current tree node      |
| `GoTo(End)`               | `Changed | None` | Move cursor to the bottom of the current tree node   |
| `Move(Down)`              | `Changed | None` | Go to next element                                   |
| `Move(Up)`                | `Changed | None` | Go to previous element                               |
| `Scroll(Down)`            | `Changed | None` | Move cursor down by defined max steps or end of node |
| `Scroll(Up)`              | `Changed | None` | Move cursor up by defined max steps or begin of node |
| `Submit`                  | `Submit`         | Just returns submit result with current state        |

**State**: the state returned is a `One(String)` containing the id of the selected node. If no node is selected `None` is returned.

**Properties**:

- `Background(Color)`: background color. The background color will be used as background for unselected entry, but will be used as foreground for the selected entry when focus is true
- `Borders(Borders)`: set borders properties for component
- `Custom($TREE_IDENT_SIZE, Size)`: Set space to render for each each depth level
- `Custom($TREE_INITIAL_NODE, String)`: Select initial node in the tree. This option has priority over `keep_state`
- `Custom($TREE_PRESERVE_STATE, Flag)`: If true, the selected entry will be kept after an update of the tree (obviously if the entry still exists in the tree).
- `FocusStyle(Style)`: inactive style
- `Foreground(Color)`: foreground color. The foreground will be used as foreground for the selected item, when focus is false, otherwise as background
- `HighlightedColor(Color)`: The provided color will be used to highlight the selected node. `Foreground` will be used if unset.
- `HighlightedStr(String)`: The provided string will be displayed on the left side of the selected entry in the tree
- `ScrollStep(Length)`: Defines the maximum amount of rows to scroll
- `TextProps(TextModifiers)`: set text modifiers
- `Title(Title)`: Set box title

### Updating the tree

FIXME: to be defined

---

## Documentation 📚

The developer documentation can be found on Rust Docs at <https://docs.rs/tui-realm-treeview>

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve tui-realm, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View tui-realm-treeview's changelog [HERE](CHANGELOG.md)

---

## Support the developer ☕

If you like tui-realm and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![Buy-me-a-coffee](https://img.shields.io/badge/-buy_me_a%C2%A0coffee-gray?style=for-the-badge&logo=buy-me-a-coffee)](https://www.buymeacoffee.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## License 📃

tui-realm-treeview is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
