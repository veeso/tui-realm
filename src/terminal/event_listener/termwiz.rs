use std::time::Duration;

use termwiz::caps::Capabilities;
use termwiz::input::{
    InputEvent, KeyCode, KeyEvent as TKeyEvent, Modifiers, MouseButtons as TMouseButtons,
    MouseEvent as TMouseEvent, PixelMouseEvent,
};
use termwiz::terminal::buffered::BufferedTerminal;
use termwiz::terminal::{SystemTerminal, Terminal};
use thiserror::Error;

use super::Event;
use crate::ListenerError;
use crate::event::{
    Key, KeyEvent, KeyModifiers, MediaKeyCode, MouseButton, MouseEvent, MouseEventKind,
};
use crate::listener::{ListenerResult, Poll};

/// The input listener for [`termwiz`].
/// If [`termwiz`] is enabled, this will already be exported as `InputEventListener` in the `adapter` module
/// or you can use it directly in the event listener, calling [`EventListenerCfg::termwiz_input_listener()`](crate::EventListenerCfg::termwiz_input_listener)
#[doc(alias = "InputEventListener")]
pub struct TermwizInputListener {
    terminal: BufferedTerminal<SystemTerminal>,
    timeout: Duration,
}

impl TermwizInputListener {
    pub fn new(timeout: Duration) -> Result<Self, termwiz::Error> {
        // For some reason, termwiz require a full terminal just to read input (unlike crossterm).
        // tui-realm currently requires that output (terminal adapters) and inputs (event listeners) be completely independant.
        let buffered_terminal =
            BufferedTerminal::new(SystemTerminal::new(Capabilities::new_from_env()?)?)?;

        Ok(Self {
            terminal: buffered_terminal,
            timeout,
        })
    }
}

impl<UserEvent> Poll<UserEvent> for TermwizInputListener
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
        match self.terminal.terminal().poll_input(Some(self.timeout)) {
            Ok(Some(ev)) => {
                let ev = Event::from(ev);
                if ev == Event::None {
                    Ok(None)
                } else {
                    Ok(Some(ev))
                }
            }
            Ok(None) => Ok(None),
            Err(_) => Err(ListenerError::PollFailed),
        }
    }
}

fn clamp_to_u16(val: usize) -> u16 {
    u16::try_from(val).unwrap_or(u16::MAX)
}

impl<UserEvent> From<InputEvent> for Event<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send,
{
    fn from(e: InputEvent) -> Self {
        match e {
            InputEvent::Key(key) => Self::Keyboard(key.into()),
            InputEvent::Mouse(ev) => Self::Mouse(ev.into()),
            // is this correct??
            InputEvent::PixelMouse(ev) => Self::Mouse(ev.into()),
            InputEvent::Resized { cols, rows } => {
                Self::WindowResize(clamp_to_u16(cols), clamp_to_u16(rows))
            }
            InputEvent::Paste(clipboard) => Self::Paste(clipboard),
            InputEvent::Wake => Self::None,
        }
    }
}

impl From<TKeyEvent> for KeyEvent {
    fn from(e: TKeyEvent) -> Self {
        Self {
            code: e.key.into(),
            modifiers: e.modifiers.into(),
        }
    }
}

