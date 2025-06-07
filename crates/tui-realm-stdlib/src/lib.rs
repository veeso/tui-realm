//! # tui-realm-stdlib
//!
//! [tui-realm-stdlib](https://github.com/veeso/tui-realm-stdlib) is the standard library component for [tui-realm](https://github.com/veeso/tui-realm).
//! This library provides you with all the essential components you'll need to build a tui-realm application.
//!
//! ## Get Started
//!
//! ### Adding `tui-realm-stdlib` as dependency
//!
//! ```toml
//! tuirealm = "3"
//! tui-realm-stdlib = "3"
//! ```
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/tui-realm-stdlib/main/docs/images/cargo/tui-realm-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/tui-realm-stdlib/main/docs/images/cargo/tui-realm-512.png"
)]

mod components;
pub mod utils;
pub use components::props;
pub use components::*;
