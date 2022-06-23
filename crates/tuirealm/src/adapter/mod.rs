//! ## adapters
//!
//! this module contains the event converter for the different backends

use crate::core::event::{Event, Key, KeyEvent, KeyModifiers};

// -- crossterm
#[cfg(feature = "with-crossterm")]
pub mod crossterm;
#[cfg(feature = "with-crossterm")]
pub use self::crossterm::CrosstermInputListener as InputEventListener;
#[cfg(feature = "with-crossterm")]
pub use self::crossterm::{Frame, Terminal};

// -- termion
#[cfg(feature = "with-termion")]
pub mod termion;
#[cfg(feature = "with-termion")]
pub use self::termion::TermionInputListener as InputEventListener;
#[cfg(feature = "with-termion")]
pub use self::termion::{Frame, Terminal};
