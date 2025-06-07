//! ## Demo
//!
//! `Demo` shows how to use tui-realm in a real case

mod utils;
use utils::Loader;

use std::time::Duration;

use tui_realm_stdlib::ProgressBar;
use tuirealm::command::CmdResult;
use tuirealm::listener::{ListenerResult, Poll};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, BorderType, Borders, Color, PropPayload, PropValue,
};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalBridge};
use tuirealm::{
    Application, Component, Event, EventListenerCfg, MockComponent, Update,
    application::PollStrategy,
    event::{Key, KeyEvent},
};
// tui
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    GaugeAlfaBlur,
    GaugeBetaBlur,
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    GaugeAlfa,
    GaugeBeta,
}

#[derive(PartialEq, Clone, PartialOrd)]
enum UserEvent {
    Loaded(f64),
}

impl Eq for UserEvent {}

struct Model {
    app: Application<Id, Msg, UserEvent>,
    quit: bool,   // Becomes true when the user presses <ESC>
    redraw: bool, // Tells whether to refresh the UI; performance optimization
}

impl Default for Model {
    fn default() -> Self {
        // Setup app
        let mut app: Application<Id, Msg, UserEvent> = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(10), 10)
                .add_port(Box::new(Loader::default()), Duration::from_millis(50), 1),
        );
        assert!(
            app.mount(Id::GaugeAlfa, Box::new(GaugeAlfa::default()), vec![])
                .is_ok()
        );
        assert!(
            app.mount(Id::GaugeBeta, Box::new(GaugeBeta::default()), vec![])
                .is_ok()
        );
        // We need to give focus to input then
        assert!(app.active(&Id::GaugeAlfa).is_ok());
        Self {
            app,
            quit: false,
            redraw: true,
        }
    }
}

impl Model {
    fn view(&mut self, terminal: &mut TerminalBridge<CrosstermTerminalAdapter>) {
        let _ = terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.area());
            self.app.view(&Id::GaugeAlfa, f, chunks[0]);
            self.app.view(&Id::GaugeBeta, f, chunks[1]);
        });
    }
}

fn main() {
    let mut terminal = TerminalBridge::init_crossterm().expect("Cannot create terminal bridge");
    let mut model = Model::default();
    let _ = terminal.enable_raw_mode();
    let _ = terminal.enter_alternate_screen();

    // Now we use the Model struct to keep track of some states

    // let's loop until quit is true
    while !model.quit {
        // Tick
        if let Ok(messages) = model.app.tick(PollStrategy::Once) {
            for msg in messages {
                let mut msg = Some(msg);
                while msg.is_some() {
                    msg = model.update(msg);
                }
            }
        }
        // Redraw
        if model.redraw {
            model.view(&mut terminal);
            model.redraw = false;
        }
    }
    // Terminate terminal
    let _ = terminal.leave_alternate_screen();
    let _ = terminal.disable_raw_mode();
    let _ = terminal.clear_screen();
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.redraw = true;
        match msg.unwrap_or(Msg::None) {
            Msg::AppClose => {
                self.quit = true;
                None
            }
            Msg::GaugeAlfaBlur => {
                assert!(self.app.active(&Id::GaugeBeta).is_ok());
                None
            }
            Msg::GaugeBetaBlur => {
                assert!(self.app.active(&Id::GaugeAlfa).is_ok());
                None
            }
            Msg::None => None,
        }
    }
}

// -- poll

impl Poll<UserEvent> for Loader {
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
        Ok(Some(Event::User(UserEvent::Loaded(self.load()))))
    }
}

// -- components

#[derive(MockComponent)]
struct GaugeAlfa {
    component: ProgressBar,
}

impl Default for GaugeAlfa {
    fn default() -> Self {
        Self {
            component: ProgressBar::default()
                .borders(
                    Borders::default()
                        .color(Color::Green)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::Green)
                .label("0%")
                .title("Loading...", Alignment::Center)
                .progress(0.0),
        }
    }
}

impl Component<Msg, UserEvent> for GaugeAlfa {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::User(UserEvent::Loaded(prog)) => {
                // Update
                let label = format!("{:02}%", (prog * 100.0) as usize);
                self.attr(
                    Attribute::Value,
                    AttrValue::Payload(PropPayload::One(PropValue::F64(prog))),
                );
                self.attr(Attribute::Text, AttrValue::String(label));
                CmdResult::None
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::GaugeAlfaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

#[derive(MockComponent)]
struct GaugeBeta {
    component: ProgressBar,
}

impl Default for GaugeBeta {
    fn default() -> Self {
        Self {
            component: ProgressBar::default()
                .borders(
                    Borders::default()
                        .color(Color::Yellow)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::Yellow)
                .label("0%")
                .title("Loading...", Alignment::Center)
                .progress(0.0),
        }
    }
}

impl Component<Msg, UserEvent> for GaugeBeta {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::User(UserEvent::Loaded(prog)) => {
                // Update
                let label = format!("{:02}%", (prog * 100.0) as usize);
                self.attr(
                    Attribute::Value,
                    AttrValue::Payload(PropPayload::One(PropValue::F64(prog))),
                );
                self.attr(Attribute::Text, AttrValue::String(label));
                CmdResult::None
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::GaugeBetaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
