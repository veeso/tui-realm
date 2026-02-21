use std::{error::Error, hash::Hash, time::Duration};

use tuirealm::{
    Application, EventListenerCfg, NoUserEvent,
    terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalResult},
};

/// The main model that stores the global state of the application.
pub struct Model<Id, Msg>
where
    Id: Eq + PartialEq + Clone + Hash,
    Msg: PartialEq,
{
    pub app: Application<Id, Msg, NoUserEvent>,
    /// Becomes true when the user presses <ESC>
    pub quit: bool,
    /// Tells whether to refresh the UI; performance optimization
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: CrosstermTerminalAdapter,
}

impl<Id, Msg> Model<Id, Msg>
where
    Id: Eq + PartialEq + Clone + Hash,
    Msg: PartialEq + 'static,
{
    /// Initialize the Terminal modes.
    fn init_adapter() -> TerminalResult<CrosstermTerminalAdapter> {
        let mut adapter = CrosstermTerminalAdapter::new()?;
        adapter.enable_raw_mode()?;
        adapter.enter_alternate_screen()?;

        Ok(adapter)
    }

    /// Create a new instance of the model, while also initializing the terminal.
    pub fn new() -> Self {
        let terminal = Self::init_adapter().expect("Couldnt initialize terminal modes");

        let app = Application::init(
            EventListenerCfg::default().crossterm_input_listener(Duration::from_millis(10), 10),
        );
        Self {
            app,
            quit: false,
            redraw: true,
            terminal,
        }
    }
}
