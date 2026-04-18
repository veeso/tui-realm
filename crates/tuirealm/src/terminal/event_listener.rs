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

/// Convert [`io::Error`](std::io::Error) to a [`PortError`], with correct Intermittent & Permanent Mapping
#[cfg(any(feature = "crossterm", feature = "termion"))]
fn io_err_to_port_err(err: std::io::Error) -> crate::listener::PortError {
    use std::io::ErrorKind;

    use crate::listener::PortError;

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

#[cfg(all(test, any(feature = "crossterm", feature = "termion")))]
mod tests {
    use std::io::{Error as ioError, ErrorKind};

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::listener::PortError;

    #[test]
    fn should_map_to_intermittend() {
        assert_eq!(
            io_err_to_port_err(ioError::new(ErrorKind::Interrupted, "test")),
            PortError::IntermittentError("test".to_string())
        );
        assert_eq!(
            io_err_to_port_err(ioError::new(ErrorKind::NetworkDown, "test")),
            PortError::IntermittentError("test".to_string())
        );
        assert_eq!(
            io_err_to_port_err(ioError::new(ErrorKind::ResourceBusy, "test")),
            PortError::IntermittentError("test".to_string())
        );
        assert_eq!(
            io_err_to_port_err(ioError::new(ErrorKind::ConnectionReset, "test")),
            PortError::IntermittentError("test".to_string())
        );
        assert_eq!(
            io_err_to_port_err(ioError::new(ErrorKind::TimedOut, "test")),
            PortError::IntermittentError("test".to_string())
        );
        assert_eq!(
            io_err_to_port_err(ioError::new(ErrorKind::QuotaExceeded, "test")),
            PortError::IntermittentError("test".to_string())
        );
        assert_eq!(
            io_err_to_port_err(ioError::new(ErrorKind::StorageFull, "test")),
            PortError::IntermittentError("test".to_string())
        );
        assert_eq!(
            io_err_to_port_err(ioError::new(ErrorKind::WouldBlock, "test")),
            PortError::IntermittentError("test".to_string())
        );
    }

    #[test]
    fn should_map_to_permanent() {
        assert_eq!(
            io_err_to_port_err(ioError::new(ErrorKind::BrokenPipe, "test")),
            PortError::PermanentError("test".to_string())
        );
    }
}
