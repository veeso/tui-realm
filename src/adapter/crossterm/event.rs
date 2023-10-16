//! ## Event
//!
//! event adapter for crossterm

use crossterm::event::{
    Event as XtermEvent, KeyCode as XtermKeyCode, KeyEvent as XtermKeyEvent,
    KeyEventKind as XtermEventKind, KeyModifiers as XtermKeyModifiers,
    MediaKeyCode as XtermMediaKeyCode,
};

use super::{Event, Key, KeyEvent, KeyModifiers, MediaKeyCode};

impl<U> From<XtermEvent> for Event<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    fn from(e: XtermEvent) -> Self {
        match e {
            XtermEvent::Key(key) if key.kind == XtermEventKind::Press => Self::Keyboard(key.into()),
            XtermEvent::Key(_) => Self::None,
            XtermEvent::Mouse(_) => Self::None,
            XtermEvent::Resize(w, h) => Self::WindowResize(w, h),
            XtermEvent::FocusGained => Self::FocusGained,
            XtermEvent::FocusLost => Self::FocusLost,
            XtermEvent::Paste(clipboard) => Self::Paste(clipboard),
        }
    }
}

impl From<XtermKeyEvent> for KeyEvent {
    fn from(e: XtermKeyEvent) -> Self {
        Self {
            code: e.code.into(),
            modifiers: e.modifiers.into(),
        }
    }
}

impl From<XtermKeyCode> for Key {
    fn from(k: XtermKeyCode) -> Self {
        match k {
            XtermKeyCode::BackTab => Self::BackTab,
            XtermKeyCode::Backspace => Self::Backspace,
            XtermKeyCode::Char(ch) => Self::Char(ch),
            XtermKeyCode::Delete => Self::Delete,
            XtermKeyCode::Down => Self::Down,
            XtermKeyCode::End => Self::End,
            XtermKeyCode::Enter => Self::Enter,
            XtermKeyCode::Esc => Self::Esc,
            XtermKeyCode::F(f) => Self::Function(f),
            XtermKeyCode::Home => Self::Home,
            XtermKeyCode::Insert => Self::Insert,
            XtermKeyCode::Left => Self::Left,
            XtermKeyCode::Null | XtermKeyCode::Modifier(_) => Self::Null,
            XtermKeyCode::PageDown => Self::PageDown,
            XtermKeyCode::PageUp => Self::PageUp,
            XtermKeyCode::Right => Self::Right,
            XtermKeyCode::Tab => Self::Tab,
            XtermKeyCode::Up => Self::Up,
            XtermKeyCode::CapsLock => Self::CapsLock,
            XtermKeyCode::ScrollLock => Self::ScrollLock,
            XtermKeyCode::NumLock => Self::NumLock,
            XtermKeyCode::PrintScreen => Self::PrintScreen,
            XtermKeyCode::Pause => Self::Pause,
            XtermKeyCode::Menu => Self::Menu,
            XtermKeyCode::KeypadBegin => Self::KeypadBegin,
            XtermKeyCode::Media(m) => Self::Media(m.into()),
        }
    }
}

impl From<XtermKeyModifiers> for KeyModifiers {
    fn from(k: XtermKeyModifiers) -> Self {
        let mut km = KeyModifiers::empty();
        if k.intersects(XtermKeyModifiers::SHIFT) {
            km.insert(KeyModifiers::SHIFT);
        }
        if k.intersects(XtermKeyModifiers::CONTROL) {
            km.insert(KeyModifiers::CONTROL);
        }
        if k.intersects(XtermKeyModifiers::ALT) {
            km.insert(KeyModifiers::ALT);
        }
        km
    }
}

impl From<XtermMediaKeyCode> for MediaKeyCode {
    fn from(m: XtermMediaKeyCode) -> Self {
        match m {
            XtermMediaKeyCode::Play => Self::Play,
            XtermMediaKeyCode::Pause => Self::Pause,
            XtermMediaKeyCode::PlayPause => Self::PlayPause,
            XtermMediaKeyCode::Reverse => Self::Reverse,
            XtermMediaKeyCode::Stop => Self::Stop,
            XtermMediaKeyCode::FastForward => Self::FastForward,
            XtermMediaKeyCode::Rewind => Self::Rewind,
            XtermMediaKeyCode::TrackNext => Self::TrackNext,
            XtermMediaKeyCode::TrackPrevious => Self::TrackPrevious,
            XtermMediaKeyCode::Record => Self::Record,
            XtermMediaKeyCode::LowerVolume => Self::LowerVolume,
            XtermMediaKeyCode::RaiseVolume => Self::RaiseVolume,
            XtermMediaKeyCode::MuteVolume => Self::MuteVolume,
        }
    }
}

#[cfg(test)]
mod test {

    use crossterm::event::{MouseEvent as XtermMouseEvent, MouseEventKind as XtermMouseEventKind};
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::mock::MockEvent;

