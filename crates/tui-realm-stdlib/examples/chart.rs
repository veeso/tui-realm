//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::components::{Chart, ChartDataset};
use tuirealm::MockComponent;
use tuirealm::application::PollStrategy;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{Component, MockComponent};
use tuirealm::event::{Event, Key, KeyEvent};
use tuirealm::listener::{Poll, PortResult, SyncPort};
use tuirealm::props::{
    AttrValue, Attribute, BorderType, Borders, Color, HorizontalAlignment, PropPayload, Style,
    Title,
};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::ratatui::symbols::Marker;
use tuirealm::ratatui::widgets::GraphType;

mod utils;
use utils::{DataGen, Model};

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

impl Model<Id, Msg, UserEvent> {
    /// Draw all components.
    fn view(&mut self) {
        self.terminal
            .raw_mut()
            .draw(|f| {
                // Prepare chunks
                let chunks = Layout::default()
                    .direction(LayoutDirection::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.area());
                self.app.view(&Id::ChartAlfa, f, chunks[0]);
            })
            .expect("Drawing to the terminal failed");
    }

    /// Handle messages
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

    /// Mount all main components for initial app stage.
    fn mount_main(&mut self) -> Result<(), Box<dyn Error>> {
        self.app
            .mount(Id::ChartAlfa, Box::new(ChartAlfa::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::ChartAlfa)?;

        Ok(())
    }
}

fn main() {
    let mut model = Model::new_ports([SyncPort::new(
        Box::new(DataGen::new((0.0, 0.0), (50.0, 35.0))),
        Duration::from_millis(100),
        1,
    )]);
    model.mount_main().expect("Mount all main components");

    // let's loop until quit is true
    while !model.quit {
        // Tick
        if let Ok(messages) = model
            .app
            .tick(PollStrategy::Once(Duration::from_millis(10)))
        {
            for msg in messages {
                let mut msg = Some(msg);
                while msg.is_some() {
                    msg = model.update(msg);
                }
            }
        }
        // Redraw
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }
}

// -- poll

impl Poll<UserEvent> for DataGen<(f64, f64)> {
    fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>> {
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
                .title(Title::from("Temperatures in room").alignment(HorizontalAlignment::Center))
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
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Msg> {
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
                let dataset = ChartDataset::default()
                    .name("Temperatures")
                    .graph_type(GraphType::Line)
                    .marker(Marker::Braille)
                    .style(Style::default().fg(Color::Cyan))
                    .data(data.clone());
                self.attr(
                    Attribute::Dataset,
                    AttrValue::Payload(PropPayload::Any(Box::new(vec![dataset]))),
                );
                CmdResult::None
            }
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
