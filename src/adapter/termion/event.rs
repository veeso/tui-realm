//! ## Event
//!
//! event adapter for termion

use super::{Event, Key, KeyEvent, KeyModifiers};

use termion::event::{Event as TonEvent, Key as TonKey};

impl<U> From<TonEvent> for Event<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    fn from(e: TonEvent) -> Self {
        match e {
            TonEvent::Key(key) => Self::Keyboard(key.into()),
            _ => Self::None,
        }
    }
}

impl From<TonKey> for KeyEvent {
    fn from(e: TonKey) -> Self {
        // Get modifiers
        let modifiers = match e {
            TonKey::Alt(c) if c.is_uppercase() => KeyModifiers::ALT | KeyModifiers::SHIFT,
            TonKey::Ctrl(c) if c.is_uppercase() => KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            TonKey::Char(c) if c.is_uppercase() => KeyModifiers::SHIFT,
            TonKey::Alt(_) => KeyModifiers::ALT,
            TonKey::Ctrl(_) => KeyModifiers::CONTROL,
            _ => KeyModifiers::NONE,
        };
        let code = match e {
            TonKey::Alt('\n') | TonKey::Char('\n') | TonKey::Ctrl('\n') => Key::Enter,
            TonKey::Alt('\t') | TonKey::Char('\t') | TonKey::Ctrl('\t') => Key::Tab,
            TonKey::Alt(c) | TonKey::Char(c) | TonKey::Ctrl(c) => Key::Char(c.to_ascii_lowercase()),
            TonKey::BackTab => Key::BackTab,
            TonKey::Backspace => Key::Backspace,
            TonKey::Delete => Key::Delete,
            TonKey::Down => Key::Down,
            TonKey::End => Key::End,
            TonKey::Left => Key::Left,
            TonKey::Right => Key::Right,
            TonKey::Up => Key::Up,
            TonKey::Home => Key::Home,
            TonKey::PageUp => Key::PageUp,
            TonKey::PageDown => Key::PageDown,
            TonKey::Insert => Key::Insert,
            TonKey::F(f) => Key::Function(f),
            TonKey::Null => Key::Null,
            TonKey::Esc => Key::Esc,
            TonKey::__IsNotComplete => Key::Null,
        };
        Self { code, modifiers }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::MockEvent;

    use pretty_assertions::assert_eq;

    use termion::event::MouseEvent;

    #[test]
    fn adapt_termion_key_event() {
        assert_eq!(
            KeyEvent::from(TonKey::BackTab),
            KeyEvent::from(Key::BackTab)
        );
        assert_eq!(
            KeyEvent::from(TonKey::Backspace),
            KeyEvent::from(Key::Backspace)
        );
        assert_eq!(
            KeyEvent::from(TonKey::Char('b')),
            KeyEvent::from(Key::Char('b'))
        );
        assert_eq!(
            KeyEvent::from(TonKey::Ctrl('b')),
            KeyEvent::from(KeyEvent {
                code: Key::Char('b'),
                modifiers: KeyModifiers::CONTROL,
            })
        );
        assert_eq!(
            KeyEvent::from(TonKey::Alt('b')),
            KeyEvent::from(KeyEvent {
                code: Key::Char('b'),
                modifiers: KeyModifiers::ALT
            })
        );
        assert_eq!(
            KeyEvent::from(TonKey::Char('B')),
            KeyEvent::from(KeyEvent {
                code: Key::Char('b'),
                modifiers: KeyModifiers::SHIFT,
            })
        );
        assert_eq!(KeyEvent::from(TonKey::Delete), KeyEvent::from(Key::Delete));
        assert_eq!(KeyEvent::from(TonKey::Down), KeyEvent::from(Key::Down));
        assert_eq!(KeyEvent::from(TonKey::End), KeyEvent::from(Key::End));
        assert_eq!(
            KeyEvent::from(TonKey::Char('\n')),
            KeyEvent::from(Key::Enter)
        );
        assert_eq!(KeyEvent::from(TonKey::Esc), KeyEvent::from(Key::Esc));
        assert_eq!(
            KeyEvent::from(TonKey::F(0)),
            KeyEvent::from(Key::Function(0))
        );
        assert_eq!(KeyEvent::from(TonKey::Home), KeyEvent::from(Key::Home));
        assert_eq!(KeyEvent::from(TonKey::Insert), KeyEvent::from(Key::Insert));
        assert_eq!(KeyEvent::from(TonKey::Left), KeyEvent::from(Key::Left));
        assert_eq!(KeyEvent::from(TonKey::Null), KeyEvent::from(Key::Null));
        assert_eq!(
            KeyEvent::from(TonKey::PageDown),
            KeyEvent::from(Key::PageDown)
        );
        assert_eq!(KeyEvent::from(TonKey::PageUp), KeyEvent::from(Key::PageUp));
        assert_eq!(KeyEvent::from(TonKey::Right), KeyEvent::from(Key::Right));
        assert_eq!(KeyEvent::from(TonKey::Char('\t')), KeyEvent::from(Key::Tab));
        assert_eq!(KeyEvent::from(TonKey::Up), KeyEvent::from(Key::Up));
        assert_eq!(
            KeyEvent::from(TonKey::__IsNotComplete),
            KeyEvent::from(Key::Null)
        );
    }

    #[test]
    fn adapt_termion_event() {
        type AppEvent = Event<MockEvent>;
        assert_eq!(
            AppEvent::from(TonEvent::Key(TonKey::Backspace)),
            Event::Keyboard(KeyEvent::from(Key::Backspace))
        );
        assert_eq!(
            AppEvent::from(TonEvent::Mouse(MouseEvent::Hold(0, 0))),
            Event::None
        );
    }
}
