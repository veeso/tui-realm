#![cfg_attr(docsrs, feature(doc_cfg))]

//! # tui-realm
//!
//! tui-realm is a **framework** for **[ratatui](https://github.com/ratatui-org/ratatui)**
//! to simplify the implementation of terminal user interfaces adding the possibility to work
//! with re-usable components with properties and states, as you'd do in React. But that's not all:
//! the components communicate with the ui engine via a system based on **Messages** and **Events**,
//! providing you with the possibility to implement `update` routines as happens in Elm.
//!
//! In addition, the components are organized inside the **View**, which manages mounting/umounting,
//! focus and event forwarding for you.
//!
//! tui-realm also comes with a standard library of components, which can be added to your dependencies,
//! that you may find very useful.
//!
//! ## Get started 🏁
//!
//! > ⚠️ Warning: currently tui-realm supports these backends: crossterm, termion
//!
//! ### Add tui-realm to your Cargo.toml 🦀
//!
//! If you want the default features, just add tuirealm 1.x version:
//!
//! ```toml
//! tuirealm = "3"
//! ```
//!
//! otherwise you can specify the features you want to add:
//!
//! ```toml
//! tuirealm = { version = "3", default-features = false, features = [ "derive", "serialize", "termion" ] }
//! ```
//!
//! Supported features are:
//!
//! - `derive` (*default*): add the `#[derive(MockComponent)]` proc macro to automatically implement `MockComponent` for `Component`. [Read more](https://github.com/veeso/tuirealm_derive).
//! - `async-ports`: add support for async ports
//! - `serialize`: add the serialize/deserialize trait implementation for `KeyEvent` and `Key`.
//! - `crossterm`: use the [crossterm](https://github.com/crossterm-rs/crossterm) terminal backend
//! - `termion`: use the [termion](https://github.com/redox-os/termion) terminal backend
//!
//! ### Create a tui-realm application 🪂
//!
//! You can read the guide to get started with tui-realm on [Github](https://github.com/veeso/tui-realm/blob/main/docs/en/get-started.md)
//!
//! ### Run examples 🔍
//!
//! Still confused about how tui-realm works? Don't worry, try with the examples:
//!
//! - [demo](https://github.com/veeso/tui-realm/blob/main/examples/demo.rs): a simple application which shows how tui-realm works
//!
//!    ```sh
//!    cargo run --example demo
//!    ```
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/tui-realm/main/docs/images/cargo/tui-realm-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/tui-realm/main/docs/images/cargo/tui-realm-512.png"
)]

#[macro_use]
extern crate lazy_regex;
extern crate self as tuirealm;
#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate tuirealm_derive;

mod core;
pub mod listener;
pub mod macros;
#[cfg(test)]
pub mod mock;
pub mod ratatui;
pub mod terminal;
pub mod utils;
// export async trait for async-ports
#[cfg(feature = "async-ports")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-ports")))]
pub use async_trait::async_trait;
pub use listener::{EventListenerCfg, ListenerError};
// -- derive
#[cfg(feature = "derive")]
#[doc(hidden)]
pub use tuirealm_derive::*;

pub use self::core::application::{self, Application, ApplicationError, PollStrategy};
pub use self::core::event::{self, Event, NoUserEvent};
pub use self::core::injector::Injector;
pub use self::core::props::{self, AttrValue, Attribute, Props};
pub use self::core::subscription::{EventClause as SubEventClause, Sub, SubClause};
pub use self::core::{Component, MockComponent, State, StateValue, Update, ViewError, command};
pub use self::ratatui::Frame;
