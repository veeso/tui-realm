use super::{ListenerResult, PollAsync};
use crate::Event;

/// [`PollAsync`] implementation to have a Async-Port for emitting [`Event::Tick`].
///
/// This will emit a [`Event::Tick`] on every [`poll`](Self::poll) call, relying on the [`tick_interval`](super::EventListener) to handle intervals.
#[derive(Debug, Clone, Copy)]
pub struct AsyncTicker();

impl AsyncTicker {
    pub fn new() -> Self {
        Self()
    }
}

#[async_trait::async_trait]
impl<U> PollAsync<U> for AsyncTicker
where
    U: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    async fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
        Ok(Some(Event::Tick))
    }
}

#[cfg(test)]
mod tests {
    use super::AsyncTicker;
    use crate::listener::PollAsync;
    use crate::{Event, NoUserEvent};

    #[tokio::test]
    async fn should_emit_tick_on_every_poll() {
        let mut ticker = AsyncTicker::new();
        assert_eq!(ticker.poll().await, Ok(Some(Event::<NoUserEvent>::Tick)));
        assert_eq!(ticker.poll().await, Ok(Some(Event::<NoUserEvent>::Tick)));
        assert_eq!(ticker.poll().await, Ok(Some(Event::<NoUserEvent>::Tick)));
    }
}
