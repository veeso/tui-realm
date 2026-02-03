use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{execute, queue};

use super::{TerminalAdapter, TerminalResult};
use crate::ratatui::backend::CrosstermBackend;
use crate::ratatui::{Terminal, TerminalOptions};
use crate::terminal::TerminalError;

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Modes: u8 {
        const NONE = 0b0000_0000;
        const RAW = 0b0000_0001;
        const ALTERNATE = 0b0000_0010;
        const MOUSE = 0b0000_0100;
    }
}

/// CrosstermTerminalAdapter is the adapter for the [`crossterm`] terminal
///
/// It implements the [`TerminalAdapter`] trait
///
/// # Restore
///
/// This implementation keeps track of modes activated via [`TerminalAdapter`] methods.
///
/// ## On Panic
///
/// Automatically restores all modes on panic (so that the message is printed correctly).
#[derive(Debug)]
pub struct CrosstermTerminalAdapter {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    modes: Arc<AtomicU8>,
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

        let modes = Arc::new(AtomicU8::new(Modes::NONE.bits()));
        let modes_c = modes.clone();

        Self::panic_handler(modes_c);

        Ok(Self { terminal, modes })
    }

    /// Access the underlying [`ratatui::backend::CrosstermBackend`] immutably.
    pub fn raw(&self) -> &Terminal<CrosstermBackend<std::io::Stdout>> {
        &self.terminal
    }

    /// Access the underlying [`ratatui::backend::CrosstermBackend`] mutably.
    pub fn raw_mut(&mut self) -> &mut Terminal<CrosstermBackend<std::io::Stdout>> {
        &mut self.terminal
    }

    /// Restore the terminal state to pre [`TerminalAdapter`] state.
    pub fn restore(&mut self) -> std::io::Result<()> {
        // NOTE: if changing something here, dont forget to change it in the panic handler too
        let writer = self.terminal.backend_mut();

        let modes =
            Modes::from_bits_truncate(self.modes.swap(Modes::NONE.bits(), Ordering::AcqRel));

        if modes.contains(Modes::MOUSE) {
            queue!(writer, DisableMouseCapture)?;
        }
        if modes.contains(Modes::ALTERNATE) {
            queue!(writer, LeaveAlternateScreen)?;
        }
        if modes.contains(Modes::RAW) {
            disable_raw_mode()?;
        }

        writer.flush()
    }

    /// Set the panic handler restore.
    fn panic_handler(modes: Arc<AtomicU8>) {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            // NOTE: if changing something here, dont forget to change it in the restore too
            let mut stdout = std::io::stdout();

            let modes = Modes::from_bits_truncate(modes.swap(Modes::NONE.bits(), Ordering::AcqRel));

            if modes.contains(Modes::MOUSE) {
                let _ = queue!(stdout, DisableMouseCapture);
            }
            if modes.contains(Modes::ALTERNATE) {
                let _ = queue!(stdout, LeaveAlternateScreen);
            }
            if modes.contains(Modes::RAW) {
                let _ = disable_raw_mode();
            }

            let _ = stdout.flush();

            hook(info);
        }));
    }

    /// Add a active mode to the "active modes" flags.
    fn set_mode(&self, mode: Modes) {
        // The following should never cause a "Time of Check, Time of Use" issue due to
        // only actually reading / resetting in the panic handler or "restore", which cannot happen in here.
        let active = self.modes.load(Ordering::SeqCst);
        self.modes.store(active | mode.bits(), Ordering::SeqCst);
    }

    /// Remove a mode from the "active modes" flags.
    fn unset_mode(&self, mode: Modes) {
        // The following should never cause a "Time of Check, Time of Use" issue due to
        // only actually reading / resetting in the panic handler or "restore", which cannot happen in here.
        let active = self.modes.load(Ordering::SeqCst);
        self.modes.store(active ^ mode.bits(), Ordering::SeqCst);
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
        enable_raw_mode()
            .map_err(|_| TerminalError::CannotToggleRawMode)
            .inspect(|_| {
                self.set_mode(Modes::RAW);
            })
    }

    fn disable_raw_mode(&mut self) -> TerminalResult<()> {
        disable_raw_mode()
            .map_err(|_| TerminalError::CannotToggleRawMode)
            .inspect(|_| {
                self.unset_mode(Modes::RAW);
            })
    }

    fn enter_alternate_screen(&mut self) -> TerminalResult<()> {
        execute!(
            self.terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        )
        .map_err(|_| TerminalError::CannotEnterAlternateMode)
        .inspect(|_| {
            self.set_mode(Modes::ALTERNATE | Modes::MOUSE);
        })
    }

    fn leave_alternate_screen(&mut self) -> TerminalResult<()> {
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .map_err(|_| TerminalError::CannotLeaveAlternateMode)
        .inspect(|_| {
            self.unset_mode(Modes::ALTERNATE | Modes::MOUSE);
        })
    }

    fn enable_mouse_capture(&mut self) -> TerminalResult<()> {
        execute!(self.raw_mut().backend_mut(), EnableMouseCapture)
            .map_err(|_| TerminalError::CannotToggleMouseCapture)
            .inspect(|_| {
                self.set_mode(Modes::MOUSE);
            })
    }

    fn disable_mouse_capture(&mut self) -> TerminalResult<()> {
        execute!(self.raw_mut().backend_mut(), DisableMouseCapture)
            .map_err(|_| TerminalError::CannotToggleMouseCapture)
            .inspect(|_| {
                self.unset_mode(Modes::MOUSE);
            })
    }
}
