//! ## Listener
//!
//! input listener adapter for crossterm

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
use super::Event;

use crate::listener::{ListenerError, ListenerResult, Poll};
use crossterm::event as xterm;
use std::marker::PhantomData;
use std::time::Duration;

/// ## CrosstermInputListener
///
/// The input listener for crossterm.
/// If crossterm is enabled, this will already be exported as `InputEventListener` in the `adapter` module
/// or you can use it directly in the event listener, calling `default_input_listener()` in the `EventListenerCfg`
pub struct CrosstermInputListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    ghost: PhantomData<U>,
}

impl<U> Default for CrosstermInputListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    fn default() -> Self {
        Self {
            ghost: PhantomData::default(),
        }
    }
}

impl<U> Poll<U> for CrosstermInputListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
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
