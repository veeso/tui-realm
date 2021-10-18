//! ## terminal
//!
//! Cross platform Terminal helper

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
use crate::Terminal;

use thiserror::Error;

// -- types
pub type TerminalResult<T> = Result<T, TerminalError>;

#[derive(Debug, Error)]
pub enum TerminalError {
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
}

/// ## TerminalBridge
///
/// An helper around `Terminal` to quickly setup and perform on terminal.
/// You can opt whether to use or not this structure to interact with the terminal
/// Anyway this structure is 100% cross-backend compatible and is really easy to use, so I suggest you to use it.
/// If you need more advance terminal command, you can get a reference to it using the `raw()` and `raw_mut()` methods.
pub struct TerminalBridge {
    terminal: Terminal,
}

impl TerminalBridge {
    /// ### new
    ///
    /// Instantiates a new Terminal bridge
    pub fn new() -> TerminalResult<Self> {
        Ok(Self {
            terminal: Self::adapt_new_terminal()?,
        })
    }

    /// ### enter_alternate_screen
    ///
    /// Enter in alternate screen using the terminal adapter
    pub fn enter_alternate_screen(&mut self) -> TerminalResult<()> {
        self.adapt_enter_alternate_screen()
    }

    /// ### leave_alternate_screen
    ///
    /// Leave the alternate screen using the terminal adapter
    pub fn leave_alternate_screen(&mut self) -> TerminalResult<()> {
        self.adapt_leave_alternate_screen()
    }

    /// ### clear_screen
    ///
    /// Clear the screen
    pub fn clear_screen(&mut self) -> TerminalResult<()> {
        self.adapt_clear_screen()
    }

    /// ### enable_raw_mode
    ///
    /// Enable terminal raw mode
    pub fn enable_raw_mode(&mut self) -> TerminalResult<()> {
        self.adapt_enable_raw_mode()
    }

    /// ### disable_raw_mode
    ///
    /// Disable terminal raw mode
    pub fn disable_raw_mode(&mut self) -> TerminalResult<()> {
        self.adapt_disable_raw_mode()
    }

    /// ### raw
    ///
    /// Returna an immutable reference to the raw `Terminal` structure
    pub fn raw(&self) -> &Terminal {
        &self.terminal
    }

    /// ### raw_mut
    ///
    /// Return a mutable reference to the raw `Terminal` structure
    pub fn raw_mut(&mut self) -> &mut Terminal {
        &mut self.terminal
    }
}
