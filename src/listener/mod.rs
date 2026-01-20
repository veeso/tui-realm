//! ## Listener
//!
//! This module exposes everything required to run the event listener to handle Input and
//! internal events in a tui-realm application.

// -- modules
#[cfg(feature = "async-ports")]
mod async_ticker;
mod builder;
mod port;
#[cfg(feature = "async-ports")]
mod task_pool;
mod worker;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::TryRecvError;
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
type ListenerResult<T> = Result<T, ListenerError>;
/// Result type for [`Poll`] and [`PollAsync`].
pub type PortResult<T> = Result<T, PortError>;
type PollResult<T> = Result<T, PollError>;

/// Start / Stop Errors in a Listener.
#[derive(Debug, Error, PartialEq)]
pub enum ListenerError {
    #[error("failed to start event listener")]
    CouldNotStart,
    #[error("failed to stop event listener")]
    CouldNotStop,
    #[error("failed to start async listener because of missing runtime handle")]
    NoHandle,
}

/// Errors from a port directly.
#[derive(Debug, Error, PartialEq)]
pub enum PortError {
    /// A Intermittend Error in a port will result in the port being polled again.
    #[error("A intermittend Error happened: {0}")]
    IntermittentError(String),
    /// A Permanent Error in a port will result in the port not being polled again.
    #[error("A permanent Error happened: {0}")]
    PermanentError(String),
}

/// Errors that can happen in a Listener `poll`.
#[derive(Debug, Error, PartialEq)]
pub enum PollError {
    #[error("the event listener has died")]
    ListenerDied,
    #[error("a port returned a error: {0}")]
    PortError(#[from] PortError),
}

/// The poll trait defines the function [`Poll::poll`], which will be called by the event listener
/// dedicated thread to poll for events when you use a [`SyncPort`].
pub trait Poll<UserEvent>: Send
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    /// Poll for a event from input listeners, or from custom ports (e.g. Network).
    /// This function must not be blocking, and will be called within the configured interval of the event listener.
    ///
    /// - If polling failed, `Err` should be returned. The port will be polled again if the error is [`PortError::IntermittendError`].
    /// - If a event is available, `Ok(Some)` needs to be returned.
    /// - If there is no event available, `Ok(None)` needs to be returned. The port will be polled again.
    fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>>;
}

/// The poll trait defines the function [`PollAsync::poll`], which will be called by the runtime
/// to poll for events when you use a [`AsyncPort`].
#[cfg(feature = "async-ports")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-ports")))]
#[async_trait::async_trait]
pub trait PollAsync<UserEvent>: Send
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    /// Poll for a event with the possibility to do it asynchronously, from input listeners, or from custom ports (e.g. Network).
    ///
    /// - If polling failed, `Err` should be returned. The port will be polled again if the error is [`PortError::IntermittendError`].
    /// - If a event is available, `Ok(Some)` needs to be returned.
    /// - If no events are available, await until one becomes available.
    /// - If there are no more events expected, `Ok(None)` should be returned. The port will not be polled again.
    async fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>>;
}

/// The event listener is a worker that runs in a separate thread and polls for events
/// from the [`Port`]s. It is responsible for sending events to the main thread and handling
/// internal events like `Tick`.
#[derive(Debug)]
pub(crate) struct EventListener<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    /// Max Time to wait when calling `recv()` on thread receiver
    poll_timeout: Duration,
    /// Indicates whether the worker should paused polling ports
    paused: Arc<AtomicBool>,
    /// Indicates whether the worker should keep running
    running: Arc<AtomicBool>,
    /// Msg receiver from worker
    recv: mpsc::Receiver<ListenerMsg<UserEvent>>,
    /// Join handle for worker
    thread: Option<JoinHandle<()>>,
    /// The taskpool to track all async ports and cancel them
    #[cfg(feature = "async-ports")]
    taskpool: Option<TaskPool>,
    /// The event emitter associated with `recv`, until `start` and `start_async` are called.
    ///
    /// This needs to be [`None`] after either [`start`](Self::start) or [`start_async`](Self::start_async) is called, otherwise the channel will never close.
    tx: Option<mpsc::Sender<ListenerMsg<UserEvent>>>,

    #[cfg(test)]
    barrier: Option<builder::test_utils::BarrierTx>,
}

