//! # Component
//!
//! This module exposes the component traits

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
use crate::tui::layout::Rect;
use crate::{
    command::{Cmd, CmdResult},
    AttrValue, Attribute, Event, Frame, State, View,
};

use std::hash::Hash;

/// ## MockComponent
///
/// A Mock Component represents a component which defines all the properties and states it can handle and represent
/// and the way it should be rendered. It must also define how to behave in case of a `Cmd` (command).
/// Despite that, it won't define how to behave after an `Event` and it won't send any `Msg`.
/// The MockComponent is intended to be used as a reusable component to implement your application component.
///
/// ### In practice
///
/// A real life example would be an Input field.
/// The mock component is represented by the `Input`, which will define the properties (e.g. max input length, input type, ...)
/// and by its behaviour (e.g. when the user types 'a', 'a' char is added to input state).
///
/// In your application though, you may use a `IpAddressInput` which is the `Component` using the `Input` mock component.
/// If you want more example, just dive into the `examples/` folder in the project root.
pub trait MockComponent {
    /// ### view
    ///
    /// Based on the current properties and states, renders the component in the provided area frame.
    /// Render can also mutate the component state if this is required
    fn view(&mut self, frame: &mut Frame, area: Rect);

    /// ### query
    ///
    /// Query attribute of component properties.
    fn query(&self, attr: Attribute) -> Option<AttrValue>;

    /// ### attr
    ///
    /// Set attribute to properties.
    /// `query` describes the name, while `attr` the value it'll take
    fn attr(&mut self, attr: Attribute, value: AttrValue);

    /// ### state
    ///
    /// Get current state from component
    fn state(&self) -> State;

    /// ### perform
    ///
    /// Perform a command on the component.
    /// The command will may change the component state.
    /// The method returns the result of the command applied (what changed if any)
    fn perform(&mut self, cmd: Cmd) -> CmdResult;
}

/// ## Component
///
/// The component describes the application level component, which is a wrapper around the `MockComponent`,
/// which, in addition to all the methods exposed by the mock, it will handle the event coming from the `View`.
/// The Event are passed to the `on` method, which will eventually return a `Msg`,
/// which is defined in your application as an enum. (Don't forget to derive `PartialEq` for your enum).
/// In your application you should have a Component for each element on your UI, but the logic to implement
/// is very tiny, since the most of the work should already be done into the `MockComponent`
/// and many of them are available in the standard library at <https://github.com/veeso/tui-realm-stdlib>.
///
/// Don't forget you can find an example in the `examples/` directory and you can discover many more information
/// about components in the repository documentation.
pub trait Component<Msg, UserEvent>: MockComponent
where
    Msg: PartialEq,
    UserEvent: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd,
{
    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view.
    /// If `None` is returned it means there's no message to return for the provided event.
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg>;
}

// -- update

/// ## Update
///
/// The update trait defines the prototype of the function to be used to handle the events coming from the View.
pub trait Update<ComponentId, Msg, UserEvent>
where
    ComponentId: std::fmt::Debug + Eq + PartialEq + Clone + Hash,
    Msg: PartialEq,
    UserEvent: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd,
{
    /// ### update
    ///
    /// update the current state handling a message from the view.
    /// This function may return a Message,
    /// so this function has to be intended to be call recursively if necessary
    fn update(
        &mut self,
        view: &mut View<ComponentId, Msg, UserEvent>,
        msg: Option<Msg>,
    ) -> Option<Msg>;
}
