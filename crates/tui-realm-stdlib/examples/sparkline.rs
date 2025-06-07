//! ## Demo
//!
//! `Demo` shows how to use tui-realm in a real case

mod utils;
use utils::DataGen;

use std::time::Duration;
use tui_realm_stdlib::Sparkline;
// tui
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};

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

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    SparklineAlfa,
}

#[derive(PartialEq, Clone, PartialOrd)]
enum UserEvent {
    DataGenerated(Vec<u64>),
}

impl Eq for UserEvent {}

struct Model {
    quit: bool,   // Becomes true when the user presses <ESC>
    redraw: bool, // Tells whether to refresh the UI; performance optimization
    app: Application<Id, Msg, UserEvent>,
}

impl Default for Model {
    fn default() -> Self {
        // Setup app
        let mut app: Application<Id, Msg, UserEvent> = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(10), 10)
                .add_port(Box::new(DataGen::new(0, 64)), Duration::from_millis(100), 1),
        );
        assert!(
            app.mount(
                Id::SparklineAlfa,
                Box::new(SparklineAlfa::default()),
                vec![]
            )
            .is_ok()
        );
        // We need to give focus to input then
        assert!(app.active(&Id::SparklineAlfa).is_ok());
        Self {
            quit: false,
            redraw: true,
            app,
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
                .constraints([Constraint::Length(20), Constraint::Length(1)].as_ref())
                .split(f.area());
            self.app.view(&Id::SparklineAlfa, f, chunks[0]);
        });
    }
}

fn main() {
    let mut model = Model::default();
    let mut terminal = TerminalBridge::init_crossterm().expect("Cannot create terminal bridge");
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
            Msg::None => None,
        }
    }
}

// -- poll

impl Poll<UserEvent> for DataGen<u64> {
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
        Ok(Some(Event::User(UserEvent::DataGenerated(self.generate()))))
    }
}

// -- components

#[derive(MockComponent)]
struct SparklineAlfa {
    component: Sparkline,
}

impl Default for SparklineAlfa {
    fn default() -> Self {
        Self {
            component: Sparkline::default()
                .title("bandwidth (Mbps) *data is fake*", Alignment::Center)
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Double)
                        .color(Color::Yellow),
                )
                .foreground(Color::LightYellow),
        }
    }
}

impl Component<Msg, UserEvent> for SparklineAlfa {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            Event::User(UserEvent::DataGenerated(data)) => {
                let data: Vec<PropValue> = data.into_iter().map(PropValue::U64).collect();
                self.attr(
                    Attribute::Dataset,
                    AttrValue::Payload(PropPayload::Vec(data)),
                );
                CmdResult::None
            }
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
