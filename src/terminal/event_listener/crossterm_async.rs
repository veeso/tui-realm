use crossterm::event::EventStream;
use futures_util::StreamExt;

use crate::listener::{ListenerResult, PollAsync};
use crate::{Event, ListenerError};

/// The async input listener for crossterm.
/// This can be manually added as a async port, or directly via [`EventListenerCfg::crossterm_input_listener()`](crate::EventListenerCfg::crossterm_input_listener)
///
/// NOTE: This relies on [`From`] implementations in [`super::crossterm`].
#[doc(alias = "InputEventListener")]
#[derive(Debug)]
pub struct CrosstermAsyncStream {
    stream: EventStream,
}

impl CrosstermAsyncStream {
    pub fn new() -> Self {
        CrosstermAsyncStream {
            stream: EventStream::new(),
        }
    }
}

impl Default for CrosstermAsyncStream {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl<U: Eq + PartialEq + Clone + PartialOrd + Send + 'static> PollAsync<U>
    for CrosstermAsyncStream
{
    async fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
        let res = match self.stream.next().await {
            Some(Ok(event)) => event,
            Some(Err(_err)) => return Err(ListenerError::PollFailed),
            None => return Ok(None),
        };

        Ok(Some(Event::from(res)))
    }
}
