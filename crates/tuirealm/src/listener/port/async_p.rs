use std::ops::Add as _;
use std::time::{Duration, Instant};

use crate::Event;
use crate::listener::{ListenerResult, PollAsync};

/// An async port is a wrapper around the [`PollAsync`] trait object, which also defines an interval, which defines
/// the amount of time between each [`PollAsync::poll`] call.
/// Its purpose is to listen for incoming events of a user-defined type
///
/// [`AsyncPort`] has the possibility to run
pub struct AsyncPort<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    poll: Box<dyn PollAsync<U>>,
    interval: Duration,
    next_poll: Instant,
    max_poll: usize,
}

impl<U> AsyncPort<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    /// Define a new [`AsyncPort`]
    ///
    /// # Parameters
    ///
    /// * `poll` - The poll trait object
    /// * `interval` - The interval between each poll
    /// * `max_poll` - The maximum amount of times the port should be polled in a single poll
    /// * `runtime` - The tokio runtime to use for async polling
    pub fn new(poll: Box<dyn PollAsync<U>>, interval: Duration, max_poll: usize) -> Self {
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

    /// Calls [`PollAsync::poll`] on the inner [`PollAsync`] trait object.
    pub async fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
        self.poll.poll().await
    }

    /// Calculate the next poll (t_now + interval)
    pub fn calc_next_poll(&mut self) {
        self.next_poll = Instant::now().add(self.interval);
    }
}
