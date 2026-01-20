use std::time::Duration;

use termion::AsyncReader;
use termion::event::{
    Event as TonEvent, Key as TonKey, MouseButton as TonMouseButton, MouseEvent as TonMouseEvent,
};
use termion::input::{Events, TermRead as _};

use super::Event;
use crate::event::{Key, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use crate::listener::{Poll, PortResult};
use crate::terminal::event_listener::io_err_to_port_err;

/// The input listener for [`termion`].
///
/// # Known Issues
///
/// - Mouse Events only have a key associated on `Down` / `Press`, not on `Up` / `Release` or `Hold` / `Drag`
/// - Mouse Hover location is not implemented in termion
/// - Mouse Events cannot have modifiers (CTRL, SHIFT; ALT) associated with it
#[doc(alias = "InputEventListener")]
pub struct TermionInputListener(Events<AsyncReader>);

impl TermionInputListener {
    pub fn new(_interval: Duration) -> Self {
        Self(termion::async_stdin().events())
    }
}

impl<UserEvent> Poll<UserEvent> for TermionInputListener
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>> {
        // termion's "AsyncReader::read" will never block and instead return "Ok(0)", which is handled as "None" here
        match self.0.next() {
            Some(Ok(ev)) => Ok(Some(Event::from(ev))),
            Some(Err(err)) => Err(io_err_to_port_err(err)),
            None => Ok(None),
        }
    }
}

impl<UserEvent> From<TonEvent> for Event<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send,
{
    fn from(e: TonEvent) -> Self {
        match e {
            TonEvent::Key(key) => Self::Keyboard(key.into()),
            // As of termion@4.0.6, the following does *not* handle mouse hover locations
            TonEvent::Mouse(key) => Self::Mouse(key.into()),
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
            TonKey::Null | TonKey::__IsNotComplete => Key::Null,
            TonKey::Esc => Key::Esc,
            TonKey::ShiftLeft => Key::ShiftLeft,
            TonKey::AltLeft => Key::AltLeft,
            TonKey::CtrlLeft => Key::CtrlLeft,
            TonKey::ShiftRight => Key::ShiftRight,
            TonKey::AltRight => Key::AltRight,
            TonKey::CtrlRight => Key::CtrlRight,
            TonKey::ShiftUp => Key::ShiftUp,
            TonKey::AltUp => Key::AltUp,
            TonKey::CtrlUp => Key::CtrlUp,
            TonKey::ShiftDown => Key::ShiftDown,
            TonKey::AltDown => Key::AltDown,
            TonKey::CtrlDown => Key::CtrlDown,
            TonKey::CtrlHome => Key::CtrlHome,
            TonKey::CtrlEnd => Key::CtrlEnd,
        };
        Self { code, modifiers }
    }
}

impl From<TonMouseEvent> for MouseEvent {
    fn from(value: TonMouseEvent) -> Self {
        match value {
            TonMouseEvent::Press(mouse_button, x, y) => Self {
                kind: mouse_button.into(),
                modifiers: KeyModifiers::NONE,
                column: x,
                row: y,
            },
            // FIXME: The following is not correct, but it cannot be implemented without termion changing or us doing key-tracking
            TonMouseEvent::Release(x, y) | TonMouseEvent::Hold(x, y) => Self {
                kind: MouseEventKind::Moved,
                modifiers: KeyModifiers::NONE,
                column: x,
                row: y,
            },
        }
    }
}

impl From<TonMouseButton> for MouseEventKind {
    fn from(value: TonMouseButton) -> Self {
        match value {
            // FIXME: This is not correct and needs to be fixed-up by the caller (if we ever implement it)
            TonMouseButton::Left => Self::Down(MouseButton::Left),
            TonMouseButton::Right => Self::Down(MouseButton::Right),
            TonMouseButton::Middle => Self::Down(MouseButton::Middle),
            TonMouseButton::WheelUp => Self::ScrollUp,
            TonMouseButton::WheelDown => Self::ScrollDown,
            TonMouseButton::WheelLeft => Self::ScrollLeft,
            TonMouseButton::WheelRight => Self::ScrollRight,
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::mock::MockEvent;

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
            KeyEvent {
                code: Key::Char('b'),
                modifiers: KeyModifiers::CONTROL,
            }
        );
        assert_eq!(
            KeyEvent::from(TonKey::Alt('b')),
            KeyEvent {
                code: Key::Char('b'),
                modifiers: KeyModifiers::ALT
            }
        );
        assert_eq!(
            KeyEvent::from(TonKey::Char('B')),
            KeyEvent {
                code: Key::Char('b'),
                modifiers: KeyModifiers::SHIFT,
            }
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
            AppEvent::from(TonEvent::Mouse(TonMouseEvent::Press(
                TonMouseButton::Left,
                1,
                1
            ))),
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                modifiers: KeyModifiers::NONE,
                column: 1,
                row: 1
            })
        );
    }

    #[test]
    fn should_adapt_mouse_event() {
        assert_eq!(
            MouseEvent::from(TonMouseEvent::Press(TonMouseButton::Left, 1, 1)),
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                modifiers: KeyModifiers::NONE,
                column: 1,
                row: 1
            }
        );
        assert_eq!(
            MouseEvent::from(TonMouseEvent::Release(1, 1)),
            MouseEvent {
                kind: MouseEventKind::Moved,
                modifiers: KeyModifiers::NONE,
                column: 1,
                row: 1
            }
        );
        // assert_eq!(
        //     MouseEvent::from(TMouseEvent {
        //         mouse_buttons: XtermMouseEventKind::Drag(XtermMouseButton::Middle),
        //         x: 1,
        //         y: 1,
        //         modifiers: Modifiers::NONE
        //     }),
        //     MouseEvent {
        //         kind: MouseEventKind::Drag(MouseButton::Middle),
        //         modifiers: KeyModifiers::NONE,
        //         column: 1,
        //         row: 1
        //     }
        // );
        assert_eq!(
            MouseEvent::from(TonMouseEvent::Press(TonMouseButton::WheelUp, 1, 1)),
            MouseEvent {
                kind: MouseEventKind::ScrollUp,
                modifiers: KeyModifiers::NONE,
                column: 1,
                row: 1
            }
        );
        assert_eq!(
            MouseEvent::from(TonMouseEvent::Press(TonMouseButton::WheelDown, 1, 1)),
            MouseEvent {
                kind: MouseEventKind::ScrollDown,
                modifiers: KeyModifiers::NONE,
                column: 1,
                row: 1
            }
        );
        assert_eq!(
            MouseEvent::from(TonMouseEvent::Press(TonMouseButton::WheelLeft, 1, 1)),
            MouseEvent {
                kind: MouseEventKind::ScrollLeft,
                modifiers: KeyModifiers::NONE,
                column: 1,
                row: 1
            }
        );
        assert_eq!(
            MouseEvent::from(TonMouseEvent::Press(TonMouseButton::WheelRight, 1, 1)),
            MouseEvent {
                kind: MouseEventKind::ScrollRight,
                modifiers: KeyModifiers::NONE,
                column: 1,
                row: 1
            }
        );
    }
}