impl From<KeyCode> for Key {
    fn from(k: KeyCode) -> Self {
        match k {
            // KeyCode::BackTab => Self::BackTab,
            KeyCode::Backspace => Self::Backspace,
            KeyCode::Char(ch) => Self::Char(ch),
            KeyCode::Delete => Self::Delete,
            KeyCode::DownArrow => Self::Down,
            KeyCode::End => Self::End,
            KeyCode::Enter => Self::Enter,
            KeyCode::Escape => Self::Esc,
            KeyCode::Function(f) => Self::Function(f),
            KeyCode::Home => Self::Home,
            KeyCode::Insert => Self::Insert,
            KeyCode::LeftArrow => Self::Left,
            KeyCode::PageDown => Self::PageDown,
            KeyCode::PageUp => Self::PageUp,
            KeyCode::RightArrow => Self::Right,
            KeyCode::Tab => Self::Tab,
            KeyCode::UpArrow => Self::Up,
            KeyCode::CapsLock => Self::CapsLock,
            KeyCode::ScrollLock => Self::ScrollLock,
            KeyCode::NumLock => Self::NumLock,
            KeyCode::PrintScreen => Self::PrintScreen,
            KeyCode::Pause => Self::Pause,
            KeyCode::Menu => Self::Menu,
            KeyCode::KeyPadBegin => Self::KeypadBegin,

            KeyCode::MediaNextTrack => Self::Media(MediaKeyCode::TrackNext),
            KeyCode::MediaPlayPause => Self::Media(MediaKeyCode::PlayPause),
            KeyCode::MediaPrevTrack => Self::Media(MediaKeyCode::TrackPrevious),
            KeyCode::MediaStop => Self::Media(MediaKeyCode::Stop),
            KeyCode::VolumeDown => Self::Media(MediaKeyCode::LowerVolume),
            KeyCode::VolumeUp => Self::Media(MediaKeyCode::RaiseVolume),
            KeyCode::VolumeMute => Self::Media(MediaKeyCode::MuteVolume),

            _ => Self::Null,
        }
    }
}

impl From<Modifiers> for KeyModifiers {
    fn from(k: Modifiers) -> Self {
        let mut km = KeyModifiers::NONE;
        if k.intersects(Modifiers::SHIFT) {
            km.insert(KeyModifiers::SHIFT);
        }
        if k.intersects(Modifiers::CTRL) {
            km.insert(KeyModifiers::CONTROL);
        }
        if k.intersects(Modifiers::ALT) {
            km.insert(KeyModifiers::ALT);
        }
        km
    }
}

impl From<TMouseEvent> for MouseEvent {
    fn from(value: TMouseEvent) -> Self {
        Self {
            kind: value.mouse_buttons.into(),
            modifiers: value.modifiers.into(),
            column: value.x,
            row: value.y,
        }
    }
}

impl From<PixelMouseEvent> for MouseEvent {
    fn from(value: PixelMouseEvent) -> Self {
        Self {
            kind: value.mouse_buttons.into(),
            modifiers: value.modifiers.into(),
            column: value.x_pixels,
            row: value.y_pixels,
        }
    }
}

impl From<TMouseButtons> for MouseEventKind {
    fn from(value: TMouseButtons) -> Self {
        // Handle TMouseButtons::NONE
        if value.is_empty() {
            return Self::Moved;
        }

        let direction_positive = value.contains(TMouseButtons::WHEEL_POSITIVE);
        // Handle TMouseButtons::VERT_WHEEL and TMouseButtons::HORZ_WHEEL
        if value.intersects(TMouseButtons::VERT_WHEEL) {
            return match direction_positive {
                true => Self::ScrollUp,
                false => Self::ScrollDown,
            };
        } else if value.intersects(TMouseButtons::HORZ_WHEEL) {
            return match direction_positive {
                true => Self::ScrollLeft,
                false => Self::ScrollRight,
            };
        }

        // Handle Other buttons
        let button = match MouseButton::try_from(value) {
            Ok(v) => v,
            // this *should* be all handled in the "is_empty" if at the start
            Err(_) => return Self::Moved,
        };
        match direction_positive {
            true => Self::Up(button),
            false => Self::Down(button),
        }
    }
}

#[derive(Debug, Error)]
#[error("Unhandled flags in bitflag")]
pub struct UnknownBitFlag(TMouseButtons);

impl TryFrom<TMouseButtons> for MouseButton {
    type Error = UnknownBitFlag;

