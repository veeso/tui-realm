use termwiz::terminal::Terminal as _;

use super::{TerminalAdapter, TerminalResult};
use crate::ratatui::Terminal;
use crate::ratatui::backend::TermwizBackend;
use crate::terminal::TerminalError;

/// [`TermwizTerminalAdapter`] is the adapter for the [`termwiz`] terminal
///
/// It implements the [`TerminalAdapter`] trait
pub struct TermwizTerminalAdapter {
    terminal: Terminal<TermwizBackend>,
}

impl TermwizTerminalAdapter {
    /// Create a new instance
    pub fn new() -> TerminalResult<Self> {
        let backend = TermwizBackend::new().map_err(|_| TerminalError::CannotConnectStdout)?;
        let terminal = Terminal::new(backend).map_err(|_| TerminalError::CannotConnectStdout)?;

        Ok(Self { terminal })
    }

    pub fn raw(&self) -> &Terminal<TermwizBackend> {
        &self.terminal
    }

    pub fn raw_mut(&mut self) -> &mut Terminal<TermwizBackend> {
        &mut self.terminal
    }
}

impl TerminalAdapter for TermwizTerminalAdapter {
    fn draw<F>(&mut self, render_callback: F) -> TerminalResult<ratatui::CompletedFrame<'_>>
    where
        F: FnOnce(&mut ratatui::Frame<'_>),
    {
        self.raw_mut()
            .draw(render_callback)
            .map_err(|_| TerminalError::CannotDrawFrame)
    }

    fn clear_screen(&mut self) -> TerminalResult<()> {
        self.terminal
            .clear()
            .map_err(|_| TerminalError::CannotClear)
    }

    fn enable_raw_mode(&mut self) -> TerminalResult<()> {
        self.terminal
            .backend_mut()
            .buffered_terminal_mut()
            .terminal()
            .set_raw_mode()
            .map_err(|_| TerminalError::CannotToggleRawMode)
    }

    /// UNSUPPORTED in termwiz
    fn disable_raw_mode(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    fn enter_alternate_screen(&mut self) -> TerminalResult<()> {
        self.terminal
            .backend_mut()
            .buffered_terminal_mut()
            .terminal()
            .enter_alternate_screen()
            .map_err(|_| TerminalError::CannotLeaveAlternateMode)
    }

    fn leave_alternate_screen(&mut self) -> TerminalResult<()> {
        self.terminal
            .backend_mut()
            .buffered_terminal_mut()
            .terminal()
            .exit_alternate_screen()
            .map_err(|_| TerminalError::CannotLeaveAlternateMode)
    }

    /// UNSUPPORTED in termwiz
    fn enable_mouse_capture(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    /// UNSUPPORTED in termwiz
    fn disable_mouse_capture(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }
}
