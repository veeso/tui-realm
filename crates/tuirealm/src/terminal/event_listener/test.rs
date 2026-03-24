//! A test event listener useful for integration tests. Can be paired with any adapter, but it's generally preferred
//! to work with [`crate::terminal::TestTerminalAdapter`].

use std::sync::mpsc::{Receiver, TryRecvError};

use tuirealm::listener::PortError;

use super::Event;
use crate::listener::{Poll, PortResult};

/// A test [`Poll`] implementation that can be used for integration tests.
///
/// It has a [`Receiver`] which can be used to enqueue events to raise to the application.
pub struct TestEventListener<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone,
{
    receiver: Receiver<Event<UserEvent>>,
}

impl<UserEvent> TestEventListener<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone,
{
    /// Creates a new [`TestEventListener`] with the provided [`Receiver`].
    ///
    /// The receiver will be checked for pending events each time [`Poll::poll`] is called.
    pub fn new(receiver: Receiver<Event<UserEvent>>) -> Self {
        Self { receiver }
    }
}

impl<UserEvent> From<Receiver<Event<UserEvent>>> for TestEventListener<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone,
{
    fn from(receiver: Receiver<Event<UserEvent>>) -> Self {
        Self::new(receiver)
    }
}

impl<UserEvent> Poll<UserEvent> for TestEventListener<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>> {
        match self.receiver.try_recv() {
            Ok(msg) => Ok(Some(msg)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(PortError::PermanentError(
                "Receiver disconnected".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{Key, KeyEvent, KeyModifiers, NoUserEvent};

    #[test]
    fn test_should_poll_from_test_event_listener() {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut listener = TestEventListener::<NoUserEvent>::new(rx);

        // poll
        assert_eq!(listener.poll(), Ok(None));

        // enqueue event
        tx.send(Event::Keyboard(KeyEvent::new(
            Key::Backspace,
            KeyModifiers::NONE,
        )))
        .expect("send keyboard event");

        assert_eq!(
            listener.poll(),
            Ok(Some(Event::Keyboard(KeyEvent::new(
                Key::Backspace,
                KeyModifiers::NONE,
            ))))
        );

        // poll empty again
        assert_eq!(listener.poll(), Ok(None));
    }

    #[test]
    fn test_should_create_test_event_listener_from() {
        let (_tx, rx) = std::sync::mpsc::channel();
        let _listener = TestEventListener::<NoUserEvent>::from(rx);
    }

    #[test]
    fn test_should_return_error_on_disconnected_tx() {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut listener = TestEventListener::<NoUserEvent>::new(rx);
        drop(tx);
        assert!(listener.poll().is_err());
    }
}
