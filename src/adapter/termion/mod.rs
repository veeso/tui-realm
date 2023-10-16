//! ## termion
//!
//! this module contains the adapters for termion

extern crate termion;

mod event;
mod listener;
mod terminal;

// -- export
use std::io::Stdout;

pub use listener::TermionInputListener;
use termion::input::MouseTerminal;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

use super::{Event, Key, KeyEvent, KeyModifiers};
use crate::tui::backend::TermionBackend;
use crate::tui::{Frame as TuiFrame, Terminal as TuiTerminal};

// -- Frame

/// Frame represents the Frame where the view will be displayed in
pub type Frame<'a> =
    TuiFrame<'a, TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>;

/// Terminal must be used to interact with the terminal in tui applications
pub type Terminal =
    TuiTerminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>;
