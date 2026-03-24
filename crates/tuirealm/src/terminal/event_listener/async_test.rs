//! An async test event listener useful for integration tests. Can be paired with any adapter, but it's generally preferred
//! to work with [`crate::terminal::TestTerminalAdapter`].

use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::error::TryRecvError;
use tuirealm::listener::PortError;

use super::Event;
use crate::listener::{PollAsync, PortResult};

/// An async test [`Poll`] implementation that can be used for integration tests.
///
/// It has a [`Receiver`] which can be used to enqueue events to raise to the application.
pub struct AsyncTestEventListener<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone,
{
    receiver: Receiver<Event<UserEvent>>,
}

impl<UserEvent> AsyncTestEventListener<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone,
{
    /// Creates a new [`crate::terminal::AsyncTestEventListener`] with the provided [`Receiver`].
    ///
    /// The receiver will be checked for pending events each time [`Poll::poll`] is called.
    pub fn new(receiver: Receiver<Event<UserEvent>>) -> Self {
        Self { receiver }
    }
}

impl<UserEvent> From<Receiver<Event<UserEvent>>> for AsyncTestEventListener<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone,
{
    fn from(receiver: Receiver<Event<UserEvent>>) -> Self {
        Self::new(receiver)
    }
}

#[async_trait::async_trait]
impl<UserEvent> PollAsync<UserEvent> for AsyncTestEventListener<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    async fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>> {
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

    #[tokio::test]
    async fn test_should_poll_from_test_event_listener() {
        let (tx, rx) = tokio::sync::mpsc::channel(512);
        let mut listener = AsyncTestEventListener::<NoUserEvent>::new(rx);

        // poll
        assert_eq!(listener.poll().await, Ok(None));

        // enqueue event
        tx.send(Event::Keyboard(KeyEvent::new(
            Key::Backspace,
            KeyModifiers::NONE,
        )))
        .await
        .expect("send keyboard event");

        assert_eq!(
            listener.poll().await,
            Ok(Some(Event::Keyboard(KeyEvent::new(
                Key::Backspace,
                KeyModifiers::NONE,
            ))))
        );

        // poll empty again
        assert_eq!(listener.poll().await, Ok(None));
    }

    #[tokio::test]
    async fn test_should_create_test_event_listener_from() {
        let (_tx, rx) = tokio::sync::mpsc::channel(512);
        let _listener = AsyncTestEventListener::<NoUserEvent>::from(rx);
    }

    #[tokio::test]
    async fn test_should_return_error_on_disconnected_tx() {
        let (tx, rx) = tokio::sync::mpsc::channel(512);
        let mut listener = AsyncTestEventListener::<NoUserEvent>::new(rx);
        drop(tx);
        assert!(listener.poll().await.is_err());
    }
}
