//! ## Listener
//!
//! input listener adapter for termion

use super::Event;

use crate::listener::{ListenerError, ListenerResult, Poll};
use std::io::stdin;
use std::marker::PhantomData;
use std::time::Duration;
use termion::input::TermRead;

/// The input listener for termion.
/// If termion is enabled, this will already be exported as `InputEventListener` in the `adapter` module
/// or you can use it directly in the event listener, calling `default_input_listener()` in the `EventListenerCfg`
pub struct TermionInputListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    ghost: PhantomData<U>,
}

impl<U> TermionInputListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    pub fn new(_interval: Duration) -> Self {
        Self { ghost: PhantomData }
    }
}

impl<U> Poll<U> for TermionInputListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
        match stdin().events().next() {
            Some(Ok(ev)) => Ok(Some(Event::from(ev))),
            Some(Err(_)) => Err(ListenerError::PollFailed),
            None => Ok(None),
        }
    }
}
