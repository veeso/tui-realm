//! ## Worker
//!
//! This module implements the worker thread for the event listener

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
use super::{ListenerMsg, Port};
use std::ops::{Add, Sub};
use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

// -- worker

/// ## EventListenerWorker
///
/// worker for event listener
pub(super) struct EventListenerWorker<U>
where
    U: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send,
{
    ports: Vec<Port<U>>,
    sender: mpsc::Sender<ListenerMsg<U>>,
    running: Arc<RwLock<bool>>,
    next_tick: Instant,
    tick_interval: Option<Duration>,
}

impl<U> EventListenerWorker<U>
where
    U: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send,
{
    pub(super) fn new(
        ports: Vec<Port<U>>,
        sender: mpsc::Sender<ListenerMsg<U>>,
        running: Arc<RwLock<bool>>,
        tick_interval: Option<Duration>,
    ) -> Self {
        Self {
            ports,
            sender,
            running,
            next_tick: Instant::now(),
            tick_interval,
        }
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

    /// ### running
    ///
    /// Returns whether should keep running
    fn running(&self) -> bool {
        if let Ok(lock) = self.running.read() {
            return *lock;
        }
        true
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

    /// ### poll
    ///
    /// Poll and send poll to listener. Calc next poll.
    /// Returns only the messages, while the None returned by poll are discarded
    #[allow(clippy::needless_collect)]
    fn poll(&mut self) -> Result<(), mpsc::SendError<ListenerMsg<U>>> {
        let msg: Vec<ListenerMsg<U>> = self
            .ports
            .iter_mut()
            .map(|x| {
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
            .flatten()
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

    /// ### run
    ///
    /// thread run method
    pub(super) fn run(&mut self) {
        loop {
            // Check if running or send_error has occurred
            if !self.running() {
                break;
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

    use super::super::{ListenerError, ListenerResult};
    use super::*;
    use crate::core::event::{Key, KeyEvent};
    use crate::mock::MockEvent;
    use crate::mock::MockPoll;
    use crate::Event;

    use pretty_assertions::assert_eq;

    #[test]
    fn worker_should_send_poll() {
        let (tx, rx) = mpsc::channel();
        let running = Arc::new(RwLock::new(true));
        let running_t = Arc::clone(&running);
        let mut worker = EventListenerWorker::<MockEvent>::new(
            vec![Port::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(5),
            )],
            tx,
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
        let running = Arc::new(RwLock::new(true));
        let running_t = Arc::clone(&running);
        let mut worker = EventListenerWorker::<MockEvent>::new(
            vec![Port::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(5),
            )],
            tx,
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
        let running = Arc::new(RwLock::new(true));
        let running_t = Arc::clone(&running);
        let mut worker = EventListenerWorker::<MockEvent>::new(
            vec![Port::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(5),
            )],
            tx,
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
        let worker = EventListenerWorker::<MockEvent>::new(
            vec![Port::new(
                Box::new(MockPoll::default()),
                Duration::from_secs(3),
            )],
            tx,
            running_t,
            None,
        );
        assert_eq!(worker.running(), true);
        // Should set next events to now
        assert!(worker.next_event() <= Duration::from_secs(3));
        assert!(worker.next_tick <= Instant::now());
        assert_eq!(worker.should_tick(), false);
        // Next event should be in 3 second (poll)
        assert!(worker.next_event() <= Duration::from_secs(3));
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
        let mut worker = EventListenerWorker::<MockEvent>::new(vec![], tx, running_t, None);
        worker.calc_next_tick();
    }
}