impl<UserEvent> EventListener<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    /// Create a new [`EventListener`].
    /// - `poll_interval` is the interval to poll for input events. It should always be at least a poll time used by `poll`
    ///
    /// It is recommended to not set the time to [`Duration::ZERO`].
    pub(self) fn new(poll_timeout: Duration) -> Self {
        let (sender, recv) = mpsc::channel();
        let paused = Arc::new(AtomicBool::new(false));
        let running = Arc::new(AtomicBool::new(false));

        Self {
            poll_timeout,
            paused,
            running,
            recv,
            thread: None,
            #[cfg(feature = "async-ports")]
            taskpool: None,
            tx: Some(sender),

            #[cfg(test)]
            barrier: None,
        }
    }

    /// Attach a test barrier to the event listener.
    ///
    /// This currently only applies to the SYNC worker.
    #[cfg(test)]
    pub fn with_test_barrier(&mut self, barrier: Option<builder::test_utils::BarrierTx>) {
        self.barrier = barrier;
    }

    /// Start a worker for Sync-Ports.
    /// - `ports` are the sync-ports to start on the worker.
    /// - `tick_interval` is the interval used to send the `Tick` event. If `None`, no tick will be sent.
    /// - `store_tx` is used to determine if the Transmitter should be stored for [`start_async`](Self::start_async). Has not effect if `async-ports` is not enabled.
    ///
    /// Tick should be used only when you need to handle the tick in the interface through the Subscriptions.
    /// The tick should have in this case, the same value (or less) of the refresh rate of the TUI.
    ///
    /// NOTE: if `store_tx` is set to `true` but [`start_async`](Self::start_async) is never called, [`poll`](Self::poll) will always timeout instead of returning `Disconnected`
    /// after [`stop`](Self::stop) due a still open channel.
    #[allow(unused_variables)] // "store_tx" is necessary when "async-ports" is active
    pub(self) fn start(
        self,
        ports: Vec<SyncPort<UserEvent>>,
        tick_interval: Option<Duration>,
        store_tx: bool,
    ) -> Self {
        #[cfg(feature = "async-ports")]
        let tx = if store_tx { self.tx.clone() } else { None };

        // Prepare channel and running state
        #[allow(unused_mut)] // mutability is necessary when "async-ports" is active
        let mut res = self.setup_sync_worker(ports, tick_interval);

        #[cfg(feature = "async-ports")]
        {
            res.tx = tx;
        }

        res
    }

    /// Start the given async `ports` on a taskpool.
    /// - `ports` are the async-ports to start on the given runtime.
    /// - `tick_interval` is the interval used to send the `Tick` event. If `None`, no tick will be sent.
    ///
    /// # Panics
    ///
    /// If this function is called more than once per [`EventListener`].
    #[cfg(feature = "async-ports")]
    pub(self) fn start_async(
        mut self,
        mut ports: Vec<AsyncPort<UserEvent>>,
        handle: tokio::runtime::Handle,
        tick_interval: Option<Duration>,
    ) -> Self {
        use async_ticker::AsyncTicker;

        let taskpool = TaskPool::new(handle);
        // unwrap is safe the first time as it is always assigned in "start"
        let tx = self.tx.take().unwrap();

        if let Some(tick_interval) = tick_interval {
            ports.push(AsyncPort::new(
                Box::new(AsyncTicker::new()),
                tick_interval,
                1,
            ));
        }

        for port in ports {
            let tx = tx.clone();
            let paused = self.paused.clone();
            taskpool.spawn(async move {
                let _ = poll_task(port, tx, paused).await;
            });
        }

        self.taskpool = Some(taskpool);

        self
    }

    /// Stop event listener(s).
    ///
    /// This will *not* wait until the Sync and Async event listeners are fully stopped.
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
        match self.thread.take().map(|x| {
            // "thread::JoinHandle" only allows blocking joins (as of rust 1.92), meaning that if the thread is doing stuff and not finished (ex deadlocking),
            // then this will wait infinitely, so the best option is to just "detach" the thread, as we dont particularly care about the result.
            // We already set the "stop" flag, so it should exit on the earliest opportunity it can (if not deadlocked).
            // A deadlock can happen if the drop order is "listener, then BarrierRx", meaning the thread will wait until the BarrierRx
            // can recieve, which will never happen as the only reciever's thread is waiting for the listener thread to stop, which results in a deadlock.
            if x.is_finished() { x.join() } else { Ok(()) }
        }) {
            Some(Ok(())) => Ok(()),
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

    /// Checks whether there are new events available, blocking until either a event is recieved or [`poll_timeout`](Self::poll_timeout).
    pub fn poll_timeout(&self) -> PollResult<Option<Event<UserEvent>>> {
        match self.recv.recv_timeout(self.poll_timeout) {
            Ok(msg) => PollResult::from(msg),
            Err(mpsc::RecvTimeoutError::Timeout) => Ok(None),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(PollError::ListenerDied),
        }
    }

    /// Checks whether there are new events available, blocking until a event is recieved.
    pub fn poll_blocking(&self) -> PollResult<Event<UserEvent>> {
        match self.recv.recv() {
            Ok(msg) => PollResult::from(msg),
            Err(mpsc::RecvError) => Err(PollError::ListenerDied),
        }
    }

    /// Checks whether there are new events available, without blocking for any amount of time
    pub fn try_poll(&self) -> PollResult<Option<Event<UserEvent>>> {
        match self.recv.try_recv() {
            Ok(msg) => PollResult::from(msg),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(PollError::ListenerDied),
        }
    }

    /// Setup the thread and returns the structs necessary to interact with it
    fn setup_sync_worker(
        mut self,
        ports: Vec<SyncPort<UserEvent>>,
        tick_interval: Option<Duration>,
    ) -> Self {
        self.running.store(true, Ordering::Relaxed);
        let paused_t = self.paused.clone();
        let running_t = self.running.clone();
        let sender_t = self.tx.take().unwrap().clone();
        #[cfg(test)]
        let barrier = self.barrier.take();
        // Start thread
        let thread = thread::spawn(move || {
            let mut worker =
                EventListenerWorker::new(ports, sender_t, paused_t, running_t, tick_interval);

            #[cfg(test)]
            worker.with_test_barrier(barrier);

            worker.run();
        });
        self.thread = Some(thread);
        self
    }
}

