//! ## Builder
//!
//! This module exposes the EventListenerCfg which is used to build the event listener

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
use super::{Duration, EventListener, InputEventListener, Listener, Poll};

/// ## EventListenerCfg
///
/// The event listener configurator is used to setup an event listener.
/// Once you're done with configuration just call `start()` and the event listener will start and the listener
/// will be returned.
pub struct EventListenerCfg {
    listeners: Vec<Listener>,
    tick_interval: Option<Duration>,
}

impl Default for EventListenerCfg {
    fn default() -> Self {
        Self {
            listeners: Vec::default(),
            tick_interval: None,
        }
    }
}

impl EventListenerCfg {
    /// ### start
    ///
    /// Create the event listener with the parameters provided and start the workers
    pub fn start(self) -> EventListener {
        EventListener::start(self.listeners, self.tick_interval)
    }

    /// ### tick_interval
    ///
    /// Defines the tick interval for the event listener.
    /// If an interval is defined, this will also enable the `Tick` event.
    pub fn tick_interval(mut self, interval: Duration) -> Self {
        self.tick_interval = Some(interval);
        self
    }

    /// ### listener
    ///
    /// Add a new listener to the the event listener
    pub fn listener(mut self, poll: Box<dyn Poll>, interval: Duration) -> Self {
        self.listeners.push(Listener::new(poll, interval));
        self
    }

    /// ### default_input_listener
    ///
    /// Add to the event listener the default input event listener for the backend configured.
    pub fn default_input_listener(self, interval: Duration) -> Self {
        self.listener(Box::new(InputEventListener::default()), interval)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::event::listener::mock::MockPoll;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_configure_and_start_event_listener() {
        let builder = EventListenerCfg::default();
        assert!(builder.listeners.is_empty());
        assert!(builder.tick_interval.is_none());
        let builder = builder.tick_interval(Duration::from_secs(10));
        assert_eq!(builder.tick_interval.unwrap(), Duration::from_secs(10));
        let builder = builder
            .default_input_listener(Duration::from_millis(200))
            .listener(Box::new(MockPoll::default()), Duration::from_secs(300));
        assert_eq!(builder.listeners.len(), 2);
        let mut listener = builder.start();
        assert!(listener.stop().is_ok());
    }
}
