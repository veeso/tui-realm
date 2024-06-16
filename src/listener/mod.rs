//! ## Listener
//!
//! This module exposes everything required to run the event listener to handle Input and
//! internal events in a tui-realm application.

// -- modules
mod builder;
mod port;
mod worker;

// -- export
use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub use builder::EventListenerCfg;
pub use port::Port;
use thiserror::Error;
use worker::EventListenerWorker;

// -- internal
use super::Event;
pub use crate::adapter::InputEventListener;

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

/// The poll trait defines the function `poll`, which will be called by the event listener
/// dedicated thread to poll for events.
pub trait Poll<UserEvent>: Send
where
    UserEvent: Eq + PartialEq + Clone + PartialOrd + 'static,
{
    /// Poll for an event from user or from another source (e.g. Network).
    /// This function mustn't be blocking, and will be called within the configured interval of the event listener.
    /// It may return Error in case something went wrong.
    /// If it was possible to poll for event, `Ok` must be returned.
    /// If an event was read, then `Some()` must be returned., otherwise `None`.
    /// The event must be converted to `Event` using the `adapters`.
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>>;
}

/// The event listener...
pub(crate) struct EventListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    /// Max Time to wait when calling `recv()` on thread receiver
    poll_timeout: Duration,
    /// Indicates whether the worker should paused polling ports
    paused: Arc<RwLock<bool>>,
    /// Msg receiver from worker
    recv: Option<mpsc::Receiver<ListenerMsg<U>>>,
    /// Join handle for worker
    thread: Option<JoinHandle<()>>,
}

impl<U> EventListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    /// Create a new `EventListener` and start it.
    /// - `poll` is the trait object which polls for input events
    /// - `poll_interval` is the interval to poll for input events. It should always be at least a poll time used by `poll`
    /// - `tick_interval` is the interval used to send the `Tick` event. If `None`, no tick will be sent.
    ///     Tick should be used only when you need to handle the tick in the interface through the Subscriptions.
    ///     The tick should have in this case, the same value (or less) of the refresh rate of the TUI.
    ///
    /// > Panics if `poll_timeout` is 0
    pub(self) fn start(
        ports: Vec<Port<U>>,
        poll_timeout: Duration,
        tick_interval: Option<Duration>,
    ) -> Self {
        if poll_timeout == Duration::ZERO {
            panic!(
                "poll timeout cannot be 0 (see <https://github.com/rust-lang/rust/issues/39364>)"
            )
        }
        // Prepare channel and running state
        let config = Self::setup_thread(ports, tick_interval);
        Self {
            paused: config.paused,
            poll_timeout,
            recv: Some(config.rx),
            thread: Some(config.thread),
        }
    }

    /// Stop event listener
    pub fn stop(&mut self) -> ListenerResult<()> {
        let _ = self.recv.take();
        // Join thread
        match self.thread.take().map(|x| x.join()) {
            Some(Ok(_)) => Ok(()),
            Some(Err(_)) => Err(ListenerError::CouldNotStop),
            None => Ok(()), // Never happens, unless someone calls stop twice
        }
    }

    /// Pause event listener worker
    pub fn pause(&mut self) -> ListenerResult<()> {
        // NOTE: keep these brackets to drop running after block
        let mut paused = match self.paused.write() {
            Ok(lock) => Ok(lock),
            Err(_) => Err(ListenerError::CouldNotStop),
        }?;
        *paused = true;
        Ok(())
    }

    /// Unpause event listener worker
    pub fn unpause(&mut self) -> ListenerResult<()> {
        // NOTE: keep these brackets to drop running after block
        let mut paused = match self.paused.write() {
            Ok(lock) => Ok(lock),
            Err(_) => Err(ListenerError::CouldNotStop),
        }?;
        *paused = false;
        Ok(())
    }

    /// Checks whether there are new events available from event
    pub fn poll(&self) -> ListenerResult<Option<Event<U>>> {
        let Some(ref recv) = self.recv else {
            return Err(ListenerError::PollFailed);
        };

        match recv.recv_timeout(self.poll_timeout) {
            Ok(msg) => ListenerResult::from(msg),
            Err(mpsc::RecvTimeoutError::Timeout) => Ok(None),
            Err(_) => Err(ListenerError::PollFailed),
        }
    }

    /// Setup the thread and returns the structs necessary to interact with it
    fn setup_thread(ports: Vec<Port<U>>, tick_interval: Option<Duration>) -> ThreadConfig<U> {
        let (sender, recv) = mpsc::channel();
        let paused = Arc::new(RwLock::new(false));
        let paused_t = Arc::clone(&paused);
        // Start thread
        let thread = thread::spawn(move || {
            EventListenerWorker::new(ports, sender, paused_t, tick_interval).run();
        });
        ThreadConfig::new(recv, paused, thread)
    }
}

