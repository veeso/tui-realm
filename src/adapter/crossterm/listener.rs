//! ## Listener
//!
//! input listener adapter for crossterm

use super::Event;

use crate::listener::{ListenerError, ListenerResult, Poll};
use crossterm::event as xterm;
use std::marker::PhantomData;
use std::time::Duration;

/// The input listener for crossterm.
/// If crossterm is enabled, this will already be exported as `InputEventListener` in the `adapter` module
/// or you can use it directly in the event listener, calling `default_input_listener()` in the `EventListenerCfg`
pub struct CrosstermInputListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    ghost: PhantomData<U>,
    interval: Duration,
}

impl<U> CrosstermInputListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    pub fn new(interval: Duration) -> Self {
        Self {
            ghost: PhantomData::default(),
            interval: interval / 2,
        }
    }
}

impl<U> Poll<U> for CrosstermInputListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
        match xterm::poll(self.interval) {
            Ok(true) => xterm::read()
                .map(|x| Some(Event::from(x)))
                .map_err(|_| ListenerError::PollFailed),
            Ok(false) => Ok(None),
            Err(_) => Err(ListenerError::PollFailed),
        }
    }
}
