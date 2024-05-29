//! ## terminal
//!
//! Cross platform Terminal helper

use thiserror::Error;

use crate::Terminal;

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
    #[error("cannot activate / deactivate mouse capture")]
    CannotToggleMouseCapture,
}

/// An helper around `Terminal` to quickly setup and perform on terminal.
/// You can opt whether to use or not this structure to interact with the terminal
/// Anyway this structure is 100% cross-backend compatible and is really easy to use, so I suggest you to use it.
/// If you need more advance terminal command, you can get a reference to it using the `raw()` and `raw_mut()` methods.
pub struct TerminalBridge {
    terminal: Terminal,
}

impl TerminalBridge {
    /// Instantiates a new Terminal bridge
    pub fn new() -> TerminalResult<Self> {
        Ok(Self {
            terminal: Self::adapt_new_terminal()?,
        })
    }

    /// Enter in alternate screen using the terminal adapter
    pub fn enter_alternate_screen(&mut self) -> TerminalResult<()> {
        self.adapt_enter_alternate_screen()
    }

    /// Leave the alternate screen using the terminal adapter
    pub fn leave_alternate_screen(&mut self) -> TerminalResult<()> {
        self.adapt_leave_alternate_screen()
    }

    /// Clear the screen
    pub fn clear_screen(&mut self) -> TerminalResult<()> {
        self.adapt_clear_screen()
    }

    /// Enable terminal raw mode
    pub fn enable_raw_mode(&mut self) -> TerminalResult<()> {
        self.adapt_enable_raw_mode()
    }

    /// Disable terminal raw mode
    pub fn disable_raw_mode(&mut self) -> TerminalResult<()> {
        self.adapt_disable_raw_mode()
    }

    /// Enable mouse-event capture, if the backend supports it
    pub fn enable_mouse_capture(&mut self) -> TerminalResult<()> {
        self.adapt_enable_mouse_capture()
    }

    /// Disable mouse-event capture, if the backend supports it
    pub fn disable_mouse_capture(&mut self) -> TerminalResult<()> {
        self.adapt_disable_mouse_capture()
    }

    /// Returna an immutable reference to the raw `Terminal` structure
    pub fn raw(&self) -> &Terminal {
        &self.terminal
    }

    /// Return a mutable reference to the raw `Terminal` structure
    pub fn raw_mut(&mut self) -> &mut Terminal {
        &mut self.terminal
    }
}
