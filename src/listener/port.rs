//! ## Port
//!
//! This module exposes the poll wrapper to include in the worker

use std::ops::Add;
use std::time::{Duration, Instant};

use super::{Event, ListenerResult, Poll};

/// A port is a wrapper around the poll trait object, which also defines an interval, which defines
/// the amount of time between each [`Poll::poll`] call.
/// Its purpose is to listen for incoming events of a user-defined type
pub struct Port<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    poll: Box<dyn Poll<U>>,
    interval: Duration,
    next_poll: Instant,
}

impl<U> Port<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    /// Define a new `Port`
    pub fn new(poll: Box<dyn Poll<U>>, interval: Duration) -> Self {
        Self {
            poll,
            interval,
            next_poll: Instant::now(),
        }
    }

    /// Returns the interval for the current [`Port`]
    pub fn interval(&self) -> &Duration {
        &self.interval
    }

    /// Returns the time of the next poll for this listener
    pub fn next_poll(&self) -> Instant {
        self.next_poll
    }

    /// Returns whether next poll is now or in the past
    pub fn should_poll(&self) -> bool {
        self.next_poll <= Instant::now()
    }

    /// Calls [`Poll::poll`] on the inner [`Poll`] trait object.
    pub fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
        self.poll.poll()
    }

    /// Calculate the next poll (t_now + interval)
    pub fn calc_next_poll(&mut self) {
        self.next_poll = Instant::now().add(self.interval);
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::mock::{MockEvent, MockPoll};

    #[test]
    fn test_single_listener() {
        let mut listener =
            Port::<MockEvent>::new(Box::new(MockPoll::default()), Duration::from_secs(5));
        assert!(listener.next_poll() <= Instant::now());
        assert_eq!(listener.should_poll(), true);
        assert!(listener.poll().ok().unwrap().is_some());
        listener.calc_next_poll();
        assert_eq!(listener.should_poll(), false);
        assert_eq!(*listener.interval(), Duration::from_secs(5));
    }
}
