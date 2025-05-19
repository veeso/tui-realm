#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(all(feature = "crossterm", feature = "async-ports"))]
mod crossterm_async;
#[cfg(feature = "termion")]
mod termion;

#[cfg(feature = "crossterm")]
pub use crossterm::CrosstermInputListener;
#[cfg(all(feature = "crossterm", feature = "async-ports"))]
pub use crossterm_async::CrosstermAsyncStream;
#[cfg(feature = "termion")]
pub use termion::TermionInputListener;

#[allow(unused_imports)] // used in the event listeners
use crate::Event;
