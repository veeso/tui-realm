//! ## crossterm
//!
//! this module contains the adapters for crossterm

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
extern crate crossterm;

use super::{Event, Key, KeyEvent, KeyModifiers};

use crate::listener::{ListenerError, ListenerResult, Poll};
use crate::tui::{backend::CrosstermBackend, Frame as TuiFrame, Terminal as TuiTerminal};
use crossterm::event::{
    self as xterm, Event as XtermEvent, KeyCode as XtermKeyCode, KeyEvent as XtermKeyEvent,
    KeyModifiers as XtermKeyModifiers,
};
use std::io::Stdout;
use std::marker::PhantomData;
use std::time::Duration;

// -- Frame
/// ## Frame
///
/// Frame represents the Frame where the view will be displayed in
pub type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;

/// ## Terminal
///
/// Terminal must be used to interact with the terminal in tui applications
pub type Terminal = TuiTerminal<CrosstermBackend<Stdout>>;

// -- converters

impl<U> From<XtermEvent> for Event<U>
where
    U: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send,
{
    fn from(e: XtermEvent) -> Self {
        match e {
            XtermEvent::Key(key) => Self::Keyboard(key.into()),
            XtermEvent::Mouse(_) => Self::None,
            XtermEvent::Resize(w, h) => Self::WindowResize(w, h),
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
            XtermKeyCode::Null => Self::Null,
            XtermKeyCode::PageDown => Self::PageDown,
            XtermKeyCode::PageUp => Self::PageUp,
            XtermKeyCode::Right => Self::Right,
            XtermKeyCode::Tab => Self::Tab,
            XtermKeyCode::Up => Self::Up,
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

// -- Event listener

/// ## CrosstermInputListener
///
/// The input listener for crossterm.
/// If crossterm is enabled, this will already be exported as `InputEventListener` in the `adapter` module
/// or you can use it directly in the event listener, calling `default_input_listener()` in the `EventListenerCfg`
pub struct CrosstermInputListener<U>
where
    U: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send,
{
    ghost: PhantomData<U>,
}

impl<U> Default for CrosstermInputListener<U>
where
    U: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send,
{
    fn default() -> Self {
        Self {
            ghost: PhantomData::default(),
        }
    }
}

impl<U> Poll<U> for CrosstermInputListener<U>
where
    U: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send,
{
    fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
        if let Ok(available) = xterm::poll(Duration::from_millis(10)) {
            match available {
                true => {
                    // Read event
                    if let Ok(ev) = xterm::read() {
                        Ok(Some(Event::from(ev)))
                    } else {
                        Err(ListenerError::PollFailed)
                    }
                }
                false => Ok(None),
            }
        } else {
            Err(ListenerError::PollFailed)
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::core::event::MockEvent;

    use pretty_assertions::assert_eq;

    use crossterm::event::{MouseEvent as XtermMouseEvent, MouseEventKind as XtermMouseEventKind};

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
    }
}
