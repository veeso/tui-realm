use std::io::{Stdout, Write};

use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode as _, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen as _};

use super::{TerminalAdapter, TerminalResult};
use crate::ratatui::{Terminal, TerminalOptions, backend};
use crate::terminal::TerminalError;

pub type TermionBackend = Terminal<backend::TermionBackend<TermionWrapper>>;

/// Wrapper around various [`termion`] terminal structs, as termion, the only one of the supported backends,
/// does not support transitioning between various modes without consuming the writer (which ratatui does not conveniently support).
pub enum TermionWrapper {
    Nothing(Stdout),
    Raw(RawTerminal<Stdout>),
    AlternateRaw(AlternateScreen<RawTerminal<Stdout>>),
    MouseAlternateRaw(MouseTerminal<AlternateScreen<RawTerminal<Stdout>>>),
    MouseRaw(MouseTerminal<RawTerminal<Stdout>>),
}

impl Write for TermionWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            TermionWrapper::Nothing(stdout) => stdout.write(buf),
            TermionWrapper::Raw(raw_terminal) => raw_terminal.write(buf),
            TermionWrapper::AlternateRaw(alternate_screen) => alternate_screen.write(buf),
            TermionWrapper::MouseAlternateRaw(mouse_terminal) => mouse_terminal.write(buf),
            TermionWrapper::MouseRaw(mouse_terminal) => mouse_terminal.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            TermionWrapper::Nothing(stdout) => stdout.flush(),
            TermionWrapper::Raw(raw_terminal) => raw_terminal.flush(),
            TermionWrapper::AlternateRaw(alternate_screen) => alternate_screen.flush(),
            TermionWrapper::MouseAlternateRaw(mouse_terminal) => mouse_terminal.flush(),
            TermionWrapper::MouseRaw(mouse_terminal) => mouse_terminal.flush(),
        }
    }
}

/// [`TermionTerminalAdapter`] is the adapter for the [`termion`] backend.
///
/// [`termion`] does not easily support transition modes, so it has to be set up-front via:
/// - [`new_raw`](Self::new_raw): Enable only raw mode, no alternate screen and no mouse capture (used for Inline & Fixed Viewports)
/// - [`new_mouse_raw`](Self::new_mouse_raw): Enable raw mode and mouse capture, no alternate screen (used for Inline & Fixed Viewports)
/// - [`new_alternate_raw`](Self::new_alternate_raw): Enable raw mode and alternate screen (used for full-screen Viewports)
/// - [`new_mouse_alternate_raw`](Self::new_mouse_alternate_raw): Enable raw mode, alternate screen and Mouse capture (used for full-screen Viewports)
///
/// It implements the [`TerminalAdapter`] trait.
pub struct TermionTerminalAdapter {
    terminal: TermionBackend,
}

impl TermionTerminalAdapter {
    /// Create a new Termion Backend with no modes activated.
    ///
    /// This is likely not what you want.
    pub fn new_nothing() -> TerminalResult<Self> {
        Self::new_nothing_with_options(TerminalOptions::default())
    }

    /// Create a new Termion Backend with no modes activated and custom ratatui Terminal options.
    ///
    /// This is likely not what you want.
    pub fn new_nothing_with_options(options: TerminalOptions) -> TerminalResult<Self> {
        let stdout = std::io::stdout();
        let stdout = TermionWrapper::Nothing(stdout);

        let terminal = Terminal::with_options(backend::TermionBackend::new(stdout), options)
            .map_err(|_| TerminalError::CannotConnectStdout)?;

        Ok(Self { terminal })
    }

    /// Create a new Termion Backend with raw-mode activated.
    pub fn new_raw() -> TerminalResult<Self> {
        Self::new_raw_with_options(TerminalOptions::default())
    }

    /// Create a new Termion Backend with raw-mode activated and custom ratatui Terminal options.
    pub fn new_raw_with_options(options: TerminalOptions) -> TerminalResult<Self> {
        let stdout = std::io::stdout()
            .into_raw_mode()
            .map_err(|_| TerminalError::CannotConnectStdout)?;
        let stdout = TermionWrapper::Raw(stdout);

        let terminal = Terminal::with_options(backend::TermionBackend::new(stdout), options)
            .map_err(|_| TerminalError::CannotConnectStdout)?;

        Ok(Self { terminal })
    }

