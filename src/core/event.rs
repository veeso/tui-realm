//! `events` contains all the types related to [`Event`].

use bitflags::bitflags;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

// -- event

/// An event raised by a port.
#[derive(Debug, Eq, PartialEq, Clone, PartialOrd)]
pub enum Event<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone,
{
    /// A keyboard event
    Keyboard(KeyEvent),
    /// A Mouse event
    Mouse(MouseEvent),
    /// This event is raised after the terminal window is resized.
    ///
    /// Contains (width, height).
    WindowResize(u16, u16),
    /// Window focus gained
    FocusGained,
    /// Window focus lost
    FocusLost,
    /// Clipboard content pasted
    Paste(String),
    /// A ui tick event (should be configurable)
    Tick,
    /// Unhandled event; Empty event
    None,
    /// User event; won't be used by standard library or by default input event listener;
    /// but can be used in user defined ports
    User(UserEvent),
}

impl<UserEvent> Event<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone,
{
    pub(crate) fn as_keyboard(&self) -> Option<&KeyEvent> {
        if let Event::Keyboard(k) = self {
            Some(k)
        } else {
            None
        }
    }

    pub(crate) fn as_mouse(&self) -> Option<&MouseEvent> {
        if let Event::Mouse(m) = self {
            Some(m)
        } else {
            None
        }
    }

    pub(crate) fn as_window_resize(&self) -> bool {
        matches!(self, Self::WindowResize(_, _))
    }

    pub(crate) fn as_tick(&self) -> bool {
        matches!(self, Self::Tick)
    }

    pub(crate) fn as_user(&self) -> Option<&UserEvent> {
        if let Event::User(u) = self {
            Some(u)
        } else {
            None
        }
    }
}

/// When using [`Event`], but dont need custom `UserEvent`s, you can use this as type parameter instead of defining a custom empty type.
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd)]
pub enum NoUserEvent {}

// -- keyboard

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

/// Defines the possible Keyboard keys.
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
    /// Caps lock pressed
    CapsLock,
    /// Scroll lock pressed
    ScrollLock,
    /// Num lock pressed
    NumLock,
    /// Print screen key
    PrintScreen,
    /// Pause key
    Pause,
    /// Menu key
    Menu,
    /// keypad begin
    KeypadBegin,
    /// Media key
    Media(MediaKeyCode),
    /// Escape key.
    Esc,

    /// Shift left
    ShiftLeft,
    /// Alt left; warning: it is supported only on termion
    AltLeft,
    /// warning: it is supported only on termion
    CtrlLeft,
    /// warning: it is supported only on termion
    ShiftRight,
    /// warning: it is supported only on termion
    AltRight,
    /// warning: it is supported only on termion
    CtrlRight,
    /// warning: it is supported only on termion
    ShiftUp,
    /// warning: it is supported only on termion
    AltUp,
    /// warning: it is supported only on termion
    CtrlUp,
    /// warning: it is supported only on termion
    ShiftDown,
    /// warning: it is supported only on termion
    AltDown,
    /// warning: it is supported only on termion
    CtrlDown,
    /// warning: it is supported only on termion
    CtrlHome,
    /// warning: it is supported only on termion
    CtrlEnd,
}

/// Defines the modifier states, such as shift, control and alt.
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug, PartialOrd, Ord)]
#[cfg_attr(feature = "serialize", derive(Deserialize, Serialize))]
pub struct KeyModifiers(u8);

bitflags! {
    impl KeyModifiers: u8 {
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
        Self::new(k, KeyModifiers::NONE)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(Deserialize, Serialize),
    serde(tag = "type", content = "args")
)]
/// Defines Possible Media-related keys.
pub enum MediaKeyCode {
    /// Play media key.
    Play,
    /// Pause media key.
    Pause,
    /// Play/Pause media key.
    PlayPause,
    /// Reverse media key.
    Reverse,
    /// Stop media key.
    Stop,
    /// Fast-forward media key.
    FastForward,
    /// Rewind media key.
    Rewind,
    /// Next-track media key.
    TrackNext,
    /// Previous-track media key.
    TrackPrevious,
    /// Record media key.
    Record,
    /// Lower-volume media key.
    LowerVolume,
    /// Raise-volume media key.
    RaiseVolume,
    /// Mute media key.
    MuteVolume,
}

