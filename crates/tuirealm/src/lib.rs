//! # tui-realm
//!
//! [tui-realm](https://github.com/veeso/tui-realm) is a [tui](https://github.com/fdehau/tui-rs) framework
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
use crossterm::event::{Event, KeyEvent};
use std::io::Stdout;
use tui::{backend::CrosstermBackend, layout::Rect, Frame};

// Modules
#[cfg(feature = "with-components")]
pub mod components;
pub mod props;
pub mod view;
// Export use
pub use self::props::{
    borders, texts, GenericPropsBuilder, InputType, PropValue, Props, PropsBuilder,
};
pub use self::view::View;

// -- Types

/// ## Canvas
///
/// Canvas represents the Frame where the view will be displayed in
pub type Canvas<'a> = Frame<'a, CrosstermBackend<Stdout>>;

// -- Msg

/// ## Msg
///
/// Msg is an enum returned after an event is raised for a certain component
/// Yep, I took inspiration from Elm.
#[derive(std::fmt::Debug, PartialEq, Eq)]
pub enum Msg {
    OnSubmit(Payload),
    OnChange(Payload),
    OnKey(KeyEvent),
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
    VecOfText(Vec<String>),
    VecOfUsize(Vec<usize>),
    None,
}

// -- Component

/// ## Component
///
/// Component is a trait which defines the behaviours for a View component.
pub trait Component {
    /// ### render
    ///
    /// Based on the current properties and states, renders the component in the provided area frame
    #[cfg(not(tarpaulin_include))]
    fn render(&self, frame: &mut Canvas, area: Rect);

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which returns
    /// the current properties, which can be used to create new properties.
    /// Returns a Msg to the view
    fn update(&mut self, props: Props) -> Msg;

    /// ### get_props
    ///
    /// Returns the current component properties.
    /// The returned properties can then be used to create a new PropsBuilder,
    /// which can lately be used to update the component's properties.
    fn get_props(&self) -> Props;

    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view
    fn on(&mut self, ev: Event) -> Msg;

    /// ### get_state
    ///
    /// Get current state from component
    fn get_state(&self) -> Payload;

    // -- state changers

    /// ### blur
    ///
    /// Blur component; basically remove focus
    fn blur(&mut self);

    /// ### active
    ///
    /// Active component; basically give focus
    fn active(&mut self);
}
