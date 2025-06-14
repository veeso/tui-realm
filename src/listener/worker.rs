//! ## Worker
//!
//! This module implements the worker thread for the event listener

use std::ops::{Add, Sub};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use super::{ListenerMsg, SyncPort};

// -- worker

/// worker for event listener
pub(super) struct EventListenerWorker<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send,
{
    ports: Vec<SyncPort<UserEvent>>,
    sender: mpsc::Sender<ListenerMsg<UserEvent>>,
    paused: Arc<AtomicBool>,
    running: Arc<AtomicBool>,
    next_tick: Instant,
    tick_interval: Option<Duration>,
}

impl<UserEvent> EventListenerWorker<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    /// Create a new Worker.
    ///
    /// If `tick_interval` is [`None`], no [`Event::Tick`](crate::Event::Tick) will be sent and a fallback interval time will be used.
    /// If `tick_interval` is [`Some`], [`Event::Tick`](crate::Event::Tick) will be sent and be used as the interval time.
    pub(super) fn new(
        ports: Vec<SyncPort<UserEvent>>,
        sender: mpsc::Sender<ListenerMsg<UserEvent>>,
        paused: Arc<AtomicBool>,
        running: Arc<AtomicBool>,
        tick_interval: Option<Duration>,
    ) -> Self {
        Self {
            ports,
            sender,
            paused,
            running,
            next_tick: Instant::now(),
            tick_interval,
        }
    }

    /// Calculate next tick time.
    /// If tick is None, panics.
    fn calc_next_tick(&mut self) {
        self.next_tick = Instant::now().add(self.tick_interval.unwrap());
    }

    /// Calc the distance in time between now and the first upcoming event
    fn next_event(&self) -> Duration {
        let now = Instant::now();
        // TODO: this time should likely be lowered
        let fallback_time = now.add(Duration::from_secs(60));
        // Get first upcoming event from ports
        let min_listener_event = self
            .ports
            .iter()
            .map(|x| x.next_poll())
            .min()
            .unwrap_or(fallback_time);
        let next_tick = if self.tick_interval.is_none() {
            fallback_time
        } else {
            self.next_tick
        };
        let min_time = std::cmp::min(min_listener_event, next_tick);
        // If min time is > now, returns diff, otherwise return 0
        if min_time > now {
            min_time.sub(now)
        } else {
            Duration::ZERO
        }
    }

    /// Returns whether should keep running
    fn running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Returns whether worker is paused
    fn paused(&self) -> bool {
        self.paused.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Returns whether it's time to tick.
    /// If tick_interval is `None` it will never return `true`
    fn should_tick(&self) -> bool {
        match self.tick_interval {
            None => false,
            Some(_) => self.next_tick <= Instant::now(),
        }
    }

    /// Send tick to listener and calc next tick
    fn send_tick(&mut self) -> Result<(), mpsc::SendError<ListenerMsg<UserEvent>>> {
        // Send tick
        self.sender.send(ListenerMsg::Tick)?;
        // Calc next tick
        self.calc_next_tick();
        Ok(())
    }

    /// Poll and send poll to listener. Calc next poll.
    /// Returns only the messages, while the None returned by poll are discarded
    fn poll(&mut self) -> Result<(), mpsc::SendError<ListenerMsg<UserEvent>>> {
        let port_iter = self.ports.iter_mut().filter(|port| port.should_poll());

        for port in port_iter {
            let mut times_remaining = port.max_poll();
            // poll a port until it has nothing anymore
            loop {
                let msg = match port.poll() {
                    Ok(Some(ev)) => ListenerMsg::User(ev),
                    Ok(None) => break,
                    Err(err) => ListenerMsg::Error(err),
                };

                self.sender.send(msg)?;

                // do this at the end to at least call it once
                times_remaining = times_remaining.saturating_sub(1);

                if times_remaining == 0 {
                    break;
                }
            }
            // Update next poll
            port.calc_next_poll();
        }

        Ok(())
    }

    /// thread run method
    pub(super) fn run(&mut self) {
        loop {
            // Check if running or send_error has occurred
            if !self.running() {
                break;
            }
            // If paused, wait and resume cycle
            if self.paused() {
                thread::sleep(Duration::from_millis(25));
                continue;
            }
            // Iter ports and Send messages
            if self.poll().is_err() {
                break;
            }
            // Tick
            if self.should_tick() && self.send_tick().is_err() {
                break;
            }
            // Sleep till next event
            thread::sleep(self.next_event());
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::super::ListenerResult;
    use super::*;
    use crate::Event;
    use crate::core::event::{Key, KeyEvent};
    use crate::listener::SyncPort;
    use crate::mock::{MockEvent, MockPoll};

    #[test]
    fn worker_should_poll_multiple_times() {
        let (tx, rx) = mpsc::channel();
        let paused = Arc::new(AtomicBool::new(false));
        let paused_t = Arc::clone(&paused);
        let running = Arc::new(AtomicBool::new(true));
        let running_t = Arc::clone(&running);

        let mock_port = SyncPort::new(Box::new(MockPoll::default()), Duration::from_secs(5), 10);

        let mut worker =
            EventListenerWorker::<MockEvent>::new(vec![mock_port], tx, paused_t, running_t, None);
        assert!(worker.poll().is_ok());
        assert!(worker.next_event() <= Duration::from_secs(5));
        let mut recieved = Vec::new();

        while let Ok(msg) = rx.try_recv() {
            recieved.push(msg);
        }

        assert_eq!(recieved.len(), 10);
    }

    #[test]
    fn worker_should_send_poll() {
        let (tx, rx) = mpsc::channel();
        let paused = Arc::new(AtomicBool::new(false));
        let paused_t = Arc::clone(&paused);
        let running = Arc::new(AtomicBool::new(true));
        let running_t = Arc::clone(&running);
        let mut worker = EventListenerWorker::<MockEvent>::new(
            vec![SyncPort::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(5),
                1,
            )],
            tx,
            paused_t,
            running_t,
            None,
        );
        assert!(worker.poll().is_ok());
        assert!(worker.next_event() <= Duration::from_secs(5));
        // Receive
        assert_eq!(
            ListenerResult::from(rx.recv().ok().unwrap()).ok().unwrap(),
            Some(Event::Keyboard(KeyEvent::from(Key::Enter)))
        );
    }

    #[test]
    fn worker_should_send_tick() {
        let (tx, rx) = mpsc::channel();
        let paused = Arc::new(AtomicBool::new(false));
        let paused_t = Arc::clone(&paused);
        let running = Arc::new(AtomicBool::new(true));
        let running_t = Arc::clone(&running);
        let mut worker = EventListenerWorker::<MockEvent>::new(
            vec![SyncPort::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(5),
                1,
            )],
            tx,
            paused_t,
            running_t,
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
        let paused = Arc::new(AtomicBool::new(false));
        let paused_t = Arc::clone(&paused);
        let running = Arc::new(AtomicBool::new(true));
        let running_t = Arc::clone(&running);
        let mut worker = EventListenerWorker::<MockEvent>::new(
            vec![SyncPort::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(5),
                1,
            )],
            tx,
            paused_t,
            running_t,
            Some(Duration::from_secs(1)),
        );
        assert_eq!(worker.running(), true);
        // Should set next events to now
        assert!(worker.next_event() <= Duration::from_secs(1));
        assert!(worker.next_tick <= Instant::now());
        assert!(worker.should_tick());
        // Calc next
        let expected_next_tick = Instant::now().add(Duration::from_secs(1));
        worker.calc_next_tick();
        assert!(worker.next_tick >= expected_next_tick);
        // Next event should be in 1 second (tick)
        assert!(worker.next_event() <= Duration::from_secs(1));
        // Now should no more tick and poll
        assert_eq!(worker.should_tick(), false);
        // Stop
        running.store(false, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(worker.running(), false);
        drop(rx);
    }

    #[test]
    fn worker_should_calc_times_correctly_without_tick() {
        let (tx, rx) = mpsc::channel();
        let paused = Arc::new(AtomicBool::new(false));
        let paused_t = Arc::clone(&paused);
        let running = Arc::new(AtomicBool::new(true));
        let running_t = Arc::clone(&running);
        let worker = EventListenerWorker::<MockEvent>::new(
            vec![SyncPort::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(3),
                1,
            )],
            tx,
            paused_t,
            running_t,
            None,
        );
        assert_eq!(worker.running(), true);
        assert_eq!(worker.paused(), false);
        // Should set next events to now
        assert!(worker.next_event() <= Duration::from_secs(3));
        assert!(worker.next_tick <= Instant::now());
        assert_eq!(worker.should_tick(), false);
        // Next event should be in 3 second (poll)
        assert!(worker.next_event() <= Duration::from_secs(3));
        // Stop
        running.store(false, std::sync::atomic::Ordering::Relaxed);

        assert_eq!(worker.running(), false);
        drop(rx);
    }

    #[test]
    #[should_panic]
    fn worker_should_panic_when_trying_next_tick_without_it() {
        let (tx, _) = mpsc::channel();
        let paused = Arc::new(AtomicBool::new(false));
        let paused_t = Arc::clone(&paused);
        let running = Arc::new(AtomicBool::new(true));
        let running_t = Arc::clone(&running);
        let mut worker =
            EventListenerWorker::<MockEvent>::new(vec![], tx, paused_t, running_t, None);
        worker.calc_next_tick();
    }
}
