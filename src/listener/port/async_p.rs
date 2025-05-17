use std::ops::Add as _;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::runtime::Handle as TokioRuntime;

use crate::listener::{ListenerResult, PollAsync};
use crate::{Event, ListenerError};

/// An async port is a wrapper around the [`PollAsync`] trait object, which also defines an interval, which defines
/// the amount of time between each [`PollAsync::poll`] call.
/// Its purpose is to listen for incoming events of a user-defined type
///
/// [`AsyncPort`] has the possibility to run
pub struct AsyncPort<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    poll: Arc<Box<dyn PollAsync<U> + Send + Sync>>,
    interval: Duration,
    next_poll: Instant,
    max_poll: usize,
    runtime: Arc<TokioRuntime>,
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
    pub fn new(
        poll: Box<dyn PollAsync<U> + Send + Sync>,
        interval: Duration,
        max_poll: usize,
        runtime: &Arc<TokioRuntime>,
    ) -> Self {
        Self {
            poll: Arc::new(poll),
            interval,
            next_poll: Instant::now(),
            max_poll,
            runtime: runtime.clone(),
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
    pub fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
        let rt = self.runtime.clone();
        let poll_impl = self.poll.clone();

        std::thread::spawn(move || rt.block_on(poll_impl.poll()))
            .join()
            .map_err(|_| ListenerError::PollFailed)?
    }

    /// Calculate the next poll (t_now + interval)
    pub fn calc_next_poll(&mut self) {
        self.next_poll = Instant::now().add(self.interval);
    }
}
