use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

use super::{TerminalAdapter, TerminalResult};
use crate::ratatui::backend::CrosstermBackend;
use crate::ratatui::{Terminal, TerminalOptions};
use crate::terminal::TerminalError;

/// CrosstermTerminalAdapter is the adapter for the [`crossterm`] terminal
///
/// It implements the [`TerminalAdapter`] trait
///
/// # Panic Handler
///
/// None; needs to be done manually; TODO
pub struct CrosstermTerminalAdapter {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl CrosstermTerminalAdapter {
    /// Create a new crossterm instance with default ratatui Terminal options
    pub fn new() -> TerminalResult<Self> {
        Self::new_with_options(TerminalOptions::default())
    }

    /// Create a new crossterm instance with custom ratatui Terminal options
    pub fn new_with_options(options: TerminalOptions) -> TerminalResult<Self> {
        let backend = CrosstermBackend::new(std::io::stdout());
        let terminal = Terminal::with_options(backend, options)
            .map_err(|_| TerminalError::CannotConnectStdout)?;

        Ok(Self { terminal })
    }

    /// Access the underlying [`ratatui::backend::CrosstermBackend`] immutably.
    pub fn raw(&self) -> &Terminal<CrosstermBackend<std::io::Stdout>> {
        &self.terminal
    }

    /// Access the underlying [`ratatui::backend::CrosstermBackend`] mutably.
    pub fn raw_mut(&mut self) -> &mut Terminal<CrosstermBackend<std::io::Stdout>> {
        &mut self.terminal
    }
}

impl TerminalAdapter for CrosstermTerminalAdapter {
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
        enable_raw_mode().map_err(|_| TerminalError::CannotToggleRawMode)
    }

    fn disable_raw_mode(&mut self) -> TerminalResult<()> {
        disable_raw_mode().map_err(|_| TerminalError::CannotToggleRawMode)
    }

    fn enter_alternate_screen(&mut self) -> TerminalResult<()> {
        execute!(
            self.terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        )
        .map_err(|_| TerminalError::CannotEnterAlternateMode)
    }

    fn leave_alternate_screen(&mut self) -> TerminalResult<()> {
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .map_err(|_| TerminalError::CannotLeaveAlternateMode)
    }

    fn enable_mouse_capture(&mut self) -> TerminalResult<()> {
        execute!(self.raw_mut().backend_mut(), EnableMouseCapture)
            .map_err(|_| TerminalError::CannotToggleMouseCapture)
    }

    fn disable_mouse_capture(&mut self) -> TerminalResult<()> {
        execute!(self.raw_mut().backend_mut(), DisableMouseCapture)
            .map_err(|_| TerminalError::CannotToggleMouseCapture)
    }
}
