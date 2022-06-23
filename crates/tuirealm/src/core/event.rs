//! ## events
//!
//! `events` exposes the event raised by a user interaction or by the runtime

use bitflags::bitflags;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// -- event

/// ## Event
///
/// An event raised by a user interaction
#[derive(Debug, Eq, PartialEq, Clone, PartialOrd)]
pub enum Event<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + PartialOrd,
{
    /// A keyboard event
    Keyboard(KeyEvent),
    /// This event is raised after the terminal window is resized
    WindowResize(u16, u16),
    /// A ui tick event (should be configurable)
    Tick,
    /// Unhandled event; Empty event
    None,
    /// User event; won't be used by standard library or by default input event listener;
    /// but can be used in user defined ports
    User(UserEvent),
}

impl<U> Event<U>
where
    U: Eq + PartialEq + Clone + PartialOrd,
{
    pub(crate) fn is_keyboard(&self) -> Option<&KeyEvent> {
        if let Event::Keyboard(k) = self {
            Some(k)
        } else {
            None
        }
    }

    pub(crate) fn is_window_resize(&self) -> bool {
        matches!(self, Self::WindowResize(_, _))
    }

    pub(crate) fn is_tick(&self) -> bool {
        matches!(self, Self::Tick)
    }

    pub(crate) fn is_user(&self) -> Option<&U> {
        if let Event::User(u) = self {
            Some(u)
        } else {
            None
        }
    }
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
#[cfg_attr(
    feature = "serialize",
    derive(Deserialize, Serialize),
    serde(tag = "type")
)]
pub struct KeyEvent {
    pub code: Key,
    pub modifiers: KeyModifiers,
}

/// ## Key
///
/// A keyboard event
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(Deserialize, Serialize),
    serde(tag = "type", content = "args")
)]
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
    #[cfg_attr(feature = "serialize", derive(Deserialize, Serialize), serde(tag = "type"))]
    pub struct KeyModifiers: u8 {
        const NONE = 0b0000_0000;
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
    use crate::mock::MockEvent;

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

    #[test]
    fn check_events() {
        let e: Event<MockEvent> = Event::Keyboard(KeyEvent::new(Key::Down, KeyModifiers::CONTROL));
        assert!(e.is_keyboard().is_some());
        assert_eq!(e.is_window_resize(), false);
        assert_eq!(e.is_tick(), false);
        assert_eq!(e.is_tick(), false);
        assert!(e.is_user().is_none());
        let e: Event<MockEvent> = Event::WindowResize(0, 24);
        assert!(e.is_window_resize());
        assert!(e.is_keyboard().is_none());
        let e: Event<MockEvent> = Event::Tick;
        assert!(e.is_tick());
        let e: Event<MockEvent> = Event::User(MockEvent::Bar);
        assert_eq!(e.is_user().unwrap(), &MockEvent::Bar);
    }

    // -- serde
    #[cfg(feature = "serialize")]
    use serde::de::DeserializeOwned;
    #[cfg(feature = "serialize")]
    use serde::{Deserialize, Serialize};
    #[cfg(feature = "serialize")]
    use std::fs::File;
    #[cfg(feature = "serialize")]
    use std::io::{Read, Write};
    #[cfg(feature = "serialize")]
    use tempfile::NamedTempFile;

    #[cfg(feature = "serde")]
    fn deserialize<R, S>(mut readable: R) -> S
    where
        R: Read,
        S: DeserializeOwned + Sized + std::fmt::Debug,
    {
        // Read file content
        let mut data: String = String::new();
        if let Err(err) = readable.read_to_string(&mut data) {
            panic!("Error: {}", err);
        }
        // Deserialize
        match toml::de::from_str(data.as_str()) {
            Ok(deserialized) => deserialized,
            Err(err) => panic!("Error: {}", err),
        }
    }

    #[cfg(feature = "serde")]
    fn serialize<S, W>(serializable: &S, mut writable: W)
    where
        S: Serialize + Sized,
        W: Write,
    {
        // Serialize content
        let data: String = match toml::ser::to_string(serializable) {
            Ok(dt) => dt,
            Err(err) => {
                panic!("Error: {}", err);
            }
        };
        // Write file
        if let Err(err) = writable.write_all(data.as_bytes()) {
            panic!("Error: {}", err)
        }
    }

    #[cfg(feature = "serde")]
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct KeyBindings {
        pub quit: KeyEvent,
        pub open: KeyEvent,
    }

    #[cfg(feature = "serde")]
    impl KeyBindings {
        pub fn new(quit: KeyEvent, open: KeyEvent) -> Self {
            Self { quit, open }
        }
    }

    #[test]
    #[cfg(feature = "serde")]
    fn should_serialize_key_bindings() {
        let temp = NamedTempFile::new().expect("Failed to open tempfile");
        let keys = KeyBindings::new(
            KeyEvent::from(Key::Esc),
            KeyEvent::new(Key::Char('o'), KeyModifiers::CONTROL),
        );
        let mut config = File::create(temp.path()).expect("Failed to open file for write");
        serialize(&keys, &mut config);
        let mut readable = File::open(temp.path()).expect("Failed to open file for read");
        let r_keys: KeyBindings = deserialize(&mut readable);
        assert_eq!(keys, r_keys);
    }
}
