//! ## Terminal
//!
//! terminal bridge adapter for termion

use std::io::stdout;

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
#[cfg(target_family = "unix")]
use termion::screen::AlternateScreen;

use crate::terminal::{TerminalBridge, TerminalError, TerminalResult};
use crate::tui::backend::TermionBackend;
use crate::Terminal;

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
