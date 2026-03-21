//! This module implements the Sync-worker thread for the event listener

use std::ops::{Add, Sub};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use super::{ListenerMsg, SyncPort};
use crate::listener::PortError;

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

    #[cfg(test)]
    barrier: Option<super::builder::test_utils::BarrierTx>,
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

            #[cfg(test)]
            barrier: None,
        }
    }

    /// Attach a test barrier to the event listener.
    #[cfg(test)]
    pub fn with_test_barrier(&mut self, barrier: Option<super::builder::test_utils::BarrierTx>) {
        self.barrier = barrier;
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
        self.sender.send(ListenerMsg::User(crate::Event::Tick))?;
        // Calc next tick
        self.calc_next_tick();
        Ok(())
    }

    /// Poll and send poll to listener. Calc next poll.
    /// Returns only the messages, while the None returned by poll are discarded
    fn poll(&mut self) -> Result<(), mpsc::SendError<ListenerMsg<UserEvent>>> {
        let mut needs_drop = false;
        let port_iter = self.ports.iter_mut().filter(|port| port.should_poll());

        for port in port_iter {
            let mut times_remaining = port.max_poll();
            // poll a port until it has nothing anymore
            loop {
                let msg = match port.poll() {
                    Ok(Some(ev)) => ListenerMsg::User(ev),
                    Ok(None) => break,
                    Err(err) => {
                        if let PortError::PermanentError(_) = &err {
                            needs_drop = true;
                            port.mark_for_drop();
                        }

                        ListenerMsg::Error(err)
                    }
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

        // this needs to be done due to us operating on a reference in the "for loop" above, which cannot drop the port itself
        // but we can make use of that "max_poll" *should* be at least 1 to mark for dropping
        if needs_drop {
            self.ports.retain(|port| port.max_poll() != 0);
        }

        Ok(())
    }

    /// thread run method
    pub(super) fn run(&mut self) {
        loop {
            // wait until the reciever is ready, which signals that the loop should start
            #[cfg(test)]
            {
                if let Some(barrier) = &mut self.barrier {
                    barrier.send_start();
                }
            }

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

            // wait until the reciever is ready, which signals that the loop has ended
            #[cfg(test)]
            {
                if let Some(barrier) = &mut self.barrier {
                    barrier.send_end();
                }
            }

            // Sleep till next event
            thread::sleep(self.next_event());
        }
    }
}

#[cfg(test)]
mod test {

    use std::sync::atomic::Ordering;
    use std::sync::mpsc::TryRecvError;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::core::event::{Key, KeyEvent};
    use crate::listener::{Poll, PollError, PollResult, SyncPort};
    use crate::mock::{MockEvent, MockPoll};
    use crate::{Event, NoUserEvent};

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
            PollResult::<Option<_>>::from(rx.recv().unwrap()).unwrap(),
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
            PollResult::<Option<_>>::from(rx.recv().unwrap()).unwrap(),
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

    #[test]
    fn worker_should_drop_port_in_perma_error() {
        #[derive(Debug)]
        struct TestPoll;

        impl Poll<NoUserEvent> for TestPoll {
            fn poll(&mut self) -> crate::listener::PortResult<Option<Event<NoUserEvent>>> {
                Err(PortError::PermanentError("Test".to_string()))
            }
        }

        let (tx, rx) = mpsc::channel();
        let paused = Arc::new(AtomicBool::new(true));
        let paused_t = Arc::clone(&paused);
        let running = Arc::new(AtomicBool::new(true));
        let running_t = Arc::clone(&running);
        let mut worker = EventListenerWorker::<NoUserEvent>::new(
            vec![SyncPort::new(Box::new(TestPoll), Duration::from_secs(5), 1)],
            tx,
            paused_t,
            running_t,
            Some(Duration::from_secs(1)),
        );
        assert_eq!(worker.ports.len(), 1);
        paused.store(false, Ordering::Relaxed);

        assert!(worker.poll().is_ok());

        paused.store(true, Ordering::Relaxed);

        assert_eq!(worker.ports.len(), 0);

        // Receive
        assert_eq!(
            PollResult::<Option<_>>::from(rx.recv().unwrap()).unwrap_err(),
            PollError::PortError(PortError::PermanentError("Test".to_string()))
        );
    }

    #[test]
    fn worker_run_should_exit_on_channel_error() {
        let (tx, rx) = mpsc::channel();
        let paused = Arc::new(AtomicBool::new(true));
        let paused_t = Arc::clone(&paused);
        let running = Arc::new(AtomicBool::new(true));
        let running_t = Arc::clone(&running);
        let mut worker: EventListenerWorker<NoUserEvent> = EventListenerWorker::<NoUserEvent>::new(
            vec![SyncPort::new(
                Box::new(MockPoll::default()),
                Duration::ZERO,
                1,
            )],
            tx,
            paused_t,
            running_t,
            Some(Duration::from_secs(1)),
        );
        assert_eq!(worker.ports.len(), 1);
        paused.store(false, Ordering::Relaxed);

        drop(rx);

        // verify that "poll" will return a error on channel closure, before potentially infinitely running "run"
        assert!(worker.poll().is_err());

        worker.run();
    }

    #[test]
    fn should_only_gather_specific_amount() {
        // this test specifically tests what happens when a Port returns "Ok(None)"
        #[derive(Debug)]
        struct TestPoll {
            slice: &'static [Event<NoUserEvent>],
        }

        impl Poll<NoUserEvent> for TestPoll {
            fn poll(&mut self) -> crate::listener::PortResult<Option<Event<NoUserEvent>>> {
                if let Some(event) = self.slice.iter().next() {
                    self.slice = &self.slice[1..];
                    return Ok(Some(event.clone()));
                }

                Ok(None)
            }
        }

        let (tx, rx) = mpsc::channel();
        let paused = Arc::new(AtomicBool::new(true));
        let paused_t = Arc::clone(&paused);
        let running = Arc::new(AtomicBool::new(true));
        let running_t = Arc::clone(&running);
        let mut worker = EventListenerWorker::<NoUserEvent>::new(
            vec![SyncPort::new(
                Box::new(TestPoll {
                    slice: &[Event::FocusGained, Event::FocusLost],
                }),
                Duration::ZERO,
                1,
            )],
            tx,
            paused_t,
            running_t,
            Some(Duration::from_secs(1)),
        );
        assert_eq!(worker.ports.len(), 1);
        paused.store(false, Ordering::Relaxed);

        // NOTE: "try_recv" is used over "poll" because a event *should* be available after each expected poll,
        // and when using "recv" it would block infinitely, never telling what the issue is aside from "test is running long".
        assert!(worker.poll().is_ok());
        assert_eq!(
            PollResult::<Event<_>>::from(rx.try_recv().unwrap()).unwrap(),
            Event::FocusGained
        );
        assert!(worker.poll().is_ok());
        assert_eq!(
            PollResult::<Event<_>>::from(rx.try_recv().unwrap()).unwrap(),
            Event::FocusLost
        );
        assert!(worker.poll().is_ok());
        assert_eq!(rx.try_recv().unwrap_err(), TryRecvError::Empty);
    }
}
