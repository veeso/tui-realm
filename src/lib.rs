#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! tui-realm is a **framework** for **[ratatui](https://github.com/ratatui-org/ratatui)**
//! to simplify the implementation of terminal user interfaces adding the possibility to work
//! with re-usable components with properties and states, as you'd do in React. But that's not all:
//! the components communicate with the ui engine via a system based on **Messages** and **Events**,
//! providing you with the possibility to implement `update` routines as happens in Elm.
//!
//! In addition, the components are organized inside the **View**, which manages mounting/umounting,
//! focus and event forwarding for you.
//!
//! `tui-realm` also comes with a standard library of components, that you may find very useful.
//! The stdlib can be found in [`tui-realm-stdlib`](https://docs.rs/tui-realm-stdlib/latest/tui_realm_stdlib/)
//!
//! ## Get started 🏁
//!
//! ### Add tui-realm to your Cargo.toml 🦀
//!
//! If you want the default features:
//!
//! ```toml
//! tuirealm = "3"
//! ```
//!
//! Alternatively you can specify the features you want to add:
//!
//! ```toml
//! tuirealm = { version = "3", default-features = false, features = ["std", "derive", "serialize", "crossterm" ] }
//! ```
//!
//! Supported features are:
//!
//! - `std` (*default*): enable std library support. **Mutually exclusive with `alloc`**.
//! - `alloc`: enable `no_std` with heap allocation support (for embedded systems). **Mutually exclusive with `std`**. Requires disabling default features.
//! - `derive` (*default*): add the `#[derive(MockComponent)]` proc macro to automatically implement `MockComponent` for `Component`. [Read more](https://github.com/veeso/tuirealm_derive).
//! - `async-ports`: add support for async ports (requires `std`)
//! - `serialize`: add the serialize/deserialize trait implementation for `KeyEvent` and `Key`.
//! - `crossterm` (*default*): enable the [crossterm](https://github.com/crossterm-rs/crossterm) terminal backend
//! - `termion`: enable the [termion](https://github.com/redox-os/termion) terminal backend
//! - `termwiz`: enable the [termwiz](https://docs.rs/termwiz/latest/termwiz/index.html) terminal backend
//!
//! ### For `no_std` environments (e.g., Commodore C64, embedded systems)
//!
//! ```toml
//! tuirealm = { version = "3", default-features = false, features = ["alloc", "derive", "serialize"] }
//! ```
//!
//! Note: Terminal backends (`crossterm`, `termion`, `termwiz`) and `async-ports` require `std` and cannot be used in `no_std` environments.
//!
//! ### Create a tui-realm application 🪂
//!
//! You can read the [Get Started guide](https://github.com/veeso/tui-realm/blob/main/docs/en/get-started.md) guide on github.
//!
//! ### Run examples 🔍
//!
//! Still confused about how tui-realm works? Don't worry, try with the examples:
//!
//! - [demo](https://github.com/veeso/tui-realm/blob/main/examples/demo/demo.rs): a simple example that shows basic tui-realm usage
//! - [user-events](https://github.com/veeso/tui-realm/blob/main/examples/user_events/user_events.rs): showcase using custom events
//! - [inline-display](https://github.com/veeso/tui-realm/blob/main/examples/inline_display.rs): showcase how tui-realm can be used without requiring a alternate screen
//! - [async-ports](https://github.com/veeso/tui-realm/blob/main/examples/async_ports.rs): showcase usage of async ports
//! - [arbitrary-data](https://github.com/veeso/tui-realm/blob/main/examples/arbitrary_data.rs): showcase usage of `PropPayload::Any` to send custom data across `query` and `attr`
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/tui-realm/main/docs/images/cargo/tui-realm-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/tui-realm/main/docs/images/cargo/tui-realm-512.png"
)]

// Compile-time check: std and alloc are mutually exclusive
#[cfg(all(feature = "std", feature = "alloc"))]
compile_error!("feature \"std\" and feature \"alloc\" cannot be enabled at the same time...");

extern crate alloc;

#[macro_use]
extern crate lazy_regex;
extern crate self as tuirealm;
#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate tuirealm_derive;

mod core;
pub mod listener;
mod macros;
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

#[cfg(feature = "std")]
pub use self::core::application::Application;
pub use self::core::application::{self, ApplicationError, CoreApplication, PollStrategy};
pub use self::core::event::{self, Event, NoUserEvent};
pub use self::core::injector::Injector;
pub use self::core::props::{self, AttrValue, Attribute, Props};
pub use self::core::subscription::{EventClause as SubEventClause, Sub, SubClause};
pub use self::core::{Component, MockComponent, State, StateValue, Update, ViewError, command};
pub use self::ratatui::Frame;
pub use self::utils::time::Clock;
#[cfg(feature = "std")]
pub use self::utils::time::StdClock;
