//! ## Builder
//!
//! This module exposes the EventListenerCfg which is used to build the event listener

#[cfg(feature = "async-ports")]
use tokio::runtime::Handle;

#[cfg(feature = "async-ports")]
use super::AsyncPort;
use super::{Duration, EventListener, ListenerError, Poll, SyncPort};

/// The event listener configurator is used to setup an event listener.
/// Once you're done with configuration just call `EventListenerCfg::start` and the event listener will start and the listener
/// will be returned.
pub struct EventListenerCfg<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send,
{
    sync_ports: Vec<SyncPort<UserEvent>>,
    #[cfg(feature = "async-ports")]
    async_ports: Vec<AsyncPort<UserEvent>>,
    #[cfg(feature = "async-ports")]
    handle: Option<Handle>,
    tick_interval: Option<Duration>,
    #[cfg(feature = "async-ports")]
    async_tick: bool,
    poll_timeout: Duration,
}

impl<UserEvent> Default for EventListenerCfg<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send,
{
    fn default() -> Self {
        Self {
            sync_ports: Vec::default(),
            #[cfg(feature = "async-ports")]
            async_ports: Vec::default(),
            #[cfg(feature = "async-ports")]
            handle: None,
            poll_timeout: Duration::from_millis(10),
            tick_interval: None,
            #[cfg(feature = "async-ports")]
            async_tick: false,
        }
    }
}

impl<UserEvent> EventListenerCfg<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    /// Create the event listener with the parameters provided and start the workers.
    ///
    /// # Errors
    ///
    /// - if there are async ports defined, but no handle is set.
    #[allow(unused_mut)] // mutability is necessary when "async-ports" is active
    pub(crate) fn start(mut self) -> Result<EventListener<UserEvent>, ListenerError> {
        #[cfg(feature = "async-ports")]
        let start_async =
            !self.async_ports.is_empty() || (self.async_tick && self.tick_interval.is_some());
        #[cfg(not(feature = "async-ports"))]
        let store_tx = false;
        #[cfg(feature = "async-ports")]
        let store_tx = start_async;
        #[cfg(not(feature = "async-ports"))]
        let sync_tick_interval = self.tick_interval;
        #[cfg(feature = "async-ports")]
        let sync_tick_interval = self.tick_interval.take_if(|_| !self.async_tick);
        let mut res = EventListener::new(self.poll_timeout);

        // dont start a sync worker if there are no sync tasks
        if !self.sync_ports.is_empty() || sync_tick_interval.is_some() {
            res = res.start(self.sync_ports, sync_tick_interval, store_tx);
        }

        // dont start a taskpool without any actual tasks
        #[cfg(feature = "async-ports")]
        if start_async {
            let Some(handle) = self.handle else {
                return Err(ListenerError::NoHandle);
            };
            res = res.start_async(
                self.async_ports,
                handle,
                self.tick_interval.take_if(|_| self.async_tick),
            );
        }

        Ok(res)
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

    /// Add a new [`SyncPort`] (Poll, Interval) to the the event listener.
    ///
    /// The interval is the amount of time between each [`Poll::poll`] call.
    /// The max_poll is the maximum amount of times the port should be polled in a single poll.
    pub fn add_port(
        self,
        poll: Box<dyn Poll<UserEvent>>,
        interval: Duration,
        max_poll: usize,
    ) -> Self {
        self.port(SyncPort::new(poll, interval, max_poll))
    }

    /// Add a new [`SyncPort`] to the the event listener
    ///
    /// The [`SyncPort`] needs to be manually constructed, unlike [`Self::add_port`]
    pub fn port(mut self, port: SyncPort<UserEvent>) -> Self {
        self.sync_ports.push(port);
        self
    }

    #[cfg(feature = "crossterm")]
    /// Add to the event listener the default crossterm input listener [`crate::terminal::CrosstermInputListener`]
    ///
    /// The interval is the amount of time between each [`Poll::poll`] call.
    /// The max_poll is the maximum amount of times the port should be polled in a `interval`.
    pub fn crossterm_input_listener(self, interval: Duration, max_poll: usize) -> Self {
        self.add_port(
            Box::new(crate::terminal::CrosstermInputListener::new(interval)),
            interval,
            max_poll,
        )
    }

    #[cfg(all(feature = "crossterm", feature = "async-ports"))]
    /// Add to the async event listener the default crossterm input listener [`crate::terminal::CrosstermAsyncStream`]
    ///
    /// The `interval` is the amount of time between each [`PollAsync::poll`](super::PollAsync) call.
    /// The `max_poll` is the maximum amount of times the port should be polled in a single `interval`.
    ///
    /// It is recommended to set `interval` to `0` to have immediate events.
    pub fn async_crossterm_input_listener(self, interval: Duration, max_poll: usize) -> Self {
        self.add_async_port(
            Box::new(crate::terminal::CrosstermAsyncStream::new()),
            interval,
            max_poll,
        )
    }

    #[cfg(feature = "termion")]
    /// Add to the event listener the default termion input listener [`crate::terminal::TermionInputListener`]
    ///
    /// The interval is the amount of time between each [`Poll::poll`] call.
    /// The max_poll is the maximum amount of times the port should be polled in a `interval`.
    pub fn termion_input_listener(self, interval: Duration, max_poll: usize) -> Self {
        self.add_port(
            Box::new(crate::terminal::TermionInputListener::new(interval)),
            interval,
            max_poll,
        )
    }
}

