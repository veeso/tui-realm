use std::ops::Add as _;
use std::time::{Duration, Instant};

use crate::Event;
use crate::listener::{ListenerResult, Poll};

/// A port is a wrapper around the poll trait object, which also defines an interval, which defines
/// the amount of time between each [`Poll::poll`] call.
/// Its purpose is to listen for incoming events of a user-defined type
pub struct SyncPort<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send,
{
    poll: Box<dyn Poll<UserEvent>>,
    interval: Duration,
    next_poll: Instant,
    max_poll: usize,
}

impl<UserEvent> SyncPort<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    /// Define a new [`SyncPort`]
    ///
    /// # Parameters
    ///
    /// * `poll` - The poll trait object
    /// * `interval` - The interval between each poll
    /// * `max_poll` - The maximum amount of times the port should be polled in a single poll
    pub fn new(poll: Box<dyn Poll<UserEvent>>, interval: Duration, max_poll: usize) -> Self {
        Self {
            poll,
            interval,
            next_poll: Instant::now(),
            max_poll,
        }
    }

    /// Get how often a port should get polled in a single poll
    pub fn max_poll(&self) -> usize {
        self.max_poll
    }

    /// Returns the interval for the current [`SyncPort`]
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
    pub fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
        self.poll.poll()
    }

    /// Calculate the next poll (t_now + interval)
    pub fn calc_next_poll(&mut self) {
        self.next_poll = Instant::now().add(self.interval);
    }
}
