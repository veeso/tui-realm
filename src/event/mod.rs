//! ## events
//!
//! `events` exposes the event raised by a user interaction or by the runtime

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
use bitflags::bitflags;
use std::fmt;

// -- modules
mod listener;
// -- export
pub use listener::{EventListener, EventListenerCfg, ListenerError, ListenerResult, Poll};

// -- event

/// ## Event
///
/// An event raised by a user interaction
#[derive(Debug, Eq, PartialEq, Clone, PartialOrd)]
pub enum Event<UserEvent>
where
    UserEvent: fmt::Debug + Eq + PartialEq + Clone + PartialOrd,
{
    /// A keyboard event
    Keyboard(KeyEvent),
    /// This event is raised after the terminal window is resized
    WindowResize(u16, u16),
    /// A ui tick event (should be configurable)
    Tick,
    /// Unhandled event; Empty event
    None,
    /// User event; won't be used by standard library or event listener;
    /// but can be used in user define event listeners
    User(UserEvent),
}

/// ## NoUserEvent
///
/// When using event you can use this as type parameter if you don't want to use user events
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd)]
pub enum NoUserEvent {}

// -- keyboard

/// ## KeyEvent
///
/// A keyboard event
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
pub struct KeyEvent {
    pub code: Key,
    pub modifiers: KeyModifiers,
}

/// ## Key
///
/// A keyboard event
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
pub enum Key {
    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page dow key.
    PageDown,
    /// Tab key.
    Tab,
    /// Shift + Tab key. (sugar)
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// Function key followed by index (F1 => `Key::Function(1)`)
    Function(u8),
    /// A character.
    ///
    /// `KeyCode::Char('c')` represents `c` character, etc.
    Char(char),
    /// Null.
    Null,
    /// Escape key.
    Esc,
}

bitflags! {
    /// ## KeyModifiers
    ///
    /// Defines special key states, such as shift, control, alt...
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
    }
}

impl KeyEvent {
    pub fn new(code: Key, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

impl From<Key> for KeyEvent {
    fn from(k: Key) -> Self {
        Self::new(k, KeyModifiers::empty())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn new_key_event() {
        let k = KeyEvent::new(Key::Down, KeyModifiers::CONTROL);
        assert_eq!(k.code, Key::Down);
        assert_eq!(k.modifiers, KeyModifiers::CONTROL);
    }

    #[test]
    fn key_event_from_key() {
        let k = KeyEvent::from(Key::Up);
        assert_eq!(k.code, Key::Up);
        assert_eq!(k.modifiers, KeyModifiers::empty());
    }
}

#[cfg(test)]
pub use mock::MockEvent;

#[cfg(test)]
pub mod mock {
    #[derive(Debug, Eq, PartialEq, Clone, PartialOrd)]
    pub enum MockEvent {
        None,
    }
}
