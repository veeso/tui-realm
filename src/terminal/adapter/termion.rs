use std::io::Stdout;

use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode as _, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen as _};

use super::{TerminalAdapter, TerminalResult};
use crate::ratatui::{Terminal, backend};
use crate::terminal::TerminalError;

pub type TermionBackend =
    Terminal<backend::TermionBackend<MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>>>;

/// TermionTerminalAdapter is the adapter for the [`termion`] terminal
///
/// It implements the [`TerminalAdapter`] trait
pub struct TermionTerminalAdapter {
    terminal: TermionBackend,
}

impl TermionTerminalAdapter {
    pub fn new() -> TerminalResult<Self> {
        let stdout = std::io::stdout()
            .into_raw_mode()
            .map_err(|_| TerminalError::CannotConnectStdout)?
            .into_alternate_screen()
            .map_err(|_| TerminalError::CannotConnectStdout)?;
        let stdout = MouseTerminal::from(stdout);

        let terminal = Terminal::new(backend::TermionBackend::new(stdout))
            .map_err(|_| TerminalError::CannotConnectStdout)?;

        Ok(Self { terminal })
    }

    pub fn raw(&self) -> &TermionBackend {
        &self.terminal
    }

    pub fn raw_mut(&mut self) -> &mut TermionBackend {
        &mut self.terminal
    }
}

impl TerminalAdapter for TermionTerminalAdapter {
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

    fn disable_raw_mode(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    fn enable_raw_mode(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    fn enter_alternate_screen(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    fn leave_alternate_screen(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    fn disable_mouse_capture(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    fn enable_mouse_capture(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }
}
