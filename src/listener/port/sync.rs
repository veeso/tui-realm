use alloc::boxed::Box;
use core::ops::Add as _;
use core::time::Duration;

use crate::Event;
use crate::core::clock::Instant;
use crate::listener::{Poll, PortResult};

/// A port is a wrapper around the [`Poll`] trait object, which also defines a interval, which defines
/// the amount of time between each [`Poll::poll`] call.
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
    /// * `max_poll` - The maximum amount of times the port should be polled in a single poll; needs to be at least 1
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
    pub fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>> {
        self.poll.poll()
    }

    /// Calculate the next poll (t_now + interval)
    pub fn calc_next_poll(&mut self) {
        self.next_poll = Instant::now().add(self.interval);
    }

    /// Mark the Port for dropping.
    pub(crate) fn mark_for_drop(&mut self) {
        self.max_poll = 0;
    }
}
