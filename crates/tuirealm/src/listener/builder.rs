//! ## Builder
//!
//! This module exposes the EventListenerCfg which is used to build the event listener

use super::{Duration, EventListener, Poll, Port};

/// The event listener configurator is used to setup an event listener.
/// Once you're done with configuration just call [`EventListenerCfg::start`] and the event listener will start and the listener
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
    /// Create the event listener with the parameters provided and start the workers
    pub(crate) fn start(self) -> EventListener<U> {
        EventListener::start(self.ports, self.poll_timeout, self.tick_interval)
    }

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

    /// Defines the tick interval for the event listener.
    /// If an interval is defined, this will also enable the `Tick` event.
    pub fn tick_interval(mut self, interval: Duration) -> Self {
        self.tick_interval = Some(interval);
        self
    }

    /// Add a new Port (Poll, Interval) to the the event listener
    pub fn port(mut self, poll: Box<dyn Poll<U>>, interval: Duration) -> Self {
        self.ports.push(Port::new(poll, interval));
        self
    }

    #[cfg(feature = "crossterm")]
    /// Add to the event listener the default crossterm input listener [`crate::terminal::CrosstermInputListener`]
    pub fn crossterm_input_listener(self, interval: Duration) -> Self {
        self.port(
            Box::new(crate::terminal::CrosstermInputListener::<U>::new(interval)),
            interval,
        )
    }

    #[cfg(feature = "termion")]
    /// Add to the event listener the default termion input listener [`crate::terminal::TermionInputListener`]
    pub fn termion_input_listener(self, interval: Duration) -> Self {
        self.port(
            Box::new(crate::terminal::TermionInputListener::<U>::new(interval)),
            interval,
        )
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::mock::{MockEvent, MockPoll};

    #[test]
    #[cfg(feature = "crossterm")]
    fn should_configure_and_start_event_listener_crossterm() {
        let builder = EventListenerCfg::<MockEvent>::default();
        assert!(builder.ports.is_empty());
        assert!(builder.tick_interval.is_none());
        assert_eq!(builder.poll_timeout, Duration::from_millis(10));
        let builder = builder.tick_interval(Duration::from_secs(10));
        assert_eq!(builder.tick_interval.unwrap(), Duration::from_secs(10));
        let builder = builder.poll_timeout(Duration::from_millis(50));
        assert_eq!(builder.poll_timeout, Duration::from_millis(50));
        let builder = builder
            .crossterm_input_listener(Duration::from_millis(200))
            .port(Box::new(MockPoll::default()), Duration::from_secs(300));
        assert_eq!(builder.ports.len(), 2);
        let mut listener = builder.start();
        assert!(listener.stop().is_ok());
    }

    #[test]
    #[cfg(feature = "termion")]
    fn should_configure_and_start_event_listener_termion() {
        let builder = EventListenerCfg::<MockEvent>::default();
        assert!(builder.ports.is_empty());
        assert!(builder.tick_interval.is_none());
        assert_eq!(builder.poll_timeout, Duration::from_millis(10));
        let builder = builder.tick_interval(Duration::from_secs(10));
        assert_eq!(builder.tick_interval.unwrap(), Duration::from_secs(10));
        let builder = builder.poll_timeout(Duration::from_millis(50));
        assert_eq!(builder.poll_timeout, Duration::from_millis(50));
        let builder = builder
            .termion_input_listener(Duration::from_millis(200))
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
