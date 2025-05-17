//! ## Port
//!
//! This module exposes the poll wrapper to include in the worker

#[cfg(feature = "async-ports")]
mod async_p;
mod sync;

use std::time::{Duration, Instant};

#[cfg(feature = "async-ports")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-ports")))]
pub use self::async_p::AsyncPort;
pub use self::sync::SyncPort;
use super::ListenerResult;
use crate::Event;

/// A port is a wrapper around the poll trait object, which also defines an interval, which defines
/// the amount of time between each [`Poll::poll`] call.
/// Its purpose is to listen for incoming events of a user-defined type
///
/// There are two types of ports:
///
/// - [`SyncPort`] - A port that is polled synchronously
/// - [`AsyncPort`] - A port that is polled asynchronously and supports tokio
pub enum Port<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    Sync(SyncPort<U>),
    #[cfg(feature = "async-ports")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async-ports")))]
    Async(AsyncPort<U>),
}

impl<U> From<SyncPort<U>> for Port<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    fn from(port: SyncPort<U>) -> Self {
        Port::Sync(port)
    }
}

#[cfg(feature = "async-ports")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-ports")))]
impl<U> From<AsyncPort<U>> for Port<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    fn from(port: AsyncPort<U>) -> Self {
        Port::Async(port)
    }
}

impl<U> Port<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    /// Get how often a port should get polled in a single poll
    pub fn max_poll(&self) -> usize {
        match self {
            Port::Sync(port) => port.max_poll(),
            #[cfg(feature = "async-ports")]
            Port::Async(port) => port.max_poll(),
        }
    }

    /// Returns the interval for the current [`Port`]
    pub fn interval(&self) -> &Duration {
        match self {
            Port::Sync(port) => port.interval(),
            #[cfg(feature = "async-ports")]
            Port::Async(port) => port.interval(),
        }
    }

    /// Returns the time of the next poll for this listener
    pub fn next_poll(&self) -> Instant {
        match self {
            Port::Sync(port) => port.next_poll(),
            #[cfg(feature = "async-ports")]
            Port::Async(port) => port.next_poll(),
        }
    }

    /// Returns whether next poll is now or in the past
    pub fn should_poll(&self) -> bool {
        match self {
            Port::Sync(port) => port.should_poll(),
            #[cfg(feature = "async-ports")]
            Port::Async(port) => port.should_poll(),
        }
    }

    /// Calls [`Poll::poll`] on the inner [`Poll`] trait object.
    pub fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
        match self {
            Port::Sync(port) => port.poll(),
            #[cfg(feature = "async-ports")]
            Port::Async(port) => port.poll(),
        }
    }

    /// Calculate the next poll (t_now + interval)
    pub fn calc_next_poll(&mut self) {
        match self {
            Port::Sync(port) => port.calc_next_poll(),
            #[cfg(feature = "async-ports")]
            Port::Async(port) => port.calc_next_poll(),
        }
    }
}

#[cfg(test)]
mod test {

    use std::time::Duration;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::mock::{MockEvent, MockPoll};

    #[test]
    fn test_single_listener() {
        let mut listener = Port::from(SyncPort::<MockEvent>::new(
            Box::new(MockPoll::default()),
            Duration::from_secs(5),
            1,
        ));
        assert!(listener.next_poll() <= Instant::now());
        assert_eq!(listener.should_poll(), true);
        assert!(listener.poll().ok().unwrap().is_some());
        listener.calc_next_poll();
        assert_eq!(listener.should_poll(), false);
        assert_eq!(*listener.interval(), Duration::from_secs(5));
    }

    #[tokio::test]
    #[cfg(feature = "async-ports")]
    async fn test_single_async_listener() {
        use crate::mock::MockPollAsync;

        let runtime = std::sync::Arc::new(tokio::runtime::Handle::current());

        let mut listener = Port::from(AsyncPort::<MockEvent>::new(
            Box::new(MockPollAsync::default()),
            Duration::from_secs(5),
            1,
            &runtime,
        ));
        assert!(listener.next_poll() <= Instant::now());
        assert_eq!(listener.should_poll(), true);
        assert!(listener.poll().ok().unwrap().is_some());
        listener.calc_next_poll();
        assert_eq!(listener.should_poll(), false);
        assert_eq!(*listener.interval(), Duration::from_secs(5));
    }
}