/// A Mouse event
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(Deserialize, Serialize),
    serde(tag = "type")
)]
pub struct MouseEvent {
    /// The kind of mouse event that was caused
    pub kind: MouseEventKind,
    /// The key modifiers active when the event occurred
    pub modifiers: KeyModifiers,
    /// The column that the event occurred on
    pub column: u16,
    /// The row that the event occurred on
    pub row: u16,
}

/// Defines what kind of [`MouseEvent`] is possible
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(Deserialize, Serialize),
    serde(tag = "type", content = "args")
)]
pub enum MouseEventKind {
    /// Pressed mouse button. Contains the button that was pressed
    Down(MouseButton),
    /// Released mouse button. Contains the button that was released
    Up(MouseButton),
    /// Moved the mouse cursor while pressing the contained mouse button
    Drag(MouseButton),
    /// Moved / Hover changed without pressing any buttons
    Moved,
    /// Scrolled mouse wheel downwards
    ScrollDown,
    /// Scrolled mouse wheel upwards
    ScrollUp,
    /// Scrolled mouse wheel left
    ScrollLeft,
    /// Scrolled mouse wheel right
    ScrollRight,
}

/// Defines all possible mouse buttons
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(Deserialize, Serialize),
    serde(tag = "type", content = "args")
)]
pub enum MouseButton {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::mock::MockEvent;

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
        assert_eq!(k.modifiers, KeyModifiers::NONE);
    }

    #[test]
    fn check_events() {
        let e: Event<MockEvent> = Event::Keyboard(KeyEvent::new(Key::Down, KeyModifiers::CONTROL));
        assert!(e.as_keyboard().is_some());
        assert_eq!(e.as_window_resize(), false);
        assert_eq!(e.as_tick(), false);
        assert_eq!(e.as_mouse().is_some(), false);
        assert!(e.as_user().is_none());
        let e: Event<MockEvent> = Event::WindowResize(0, 24);
        assert!(e.as_window_resize());
        assert!(e.as_keyboard().is_none());
        let e: Event<MockEvent> = Event::Tick;
        assert!(e.as_tick());
        let e: Event<MockEvent> = Event::User(MockEvent::Bar);
        assert_eq!(e.as_user().unwrap(), &MockEvent::Bar);

        let e: Event<MockEvent> = Event::Mouse(MouseEvent {
            kind: MouseEventKind::Moved,
            modifiers: KeyModifiers::NONE,
            column: 0,
            row: 0,
        });
        assert!(e.as_mouse().is_some());
        assert_eq!(e.as_keyboard().is_some(), false);
        assert_eq!(e.as_tick(), false);
        assert_eq!(e.as_window_resize(), false);
    }

    // -- serde
    #[cfg(feature = "serialize")]
    use std::fs::File;
    #[cfg(feature = "serialize")]
    use std::io::{Read, Write};

    #[cfg(feature = "serialize")]
    use serde::de::DeserializeOwned;
    #[cfg(feature = "serialize")]
    use serde::{Deserialize, Serialize};
    #[cfg(feature = "serialize")]
    use tempfile::NamedTempFile;

    #[cfg(feature = "serialize")]
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

    #[cfg(feature = "serialize")]
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

    #[cfg(feature = "serialize")]
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct KeyBindings {
        pub quit: KeyEvent,
        pub open: KeyEvent,
    }

    #[cfg(feature = "serialize")]
    impl KeyBindings {
        pub fn new(quit: KeyEvent, open: KeyEvent) -> Self {
            Self { quit, open }
        }
    }

    #[test]
    #[cfg(feature = "serialize")]
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