/// Implementations for feature `async-ports`
impl<UserEvent> EventListenerCfg<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    /// Add a new [`AsyncPort`] (Poll, Interval) to the the event listener.
    ///
    /// The interval is the amount of time between each [`super::PollAsync::poll`] call.
    /// The max_poll is the maximum amount of times the port should be polled in a single poll.
    #[cfg(feature = "async-ports")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async-ports")))]
    pub fn add_async_port(
        self,
        poll: Box<dyn super::PollAsync<UserEvent>>,
        interval: Duration,
        max_poll: usize,
    ) -> Self {
        self.async_port(AsyncPort::new(poll, interval, max_poll))
    }

    /// Add a new [`AsyncPort`] to the the event listener.
    ///
    /// The [`AsyncPort`] needs to be manually constructed, unlike [`Self::add_async_port`].
    #[cfg(feature = "async-ports")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async-ports")))]
    pub fn async_port(mut self, port: AsyncPort<UserEvent>) -> Self {
        self.async_ports.push(port);
        self
    }

    /// Set the async runtime handle to use to spawn the async ports on.
    ///
    /// If this is not set, a Error is returned on [`start`](Self::start).
    #[cfg(feature = "async-ports")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async-ports")))]
    pub fn with_handle(mut self, handle: tokio::runtime::Handle) -> Self {
        self.handle = Some(handle);
        self
    }

    /// Change the way [`Event::Tick`](crate::Event::Tick) is emitted from being on a [`SyncPort`] to be a [`AsyncPort`].
    #[cfg(feature = "async-ports")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async-ports")))]
    pub fn async_tick(mut self, value: bool) -> Self {
        self.async_tick = value;
        self
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
        assert!(builder.sync_ports.is_empty());
        assert!(builder.tick_interval.is_none());
        assert_eq!(builder.poll_timeout, Duration::from_millis(10));
        let builder = builder.tick_interval(Duration::from_secs(10));
        assert_eq!(builder.tick_interval.unwrap(), Duration::from_secs(10));
        let builder = builder.poll_timeout(Duration::from_millis(50));
        assert_eq!(builder.poll_timeout, Duration::from_millis(50));
        let builder = builder
            .crossterm_input_listener(Duration::from_millis(200), 1)
            .add_port(Box::new(MockPoll::default()), Duration::from_secs(300), 1);
        assert_eq!(builder.sync_ports.len(), 2);
        let mut listener = builder.start().unwrap();
        assert!(listener.stop().is_ok());
    }

    #[test]
    #[cfg(feature = "termion")]
    fn should_configure_and_start_event_listener_termion() {
        let builder = EventListenerCfg::<MockEvent>::default();
        assert!(builder.sync_ports.is_empty());
        assert!(builder.tick_interval.is_none());
        assert_eq!(builder.poll_timeout, Duration::from_millis(10));
        let builder = builder.tick_interval(Duration::from_secs(10));
        assert_eq!(builder.tick_interval.unwrap(), Duration::from_secs(10));
        let builder = builder.poll_timeout(Duration::from_millis(50));
        assert_eq!(builder.poll_timeout, Duration::from_millis(50));
        let builder = builder
            .termion_input_listener(Duration::from_millis(200), 1)
            .add_port(Box::new(MockPoll::default()), Duration::from_secs(300), 1);
        assert_eq!(builder.sync_ports.len(), 2);
        let mut listener = builder.start().unwrap();
        assert!(listener.stop().is_ok());
    }

    #[test]
    #[should_panic]
    fn event_listener_cfg_should_panic_with_poll_timeout_zero() {
        let _ = EventListenerCfg::<MockEvent>::default()
            .poll_timeout(Duration::from_secs(0))
            .start();
    }

    #[test]
    fn should_add_port_via_port_1() {
        let builder = EventListenerCfg::<MockEvent>::default();
        assert!(builder.sync_ports.is_empty());
        let builder = builder.port(SyncPort::new(
            Box::new(MockPoll::default()),
            Duration::from_millis(1),
            1,
        ));
        assert_eq!(builder.sync_ports.len(), 1);
    }

    #[test]
    #[cfg(feature = "async-ports")]
    fn should_error_without_handle() {
        use crate::mock::MockPollAsync;

        let port = AsyncPort::<MockEvent>::new(
            Box::new(MockPollAsync::default()),
            Duration::from_secs(5),
            1,
        );

        let builder = EventListenerCfg::<MockEvent>::default();
        assert!(builder.async_ports.is_empty());
        let builder = builder.async_port(port);

        assert_eq!(builder.start().unwrap_err(), ListenerError::NoHandle);
    }

    #[tokio::test]
    #[cfg(feature = "async-ports")]
    async fn should_spawn_async_ticker() {
        use tokio::time::sleep;

        use crate::Event;

        let builder = EventListenerCfg::<MockEvent>::default()
            .with_handle(Handle::current())
            .async_tick(true)
            .tick_interval(Duration::from_millis(10));
        assert!(builder.async_ports.is_empty());
        assert_eq!(Handle::current().metrics().num_alive_tasks(), 0);

        let mut listener = builder.start().unwrap();
        assert_eq!(Handle::current().metrics().num_alive_tasks(), 1);
        assert!(listener.thread.is_none()); // there are no sync ports or tasks, so no sync worker
        assert!(listener.taskpool.is_some());
        sleep(Duration::from_millis(25)).await; // wait for at least 1 event

        listener.stop().unwrap();
        assert_eq!(listener.poll(), Ok(Some(Event::Tick)));
    }
}
