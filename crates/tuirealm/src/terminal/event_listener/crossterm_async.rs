use crossterm::event::EventStream;
use futures_util::StreamExt;

use crate::Event;
use crate::listener::{PollAsync, PortResult};
use crate::terminal::event_listener::io_err_to_port_err;

/// The async input listener for crossterm.
/// This can be manually added as a async port, or directly via [`EventListenerCfg::async_crossterm_input_listener()`](crate::EventListenerCfg::async_crossterm_input_listener)
// NOTE: This relies on [`From`] implementations in [`super::crossterm`].
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
impl<UserEvent> PollAsync<UserEvent> for CrosstermAsyncStream
where
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    async fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>> {
        let res = match self.stream.next().await {
            Some(Ok(event)) => event,
            Some(Err(err)) => return Err(io_err_to_port_err(err)),
            None => return Ok(None),
        };

        Ok(Some(Event::from(res)))
    }
}
