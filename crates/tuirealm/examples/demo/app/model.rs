//! ## Model
//!
//! app model

use std::error::Error;
use std::time::{Duration, SystemTime};

use tuirealm::event::NoUserEvent;
use tuirealm::props::{Color, HorizontalAlignment, TextModifiers};
use tuirealm::ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalResult};
use tuirealm::{
    Application, AttrValue, Attribute, EventListenerCfg, Sub, SubClause, SubEventClause, Update,
};

use super::components::{Clock, DigitCounter, Label, LetterCounter};
use super::{Id, Msg};

pub struct Model<T>
where
    T: TerminalAdapter,
{
    /// Application
    pub app: Application<Id, Msg, NoUserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: T,
}

impl Default for Model<CrosstermTerminalAdapter> {
    fn default() -> Self {
        Self {
            app: Self::init_app().expect("Failed to mount components"),
            quit: false,
            redraw: true,
            terminal: Self::init_adapter().expect("Cannot initialize terminal"),
        }
    }
}

impl Model<CrosstermTerminalAdapter> {
    fn init_adapter() -> TerminalResult<CrosstermTerminalAdapter> {
        let mut adapter = CrosstermTerminalAdapter::new()?;
        adapter.enable_raw_mode()?;
        adapter.enter_alternate_screen()?;
        // adapter.enable_mouse_capture()?; // Not necessary for this example

        Ok(adapter)
    }
}

impl<T> Model<T>
where
    T: TerminalAdapter,
{
    pub fn view(&mut self) {
        assert!(
            self.terminal
                .draw(|f| {
                    let [help, clock, letter, digit, label] = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [
                                Constraint::Length(1), // Help text
                                Constraint::Length(2), // Clock
                                Constraint::Length(3), // Letter Counter
                                Constraint::Length(3), // Digit Counter
                                Constraint::Length(1), // Label
                            ]
                            .as_ref(),
                        )
                        .areas(f.area());
                    self.app.view(&Id::Help, f, help);
                    self.app.view(&Id::Clock, f, clock);
                    self.app.view(&Id::LetterCounter, f, letter);
                    self.app.view(&Id::DigitCounter, f, digit);
                    self.app.view(&Id::Label, f, label);
                })
                .is_ok()
        );
    }

    fn init_app() -> Result<Application<Id, Msg, NoUserEvent>, Box<dyn Error>> {
        // Setup application
        // NOTE: NoUserEvent is a shorthand to tell tui-realm we're not going to use any custom user event
        // NOTE: the event listener is configured to use the default crossterm input listener and to raise a Tick event each second
        // which we will use to update the clock

        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(20), 3)
                .tick_interval(Duration::from_secs(1)),
        );
        // Mount Help text
        app.mount(
            Id::Help,
            Box::new(
                Label::default()
                    .text("Switch counters with <TAB>, exit with <ESC>")
                    .alignment(HorizontalAlignment::Left)
                    .background(Color::Reset)
                    .foreground(Color::DarkGray),
            ),
            Vec::default(),
        )?;
        // Mount Message label
        app.mount(
            Id::Label,
            Box::new(
                Label::default()
                    .text("Waiting for a Msg...")
                    .alignment(HorizontalAlignment::Left)
                    .background(Color::Reset)
                    .foreground(Color::LightYellow)
                    .modifiers(TextModifiers::BOLD),
            ),
            Vec::default(),
        )?;
        // Mount clock, subscribe to tick
        app.mount(
            Id::Clock,
            Box::new(
                Clock::new(SystemTime::now())
                    .alignment(HorizontalAlignment::Center)
                    .background(Color::Reset)
                    .foreground(Color::Cyan)
                    .modifiers(TextModifiers::BOLD),
            ),
            vec![Sub::new(SubEventClause::Tick, SubClause::Always)],
        )?;
        // Mount counters
        app.mount(
            Id::LetterCounter,
            Box::new(LetterCounter::new(0)),
            Vec::new(),
        )?;
        app.mount(
            Id::DigitCounter,
            Box::new(DigitCounter::new(5)),
            Vec::default(),
        )?;
        // Active letter counter
        app.active(&Id::LetterCounter)?;
        Ok(app)
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
                Msg::Clock => None,
                Msg::DigitCounterBlur => {
                    // Give focus to letter counter
                    assert!(self.app.active(&Id::LetterCounter).is_ok());
                    None
                }
                Msg::DigitCounterChanged(v) => {
                    // Update label
                    assert!(
                        self.app
                            .attr(
                                &Id::Label,
                                Attribute::Text,
                                AttrValue::String(format!("DigitCounter has now value: {v}"))
                            )
                            .is_ok()
                    );
                    None
                }
                Msg::LetterCounterBlur => {
                    // Give focus to digit counter
                    assert!(self.app.active(&Id::DigitCounter).is_ok());
                    None
                }
                Msg::LetterCounterChanged(v) => {
                    // Update label
                    assert!(
                        self.app
                            .attr(
                                &Id::Label,
                                Attribute::Text,
                                AttrValue::String(format!("LetterCounter has now value: {v}"))
                            )
                            .is_ok()
                    );
                    None
                }
            }
        } else {
            None
        }
    }
}
