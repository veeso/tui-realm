//! This module exposes the component traits

use std::any::Any;

use ratatui::Frame;

use super::event::Event;
use super::props::{AttrValue, Attribute};
use super::state::State;
use crate::command::{Cmd, CmdResult};
use crate::ratatui::layout::Rect;

/// The base component trait. Defines properties, states, rendering, and command handling for a reusable component.
///
/// A `Component` won't define how to behave after an [`Event`] and it won't send any `Msg`.
/// It is intended to be used as a reusable building block to implement your [`AppComponent`].
///
/// ### In practice
///
/// A real life example would be an Input field.
/// The component is represented by the `Input`, which will define the properties (e.g. max input length, input type, ...)
/// and by its behaviour (e.g. when the user types 'a', 'a' char is added to input state).
///
/// In your application though, you may use a `IpAddressInput` which is the [`AppComponent`] using the `Input` component.
/// If you want more example, just dive into the `examples/` folder in the project root.
pub trait Component {
    /// Based on the current properties and states, renders the component in the provided area frame.
    /// Render can also mutate the component state if this is required
    fn view(&mut self, frame: &mut Frame, area: Rect);

    /// Query attribute of component properties.
    fn query(&self, attr: Attribute) -> Option<AttrValue>;

    /// Set attribute to properties.
    /// `query` describes the name, while `attr` the value it'll take
    fn attr(&mut self, attr: Attribute, value: AttrValue);

    /// Get current state from component
    fn state(&self) -> State;

    /// Perform a command on the component.
    /// The command will may change the component state.
    /// The method returns the result of the command applied (what changed if any)
    fn perform(&mut self, cmd: Cmd) -> CmdResult;
}

/// The app component describes the application level component, which is a wrapper around [`Component`],
/// which, in addition to all the methods exposed by the base component, it will handle the [`Event`]s coming from the `View`.
///
/// The Event are passed to the `on` method, which will eventually return a `Msg`,
/// which is defined in your application as an enum.
/// In your application you should have an AppComponent for each element on your UI, but the logic to implement
/// is very tiny, since the most of the work should already be done into the [`Component`]
/// and many of them are available in the standard library at [`tui-realm-stdlib`](https://github.com/veeso/tui-realm/tree/main/crates/tui-realm-stdlib).
///
/// Don't forget you can find an example in the `examples/` directory and you can discover many more information
/// about components in the repository documentation.
pub trait AppComponent<Msg, UserEvent>: Component + Any
where
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone,
{
    /// Handle input event and update internal states.
    /// Returns a Msg to the view.
    /// If [`None`] is returned it means there's no message to return for the provided event.
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Msg>;
}

impl<Msg, UserEvent> dyn AppComponent<Msg, UserEvent>
where
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone,
{
    /// Convenience function to cast to [`Any`].
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    /// Convenience function to cast to [`Any`] mutably.
    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