    /// Create a new Termion Backend with raw-mode and alternate screen activated.
    pub fn new_alternate_raw() -> TerminalResult<Self> {
        Self::new_alternate_raw_with_options(TerminalOptions::default())
    }

    /// Create a new Termion Backend with raw-mode and alternate screen activated and custom ratatui Terminal options.
    pub fn new_alternate_raw_with_options(options: TerminalOptions) -> TerminalResult<Self> {
        let stdout = std::io::stdout()
            .into_raw_mode()
            .map_err(|_| TerminalError::CannotConnectStdout)?
            .into_alternate_screen()
            .map_err(|_| TerminalError::CannotConnectStdout)?;
        let stdout = TermionWrapper::AlternateRaw(stdout);

        let terminal = Terminal::with_options(backend::TermionBackend::new(stdout), options)
            .map_err(|_| TerminalError::CannotConnectStdout)?;

        Ok(Self { terminal })
    }

    /// Create a new Termion Backend with raw-mode, alternate screen and mouse capture activated.
    pub fn new_mouse_alternate_raw() -> TerminalResult<Self> {
        Self::new_mouse_alternate_raw_with_options(TerminalOptions::default())
    }

    /// Create a new Termion Backend with raw-mode, alternate screen, mouse capture activated and custom ratatui Terminal options.
    pub fn new_mouse_alternate_raw_with_options(options: TerminalOptions) -> TerminalResult<Self> {
        let stdout = std::io::stdout()
            .into_raw_mode()
            .map_err(|_| TerminalError::CannotConnectStdout)?
            .into_alternate_screen()
            .map_err(|_| TerminalError::CannotConnectStdout)?;
        let stdout = TermionWrapper::MouseAlternateRaw(MouseTerminal::from(stdout));

        let terminal = Terminal::with_options(backend::TermionBackend::new(stdout), options)
            .map_err(|_| TerminalError::CannotConnectStdout)?;

        Ok(Self { terminal })
    }

    /// Create a new Termion Backend with raw-mode and mouse capture activated.
    pub fn new_mouse_raw() -> TerminalResult<Self> {
        Self::new_mouse_raw_with_options(TerminalOptions::default())
    }

    /// Create a new Termion Backend with raw-mode and mouse capture activated and with custom ratatui Terminal options.
    pub fn new_mouse_raw_with_options(options: TerminalOptions) -> TerminalResult<Self> {
        let stdout = std::io::stdout()
            .into_raw_mode()
            .map_err(|_| TerminalError::CannotConnectStdout)?;
        let stdout = TermionWrapper::MouseRaw(MouseTerminal::from(stdout));

        let terminal = Terminal::with_options(backend::TermionBackend::new(stdout), options)
            .map_err(|_| TerminalError::CannotConnectStdout)?;

        Ok(Self { terminal })
    }

    /// Access the underlying [`ratatui::backend::TermionBackend`] immutably.
    pub fn raw(&self) -> &TermionBackend {
        &self.terminal
    }

    /// Access the underlying [`ratatui::backend::TermionBackend`] mutably.
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

    /// UNSUPPORTED in termion
    ///
    /// Use [`TermionTerminalAdapter::new_raw`] (or related).
    fn enable_raw_mode(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    /// UNSUPPORTED in termion
    fn disable_raw_mode(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    /// UNSUPPORTED in termion
    ///
    /// Use [`TermionTerminalAdapter::new_alternate_raw`] (or related).
    fn enter_alternate_screen(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    /// UNSUPPORTED in termion
    fn leave_alternate_screen(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    /// UNSUPPORTED in termion
    ///
    /// Use [`TermionTerminalAdapter::new_mouse_alternate_raw`] (or related).
    fn enable_mouse_capture(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    /// UNSUPPORTED in termion
    fn disable_mouse_capture(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }
}
