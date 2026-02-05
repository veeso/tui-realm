//! All render and input backends

mod adapter;
mod event_listener;

use thiserror::Error;

#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
pub use self::adapter::CrosstermTerminalAdapter;
pub use self::adapter::TerminalAdapter;
#[cfg(feature = "termion")]
#[cfg_attr(docsrs, doc(cfg(feature = "termion")))]
pub use self::adapter::TermionTerminalAdapter;
#[cfg(feature = "termwiz")]
#[cfg_attr(docsrs, doc(cfg(feature = "termwiz")))]
pub use self::adapter::TermwizTerminalAdapter;
#[cfg(all(feature = "crossterm", feature = "async-ports"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "crossterm", feature = "async-ports"))))]
pub use self::event_listener::CrosstermAsyncStream;
#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
pub use self::event_listener::CrosstermInputListener;
#[cfg(feature = "termion")]
#[cfg_attr(docsrs, doc(cfg(feature = "termion")))]
pub use self::event_listener::TermionInputListener;
#[cfg(feature = "termwiz")]
#[cfg_attr(docsrs, doc(cfg(feature = "termwiz")))]
pub use self::event_listener::TermwizInputListener;

/// TerminalResult is a type alias for a Result that uses [`TerminalError`] as the error type.
pub type TerminalResult<T> = Result<T, TerminalError>;

/// Errors that can happen when calling any method in [`TerminalAdapter`].
#[derive(Debug, Error)]
pub enum TerminalError {
    #[error("cannot draw frame")]
    CannotDrawFrame,
    #[error("cannot connect to stdout")]
    CannotConnectStdout,
    #[error("cannot enter alternate mode")]
    CannotEnterAlternateMode,
    #[error("cannot leave alternate mode")]
    CannotLeaveAlternateMode,
    #[error("cannot toggle raw mode")]
    CannotToggleRawMode,
    #[error("cannot clear screen")]
    CannotClear,
    #[error("backend doesn't support this command")]
    Unsupported,
    #[error("cannot activate / deactivate mouse capture")]
    CannotToggleMouseCapture,
}