    #[test]
    fn adapt_crossterm_keycode() {
        assert_eq!(Key::from(XtermKeyCode::BackTab), Key::BackTab);
        assert_eq!(Key::from(XtermKeyCode::Backspace), Key::Backspace);
        assert_eq!(Key::from(XtermKeyCode::Char('b')), Key::Char('b'));
        assert_eq!(Key::from(XtermKeyCode::Delete), Key::Delete);
        assert_eq!(Key::from(XtermKeyCode::Down), Key::Down);
        assert_eq!(Key::from(XtermKeyCode::End), Key::End);
        assert_eq!(Key::from(XtermKeyCode::Enter), Key::Enter);
        assert_eq!(Key::from(XtermKeyCode::Esc), Key::Esc);
        assert_eq!(Key::from(XtermKeyCode::F(0)), Key::Function(0));
        assert_eq!(Key::from(XtermKeyCode::Home), Key::Home);
        assert_eq!(Key::from(XtermKeyCode::Insert), Key::Insert);
        assert_eq!(Key::from(XtermKeyCode::Left), Key::Left);
        assert_eq!(Key::from(XtermKeyCode::Null), Key::Null);
        assert_eq!(Key::from(XtermKeyCode::PageDown), Key::PageDown);
        assert_eq!(Key::from(XtermKeyCode::PageUp), Key::PageUp);
        assert_eq!(Key::from(XtermKeyCode::Right), Key::Right);
        assert_eq!(Key::from(XtermKeyCode::Tab), Key::Tab);
        assert_eq!(Key::from(XtermKeyCode::Up), Key::Up);
    }

    #[test]
    fn adapt_crossterm_key_modifiers() {
        assert_eq!(
            KeyModifiers::from(
                XtermKeyModifiers::CONTROL | XtermKeyModifiers::SHIFT | XtermKeyModifiers::ALT
            ),
            KeyModifiers::all()
        );
        assert_eq!(
            KeyModifiers::from(XtermKeyModifiers::ALT),
            KeyModifiers::ALT
        );
    }

    #[test]
    fn should_adapt_media_key() {
        assert_eq!(
            MediaKeyCode::from(XtermMediaKeyCode::Play),
            MediaKeyCode::Play
        );
        assert_eq!(
            MediaKeyCode::from(XtermMediaKeyCode::Pause),
            MediaKeyCode::Pause
        );
        assert_eq!(
            MediaKeyCode::from(XtermMediaKeyCode::PlayPause),
            MediaKeyCode::PlayPause
        );
        assert_eq!(
            MediaKeyCode::from(XtermMediaKeyCode::Reverse),
            MediaKeyCode::Reverse
        );
        assert_eq!(
            MediaKeyCode::from(XtermMediaKeyCode::Stop),
            MediaKeyCode::Stop
        );
        assert_eq!(
            MediaKeyCode::from(XtermMediaKeyCode::FastForward),
            MediaKeyCode::FastForward
        );
        assert_eq!(
            MediaKeyCode::from(XtermMediaKeyCode::Rewind),
            MediaKeyCode::Rewind
        );
        assert_eq!(
            MediaKeyCode::from(XtermMediaKeyCode::TrackNext),
            MediaKeyCode::TrackNext
        );
        assert_eq!(
            MediaKeyCode::from(XtermMediaKeyCode::TrackPrevious),
            MediaKeyCode::TrackPrevious
        );
        assert_eq!(
            MediaKeyCode::from(XtermMediaKeyCode::Record),
            MediaKeyCode::Record
        );
        assert_eq!(
            MediaKeyCode::from(XtermMediaKeyCode::LowerVolume),
            MediaKeyCode::LowerVolume
        );
        assert_eq!(
            MediaKeyCode::from(XtermMediaKeyCode::RaiseVolume),
            MediaKeyCode::RaiseVolume
        );
        assert_eq!(
            MediaKeyCode::from(XtermMediaKeyCode::MuteVolume),
            MediaKeyCode::MuteVolume
        );
    }

    #[test]
    fn adapt_crossterm_key_event() {
        assert_eq!(
            KeyEvent::from(XtermKeyEvent::new(
                XtermKeyCode::Backspace,
                XtermKeyModifiers::CONTROL
            )),
            KeyEvent::new(Key::Backspace, KeyModifiers::CONTROL)
        );
    }

    #[test]
    fn adapt_crossterm_event() {
        type AppEvent = Event<MockEvent>;
        assert_eq!(
            AppEvent::from(XtermEvent::Resize(24, 48)),
            Event::WindowResize(24, 48)
        );
        assert_eq!(
            AppEvent::from(XtermEvent::Key(XtermKeyEvent::from(
                XtermKeyCode::Backspace
            ))),
            Event::Keyboard(KeyEvent::from(Key::Backspace))
        );
        assert_eq!(
            AppEvent::from(XtermEvent::Mouse(XtermMouseEvent {
                kind: XtermMouseEventKind::Moved,
                column: 0,
                row: 0,
                modifiers: XtermKeyModifiers::empty(),
            })),
            Event::None
        );
        assert_eq!(
            AppEvent::from(XtermEvent::FocusGained),
            AppEvent::FocusGained
        );
        assert_eq!(AppEvent::from(XtermEvent::FocusLost), AppEvent::FocusLost);
        assert_eq!(
            AppEvent::from(XtermEvent::Paste(String::from("a"))),
            AppEvent::Paste(String::from("a"))
        );
    }
}
