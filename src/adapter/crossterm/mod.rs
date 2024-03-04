//! ## crossterm
//!
//! this module contains the adapters for crossterm

extern crate crossterm;

mod event;
mod listener;
mod terminal;

// -- export
use std::io::Stdout;

pub use listener::CrosstermInputListener;

use super::{Event, Key, KeyEvent, KeyModifiers, MediaKeyCode};
use crate::tui::backend::CrosstermBackend;
use crate::tui::{Frame as TuiFrame, Terminal as TuiTerminal};

// -- Frame

/// Frame represents the Frame where the view will be displayed in
#[cfg(feature = "ratatui")]
pub type Frame<'a> = TuiFrame<'a>;

#[cfg(feature = "tui")]
pub type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;

/// Terminal must be used to interact with the terminal in tui applications
pub type Terminal = TuiTerminal<CrosstermBackend<Stdout>>;