    fn try_from(value: TMouseButtons) -> Result<Self, Self::Error> {
        Ok(if value.intersects(TMouseButtons::LEFT) {
            Self::Left
        } else if value.intersects(TMouseButtons::RIGHT) {
            Self::Right
        } else if value.intersects(TMouseButtons::MIDDLE) {
            Self::Middle
        } else {
            return Err(UnknownBitFlag(value));
        })
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::event::{Key, MediaKeyCode};
    use crate::mock::MockEvent;

    #[test]
    fn adapt_termwiz_keycode() {
        // assert_eq!(Key::from(KeyCode::BackTab), Key::BackTab);
        assert_eq!(Key::from(KeyCode::Backspace), Key::Backspace);
        assert_eq!(Key::from(KeyCode::Char('b')), Key::Char('b'));
        assert_eq!(Key::from(KeyCode::Delete), Key::Delete);
        assert_eq!(Key::from(KeyCode::DownArrow), Key::Down);
        assert_eq!(Key::from(KeyCode::End), Key::End);
        assert_eq!(Key::from(KeyCode::Enter), Key::Enter);
        assert_eq!(Key::from(KeyCode::Escape), Key::Esc);
        assert_eq!(Key::from(KeyCode::Function(0)), Key::Function(0));
        assert_eq!(Key::from(KeyCode::Home), Key::Home);
        assert_eq!(Key::from(KeyCode::Insert), Key::Insert);
        assert_eq!(Key::from(KeyCode::LeftArrow), Key::Left);
        // assert_eq!(Key::from(KeyCode::Null), Key::Null);
        assert_eq!(Key::from(KeyCode::PageDown), Key::PageDown);
        assert_eq!(Key::from(KeyCode::PageUp), Key::PageUp);
        assert_eq!(Key::from(KeyCode::RightArrow), Key::Right);
        assert_eq!(Key::from(KeyCode::Tab), Key::Tab);
        assert_eq!(Key::from(KeyCode::UpArrow), Key::Up);
    }

    #[test]
    fn adapt_termwiz_key_modifiers() {
        assert_eq!(
            KeyModifiers::from(Modifiers::CTRL | Modifiers::SHIFT | Modifiers::ALT),
            KeyModifiers::all()
        );
        assert_eq!(KeyModifiers::from(Modifiers::ALT), KeyModifiers::ALT);
    }

    #[test]
    fn should_adapt_media_key() {
        // assert_eq!(
        //     Key::from(KeyCode::Play),
        //     Key::Media(MediaKeyCode::Play)
        // );
        assert_eq!(Key::from(KeyCode::Pause), Key::Pause);
        assert_eq!(
            Key::from(KeyCode::MediaPlayPause),
            Key::Media(MediaKeyCode::PlayPause)
        );
        // assert_eq!(
        //     Key::from(KeyCode::Reverse),
        //     Key::Media(MediaKeyCode::Reverse)
        // );
        assert_eq!(
            Key::from(KeyCode::MediaStop),
            Key::Media(MediaKeyCode::Stop)
        );
        // assert_eq!(
        //     Key::from(KeyCode::FastForward),
        //     Key::Media(MediaKeyCode::FastForward)
        // );
        // assert_eq!(
        //     Key::from(KeyCode::Rewind),
        //     Key::Media(MediaKeyCode::Rewind)
        // );
        assert_eq!(
            Key::from(KeyCode::MediaNextTrack),
            Key::Media(MediaKeyCode::TrackNext)
        );
        assert_eq!(
            Key::from(KeyCode::MediaPrevTrack),
            Key::Media(MediaKeyCode::TrackPrevious)
        );
        // assert_eq!(
        //     Key::from(KeyCode::Record),
        //     Key::Media(MediaKeyCode::Record)
        // );
        assert_eq!(
            Key::from(KeyCode::VolumeDown),
            Key::Media(MediaKeyCode::LowerVolume)
        );
        assert_eq!(
            Key::from(KeyCode::VolumeUp),
            Key::Media(MediaKeyCode::RaiseVolume)
        );
        assert_eq!(
            Key::from(KeyCode::VolumeMute),
            Key::Media(MediaKeyCode::MuteVolume)
        );
    }

    #[test]
    fn should_adapt_mouse_event() {
        assert_eq!(
            MouseEvent::from(TMouseEvent {
                mouse_buttons: TMouseButtons::NONE,
                x: 1,
                y: 1,
                modifiers: Modifiers::NONE
            }),
            MouseEvent {
                kind: MouseEventKind::Moved,
                modifiers: KeyModifiers::NONE,
                column: 1,
                row: 1
            }
        );
        assert_eq!(
            MouseEvent::from(TMouseEvent {
                mouse_buttons: TMouseButtons::LEFT,
                x: 1,
                y: 1,
                modifiers: Modifiers::NONE
            }),
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                modifiers: KeyModifiers::NONE,
                column: 1,
                row: 1
            }
        );
        assert_eq!(
            MouseEvent::from(TMouseEvent {
                mouse_buttons: TMouseButtons::WHEEL_POSITIVE | TMouseButtons::RIGHT,
                x: 1,
                y: 1,
                modifiers: Modifiers::NONE
            }),
            MouseEvent {
                kind: MouseEventKind::Up(MouseButton::Right),
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
            MouseEvent::from(TMouseEvent {
                mouse_buttons: TMouseButtons::WHEEL_POSITIVE | TMouseButtons::VERT_WHEEL,
                x: 1,
                y: 1,
                modifiers: Modifiers::NONE
            }),
            MouseEvent {
                kind: MouseEventKind::ScrollUp,
                modifiers: KeyModifiers::NONE,
                column: 1,
                row: 1
            }
        );
        assert_eq!(
            MouseEvent::from(TMouseEvent {
                mouse_buttons: TMouseButtons::VERT_WHEEL,
                x: 1,
                y: 1,
                modifiers: Modifiers::NONE
            }),
            MouseEvent {
                kind: MouseEventKind::ScrollDown,
                modifiers: KeyModifiers::NONE,
                column: 1,
                row: 1
            }
        );
        assert_eq!(
            MouseEvent::from(TMouseEvent {
                mouse_buttons: TMouseButtons::WHEEL_POSITIVE | TMouseButtons::HORZ_WHEEL,
                x: 1,
                y: 1,
                modifiers: Modifiers::NONE
            }),
            MouseEvent {
                kind: MouseEventKind::ScrollLeft,
                modifiers: KeyModifiers::NONE,
                column: 1,
                row: 1
            }
        );
        assert_eq!(
            MouseEvent::from(TMouseEvent {
                mouse_buttons: TMouseButtons::HORZ_WHEEL,
                x: 1,
                y: 1,
                modifiers: Modifiers::NONE
            }),
            MouseEvent {
                kind: MouseEventKind::ScrollRight,
                modifiers: KeyModifiers::NONE,
                column: 1,
                row: 1
            }
        );
    }

    #[test]
    fn adapt_termwiz_key_event() {
        assert_eq!(
            KeyEvent::from(TKeyEvent {
                key: KeyCode::Backspace,
                modifiers: Modifiers::CTRL
            }),
            KeyEvent::new(Key::Backspace, KeyModifiers::CONTROL)
        );
    }

    #[test]
    fn adapt_termwiz_event() {
        type AppEvent = Event<MockEvent>;
        assert_eq!(
            AppEvent::from(InputEvent::Resized { cols: 24, rows: 48 }),
            Event::WindowResize(24, 48)
        );
        assert_eq!(
            AppEvent::from(InputEvent::Key(TKeyEvent {
                key: KeyCode::Backspace,
                modifiers: Modifiers::NONE
            })),
            Event::Keyboard(KeyEvent::from(Key::Backspace))
        );
        assert_eq!(
            AppEvent::from(InputEvent::Mouse(TMouseEvent {
                mouse_buttons: TMouseButtons::NONE,
                x: 0,
                y: 0,
                modifiers: Modifiers::NONE,
            })),
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Moved,
                modifiers: KeyModifiers::NONE,
                column: 0,
                row: 0
            })
        );
        assert_eq!(
            AppEvent::from(InputEvent::Paste(String::from("a"))),
            AppEvent::Paste(String::from("a"))
        );
    }
}
