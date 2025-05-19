//! ## Listener
//!
//! This module exposes everything required to run the event listener to handle Input and
//! internal events in a tui-realm application.

// -- modules
mod builder;
mod port;
#[cfg(feature = "async-ports")]
mod task_pool;
mod worker;

use std::sync::atomic::{AtomicBool, Ordering};
// -- export
use std::sync::{Arc, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use thiserror::Error;

pub use self::builder::EventListenerCfg;
#[cfg(feature = "async-ports")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-ports")))]
pub use self::port::AsyncPort;
pub use self::port::SyncPort;
// -- internal
#[cfg(feature = "async-ports")]
use self::task_pool::TaskPool;
use self::worker::EventListenerWorker;
use super::Event;

/// Result returned by `EventListener`. [`Ok`] value depends on the method, while the
/// Err value is always [`ListenerError`].
pub type ListenerResult<T> = Result<T, ListenerError>;

#[derive(Debug, Error, PartialEq)]
pub enum ListenerError {
    #[error("failed to start event listener")]
    CouldNotStart,
    #[error("failed to stop event listener")]
    CouldNotStop,
    #[error("the event listener has died")]
    ListenerDied,
    #[error("poll() call returned error")]
    PollFailed,
    #[error("failed to start async listener because of missing runtime handle")]
    NoHandle,
}

/// The poll trait defines the function [`Poll::poll`], which will be called by the event listener
/// dedicated thread to poll for events if you use a [`SyncPort`].
pub trait Poll<UserEvent>: Send
where
    UserEvent: Eq + PartialEq + Clone + PartialOrd + 'static,
{
    /// Poll for an event from user or from another source (e.g. Network).
    /// This function mustn't be blocking, and will be called within the configured interval of the event listener.
    /// It may return Error in case something went wrong.
    /// If it was possible to poll for event, `Ok` must be returned.
    /// If an event was read, then [`Some`] must be returned, otherwise [`None`].
    /// The event must be converted to `Event` using the `adapters`.
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>>;
}

/// The poll trait defines the function [`PollAsync::poll`], which will be called by the event listener
/// dedicated thread to poll for events if you use a [`AsyncPort`].
#[cfg(feature = "async-ports")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-ports")))]
#[async_trait::async_trait]
pub trait PollAsync<UserEvent>: Send
where
    UserEvent: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    /// Poll for an with the possibility to do it asynchronously, from user or from another source (e.g. Network).
    /// This function mustn't be blocking, and will be called within the configured interval of the event listener.
    /// It may return Error in case something went wrong.
    /// If it was possible to poll for event, `Ok` must be returned.
    /// If an event was read, then [`Some`] must be returned, otherwise [`None`].
    /// The event must be converted to `Event` using the `adapters`.
    async fn poll(&self) -> ListenerResult<Option<Event<UserEvent>>>;
}

/// The event listener is a worker that runs in a separate thread and polls for events
/// from the [`Port`]s. It is responsible for sending events to the main thread and handling
/// internal events like `Tick`.
#[derive(Debug)]
pub(crate) struct EventListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    /// Max Time to wait when calling `recv()` on thread receiver
    poll_timeout: Duration,
    /// Indicates whether the worker should paused polling ports
    paused: Arc<AtomicBool>,
    /// Indicates whether the worker should keep running
    running: Arc<AtomicBool>,
    /// Msg receiver from worker
    recv: mpsc::Receiver<ListenerMsg<U>>,
    /// Join handle for worker
    thread: Option<JoinHandle<()>>,
    /// The taskpool to track all async ports and cancel them
    #[cfg(feature = "async-ports")]
    taskpool: Option<TaskPool>,
    /// The event transmitter to allow async ports from a different function
    #[cfg(feature = "async-ports")]
    tx: Option<mpsc::Sender<ListenerMsg<U>>>,
}

