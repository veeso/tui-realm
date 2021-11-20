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
use super::{Duration, EventListener, InputEventListener, Poll, Port};

/// ## EventListenerCfg
///
/// The event listener configurator is used to setup an event listener.
/// Once you're done with configuration just call `start()` and the event listener will start and the listener
/// will be returned.
pub struct EventListenerCfg<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    ports: Vec<Port<U>>,
    tick_interval: Option<Duration>,
    poll_timeout: Duration,
}

impl<U> Default for EventListenerCfg<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    fn default() -> Self {
        Self {
            ports: Vec::default(),
            poll_timeout: Duration::from_millis(10),
            tick_interval: None,
        }
    }
}

impl<U> EventListenerCfg<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    /// ### start
    ///
    /// Create the event listener with the parameters provided and start the workers
    pub(crate) fn start(self) -> EventListener<U> {
        EventListener::start(self.ports, self.poll_timeout, self.tick_interval)
    }

    /// ### poll_timeout
    ///
    /// Set poll timeout.
    /// Poll timeout is the maximum time to wait when fetching the thread receiver.
    ///
    /// > Panics if timeout is 0
    pub fn poll_timeout(mut self, timeout: Duration) -> Self {
        if timeout == Duration::ZERO {
            panic!(
                "poll timeout cannot be 0 (see <https://github.com/rust-lang/rust/issues/39364>)"
            )
        }
        self.poll_timeout = timeout;
        self
    }

    /// ### tick_interval
    ///
    /// Defines the tick interval for the event listener.
    /// If an interval is defined, this will also enable the `Tick` event.
    pub fn tick_interval(mut self, interval: Duration) -> Self {
        self.tick_interval = Some(interval);
        self
    }

    /// ### port
    ///
    /// Add a new Port (Poll, Interval) to the the event listener
    pub fn port(mut self, poll: Box<dyn Poll<U>>, interval: Duration) -> Self {
        self.ports.push(Port::new(poll, interval));
        self
    }

    /// ### default_input_listener
    ///
    /// Add to the event listener the default input event listener for the backend configured.
    pub fn default_input_listener(self, interval: Duration) -> Self {
        self.port(Box::new(InputEventListener::<U>::new(interval)), interval)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::MockEvent;
    use crate::mock::MockPoll;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_configure_and_start_event_listener() {
        let builder = EventListenerCfg::<MockEvent>::default();
        assert!(builder.ports.is_empty());
        assert!(builder.tick_interval.is_none());
        assert_eq!(builder.poll_timeout, Duration::from_millis(10));
        let builder = builder.tick_interval(Duration::from_secs(10));
        assert_eq!(builder.tick_interval.unwrap(), Duration::from_secs(10));
        let builder = builder.poll_timeout(Duration::from_millis(50));
        assert_eq!(builder.poll_timeout, Duration::from_millis(50));
        let builder = builder
            .default_input_listener(Duration::from_millis(200))
            .port(Box::new(MockPoll::default()), Duration::from_secs(300));
        assert_eq!(builder.ports.len(), 2);
        let mut listener = builder.start();
        assert!(listener.stop().is_ok());
    }

    #[test]
    #[should_panic]
    fn event_listener_cfg_should_panic_with_poll_timeout_zero() {
        EventListenerCfg::<MockEvent>::default()
            .poll_timeout(Duration::from_secs(0))
            .start();
    }
}
