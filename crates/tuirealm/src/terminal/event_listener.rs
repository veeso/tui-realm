#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "termion")]
mod termion;

#[cfg(feature = "crossterm")]
pub use crossterm::CrosstermInputListener;
#[cfg(feature = "termion")]
pub use termion::TermionInputListener;

use crate::Event;
