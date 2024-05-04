//! ## Worker
//!
//! This module implements the worker thread for the event listener

use std::ops::{Add, Sub};
use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use super::{ListenerMsg, Port};

// -- worker

/// worker for event listener
pub(super) struct EventListenerWorker<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    ports: Vec<Port<U>>,
    sender: mpsc::Sender<ListenerMsg<U>>,
    paused: Arc<RwLock<bool>>,
    next_tick: Instant,
    tick_interval: Option<Duration>,
    /// indicate if the thread is alive in the loop for testing
    #[cfg(test)]
    is_running: Arc<RwLock<bool>>,
}

impl<U> EventListenerWorker<U>
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    pub(super) fn new(
        ports: Vec<Port<U>>,
        sender: mpsc::Sender<ListenerMsg<U>>,
        paused: Arc<RwLock<bool>>,
        tick_interval: Option<Duration>,
    ) -> Self {
        Self {
            ports,
            sender,
            paused,
            next_tick: Instant::now(),
            tick_interval,
            #[cfg(test)]
            is_running: Arc::new(RwLock::new(false)),
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
        let fallback_time = now.add(Duration::from_secs(60));
        // Get first upcoming event from ports
        let min_listener_event = self
            .ports
            .iter()
            .map(|x| x.next_poll())
            .min()
            .unwrap_or(fallback_time);
        let next_tick = match self.tick_interval.is_some() {
            false => fallback_time,
            true => self.next_tick,
        };
        let min_time = std::cmp::min(min_listener_event, next_tick);
        // If min time is > now, returns diff, otherwise return 0
        if min_time > now {
            min_time.sub(now)
        } else {
            Duration::ZERO
        }
    }

    /// Returns whether worker is paused
    fn paused(&self) -> bool {
        if let Ok(lock) = self.paused.read() {
            return *lock;
        }
        false
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
    fn send_tick(&mut self) -> Result<(), mpsc::SendError<ListenerMsg<U>>> {
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

    /// Poll and send poll to listener. Calc next poll.
    /// Returns only the messages, while the None returned by poll are discarded
    #[allow(clippy::needless_collect)]
    fn poll(&mut self) -> Result<(), mpsc::SendError<ListenerMsg<U>>> {
        let msg: Vec<ListenerMsg<U>> = self
            .ports
            .iter_mut()
            .filter_map(|x| {
                if x.should_poll() {
                    let msg = match x.poll() {
                        Ok(Some(ev)) => Some(ListenerMsg::User(ev)),
                        Ok(None) => None,
                        Err(err) => Some(ListenerMsg::Error(err)),
                    };
                    // Update next poll
                    x.calc_next_poll();
                    msg
                } else {
                    None
                }
            })
            .collect();
        // Send messages
        match msg
            .into_iter()
            .map(|x| self.sender.send(x))
            .filter(|x| x.is_err())
            .map(|x| x.err().unwrap())
            .next()
        {
            None => Ok(()),
            Some(e) => Err(e),
        }
    }

    /// thread run method
    ///
    /// Exits if the Channel is closed
    pub(super) fn run(&mut self) {
        #[cfg(test)]
        {
            *self.is_running.write().unwrap() = true;
        }
        loop {
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
        #[cfg(test)]
        {
            *self.is_running.write().unwrap() = false;
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::super::ListenerResult;
    use super::*;
    use crate::core::event::{Key, KeyEvent};
    use crate::mock::{MockEvent, MockPoll};
    use crate::Event;

    #[test]
    fn should_work_and_get_events() {
        let (tx, rx) = mpsc::channel();
        let paused_t = Arc::new(RwLock::new(false));
        let mut worker = EventListenerWorker::<MockEvent>::new(
            vec![Port::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(1),
            )],
            tx,
            paused_t,
            None,
        );
        let is_running = worker.is_running.clone();
        assert_eq!(*is_running.read().unwrap(), false);

        let handle = std::thread::spawn(move || worker.run());

        // Wait 1 second because threads are independent
        thread::sleep(Duration::from_secs(1));
        assert_eq!(*is_running.read().unwrap(), true);

        assert_eq!(
            ListenerResult::from(rx.recv().unwrap()).unwrap().unwrap(),
            Event::Keyboard(KeyEvent::from(Key::Enter))
        );

        drop(rx);

        // Wait 2 second because the port poll timeout is 1 second and the thread may be in a sleep
        thread::sleep(Duration::from_secs(2));

        assert_eq!(*is_running.read().unwrap(), false);

        let _ = handle.join();
    }

    #[test]
    fn worker_should_send_poll() {
        let (tx, rx) = mpsc::channel();
        let paused = Arc::new(RwLock::new(false));
        let paused_t = Arc::clone(&paused);
        let mut worker = EventListenerWorker::<MockEvent>::new(
            vec![Port::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(5),
            )],
            tx,
            paused_t,
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
        let paused = Arc::new(RwLock::new(false));
        let paused_t = Arc::clone(&paused);
        let mut worker = EventListenerWorker::<MockEvent>::new(
            vec![Port::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(5),
            )],
            tx,
            paused_t,
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
        let paused = Arc::new(RwLock::new(false));
        let paused_t = Arc::clone(&paused);
        let mut worker = EventListenerWorker::<MockEvent>::new(
            vec![Port::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(5),
            )],
            tx,
            paused_t,
            Some(Duration::from_secs(1)),
        );
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
        drop(rx);
    }

    #[test]
    fn worker_should_calc_times_correctly_without_tick() {
        let (tx, rx) = mpsc::channel();
        let paused = Arc::new(RwLock::new(false));
        let paused_t = Arc::clone(&paused);
        let worker = EventListenerWorker::<MockEvent>::new(
            vec![Port::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(3),
            )],
            tx,
            paused_t,
            None,
        );
        assert_eq!(worker.paused(), false);
        // Should set next events to now
        assert!(worker.next_event() <= Duration::from_secs(3));
        assert!(worker.next_tick <= Instant::now());
        assert_eq!(worker.should_tick(), false);
        // Next event should be in 3 second (poll)
        assert!(worker.next_event() <= Duration::from_secs(3));
        drop(rx);
    }

    #[test]
    #[should_panic]
    fn worker_should_panic_when_trying_next_tick_without_it() {
        let (tx, _) = mpsc::channel();
        let paused = Arc::new(RwLock::new(false));
        let paused_t = Arc::clone(&paused);
        let mut worker = EventListenerWorker::<MockEvent>::new(vec![], tx, paused_t, None);
        worker.calc_next_tick();
    }
}
