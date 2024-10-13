//! ## Model
//!
//! app model

use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::terminal::{TerminalAdapter, TerminalBridge};
use tuirealm::{Application, Update};

use super::{Id, Msg, UserEvent};

pub struct Model<T>
where
    T: TerminalAdapter,
{
    /// Application
    pub app: Application<Id, Msg, UserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: TerminalBridge<T>,
}

impl<T> Model<T>
where
    T: TerminalAdapter,
{
    pub fn new(app: Application<Id, Msg, UserEvent>, adapter: T) -> Self {
        Self {
            app,
            quit: false,
            redraw: true,
            terminal: TerminalBridge::init(adapter).expect("Cannot initialize terminal"),
        }
    }

    pub fn view(&mut self) {
        assert!(self
            .terminal
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
            .is_ok());
    }
}

// Let's implement Update for model

impl<T> Update<Msg> for Model<T>
where
    T: TerminalAdapter,
{
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
