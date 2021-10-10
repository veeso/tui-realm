//! ## listener
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
use super::Event;

use std::ops::Add;
use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use thiserror::Error;

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
pub trait Poll: Send {
    /// ### poll
    ///
    /// Poll for an input event from user.
    /// This function mustn't be blocking, and will be called within the configured interval of the event listener.
    /// It may return Error in case something went wrong.
    /// If it was possible to poll for event, `Ok` must be returned.
    /// If an event was read, then `Some()` must be returned., otherwise `None`.
    /// The event must be converted to `Event` using the `adapters`.
    fn poll(&self) -> ListenerResult<Option<Event>>;
}

/// ## EventListener
///
/// The event listener...
pub struct EventListener {
    poll_interval: Duration,
    running: Arc<RwLock<bool>>,
    tick_interval: Duration,
    recv: mpsc::Receiver<ListenerMsg>,
    thread: Option<JoinHandle<()>>,
}

impl EventListener {
    /// ### start
    ///
    /// Create a new `EventListener` and start it
    pub fn start(poll: Box<dyn Poll>, poll_interval: Duration, tick_interval: Duration) -> Self {
        // Prepare channel and running state
        let (sender, recv) = mpsc::channel();
        let running = Arc::new(RwLock::new(true));
        let running_t = Arc::clone(&running);
        // Start thread
        let thread = thread::spawn(move || {
            let config = Config::new(poll, sender, running_t, poll_interval, tick_interval);
            Self::run(config);
        });
        Self {
            running,
            poll_interval,
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

    // TODO: poll

    /// ### run
    ///
    /// thread run method
    fn run(cfg: Config) {}
}

// -- listener thread

/// ## ListenerMsg
///
/// Listener message is returned by the listener thread
enum ListenerMsg {
    Error(ListenerError),
    Tick,
    User(Event),
}

impl From<ListenerMsg> for ListenerResult<Event> {
    fn from(msg: ListenerMsg) -> Self {
        match msg {
            ListenerMsg::Error(err) => Err(err),
            ListenerMsg::Tick => Ok(Event::Tick),
            ListenerMsg::User(ev) => Ok(ev),
        }
    }
}

/// ## Config
///
/// Listener thread configuration
struct Config {
    poll: Box<dyn Poll>,
    sender: mpsc::Sender<ListenerMsg>,
    running: Arc<RwLock<bool>>,
    sleep_interval: Duration,
    next_poll: Instant,
    next_tick: Instant,
    poll_interval: Duration,
    tick_interval: Duration,
}

impl Config {
    fn new(
        poll: Box<dyn Poll>,
        sender: mpsc::Sender<ListenerMsg>,
        running: Arc<RwLock<bool>>,
        poll_interval: Duration,
        tick_interval: Duration,
    ) -> Self {
        Self {
            poll,
            sender,
            running,
            sleep_interval: *[poll_interval, tick_interval].iter().min().unwrap(),
            next_poll: Instant::now(),
            next_tick: Instant::now(),
            poll_interval,
            tick_interval,
        }
    }

    fn calc_next_poll(&mut self) {
        self.next_poll = Instant::now().add(self.poll_interval);
    }

    fn calc_next_tick(&mut self) {
        self.next_tick = Instant::now().add(self.tick_interval);
    }
}