/// Continuesly drive a given port in a async fashion.
#[cfg(feature = "async-ports")]
async fn poll_task<UserEvent>(
    mut port: AsyncPort<UserEvent>,
    tx: mpsc::Sender<ListenerMsg<UserEvent>>,
    paused: Arc<AtomicBool>,
) -> Result<(), mpsc::SendError<ListenerMsg<UserEvent>>>
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    let mut times_remaining = port.max_poll();
    loop {
        let mut should_stop = false;
        if paused.load(Ordering::Relaxed) {
            tokio::time::sleep(Duration::from_millis(25)).await;
            continue;
        }

        let msg = match port.poll().await {
            Ok(Some(ev)) => ListenerMsg::User(ev),
            Ok(None) => break,
            Err(err) => {
                if let PortError::PermanentError(_) = &err {
                    should_stop = true;
                }
                ListenerMsg::Error(err)
            }
        };

        tx.send(msg)?;

        if should_stop {
            break;
        }

        // do this at the end to at least call it once
        times_remaining = times_remaining.saturating_sub(1);

        if times_remaining == 0 {
            tokio::time::sleep(*port.interval()).await;
            times_remaining = port.max_poll();
        }
    }

    Ok(())
}

impl<UserEvent> Drop for EventListener<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

// -- listener thread

/// Listener message is returned by the listener thread
#[derive(Debug)]
enum ListenerMsg<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send,
{
    Error(PortError),
    User(Event<UserEvent>),
}

impl<UserEvent> From<ListenerMsg<UserEvent>> for PollResult<Option<Event<UserEvent>>>
where
    UserEvent: Eq + PartialEq + Clone + Send,
{
    fn from(msg: ListenerMsg<UserEvent>) -> Self {
        match msg {
            ListenerMsg::Error(err) => Err(err.into()),
            ListenerMsg::User(ev) => Ok(Some(ev)),
        }
    }
}

