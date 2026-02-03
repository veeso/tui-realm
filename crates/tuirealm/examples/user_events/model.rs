//! ## Model
//!
//! app model

use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalResult};
use tuirealm::{Application, Update};

use super::{Id, Msg, UserEvent};

pub struct Model {
    /// Application
    pub app: Application<Id, Msg, UserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: CrosstermTerminalAdapter,
}

impl Model {
    fn init_adapter() -> TerminalResult<CrosstermTerminalAdapter> {
        let mut adapter = CrosstermTerminalAdapter::new()?;
        adapter.enable_raw_mode()?;
        adapter.enter_alternate_screen()?;

        Ok(adapter)
    }

    pub fn new(app: Application<Id, Msg, UserEvent>) -> TerminalResult<Self> {
        Ok(Self {
            app,
            quit: false,
            redraw: true,
            terminal: Self::init_adapter()?,
        })
    }

    pub fn view(&mut self) {
        assert!(
            self.terminal
                .draw(|f| {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [
                                Constraint::Length(3), // Label
                                Constraint::Length(3), // Other
                            ]
                            .as_ref(),
                        )
                        .split(f.area());
                    self.app.view(&Id::Label, f, chunks[0]);
                    self.app.view(&Id::Other, f, chunks[1]);
                })
                .is_ok()
        );
    }
}

// Let's implement Update for model

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            // Set redraw
            self.redraw = true;
            // Match message
            match msg {
                Msg::AppClose => {
                    self.quit = true; // Terminate
                    None
                }
                Msg::None => None,
            }
        } else {
            None
        }
    }
}
