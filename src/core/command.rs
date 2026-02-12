//! This module exposes the [`Cmd`] type, which must be used when sending commands to the [`MockComponent`](crate::MockComponent) from the
//! [`Component`](crate::Component) after an `Event`.
use alloc::vec::Vec;

use super::State;

// -- Command

/// A command defines the "abstract" operation to perform in front of an Event.
/// The command must be passed in the `on` method of the `Component`
/// when calling `perform` method of the `MockComponent`.
/// There is not a default conversion from `Event -> Cmd`, but it must be implmented by the user in the
/// `Component` in a match case.
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
pub enum Cmd {
    /// Describes a "user" typed a character
    Type(char),
    /// Describes a "cursor" movement, or a movement of another kind
    Move(Direction),
    /// An expansion of `Move` which defines the scroll. The step should be defined in props, if any.
    Scroll(Direction),
    /// Describes a movement with a position
    GoTo(Position),
    /// User submit field
    Submit,
    /// User "deleted" something
    Delete,
    /// User "cancelled" something; used to distinguish between Del and Canc
    Cancel,
    /// User toggled something
    Toggle,
    /// User changed something
    Change,
    /// A user defined amount of time has passed and the component should be updated
    Tick,
    /// A user defined command type. You won't find these kind of Command in the stdlib, but you can use them in your own components.
    Custom(&'static str),
    /// `None` won't do anything
    None,
}

/// Defines the 4 2D directions a cursor can move.
/// This may be used after a `Arrow::Up` event or if you want something more geeky
/// when using `WASD`
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

/// Describes specific positions. Mostly used for exact cursor movement.
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
pub enum Position {
    Begin,
    End,
    At(usize),
}

// -- Command result

/// A command result describes the output of a [`Cmd`] performed on a Component.
/// It reports a "logical" change on the `MockComponent`.
/// The `Component` then, must return a certain user defined `Msg` based on the value of the [`CmdResult`].
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum CmdResult {
    /// The component has changed state. The new state is reported.
    /// Box is used to reduce size
    Changed(State),
    /// Value submit result
    Submit(State),
    /// The command could not be applied. Useful to report errors
    Invalid(Cmd),
    /// Custom cmd result
    Custom(&'static str, State),
    /// An array of Command result
    Batch(Vec<CmdResult>),
    /// No result to report
    None,
}
