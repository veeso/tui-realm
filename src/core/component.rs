//! # Component
//!
//! This module exposes the component traits

use ratatui::Frame;

use crate::command::{Cmd, CmdResult};
use crate::ratatui::layout::Rect;
use crate::{AttrValue, Attribute, Event, State};

/// A Mock Component represents a component which defines all the properties and states it can handle and represent
/// and the way it should be rendered. It must also define how to behave in case of a [`Cmd`] (command).
/// Despite that, it won't define how to behave after an [`Event`] and it won't send any `Msg`.
/// The MockComponent is intended to be used as a reusable component to implement your application component.
///
/// ### In practice
///
/// A real life example would be an Input field.
/// The mock component is represented by the `Input`, which will define the properties (e.g. max input length, input type, ...)
/// and by its behaviour (e.g. when the user types 'a', 'a' char is added to input state).
///
/// In your application though, you may use a `IpAddressInput` which is the [`Component`] using the `Input` mock component.
/// If you want more example, just dive into the `examples/` folder in the project root.
pub trait MockComponent {
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

/// The component describes the application level component, which is a wrapper around the [`MockComponent`],
/// which, in addition to all the methods exposed by the mock, it will handle the event coming from the `View`.
/// The Event are passed to the `on` method, which will eventually return a `Msg`,
/// which is defined in your application as an enum. (Don't forget to derive [`PartialEq`] for your enum).
/// In your application you should have a Component for each element on your UI, but the logic to implement
/// is very tiny, since the most of the work should already be done into the [`MockComponent`]
/// and many of them are available in the standard library at <https://github.com/veeso/tui-realm-stdlib>.
///
/// Don't forget you can find an example in the `examples/` directory and you can discover many more information
/// about components in the repository documentation.
pub trait Component<Msg, UserEvent>: MockComponent
where
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone + PartialOrd,
{
    /// Handle input event and update internal states.
    /// Returns a Msg to the view.
    /// If [`None`] is returned it means there's no message to return for the provided event.
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg>;
}
