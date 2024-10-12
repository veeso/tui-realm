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
}

/// An helper around [`Terminal`] to quickly setup and perform on terminal.
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

    /// Initialize a terminal with reasonable defaults for most applications.
    ///
    /// This will create a new [`TerminalBridge`] and initialize it with the following defaults:
    ///
    /// - Raw mode is enabled
    /// - Alternate screen buffer enabled
    /// - A panic hook is installed that restores the terminal before panicking. Ensure that this method
    ///   is called after any other panic hooks that may be installed to ensure that the terminal is
    ///   restored before those hooks are called.
    ///
    /// For more control over the terminal initialization, use [`TerminalBridge::new`].
    pub fn init() -> TerminalResult<Self> {
        let mut terminal = Self::new()?;
        terminal.enable_raw_mode()?;
        terminal.enter_alternate_screen()?;
        Self::set_panic_hook();

        Ok(terminal)
    }

    /// Restore the terminal to its original state.
    ///
    /// This function will attempt to restore the terminal to its original state by leaving the alternate screen
    /// and disabling raw mode. If either of these operations fail, the error will be returned.
    pub fn restore(&mut self) -> TerminalResult<()> {
        self.leave_alternate_screen()?;
        self.disable_raw_mode()
    }

    /// Sets a panic hook that restores the terminal before panicking.
    ///
    /// Replaces the panic hook with a one that will restore the terminal state before calling the
    /// original panic hook. This ensures that the terminal is left in a good state when a panic occurs.
    pub fn set_panic_hook() {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            #[cfg(feature = "crossterm")]
            {
                let _ = crossterm::terminal::disable_raw_mode();
                let _ = crossterm::execute!(
                    std::io::stdout(),
                    crossterm::terminal::LeaveAlternateScreen
                );
            }

            hook(info);
        }));
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

    /// Returna an immutable reference to the raw `Terminal` structure
    pub fn raw(&self) -> &Terminal {
        &self.terminal
    }

    /// Return a mutable reference to the raw `Terminal` structure
    pub fn raw_mut(&mut self) -> &mut Terminal {
        &mut self.terminal
    }
}
