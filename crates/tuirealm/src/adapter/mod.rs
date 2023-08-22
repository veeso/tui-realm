//! ## adapters
//!
//! this module contains the event converter for the different backends

#[cfg(feature = "crossterm")]
use crate::core::event::MediaKeyCode;
use crate::core::event::{Event, Key, KeyEvent, KeyModifiers};

// -- crossterm
#[cfg(feature = "crossterm")]
pub mod crossterm;
#[cfg(feature = "crossterm")]
pub use self::crossterm::CrosstermInputListener as InputEventListener;
#[cfg(feature = "crossterm")]
pub use self::crossterm::{Frame, Terminal};

// -- termion
#[cfg(feature = "termion")]
pub mod termion;
#[cfg(feature = "termion")]
pub use self::termion::TermionInputListener as InputEventListener;
#[cfg(feature = "termion")]
pub use self::termion::{Frame, Terminal};
