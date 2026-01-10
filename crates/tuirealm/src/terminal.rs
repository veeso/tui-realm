//! ## terminal
//!
//! Cross platform Terminal helper

mod adapter;
mod event_listener;

use ratatui::{CompletedFrame, Frame};
use thiserror::Error;

#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
pub use self::adapter::CrosstermTerminalAdapter;
pub use self::adapter::TerminalAdapter;
#[cfg(feature = "termion")]
#[cfg_attr(docsrs, doc(cfg(feature = "termion")))]
pub use self::adapter::TermionTerminalAdapter;
#[cfg(all(feature = "crossterm", feature = "async-ports"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "crossterm", feature = "async-ports"))))]
pub use self::event_listener::CrosstermAsyncStream;
#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
pub use self::event_listener::CrosstermInputListener;
#[cfg(feature = "termion")]
#[cfg_attr(docsrs, doc(cfg(feature = "termion")))]
pub use self::event_listener::TermionInputListener;
// #[cfg(feature = "termwiz")]
// #[cfg_attr(docsrs, doc(cfg(feature = "termwiz")))]
// pub use self::event_listener::TermwizInputListener;

/// TerminalResult is a type alias for a Result that uses [`TerminalError`] as the error type.
pub type TerminalResult<T> = Result<T, TerminalError>;

