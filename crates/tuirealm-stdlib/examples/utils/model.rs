use std::error::Error;
use std::hash::Hash;
use std::time::Duration;

use tuirealm::application::Application;
use tuirealm::event::NoUserEvent;
use tuirealm::listener::{EventListenerCfg, SyncPort};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalResult};

/// The main model that stores the global state of the application.
pub struct Model<Id, Msg, UserEvent = NoUserEvent>
where
    Id: Eq + PartialEq + Clone + Hash,
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    pub app: Application<Id, Msg, UserEvent>,
    /// Becomes true when the user presses <ESC>
    pub quit: bool,
    /// Tells whether to refresh the UI; performance optimization
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: CrosstermTerminalAdapter,
}

impl<Id, Msg, UserEvent> Model<Id, Msg, UserEvent>
where
    Id: Eq + PartialEq + Clone + Hash,
    Msg: PartialEq + 'static,
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
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
        Self::new_ports([])
    }

    /// Create a new instance of the model, while also initializing the terminal.
    ///
    /// With extra custom ports.
    pub fn new_ports(ports: impl IntoIterator<Item = SyncPort<UserEvent>>) -> Self {
        let terminal = Self::init_adapter().expect("Couldnt initialize terminal modes");

        let mut eventlistener = EventListenerCfg::default()
            .crossterm_input_listener(Duration::from_millis(10), 10)
            .tick_interval(Duration::from_millis(500));

        for port in ports {
            eventlistener = eventlistener.port(port);
        }

        let app = Application::init(eventlistener);
        Self {
            app,
            quit: false,
            redraw: true,
            terminal,
        }
    }
}