impl<UserEvent> From<ListenerMsg<UserEvent>> for PollResult<Event<UserEvent>>
where
    UserEvent: Eq + PartialEq + Clone + Send,
{
    fn from(msg: ListenerMsg<UserEvent>) -> Self {
        match msg {
            ListenerMsg::Error(err) => Err(err.into()),
            ListenerMsg::User(ev) => Ok(ev),
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::core::event::{Key, KeyEvent};
    use crate::mock::{MockEvent, MockPoll};

    #[test]
    fn worker_should_run_thread() {
        let mut listener = EventListener::<MockEvent>::new(Duration::from_millis(10)).start(
            vec![SyncPort::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(10),
                1,
            )],
            Some(Duration::from_secs(3)),
            false,
        );
        // Wait 1 second
        thread::sleep(Duration::from_secs(1));
        // Poll (event)
        assert_eq!(
            listener.poll_timeout().ok().unwrap().unwrap(),
            Event::Keyboard(KeyEvent::from(Key::Enter))
        );
        // Poll (tick)
        assert_eq!(listener.poll_timeout().ok().unwrap().unwrap(), Event::Tick);
        // Poll (None)
        assert!(listener.poll_timeout().ok().unwrap().is_none());
        // Wait 3 seconds
        thread::sleep(Duration::from_secs(3));
        // New tick
        assert_eq!(listener.poll_timeout().ok().unwrap().unwrap(), Event::Tick);
        // Stop
        assert!(listener.stop().is_ok());
    }

    #[test]
    fn worker_should_be_paused() {
        let mut listener = EventListener::<MockEvent>::new(Duration::from_millis(10)).start(
            vec![],
            Some(Duration::from_millis(750)),
            false,
        );
        thread::sleep(Duration::from_millis(100));
        assert!(listener.pause().is_ok());
        // Should be some
        assert_eq!(listener.poll_timeout().ok().unwrap().unwrap(), Event::Tick);
        // Wait tick time
        thread::sleep(Duration::from_secs(1));
        assert_eq!(listener.poll_timeout().ok().unwrap(), None);
        // Unpause
        assert!(listener.unpause().is_ok());
        thread::sleep(Duration::from_millis(300));
        assert_eq!(listener.poll_timeout().ok().unwrap().unwrap(), Event::Tick);
        // Stop
        assert!(listener.stop().is_ok());
    }

    #[test]
    fn try_poll_should_work() {
        let mut listener = EventListener::<MockEvent>::new(Duration::from_millis(10)).start(
            vec![SyncPort::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(10),
                1,
            )],
            Some(Duration::from_secs(3)),
            false,
        );
        // Wait 1 second
        thread::sleep(Duration::from_secs(1));
        // Poll (event)
        assert_eq!(
            listener.try_poll().ok().unwrap().unwrap(),
            Event::Keyboard(KeyEvent::from(Key::Enter))
        );
        // Poll (tick)
        assert_eq!(listener.try_poll().ok().unwrap().unwrap(), Event::Tick);
        // Poll (None)
        assert!(listener.try_poll().ok().unwrap().is_none());
        // Wait 3 seconds
        thread::sleep(Duration::from_secs(3));
        // New tick
        assert_eq!(listener.try_poll().ok().unwrap().unwrap(), Event::Tick);
        // Stop
        assert!(listener.stop().is_ok());
    }

    #[test]
    fn poll_blocking_should_work() {
        let mut listener = EventListener::<MockEvent>::new(Duration::from_millis(10)).start(
            vec![SyncPort::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(10),
                1,
            )],
            Some(Duration::from_secs(3)),
            false,
        );
        // Wait 1 second
        thread::sleep(Duration::from_secs(1));
        // Poll (event)
        assert_eq!(
            listener.poll_blocking().ok().unwrap(),
            Event::Keyboard(KeyEvent::from(Key::Enter))
        );
        // Poll (tick)
        assert_eq!(listener.poll_blocking().ok().unwrap(), Event::Tick);
        let before = Instant::now();
        // Poll (tick) (with time waiting)
        // blocking should wait until there is another event avaialble
        assert_eq!(listener.poll_blocking().ok().unwrap(), Event::Tick);
        let diff = Instant::now().duration_since(before);
        // and the time for a tick is 3 seconds, and it is called immediately after the last poll, so it should be at least half that time between then
        assert!(diff > Duration::from_millis(1500));
        // Stop
        assert!(listener.stop().is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "async-ports")]
    async fn async_ports_should_work() {
        use tokio::runtime::Handle;
        use tokio::time::sleep;

        use crate::mock::MockPollAsync;

        let port = AsyncPort::new(
            Box::new(MockPollAsync::default()),
            Duration::from_secs(1),
            1,
        );
        assert_eq!(Handle::current().metrics().num_alive_tasks(), 0);

        let mut listener = EventListener::<MockEvent>::new(Duration::from_millis(10)).start_async(
            vec![port],
            Handle::current(),
            Some(Duration::from_secs(3)),
        );
        sleep(Duration::from_millis(5)).await; // ensure the tasks are spawned and have a chance to generate already
        assert_eq!(Handle::current().metrics().num_alive_tasks(), 2);

        let mut events = Vec::with_capacity(3);

        // due to how execution on the runtime works, events may arrive in any order
        while let Some(event) = listener.poll_timeout().ok().flatten() {
            events.push(event);
        }

        // Poll (event)
        assert!(events.contains(&Event::Keyboard(KeyEvent::from(Key::Enter))));
        // Poll (tick)
        assert!(events.contains(&Event::Tick));
        // Stop
        assert!(listener.stop().is_ok());
        sleep(Duration::from_millis(5)).await; // ensure the tasks have a chance to execute again to cancel themself
        assert_eq!(Handle::current().metrics().num_alive_tasks(), 0);
    }
}
