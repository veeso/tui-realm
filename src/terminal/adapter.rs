#[cfg(feature = "crossterm")]
mod crossterm;
#[cfg(feature = "termion")]
mod termion;
#[cfg(feature = "termwiz")]
mod termwiz;

#[cfg(feature = "crossterm")]
pub use crossterm::CrosstermTerminalAdapter;
use ratatui::{CompletedFrame, Frame};
#[cfg(feature = "termion")]
pub use termion::TermionTerminalAdapter;
#[cfg(feature = "termwiz")]
pub use termwiz::TermwizTerminalAdapter;

use super::TerminalResult;

/// [`TerminalAdapter`] is a trait that defines the methods that a terminal adapter should implement.
///
/// This trait is used to abstract the terminal implementation from the rest of the application.
/// This allows tui-realm to be used with different terminal libraries, such as crossterm, termion, termwiz, etc.
///
/// # Expectations
///
/// All backends use different methods to enable modes, so the only required part to be implemented is [`draw`](Self::draw).
/// Otherwise, there is expected to be `new` functions, which includes calling with default [`TerminalOptions`](ratatui::TerminalOptions) and with custom ones.
///
/// It is also expected of all backends to automatically restore modes on [`Drop`].
pub trait TerminalAdapter {
    /// Draws a single frame to the terminal.
    ///
    /// Returns a [`CompletedFrame`] if successful, otherwise a [`TerminalError`](super::TerminalError).
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
    ///
    /// This function will call [`ratatui::Terminal::draw`].
    fn draw<F>(&mut self, render_callback: F) -> TerminalResult<CompletedFrame<'_>>
    where
        F: FnOnce(&mut Frame<'_>);

    /// Clear the screen
    fn clear_screen(&mut self) -> TerminalResult<()>;

    /// Enable terminal raw mode
    fn enable_raw_mode(&mut self) -> TerminalResult<()>;

    /// Disable terminal raw mode
    fn disable_raw_mode(&mut self) -> TerminalResult<()>;

    /// Enter in alternate screen using the terminal adapter
    fn enter_alternate_screen(&mut self) -> TerminalResult<()>;

    /// Leave the alternate screen using the terminal adapter
    fn leave_alternate_screen(&mut self) -> TerminalResult<()>;

    /// Enable mouse capture using the terminal adapter
    fn enable_mouse_capture(&mut self) -> TerminalResult<()>;

    /// Disable mouse capture using the terminal adapter
    fn disable_mouse_capture(&mut self) -> TerminalResult<()>;
}
