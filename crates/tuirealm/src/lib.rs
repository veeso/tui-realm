//! # tui-realm
//!
//! [tui-realm](https://github.com/veeso/tui-realm) is a [tui-rs](https://github.com/fdehau/tui-rs) framework
//! to build applications with a React/Elm inspired-by approach
//!
//! ## Get Started
//!
//! TODO: put from readme
//!
//! ## Update
//!
//! TODO: update
//!
//! ## Components
//!
//! TODO: write

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
extern crate crossterm;
extern crate tui;

// Ext
use crossterm::event::{KeyEvent, MouseEvent};
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Frame;

// Modules
pub mod component;
pub mod props;
pub mod view;

// Export use
// TODO:
pub use self::component::Component;
pub use self::props::{
    borders, texts, GenericPropsBuilder, InputType, PropValue, Props, PropsBuilder,
};
pub use self::view::View;

// -- Types

/// ## Canvas
///
/// Canvas represents the Frame where the view will be displayed in
pub type Canvas<'a> = Frame<'a, CrosstermBackend<Stdout>>;

/// ## Msg
///
/// Msg is an enum returned after an event is raised for a certain component
/// Yep, I took inspiration from Elm.
#[derive(std::fmt::Debug, PartialEq, Eq)]
pub enum Msg {
    OnSubmit(Payload),
    OnChange(Payload),
    OnKey(KeyEvent),
    OnMouse(MouseEvent),
    None,
}

/// ## Payload
///
/// Payload describes a component value
#[derive(std::fmt::Debug, PartialEq, Eq)]
pub enum Payload {
    Boolean(bool),
    Signed(isize),
    Text(String),
    Unsigned(usize),
    None,
}
