//! ## Builder
//!
//! This module exposes the EventListenerCfg which is used to build the event listener

use super::{Duration, EventListener, Poll, Port};

/// The event listener configurator is used to setup an event listener.
/// Once you're done with configuration just call `EventListenerCfg::start` and the event listener will start and the listener
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

    /// Add a new [`Port`] (Poll, Interval) to the the event listener.
    ///
    /// The interval is the amount of time between each [`Poll::poll`] call.
    /// The max_poll is the maximum amount of times the port should be polled in a single poll.
    pub fn add_port(self, poll: Box<dyn Poll<U>>, interval: Duration, max_poll: usize) -> Self {
        self.port(Port::new(poll, interval, max_poll))
    }

    /// Add a new [`Port`] to the the event listener
    ///
    /// The [`Port`] needs to be manually constructed, unlike [`Self::add_port`]
    pub fn port(mut self, port: Port<U>) -> Self {
        self.ports.push(port);
        self
    }

    #[cfg(feature = "crossterm")]
    /// Add to the event listener the default crossterm input listener [`crate::terminal::CrosstermInputListener`]
    ///
    /// The interval is the amount of time between each [`Poll::poll`] call.
    /// The max_poll is the maximum amount of times the port should be polled in a single poll.
    pub fn crossterm_input_listener(self, interval: Duration, max_poll: usize) -> Self {
        self.add_port(
            Box::new(crate::terminal::CrosstermInputListener::<U>::new(interval)),
            interval,
            max_poll,
        )
    }

    #[cfg(feature = "termion")]
    /// Add to the event listener the default termion input listener [`crate::terminal::TermionInputListener`]
    ///
    /// The interval is the amount of time between each [`Poll::poll`] call.
    /// The max_poll is the maximum amount of times the port should be polled in a single poll.
    pub fn termion_input_listener(self, interval: Duration, max_poll: usize) -> Self {
        self.add_port(
            Box::new(crate::terminal::TermionInputListener::<U>::new(interval)),
            interval,
            max_poll,
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
            .crossterm_input_listener(Duration::from_millis(200), 1)
            .add_port(Box::new(MockPoll::default()), Duration::from_secs(300), 1);
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
            .termion_input_listener(Duration::from_millis(200), 1)
            .add_port(Box::new(MockPoll::default()), Duration::from_secs(300), 1);
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

    #[test]
    fn should_add_port_via_port_1() {
        let builder = EventListenerCfg::<MockEvent>::default();
        assert!(builder.ports.is_empty());
        let builder = builder.port(Port::new(
            Box::new(MockPoll::default()),
            Duration::from_millis(1),
            1,
        ));
        assert_eq!(builder.ports.len(), 1);
    }
}
