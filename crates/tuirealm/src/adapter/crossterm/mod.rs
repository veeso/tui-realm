//! ## crossterm
//!
//! this module contains the adapters for crossterm

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
extern crate crossterm;

mod event;
mod listener;
mod terminal;

// -- export
pub use listener::CrosstermInputListener;

use super::{Event, Key, KeyEvent, KeyModifiers};
use crate::tui::{backend::CrosstermBackend, Frame as TuiFrame, Terminal as TuiTerminal};
use std::io::Stdout;

// -- Frame
/// ## Frame
///
/// Frame represents the Frame where the view will be displayed in
pub type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;

/// ## Terminal
///
/// Terminal must be used to interact with the terminal in tui applications
pub type Terminal = TuiTerminal<CrosstermBackend<Stdout>>;

// -- converters

// -- Event listener

// -- terminal adapter
