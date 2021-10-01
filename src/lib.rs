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
//! tuirealm = "0.6.0"
//! ```
//!
//! ## Examples
//!
//! See the `examples` directory to check out how to setup tui-realm.
//! If you want a real in-production implementation, check out my project
//! [termscp](https://github.com/veeso/termscp)
//!
//! ## Update
//!
//! The update function is used to handle the messages returned from the view. The name `update` is just a convention, you can call this function as you prefer.
//! I suggest you to use update, though, since it's like in Elm.
//! The usual signature of this function is `fn update(model: &mut MyModel, msg: Option<(String, Msg)>) -> Option<(String, Msg)>`
//! We don't have to mind what model is right now, it may be everything, probably it contains the View and some states, who cares;
//! what is important here is the `msg`. The message is a tuple made up by a String, which is the identifier of the component which raised the event
//! and a `Msg` enum, which contains the message itself.
//! What can we do with this then? Well, we can make a super-cool match case where we match the tuple (ID, Msg) and for each branch, we can make everything we want.
//! And as you've probably already noticed this function returns a Message tuple too. This should already have triggered you! Yes, you can call this function
//! recursively and chain different events.
//! Just a friendly reminder: you can obviously match only the event or only the ID of the component in the match case, just mind of the top-bottom priority ;)
//!
//! ### Match Keys in Update
//!
//! If you're in trouble handling key events, you're not a bad dev, it happened to me too the first time. To handle them, my suggestion is to create a keymap module
//! where you store as const all the key you need. Just follow this example:
//!
//! ```rust,no_run
//! extern crate crossterm;
//! extern crate tuirealm;
//!
//! use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
//! use tuirealm::Msg;
//!
//! // -- keys
//!
//! pub const MSG_KEY_ESC: Msg = Msg::OnKey(KeyEvent {
//!     code: KeyCode::Esc,
//!     modifiers: KeyModifiers::NONE,
//! });
//!
//! pub const MSG_KEY_TAB: Msg = Msg::OnKey(KeyEvent {
//!     code: KeyCode::Tab,
//!     modifiers: KeyModifiers::NONE,
//! });
//! ```
//!
//! Then you'll be able to match them as in the examples: `(MY_COMPONENT, &MSG_KEY_TAB) => {...}`
//!
//! ## Components
//!
//! Components represents a logical layer above a tui widget. Components allows you to handle properties and states for widgets and this will be rendered.
//! The component is implemented through a trait which is called `Component` indeed. For each component you must implement these methods:
//!
//! - render: this method renders the component inside the area passed as argument. Mind that the component should be rendered only if `props.visible` is `true`.
//! - update: update the component properties. You can return a `Msg` if you want and change the states if needed.
//! - get_props: returns a copy of the component properties
//! - on: handle an input event; returns a Msg
//! - get_state: returns the current state to be exposed to the user.
//! - blur: change the focus state for the component. This won't affect the view.
//! - active: enable the focus state for the component. This won't affect the view.
//!
//! Each component should have properties, provided by the `Props` struct and, if required
//! some states, which are defined for each struct. States mustn't be public.
//! I strongly suggest to implement a `PropsBuilder`, via the namesake trait, for each component properties, in order to make easier to define
//! properties for each component, while if you don't want to do so, you can use the `GenericPropsBuilder`.
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
extern crate tui as tuirs;

// Ext
use std::io::Stdout;
use tuirs::{backend::CrosstermBackend, layout::Rect, Frame as TuiFrame};

// -- modules
pub mod event;
pub mod legacy;
//pub mod props;
pub mod tui;
mod view;
// -- export
/*
pub use self::props::{
    borders, texts, GenericPropsBuilder, InputType, PropPayload, PropValue, Props, PropsBuilder,
};
 */
pub use event::Event;
pub use view::View;

// -- Types

/// ## Frame
///
/// Frame represents the Frame where the view will be displayed in
pub type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;

// -- Component traits

/// ## StaticComponent
///
/// TODO: define
pub trait StaticComponent {
    /// ### render
    ///
    /// Based on the current properties and states, renders the component in the provided area frame
    fn render(&mut self, frame: &mut Frame, area: Rect);

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which returns
    /// the current properties, which can be used to create new properties.
    fn update(&mut self, props: Props);

    /// ### get_props
    ///
    /// Returns the current component properties.
    /// The returned properties can then be used to create a new PropsBuilder,
    /// which can lately be used to update the component's properties.
    fn get_props(&self) -> Props;

    /// ### get_state
    ///
    /// Get current state from component
    fn get_state(&self) -> State;

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

/// ## Component
///
/// TODO: complete
pub trait Component<Msg>: StaticComponent {
    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view
    fn on(&mut self, ev: Event) -> Msg;
}

// -- update

/// ## Update
pub trait Update<Msg> {
    /// ### update
    ///
    /// update the current state handling a message from the view.
    /// This function may return a Message, so this function has to be intended to be call recursively.
    fn update(&mut self, msg: Option<(&str, Msg)>) -> Option<(&str, Msg)>;
}

#[cfg(test)]
mod test {}
