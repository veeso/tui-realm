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
use super::Event;

use std::ops::{Add, Sub};
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
    /// Interval between each poll() call in worker
    poll_interval: Duration,
    /// Indicates whether the worker should keep running
    running: Arc<RwLock<bool>>,
    /// Interval between each Tick event. If `None` no Tick will be sent
    tick_interval: Option<Duration>,
    /// Msg receiver from worker
    recv: mpsc::Receiver<ListenerMsg>,
    /// Join handle for worker
    thread: Option<JoinHandle<()>>,
}

impl EventListener {
    /// ### start
    ///
    /// Create a new `EventListener` and start it.
    /// - `poll` is the trait object which polls for input events
    /// - `poll_interval` is the interval to poll for input events. It should always be at least a poll time used by `poll`
    /// - `tick_interval` is the interval used to send the `Tick` event. If `None`, no tick will be sent.
    ///     Tick should be used only when you need to handle the tick in the interface through the Subscriptions.
    ///     The tick should have in this case, the same value (or less) of the refresh rate of the TUI.
    pub fn start(
        poll: Box<dyn Poll>,
        poll_interval: Duration,
        tick_interval: Option<Duration>,
    ) -> Self {
        // Prepare channel and running state
        let (recv, running, thread) = Self::setup_thread(poll, poll_interval, tick_interval);
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

    /// ### restart
    ///
    /// Restart worker if previously died.
    /// Blocks if the thread hadn't actually died before, since this function will first try to join previously thread
    pub fn restart(&mut self, poll: Box<dyn Poll>) -> ListenerResult<()> {
        // Stop first
        self.stop()?;
        // Re-init thread
        let (recv, running, thread) =
            Self::setup_thread(poll, self.poll_interval, self.tick_interval);
        self.recv = recv;
        self.running = running;
        self.thread = Some(thread);
        Ok(())
    }

    /// ### poll
    ///
    /// Checks whether there are new events available from event
    pub fn poll(&self, timeout: Duration) -> ListenerResult<Option<Event>> {
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
        poll: Box<dyn Poll>,
        poll_interval: Duration,
        tick_interval: Option<Duration>,
    ) -> (
        mpsc::Receiver<ListenerMsg>,
        Arc<RwLock<bool>>,
        JoinHandle<()>,
    ) {
        let (sender, recv) = mpsc::channel();
        let running = Arc::new(RwLock::new(true));
        let running_t = Arc::clone(&running);
        // Start thread
        let thread = thread::spawn(move || {
            EventListenerWorker::new(poll, sender, running_t, poll_interval, tick_interval).run();
        });
        (recv, running, thread)
    }
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

impl From<ListenerMsg> for ListenerResult<Option<Event>> {
    fn from(msg: ListenerMsg) -> Self {
        match msg {
            ListenerMsg::Error(err) => Err(err),
            ListenerMsg::Tick => Ok(Some(Event::Tick)),
            ListenerMsg::User(ev) => Ok(Some(ev)),
        }
    }
}

// -- worker

/// ## EventListenerWorker
///
/// worker for event listener
struct EventListenerWorker {
    poll: Box<dyn Poll>,
    sender: mpsc::Sender<ListenerMsg>,
    running: Arc<RwLock<bool>>,
    next_poll: Instant,
    next_tick: Instant,
    poll_interval: Duration,
    tick_interval: Option<Duration>,
    // TODO: custom closures to poll
}

impl EventListenerWorker {
    fn new(
        poll: Box<dyn Poll>,
        sender: mpsc::Sender<ListenerMsg>,
        running: Arc<RwLock<bool>>,
        poll_interval: Duration,
        tick_interval: Option<Duration>,
    ) -> Self {
        Self {
            poll,
            sender,
            running,
            next_poll: Instant::now(),
            next_tick: Instant::now(),
            poll_interval,
            tick_interval,
        }
    }

    /// ### calc_next_poll
    ///
    /// Calc next poll time
    fn calc_next_poll(&mut self) {
        self.next_poll = Instant::now().add(self.poll_interval);
    }

    /// ### calc_next_tick
    ///
    /// Calculate next tick time.
    /// If tick is None, panics.
    fn calc_next_tick(&mut self) {
        self.next_tick = Instant::now().add(self.tick_interval.unwrap());
    }

    /// ### next_event
    ///
    /// Calc the distance in time between now and the first upcoming event
    fn next_event(&self) -> Duration {
        let now = Instant::now();
        if self.tick_interval.is_none() {
            // If tick is None, returns difference between next poll and now
            self.next_poll.sub(now)
        } else {
            // Get min next event and subtract now
            [self.next_poll, self.next_tick]
                .iter()
                .min()
                .unwrap_or(&now)
                .sub(now)
        }
    }

    /// ### running
    ///
    /// Returns whether should keep running
    fn running(&self) -> bool {
        if let Ok(lock) = self.running.read() {
            return *lock;
        }
        true
    }

    /// ### should_poll
    ///
    /// Returns whether it's time to poll
    fn should_poll(&self) -> bool {
        self.next_poll <= Instant::now()
    }

    /// ### should_tick
    ///
    /// Returns whether it's time to tick.
    /// If tick_interval is `None` it will never return `true`
    fn should_tick(&self) -> bool {
        match self.tick_interval {
            None => false,
            Some(_) => self.next_tick <= Instant::now(),
        }
    }

    /// ### send_tick
    ///
    /// Send tick to listener and calc next tick
    fn send_tick(&mut self) -> Result<(), mpsc::SendError<ListenerMsg>> {
        // Send tick
        match self.sender.send(ListenerMsg::Tick) {
            // Terminate thread on send failed
            Err(err) => Err(err),
            Ok(_) => {
                // Calc next tick
                self.calc_next_tick();
                Ok(())
            }
        }
    }

    /// ### poll
    ///
    /// Poll and send poll to listener. Calc next poll.
    /// If send fails, the first error returns Err
    /// Otherwise returns the result of the poll call
    fn poll(&mut self) -> Result<(), mpsc::SendError<ListenerMsg>> {
        // Poll
        let msg = match self.poll.poll() {
            Ok(Some(ev)) => Some(ListenerMsg::User(ev)),
            Ok(None) => None,
            Err(err) => Some(ListenerMsg::Error(err)),
        };
        // Send msg if any
        match msg {
            None => {
                // Calc next poll
                self.calc_next_poll();
                Ok(())
            }
            Some(msg) => match self.sender.send(msg) {
                Ok(_) => {
                    // Calc next poll
                    self.calc_next_poll();
                    Ok(())
                }
                Err(err) => Err(err),
            },
        }
    }

    /// ### run
    ///
    /// thread run method
    fn run(&mut self) {
        loop {
            // Check if running
            if !self.running() {
                break;
            }
            // Check whether to poll
            if self.should_poll() && self.poll().is_err() {
                break;
            }
            // Tick
            if self.should_tick() && self.send_tick().is_err() {
                break;
            }
            // Sleep till next event
            thread::sleep(self.next_event())
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::event::{Key, KeyEvent};

    use pretty_assertions::assert_eq;

    #[test]
    fn worker_should_run_thread() {
        const POLL_TIMEOUT: Duration = Duration::from_millis(100);
        let mut listener = EventListener::start(
            Box::new(FakePoll {}),
            Duration::from_secs(5),
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
        assert!(listener.restart(Box::new(FakePoll {})).is_ok());
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

    #[test]
    fn worker_should_send_poll() {
        let (tx, rx) = mpsc::channel();
        let running = Arc::new(RwLock::new(true));
        let running_t = Arc::clone(&running);
        let mut worker = EventListenerWorker::new(
            Box::new(FakePoll {}),
            tx,
            running_t,
            Duration::from_secs(5),
            None,
        );
        assert!(worker.poll().is_ok());
        assert!(worker.next_poll > Instant::now());
        // Receive
        assert_eq!(
            ListenerResult::from(rx.recv().ok().unwrap()).ok().unwrap(),
            Some(Event::Keyboard(KeyEvent::from(Key::Enter)))
        );
    }

    #[test]
    fn worker_should_send_tick() {
        let (tx, rx) = mpsc::channel();
        let running = Arc::new(RwLock::new(true));
        let running_t = Arc::clone(&running);
        let mut worker = EventListenerWorker::new(
            Box::new(FakePoll {}),
            tx,
            running_t,
            Duration::from_secs(5),
            Some(Duration::from_secs(1)),
        );
        assert!(worker.send_tick().is_ok());
        assert!(worker.next_tick > Instant::now());
        // Receive
        assert_eq!(
            ListenerResult::from(rx.recv().ok().unwrap()).ok().unwrap(),
            Some(Event::Tick)
        );
    }

    #[test]
    fn worker_should_calc_times_correctly_with_tick() {
        let (tx, rx) = mpsc::channel();
        let running = Arc::new(RwLock::new(true));
        let running_t = Arc::clone(&running);
        let mut worker = EventListenerWorker::new(
            Box::new(FakePoll {}),
            tx,
            running_t,
            Duration::from_secs(5),
            Some(Duration::from_secs(1)),
        );
        assert_eq!(worker.running(), true);
        // Should set next events to now
        assert!(worker.next_poll <= Instant::now());
        assert!(worker.next_tick <= Instant::now());
        assert!(worker.should_poll());
        assert!(worker.should_tick());
        // Calc next
        let expected_next_poll = Instant::now().add(Duration::from_secs(5));
        worker.calc_next_poll();
        assert!(worker.next_poll >= expected_next_poll);
        let expected_next_tick = Instant::now().add(Duration::from_secs(1));
        worker.calc_next_tick();
        assert!(worker.next_tick >= expected_next_tick);
        // Next event should be in 1 second (tick)
        assert!(worker.next_event() <= Duration::from_secs(1));
        // Now should no more tick and poll
        assert_eq!(worker.should_poll(), false);
        assert_eq!(worker.should_tick(), false);
        // Stop
        {
            let mut running_flag = match running.write() {
                Ok(lock) => Ok(lock),
                Err(_) => Err(ListenerError::CouldNotStop),
            }
            .ok()
            .unwrap();
            *running_flag = false;
        }
        assert_eq!(worker.running(), false);
        drop(rx);
    }

    #[test]
    fn worker_should_calc_times_correctly_without_tick() {
        let (tx, rx) = mpsc::channel();
        let running = Arc::new(RwLock::new(true));
        let running_t = Arc::clone(&running);
        let mut worker = EventListenerWorker::new(
            Box::new(FakePoll {}),
            tx,
            running_t,
            Duration::from_secs(3),
            None,
        );
        assert_eq!(worker.running(), true);
        // Should set next events to now
        assert!(worker.next_poll <= Instant::now());
        assert!(worker.next_tick <= Instant::now());
        assert!(worker.should_poll());
        assert_eq!(worker.should_tick(), false);
        // Calc next
        let expected_next_poll = Instant::now().add(Duration::from_secs(3));
        worker.calc_next_poll();
        assert!(worker.next_poll >= expected_next_poll);
        // Next event should be in 3 second (poll)
        assert!(worker.next_event() <= Duration::from_secs(3));
        // Now should no more poll
        assert_eq!(worker.should_poll(), false);
        // Stop
        {
            let mut running_flag = match running.write() {
                Ok(lock) => Ok(lock),
                Err(_) => Err(ListenerError::CouldNotStop),
            }
            .ok()
            .unwrap();
            *running_flag = false;
        }
        assert_eq!(worker.running(), false);
        drop(rx);
    }

    #[test]
    #[should_panic]
    fn worker_should_panic_when_trying_next_tick_without_it() {
        let (tx, _) = mpsc::channel();
        let running = Arc::new(RwLock::new(true));
        let running_t = Arc::clone(&running);
        let mut worker = EventListenerWorker::new(
            Box::new(FakePoll {}),
            tx,
            running_t,
            Duration::from_secs(1),
            None,
        );
        worker.calc_next_tick();
    }

    // -- fake poll

    struct FakePoll;

    impl Poll for FakePoll {
        fn poll(&self) -> ListenerResult<Option<Event>> {
            Ok(Some(Event::Keyboard(KeyEvent::from(Key::Enter))))
        }
    }
}
