//! ## Demo
//!
//! `Demo` shows how to use tui-realm in a real case

mod utils;
use utils::DataGen;

use std::time::Duration;
use tui_realm_stdlib::Chart;
// tui
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::tui::symbols::Marker;
use tuirealm::tui::widgets::GraphType;

use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::listener::{ListenerResult, Poll};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, BorderType, Borders, Color, Dataset, PropPayload, PropValue,
    Style,
};
use tuirealm::terminal::TerminalBridge;
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, Update,
};

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    ChartAlfa,
}

#[derive(PartialEq, Clone, PartialOrd)]
enum UserEvent {
    DataGenerated(Vec<(f64, f64)>),
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
                .default_input_listener(Duration::from_millis(10))
                .port(
                    Box::new(DataGen::new((0.0, 0.0), (50.0, 35.0))),
                    Duration::from_millis(100),
                ),
        );
        assert!(app
            .mount(Id::ChartAlfa, Box::new(ChartAlfa::default()), vec![])
            .is_ok());
        // We need to give focus to input then
        assert!(app.active(&Id::ChartAlfa).is_ok());
        Self {
            quit: false,
            redraw: true,
            app,
        }
    }
}

impl Model {
    fn view(&mut self, terminal: &mut TerminalBridge) {
        let _ = terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());
            self.app.view(&Id::ChartAlfa, f, chunks[0]);
        });
    }
}

fn main() {
    let mut model = Model::default();
    let mut terminal = TerminalBridge::new().expect("Cannot create terminal bridge");
    let _ = terminal.enable_raw_mode();
    let _ = terminal.enter_alternate_screen();
    // Now we use the Model struct to keep track of some states

    // let's loop until quit is true
    while !model.quit {
        // Tick
        if let Ok(messages) = model.app.tick(PollStrategy::Once) {
            for msg in messages.into_iter() {
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

impl Poll<UserEvent> for DataGen<(f64, f64)> {
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
        Ok(Some(Event::User(UserEvent::DataGenerated(self.generate()))))
    }
}

// -- components

#[derive(MockComponent)]
struct ChartAlfa {
    component: Chart,
}

impl Default for ChartAlfa {
    fn default() -> Self {
        Self {
            component: Chart::default()
                .disabled(false)
                .title("Temperatures in room", Alignment::Center)
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Double)
                        .color(Color::Yellow),
                )
                .x_style(Style::default().fg(Color::LightBlue))
                .x_title("Time")
                .x_bounds((0.0, 50.0))
                .x_labels(&["1Y", "10M", "8M", "6M", "4M", "2M", "now"])
                .y_style(Style::default().fg(Color::Yellow))
                .y_title("Temperature (°C)")
                .y_bounds((0.0, 50.0))
                .y_labels(&[
                    "0", "5", "10", "15", "20", "25", "30", "35", "40", "45", "50",
                ]),
        }
    }
}

impl Component<Msg, UserEvent> for ChartAlfa {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            Event::User(UserEvent::DataGenerated(data)) => {
                // Update data
                let dataset = Dataset::default()
                    .name("Temperatures")
                    .graph_type(GraphType::Line)
                    .marker(Marker::Braille)
                    .style(Style::default().fg(Color::Cyan))
                    .data(data);
                self.attr(
                    Attribute::Dataset,
                    AttrValue::Payload(PropPayload::Vec(vec![PropValue::Dataset(dataset)])),
                );
                CmdResult::None
            }
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
