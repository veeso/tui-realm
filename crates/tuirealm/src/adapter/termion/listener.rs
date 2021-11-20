//! ## Listener
//!
//! input listener adapter for termion

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
use std::io::stdin;
use std::marker::PhantomData;
use std::time::Duration;
use termion::input::TermRead;

/// ## TermionInputListener
///
/// The input listener for termion.
/// If termion is enabled, this will already be exported as `InputEventListener` in the `adapter` module
/// or you can use it directly in the event listener, calling `default_input_listener()` in the `EventListenerCfg`
pub struct TermionInputListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    ghost: PhantomData<U>,
}

impl<U> TermionInputListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    pub fn new(_interval: Duration) -> Self {
        Self {
            ghost: PhantomData::default(),
        }
    }
}

impl<U> Poll<U> for TermionInputListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
        match stdin().events().next() {
            Some(Ok(ev)) => Ok(Some(Event::from(ev))),
            Some(Err(_)) => Err(ListenerError::PollFailed),
            None => Ok(None),
        }
    }
}
