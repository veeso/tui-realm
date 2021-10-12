//! ## Listener
//!
//! This module exposes the poll wrapper to include in the worker

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
use super::{Event, ListenerResult, Poll};

use std::ops::Add;
use std::time::{Duration, Instant};

/// ## Listener
///
/// A listener is a wrapper around the poll trait object, which also defines an interval, which defines
/// the amount of time between each poll() call.
pub struct Listener<U>
where
    U: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send,
{
    poll: Box<dyn Poll<U>>,
    interval: Duration,
    next_poll: Instant,
}

impl<U> Listener<U>
where
    U: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send,
{
    /// ### new
    ///
    /// Define a new `Listener`
    pub fn new(poll: Box<dyn Poll<U>>, interval: Duration) -> Self {
        Self {
            poll,
            interval,
            next_poll: Instant::now(),
        }
    }

    /// ### interval
    ///
    /// Returns the interval for the current `Listener`
    pub fn interval(&self) -> &Duration {
        &self.interval
    }

    /// ### next_poll
    ///
    /// Returns the time of the next poll for this listener
    pub fn next_poll(&self) -> Instant {
        self.next_poll
    }

    /// ### should_poll
    ///
    /// Returns whether next poll is now or in the past
    pub fn should_poll(&self) -> bool {
        self.next_poll <= Instant::now()
    }

    /// ### poll
    ///
    /// Calls poll on the inner `Poll` trait object.
    pub fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
        self.poll.poll()
    }

    /// ### calc_next_poll
    ///
    /// Calculate the next poll (t_now + interval)
    pub fn calc_next_poll(&mut self) {
        self.next_poll = Instant::now().add(self.interval);
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::event::listener::mock::MockPoll;
    use crate::event::MockEvent;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_single_listener() {
        let mut listener =
            Listener::<MockEvent>::new(Box::new(MockPoll::default()), Duration::from_secs(5));
        assert!(listener.next_poll() <= Instant::now());
        assert_eq!(listener.should_poll(), true);
        assert!(listener.poll().ok().unwrap().is_some());
        listener.calc_next_poll();
        assert_eq!(listener.should_poll(), false);
        assert_eq!(*listener.interval(), Duration::from_secs(5));
    }
}
