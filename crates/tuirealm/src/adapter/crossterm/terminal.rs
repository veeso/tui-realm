//! ## Terminal
//!
//! terminal bridge adapter for crossterm

use std::io::stdout;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use crate::ratatui::backend::CrosstermBackend;
use crate::terminal::{TerminalBridge, TerminalError, TerminalResult};
use crate::Terminal;

impl TerminalBridge {
    pub(crate) fn adapt_new_terminal() -> TerminalResult<Terminal> {
        Terminal::new(CrosstermBackend::new(stdout()))
            .map_err(|_| TerminalError::CannotConnectStdout)
    }

    pub(crate) fn adapt_enter_alternate_screen(&mut self) -> TerminalResult<()> {
        execute!(
            self.raw_mut().backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        )
        .map_err(|_| TerminalError::CannotEnterAlternateMode)
    }

    pub(crate) fn adapt_leave_alternate_screen(&mut self) -> TerminalResult<()> {
        execute!(
            self.raw_mut().backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .map_err(|_| TerminalError::CannotLeaveAlternateMode)
    }

    pub(crate) fn adapt_clear_screen(&mut self) -> TerminalResult<()> {
        self.raw_mut()
            .clear()
            .map_err(|_| TerminalError::CannotClear)
    }

    pub(crate) fn adapt_enable_raw_mode(&mut self) -> TerminalResult<()> {
        enable_raw_mode().map_err(|_| TerminalError::CannotToggleRawMode)
    }

    pub(crate) fn adapt_disable_raw_mode(&mut self) -> TerminalResult<()> {
        disable_raw_mode().map_err(|_| TerminalError::CannotToggleRawMode)
    }
}
