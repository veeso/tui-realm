//! ## Terminal
//!
//! terminal bridge adapter for termion

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
use crate::terminal::{TerminalBridge, TerminalError, TerminalResult};
use crate::tui::backend::TermionBackend;
use crate::Terminal;
use std::io::stdout;
#[cfg(target_family = "unix")]
use termion::screen::AlternateScreen;
use termion::{input::MouseTerminal, raw::IntoRawMode};

impl TerminalBridge {
    pub(crate) fn adapt_new_terminal() -> TerminalResult<Terminal> {
        let screen = stdout()
            .into_raw_mode()
            .map_err(|_| TerminalError::CannotConnectStdout)
            .map(MouseTerminal::from)
            .map(AlternateScreen::from)?;
        Terminal::new(TermionBackend::new(screen)).map_err(|_| TerminalError::CannotConnectStdout)
    }

    pub(crate) fn adapt_enter_alternate_screen(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    pub(crate) fn adapt_leave_alternate_screen(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    pub(crate) fn adapt_clear_screen(&mut self) -> TerminalResult<()> {
        self.raw_mut()
            .clear()
            .map_err(|_| TerminalError::CannotClear)
    }

    pub(crate) fn adapt_enable_raw_mode(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    pub(crate) fn adapt_disable_raw_mode(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }
}
