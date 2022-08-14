//! ## crossterm
//!
//! this module contains the adapters for crossterm

extern crate crossterm;

mod event;
mod listener;
mod terminal;

// -- export
pub use listener::CrosstermInputListener;

use super::{Event, Key, KeyEvent, KeyModifiers, MediaKeyCode};
use crate::tui::{backend::CrosstermBackend, Frame as TuiFrame, Terminal as TuiTerminal};
use std::io::Stdout;

// -- Frame

/// Frame represents the Frame where the view will be displayed in
pub type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;

/// Terminal must be used to interact with the terminal in tui applications
pub type Terminal = TuiTerminal<CrosstermBackend<Stdout>>;
