#[cfg(feature = "async-ports")]
mod async_test;
#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(all(feature = "crossterm", feature = "async-ports"))]
mod crossterm_async;
#[cfg(feature = "termion")]
mod termion;
#[cfg(feature = "termwiz")]
mod termwiz;
mod test;

#[cfg(feature = "async-ports")]
pub use async_test::AsyncTestEventListener;
#[cfg(feature = "crossterm")]
pub use crossterm::CrosstermInputListener;
#[cfg(all(feature = "crossterm", feature = "async-ports"))]
pub use crossterm_async::CrosstermAsyncStream;
#[cfg(feature = "termion")]
pub use termion::TermionInputListener;
#[cfg(feature = "termwiz")]
pub use termwiz::TermwizInputListener;
pub use test::TestEventListener;

#[allow(unused_imports)] // used in the event listeners
use crate::event::Event;
use crate::listener::PortError;

/// Convert [`io::Error`](std::io::Error) to a [`PortError`], with correct Intermittent & Permanent Mapping
fn io_err_to_port_err(err: std::io::Error) -> PortError {
    use std::io::ErrorKind;

    match err.kind() {
        ErrorKind::Interrupted
        | ErrorKind::NetworkDown
        | ErrorKind::ResourceBusy
        | ErrorKind::ConnectionReset
        | ErrorKind::TimedOut
        | ErrorKind::QuotaExceeded
        | ErrorKind::StorageFull
        | ErrorKind::WouldBlock => PortError::IntermittentError(err.to_string()),
        _ => PortError::PermanentError(err.to_string()),
    }
}
