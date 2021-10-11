//! ## Listener
//!
//! This module exposes everything required to run the event listener to handle Input and
//! internal events in a tui-realm application.

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// -- modules
mod builder;
mod ev_listener;
mod worker;

// -- export
pub use crate::adapter::InputEventListener;
pub use builder::EventListenerCfg;

// -- internal
use super::Event;
pub use ev_listener::Listener;
use worker::EventListenerWorker;

use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use thiserror::Error;

/// ## ListenerResult
///
/// Result returned by `EventListener`. Ok value depends on the method, while the
/// Err value is always `ListenerError`.
pub type ListenerResult<T> = Result<T, ListenerError>;

#[derive(Debug, Error)]
pub enum ListenerError {
    #[error("failed to start event listener")]
    CouldNotStart,
    #[error("failed to stop event listener")]
    CouldNotStop,
    #[error("the event listener has died")]
    ListenerDied,
    #[error("poll() call returned error")]
    PollFailed,
}

/// ## Poll
///
/// The poll trait defines the function `poll`, which will be called by the event listener
/// dedicated thread to poll for events.
pub trait Poll<UserEvent: std::fmt::Debug + Eq + PartialEq + Copy + Clone + PartialOrd>:
    Send
{
    /// ### poll
    ///
    /// Poll for an event from user or from another source (e.g. Network).
    /// This function mustn't be blocking, and will be called within the configured interval of the event listener.
    /// It may return Error in case something went wrong.
    /// If it was possible to poll for event, `Ok` must be returned.
    /// If an event was read, then `Some()` must be returned., otherwise `None`.
    /// The event must be converted to `Event` using the `adapters`.
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>>;
}

/// ## EventListener
///
/// The event listener...
pub struct EventListener<U: std::fmt::Debug + Eq + PartialEq + Copy + Clone + PartialOrd + Send> {
    /// Indicates whether the worker should keep running
    running: Arc<RwLock<bool>>,
    /// Interval between each Tick event. If `None` no Tick will be sent
    tick_interval: Option<Duration>,
    /// Msg receiver from worker
    recv: mpsc::Receiver<ListenerMsg<U>>,
    /// Join handle for worker
    thread: Option<JoinHandle<()>>,
}

impl<U: std::fmt::Debug + Eq + PartialEq + Copy + Clone + PartialOrd + Send> EventListener<U> {
    /// ### start
    ///
    /// Create a new `EventListener` and start it.
    /// - `poll` is the trait object which polls for input events
    /// - `poll_interval` is the interval to poll for input events. It should always be at least a poll time used by `poll`
    /// - `tick_interval` is the interval used to send the `Tick` event. If `None`, no tick will be sent.
    ///     Tick should be used only when you need to handle the tick in the interface through the Subscriptions.
    ///     The tick should have in this case, the same value (or less) of the refresh rate of the TUI.
    pub(self) fn start(listeners: Vec<Listener<U>>, tick_interval: Option<Duration>) -> Self {
        // Prepare channel and running state
        let (recv, running, thread) = Self::setup_thread(listeners, tick_interval);
        Self {
            running,
            tick_interval,
            recv,
            thread: Some(thread),
        }
    }

    /// ### stop
    ///
    /// Stop event listener
    pub fn stop(&mut self) -> ListenerResult<()> {
        {
            // NOTE: keep these brackets to drop running after block
            let mut running = match self.running.write() {
                Ok(lock) => Ok(lock),
                Err(_) => Err(ListenerError::CouldNotStop),
            }?;
            *running = false;
        }
        // Join thread
        match self.thread.take().map(|x| x.join()) {
            Some(Ok(_)) => Ok(()),
            Some(Err(_)) => Err(ListenerError::CouldNotStop),
            None => Ok(()), // Never happens, unless someone calls stop twice
        }
    }

    /// ### restart
    ///
    /// Restart worker if previously died.
    /// Blocks if the thread hadn't actually died before, since this function will first try to join previously thread
    pub fn restart(&mut self, listeners: Vec<Listener<U>>) -> ListenerResult<()> {
        // Stop first
        self.stop()?;
        // Re-init thread
        let (recv, running, thread) = Self::setup_thread(listeners, self.tick_interval);
        self.recv = recv;
        self.running = running;
        self.thread = Some(thread);
        Ok(())
    }