#[derive(Debug, Error)]
pub enum TerminalError {
    #[error("cannot draw frame")]
    CannotDrawFrame,
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

/// An helper around [`crate::ratatui::Terminal`] to quickly setup and perform on terminal.
/// You can opt whether to use or not this structure to interact with the terminal.
/// Anyway this structure is 100% cross-backend compatible and is really easy to use, so I suggest you to use it.
/// If you need more advance terminal command, you can get a reference to it using the `raw()` and `raw_mut()` methods.
///
/// To quickly setup a terminal with default settings, you can use the [`TerminalBridge::init()`] method.
///
/// ```rust,ignore
/// use tuirealm::terminal::TerminalBridge;
///
/// #[cfg(feature = "crossterm")]
/// let mut terminal = TerminalBridge::init_crossterm().unwrap();
/// #[cfg(feature = "termion")]
/// let mut terminal = TerminalBridge::init_termion().unwrap();
/// #[cfg(feature = "termwiz")]
/// let mut terminal = TerminalBridge::init_termwiz().unwrap();
/// ```
pub struct TerminalBridge<T>
where
    T: TerminalAdapter,
{
    terminal: T,
}

impl<T> TerminalBridge<T>
where
    T: TerminalAdapter,
{
    /// Instantiates a new Terminal bridge from a [`TerminalAdapter`]
    pub fn new(terminal: T) -> Self {
        Self { terminal }
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
    pub fn init(terminal: T) -> TerminalResult<Self> {
        let mut terminal = Self::new(terminal);
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
        self.terminal.enter_alternate_screen()
    }

    /// Leave the alternate screen using the terminal adapter
    pub fn leave_alternate_screen(&mut self) -> TerminalResult<()> {
        self.terminal.leave_alternate_screen()
    }

    /// Clear the screen
    pub fn clear_screen(&mut self) -> TerminalResult<()> {
        self.terminal.clear_screen()
    }

    /// Enable terminal raw mode
    pub fn enable_raw_mode(&mut self) -> TerminalResult<()> {
        self.terminal.enable_raw_mode()
    }

    /// Disable terminal raw mode
    pub fn disable_raw_mode(&mut self) -> TerminalResult<()> {
        self.terminal.disable_raw_mode()
    }

    /// Enable mouse-event capture, if the backend supports it
    pub fn enable_mouse_capture(&mut self) -> TerminalResult<()> {
        self.terminal.enable_mouse_capture()
    }

    /// Disable mouse-event capture, if the backend supports it
    pub fn disable_mouse_capture(&mut self) -> TerminalResult<()> {
        self.terminal.disable_mouse_capture()
    }

    /// Draws a single frame to the terminal.
    ///
    /// Returns a [`CompletedFrame`] if successful, otherwise a [`TerminalError`].
    ///
    /// This method will:
    ///
    /// - autoresize the terminal if necessary
    /// - call the render callback, passing it a [`Frame`] reference to render to
    /// - flush the current internal state by copying the current buffer to the backend
    /// - move the cursor to the last known position if it was set during the rendering closure
    /// - return a [`CompletedFrame`] with the current buffer and the area of the terminal
    ///
    /// The [`CompletedFrame`] returned by this method can be useful for debugging or testing
    /// purposes, but it is often not used in regular applicationss.
    ///
    /// The render callback should fully render the entire frame when called, including areas that
    /// are unchanged from the previous frame. This is because each frame is compared to the
    /// previous frame to determine what has changed, and only the changes are written to the
    /// terminal. If the render callback does not fully render the frame, the terminal will not be
    /// in a consistent state.
    pub fn draw<F>(&mut self, render_callback: F) -> TerminalResult<CompletedFrame<'_>>
    where
        F: FnOnce(&mut Frame<'_>),
    {
        self.terminal.draw(render_callback)
    }
}

#[cfg(feature = "crossterm")]
impl TerminalBridge<adapter::CrosstermTerminalAdapter> {
    /// Create a new instance of the [`TerminalBridge`] using [`crossterm`] as backend
    pub fn new_crossterm() -> TerminalResult<Self> {
        Ok(Self::new(adapter::CrosstermTerminalAdapter::new()?))
    }

    /// Initialize a terminal with reasonable defaults for most applications using [`crossterm`] as backend.
    ///
    /// See [`TerminalBridge::init`] for more information.
    pub fn init_crossterm() -> TerminalResult<Self> {
        Self::init(adapter::CrosstermTerminalAdapter::new()?)
    }

    /// Returns a reference to the underlying [`crate::ratatui::Terminal`]
    pub fn raw(
        &self,
    ) -> &crate::ratatui::Terminal<crate::ratatui::backend::CrosstermBackend<std::io::Stdout>> {
        self.terminal.raw()
    }

    /// Returns a mutable reference the underlying [`crate::ratatui::Terminal`]
    pub fn raw_mut(
        &mut self,
    ) -> &mut crate::ratatui::Terminal<crate::ratatui::backend::CrosstermBackend<std::io::Stdout>>
    {
        self.terminal.raw_mut()
    }
}

#[cfg(feature = "termion")]
impl TerminalBridge<adapter::TermionTerminalAdapter> {
    /// Initialize a terminal with reasonable defaults for most applications using [`termion`] as backend.
    ///
    /// See [`TerminalBridge::init`] for more information.
    pub fn new_init_termion() -> TerminalResult<Self> {
        Self::init(adapter::TermionTerminalAdapter::new_alternate_raw().unwrap())
    }

    /// Returns a reference to the underlying Terminal
    pub fn raw(&self) -> &adapter::TermionBackend {
        self.terminal.raw()
    }

    /// Returns a mutable reference to the underlying Terminal
    pub fn raw_mut(&mut self) -> &mut adapter::TermionBackend {
        self.terminal.raw_mut()
    }
}

#[cfg(feature = "termwiz")]
impl TerminalBridge<adapter::TermwizTerminalAdapter> {
    /// Create a new instance of the [`TerminalBridge`] using [`termwiz`] as backend
    pub fn new_termwiz() -> TerminalResult<Self> {
        Ok(Self::new(adapter::TermwizTerminalAdapter::new()?))
    }

    /// Initialize a terminal with reasonable defaults for most applications using [`termwiz`] as backend.
    ///
    /// See [`TerminalBridge::init`] for more information.
    pub fn init_termwiz() -> TerminalResult<Self> {
        Self::init(adapter::TermwizTerminalAdapter::new()?)
    }

    /// Returns a reference to the underlying [`crate::ratatui::Terminal`]
    pub fn raw(&self) -> &crate::ratatui::Terminal<crate::ratatui::backend::TermwizBackend> {
        self.terminal.raw()
    }

    /// Returns a mutable reference the underlying [`crate::ratatui::Terminal`]
    pub fn raw_mut(
        &mut self,
    ) -> &mut crate::ratatui::Terminal<crate::ratatui::backend::TermwizBackend> {
        self.terminal.raw_mut()
    }
}
