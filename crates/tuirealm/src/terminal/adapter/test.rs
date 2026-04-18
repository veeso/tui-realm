//! Test backend provides the [`TestBackend`](ratatui::backend::TestBackend) which can be used for
//! integration tests: [`TestBackend`].

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Size;

use crate::ratatui::TerminalOptions;
use crate::terminal::{TerminalAdapter, TerminalError, TerminalResult};

/// A [`TerminalAdapter`] implementing the ratatui [`TestBackend`].
///
/// This backend is useful for doing integration tests.
pub struct TestTerminalAdapter {
    terminal: Terminal<TestBackend>,
}

impl TestTerminalAdapter {
    /// Create a new [`TestTerminalAdapter`] instance with default ratatui Terminal options and the provided
    /// height and width.
    pub fn new(size: Size) -> TerminalResult<Self> {
        Self::new_with_options(size, TerminalOptions::default())
    }

    /// Create a new [`TestTerminalAdapter`] instance with custom ratatui Terminal options, and the provided
    /// sizes.
    pub fn new_with_options(size: Size, options: TerminalOptions) -> TerminalResult<Self> {
        let backend = TestBackend::new(size.width, size.height);
        let terminal = Terminal::with_options(backend, options)
            .map_err(|_| TerminalError::Other("Failed creating terminal"))?;

        Ok(Self { terminal })
    }
}

impl TerminalAdapter for TestTerminalAdapter {
    type Backend = TestBackend;

    /// UNSUPPORTED in test backend
    fn enable_raw_mode(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    /// UNSUPPORTED in test backend
    fn disable_raw_mode(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    /// UNSUPPORTED in test backend
    fn enter_alternate_screen(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    /// UNSUPPORTED in test backend
    fn leave_alternate_screen(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    /// UNSUPPORTED in test backend
    fn enable_mouse_capture(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    /// UNSUPPORTED in test backend
    fn disable_mouse_capture(&mut self) -> TerminalResult<()> {
        Err(TerminalError::Unsupported)
    }

    fn raw_mut(&mut self) -> &mut Terminal<ratatui::backend::TestBackend> {
        &mut self.terminal
    }

    fn raw(&self) -> &Terminal<ratatui::backend::TestBackend> {
        &self.terminal
    }
}