    /// ### poll
    ///
    /// Checks whether there are new events available from event
    pub fn poll(&self, timeout: Duration) -> ListenerResult<Option<Event<U>>> {
        match self.recv.recv_timeout(timeout) {
            Ok(msg) => ListenerResult::from(msg),
            Err(mpsc::RecvTimeoutError::Timeout) => Ok(None),
            Err(_) => Err(ListenerError::PollFailed),
        }
    }

    /// ### setup_thread
    ///
    /// Setup the thread and returns the structs necessary to interact with it
    fn setup_thread(
        listeners: Vec<Listener<U>>,
        tick_interval: Option<Duration>,
    ) -> (
        mpsc::Receiver<ListenerMsg<U>>,
        Arc<RwLock<bool>>,
        JoinHandle<()>,
    ) {
        let (sender, recv) = mpsc::channel();
        let running = Arc::new(RwLock::new(true));
        let running_t = Arc::clone(&running);
        // Start thread
        let thread = thread::spawn(move || {
            EventListenerWorker::new(listeners, sender, running_t, tick_interval).run();
        });
        (recv, running, thread)
    }
}

// -- listener thread

/// ## ListenerMsg
///
/// Listener message is returned by the listener thread
enum ListenerMsg<U: std::fmt::Debug + Eq + PartialEq + Copy + Clone + PartialOrd + Send> {
    Error(ListenerError),
    Tick,
    User(Event<U>),
}

impl<U: std::fmt::Debug + Eq + PartialEq + Copy + Clone + PartialOrd + Send> From<ListenerMsg<U>>
    for ListenerResult<Option<Event<U>>>
{
    fn from(msg: ListenerMsg<U>) -> Self {
        match msg {
            ListenerMsg::Error(err) => Err(err),
            ListenerMsg::Tick => Ok(Some(Event::Tick)),
            ListenerMsg::User(ev) => Ok(Some(ev)),
        }
    }
}

#[cfg(test)]
mod test {

    use super::mock::MockPoll;
    use super::*;
    use crate::event::MockEvent;
    use crate::event::{Key, KeyEvent};

    use pretty_assertions::assert_eq;

    #[test]
    fn worker_should_run_thread() {
        const POLL_TIMEOUT: Duration = Duration::from_millis(100);
        let mut listener = EventListener::<MockEvent>::start(
            vec![Listener::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(10),
            )],
            Some(Duration::from_secs(3)),
        );
        // Wait 1 second
        thread::sleep(Duration::from_secs(1));
        // Poll (event)
        assert_eq!(
            listener.poll(POLL_TIMEOUT).ok().unwrap().unwrap(),
            Event::Keyboard(KeyEvent::from(Key::Enter))
        );
        // Poll (tick)
        assert_eq!(
            listener.poll(POLL_TIMEOUT).ok().unwrap().unwrap(),
            Event::Tick
        );
        // Poll (None)
        assert!(listener.poll(POLL_TIMEOUT).ok().unwrap().is_none());
        // Wait 3 seconds
        thread::sleep(Duration::from_secs(3));
        // New tick
        assert_eq!(
            listener.poll(POLL_TIMEOUT).ok().unwrap().unwrap(),
            Event::Tick
        );
        // Stop
        assert!(listener.stop().is_ok());
        // Restart
        assert!(listener
            .restart(vec![Listener::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(5)
            )])
            .is_ok());
        // Wait 1 second
        thread::sleep(Duration::from_secs(1));
        // Poll (event)
        assert_eq!(
            listener.poll(POLL_TIMEOUT).ok().unwrap().unwrap(),
            Event::Keyboard(KeyEvent::from(Key::Enter))
        );
        // Stop
        assert!(listener.stop().is_ok());
    }
}
#[cfg(test)]
pub mod mock {

    use super::{Event, ListenerResult, Poll};
    use crate::event::{Key, KeyEvent};

    use std::marker::PhantomData;

    pub struct MockPoll<U: std::fmt::Debug + Eq + PartialEq + Copy + Clone + PartialOrd + Send> {
        ghost: PhantomData<U>,
    }

    impl<U: std::fmt::Debug + Eq + PartialEq + Copy + Clone + PartialOrd + Send> Default
        for MockPoll<U>
    {
        fn default() -> Self {
            Self {
                ghost: PhantomData::default(),
            }
        }
    }

    impl<U: std::fmt::Debug + Eq + PartialEq + Copy + Clone + PartialOrd + Send> Poll<U>
        for MockPoll<U>
    {
        fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
            Ok(Some(Event::Keyboard(KeyEvent::from(Key::Enter))))
        }
    }
}