impl<U> Drop for EventListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

// -- thread config

/// Config returned by thread setup
struct ThreadConfig<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    rx: mpsc::Receiver<ListenerMsg<U>>,
    paused: Arc<RwLock<bool>>,
    thread: JoinHandle<()>,
}

impl<U> ThreadConfig<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    pub fn new(
        rx: mpsc::Receiver<ListenerMsg<U>>,
        paused: Arc<RwLock<bool>>,
        thread: JoinHandle<()>,
    ) -> Self {
        Self { rx, paused, thread }
    }
}

// -- listener thread

/// Listener message is returned by the listener thread
enum ListenerMsg<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    Error(ListenerError),
    Tick,
    User(Event<U>),
}

impl<U> From<ListenerMsg<U>> for ListenerResult<Option<Event<U>>>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
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

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::core::event::{Key, KeyEvent};
    use crate::mock::{MockEvent, MockPoll};

    #[test]
    fn worker_should_run_thread() {
        let mut listener = EventListener::<MockEvent>::start(
            vec![Port::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(10),
            )],
            Duration::from_millis(10),
            Some(Duration::from_secs(3)),
        );
        // Wait 1 second
        thread::sleep(Duration::from_secs(1));
        // Poll (event)
        assert_eq!(
            listener.poll().ok().unwrap().unwrap(),
            Event::Keyboard(KeyEvent::from(Key::Enter))
        );
        // Poll (tick)
        assert_eq!(listener.poll().ok().unwrap().unwrap(), Event::Tick);
        // Poll (None)
        assert!(listener.poll().ok().unwrap().is_none());
        // Wait 3 seconds
        thread::sleep(Duration::from_secs(3));
        // New tick
        assert_eq!(listener.poll().ok().unwrap().unwrap(), Event::Tick);
        // Stop
        assert!(listener.stop().is_ok());
    }

    #[test]
    fn worker_should_be_paused() {
        let mut listener = EventListener::<MockEvent>::start(
            vec![],
            Duration::from_millis(10),
            Some(Duration::from_millis(750)),
        );
        thread::sleep(Duration::from_millis(100));
        assert!(listener.pause().is_ok());
        // Should be some
        assert_eq!(listener.poll().ok().unwrap().unwrap(), Event::Tick);
        // Wait tick time
        thread::sleep(Duration::from_secs(1));
        assert_eq!(listener.poll().ok().unwrap(), None);
        // Unpause
        assert!(listener.unpause().is_ok());
        thread::sleep(Duration::from_millis(300));
        assert_eq!(listener.poll().ok().unwrap().unwrap(), Event::Tick);
        // Stop
        assert!(listener.stop().is_ok());
    }

    #[test]
    #[should_panic]
    fn event_listener_with_poll_timeout_zero_should_panic() {
        EventListener::<MockEvent>::start(
            vec![],
            Duration::from_millis(0),
            Some(Duration::from_secs(3)),
        );
    }
}