impl<U> EventListener<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    /// Create a new [`EventListener`] and start it.
    /// - `poll` is the trait object which polls for input events
    /// - `poll_interval` is the interval to poll for input events. It should always be at least a poll time used by `poll`
    /// - `tick_interval` is the interval used to send the `Tick` event. If `None`, no tick will be sent.
    /// - `store_tx` is used to determine if the Transmitter should be stored for [`start_async`](Self::start_async). Has not effect if `async-ports` is not enabled.
    ///
    /// Tick should be used only when you need to handle the tick in the interface through the Subscriptions.
    /// The tick should have in this case, the same value (or less) of the refresh rate of the TUI.
    ///
    /// NOTE: if `store_tx` is set to `true` but [`start_async`](Self::start_async) is never called, [`poll`](Self::poll) will always timeout instead of returning `Disconnected`
    /// after [`stop`](Self::stop) due a still open channel.
    ///
    /// # Panics
    ///
    /// - if `poll_timeout` is 0
    #[allow(unused_variables)] // "store_tx" is necessary when "async-ports" is active
    pub(self) fn start(
        ports: Vec<SyncPort<U>>,
        poll_timeout: Duration,
        tick_interval: Option<Duration>,
        store_tx: bool,
    ) -> Self {
        if poll_timeout == Duration::ZERO {
            panic!(
                "poll timeout cannot be 0 (see <https://github.com/rust-lang/rust/issues/39364>)"
            )
        }
        // Prepare channel and running state
        let config = Self::setup_thread(ports, tick_interval);

        #[cfg(feature = "async-ports")]
        let tx = if store_tx { Some(config.tx) } else { None };

        Self {
            paused: config.paused,
            running: config.running,
            poll_timeout,
            recv: config.rx,
            thread: Some(config.thread),
            #[cfg(feature = "async-ports")]
            taskpool: None,
            #[cfg(feature = "async-ports")]
            tx,
        }
    }

    /// Start the given async `ports` on a taskpool.
    ///
    /// # Panics
    ///
    /// If this function is called more than once per [`EventListener`].
    #[cfg(feature = "async-ports")]
    pub(self) fn start_async(
        mut self,
        ports: Vec<AsyncPort<U>>,
        handle: tokio::runtime::Handle,
    ) -> Self {
        let taskpool = TaskPool::new(handle);
        // unwrap is safe the first time as it is always assigned in "start"
        let tx = self.tx.take().unwrap();

        for port in ports {
            let tx = tx.clone();
            let paused = self.paused.clone();
            taskpool.spawn(async move {
                let _ = poll_task(port, tx, paused).await;
            });
        }

        self
    }

    /// Stop event listener(s)
    pub fn stop(&mut self) -> ListenerResult<()> {
        self.running.store(false, Ordering::Relaxed);

        #[cfg(feature = "async-ports")]
        if let Some(taskpool) = self.taskpool.as_ref() {
            taskpool.close();
            taskpool.cancel_all();
            // not waiting until all tasks are closed due to this function not being async
            // and "stop" potentially being called on a async context, which would then panic.
        }

        // Join thread
        match self.thread.take().map(|x| x.join()) {
            Some(Ok(_)) => Ok(()),
            Some(Err(_)) => Err(ListenerError::CouldNotStop),
            None => Ok(()), // Never happens, unless someone calls stop twice
        }
    }

    /// Pause event listener worker
    pub fn pause(&mut self) -> ListenerResult<()> {
        self.paused.store(true, Ordering::Relaxed);
        Ok(())
    }

    /// Unpause event listener worker
    pub fn unpause(&mut self) -> ListenerResult<()> {
        self.paused.store(false, Ordering::Relaxed);
        Ok(())
    }

    /// Checks whether there are new events available from event
    pub fn poll(&self) -> ListenerResult<Option<Event<U>>> {
        match self.recv.recv_timeout(self.poll_timeout) {
            Ok(msg) => ListenerResult::from(msg),
            Err(mpsc::RecvTimeoutError::Timeout) => Ok(None),
            Err(_) => Err(ListenerError::PollFailed),
        }
    }

    /// Setup the thread and returns the structs necessary to interact with it
    fn setup_thread(ports: Vec<SyncPort<U>>, tick_interval: Option<Duration>) -> ThreadConfig<U> {
        let (sender, recv) = mpsc::channel();
        let paused = Arc::new(AtomicBool::new(false));
        let paused_t = Arc::clone(&paused);
        let running = Arc::new(AtomicBool::new(true));
        let running_t = Arc::clone(&running);
        let sender_t = sender.clone();
        // Start thread
        let thread = thread::spawn(move || {
            EventListenerWorker::new(ports, sender_t, paused_t, running_t, tick_interval).run();
        });
        ThreadConfig::new(recv, sender, paused, running, thread)
    }
}

/// Continuesly drive a given port in a async fashion.
#[cfg(feature = "async-ports")]
async fn poll_task<U>(
    mut port: AsyncPort<U>,
    tx: mpsc::Sender<ListenerMsg<U>>,
    paused: Arc<AtomicBool>,
) -> Result<(), mpsc::SendError<ListenerMsg<U>>>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    let mut times_remaining = port.max_poll();
    loop {
        if paused.load(Ordering::Relaxed) {
            tokio::time::sleep(Duration::from_millis(25)).await;
            continue;
        }

        let msg = match port.poll().await {
            Ok(Some(ev)) => ListenerMsg::User(ev),
            Ok(None) => break,
            Err(err) => ListenerMsg::Error(err),
        };

        tx.send(msg)?;

        // do this at the end to at least call it once
        times_remaining = times_remaining.saturating_sub(1);

        if times_remaining == 0 {
            tokio::time::sleep(*port.interval()).await;
            times_remaining = port.max_poll();
        }
    }

    Ok(())
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
    #[allow(dead_code)] // used when "async-ports" is active, but always assigned
    tx: mpsc::Sender<ListenerMsg<U>>,
    paused: Arc<AtomicBool>,
    running: Arc<AtomicBool>,
    thread: JoinHandle<()>,
}

impl<U> ThreadConfig<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    pub fn new(
        rx: mpsc::Receiver<ListenerMsg<U>>,
        tx: mpsc::Sender<ListenerMsg<U>>,
        paused: Arc<AtomicBool>,
        running: Arc<AtomicBool>,
        thread: JoinHandle<()>,
    ) -> Self {
        Self {
            rx,
            tx,
            paused,
            running,
            thread,
        }
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
            vec![SyncPort::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(10),
                1,
            )],
            Duration::from_millis(10),
            Some(Duration::from_secs(3)),
            false,
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
            false,
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
            false,
        );
    }
}
