//! ## Command
//!
//! This module exposes the Command type, which must be used when sending command to the `MockComponent` from the
//! `Component` after an `Event`.

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
use super::State;

// -- Command

/// ## Cmd
///
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
    /// User submit field
    Submit,
    /// User "deleted" something
    Delete,
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

/// ## Direction
///
/// Defines the 4 directions in front of a cursor movement.
/// This may be used after a `Arrow::Up` event or for example if you want something more geeky
/// when using `WASD`
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

// -- Command result

/// ## CmdResult
///
/// A command result describes the output of a `Cmd` performed on a Component.
/// It reports a "logical" change on the `MockComponent`.
/// The `Component` then, must return a certain user defined `Msg` based on the value of the `CmdResult`.
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum CmdResult {
    /// The component has changed state. The new state is reported.
    /// Box is used to reduce size
    Changed(State),
    /// The command could not be applied. Useful to report errors
    Invalid(Cmd),
    /// Custom cmd result
    Custom(&'static str),
    /// No result to report
    None,
}
