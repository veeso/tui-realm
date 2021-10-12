//! # tui-realm
//!
//! [tui-realm](https://github.com/veeso/tui-realm) is a [tui](https://github.com/fdehau/tui-rs) framework
//! to build applications with a React/Elm inspired-by approach
//!
//! ## Get Started
//!
//! ### Adding `tui-realm` as dependency
//!
//! ```toml
//! tuirealm = "1.0.0"
//! ```
//!
//! ## Examples
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/tui-realm/main/docs/images/cargo/tui-realm-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/tui-realm/main/docs/images/cargo/tui-realm-512.png"
)]

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate thiserror;
extern crate tui as tuirs;

// -- modules
pub mod adapter;
mod core;
pub mod listener;
#[cfg(test)]
pub mod mock;
// TODO: terminal
pub mod tui;
pub mod utils;
// -- export
pub use self::core::command::{self, Cmd, CmdResult};
pub use self::core::event::{self, Event};
pub use self::core::props::{self, AttrSelector, Attribute, Props, PropsBuilder};
pub use self::core::subscription;
pub use self::core::{Component, MockComponent, Update, View};
pub use self::core::{State, Value};
pub use adapter::{Frame, Terminal};
