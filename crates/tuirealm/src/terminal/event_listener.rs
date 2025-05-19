#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "termion")]
mod termion;

#[cfg(feature = "crossterm")]
pub use crossterm::CrosstermInputListener;
#[cfg(feature = "termion")]
pub use termion::TermionInputListener;

#[allow(unused_imports)] // used in the event listeners
use crate::Event;
