//! ## Core
//!
//! Core implements the core functionalities and types for tui-realm

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
pub mod application;
pub mod command;
mod component;
pub mod event;
pub mod props;
mod state;
pub mod subscription;
mod view;

// -- export
pub use command::Cmd;
pub use component::{Component, MockComponent};
pub use state::{State, StateValue};
pub use view::{View, ViewError};

// -- internal
pub(crate) use subscription::Subscription;
pub(crate) use view::WrappedComponent;

// -- Update

/// ## Update
///
/// The update trait defines the prototype of the function to be used to handle the events coming from the View.
pub trait Update<Msg>
where
    Msg: PartialEq,
{
    /// ### update
    ///
    /// update the current state handling a message from the view.
    /// This function may return a Message,
    /// so this function has to be intended to be call recursively if necessary
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg>;
}
