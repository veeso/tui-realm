//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::components::Sparkline;
use tuirealm::application::PollStrategy;
use tuirealm::command::CmdResult;
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent};
use tuirealm::listener::{Poll, PortResult, SyncPort};
use tuirealm::props::{
    AttrValue, Attribute, BorderType, Borders, Color, HorizontalAlignment, PropPayload, PropValue,
    Title,
};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::terminal::TerminalAdapter;

mod utils;
use utils::{DataGen, Model};

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    Redraw,
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

impl Model<Id, Msg, UserEvent> {
    /// Draw all components.
    fn view(&mut self) {
        self.terminal
            .draw(|f| {
                // Prepare chunks
                let chunks = Layout::default()
                    .direction(LayoutDirection::Vertical)
                    .margin(1)
                    .constraints([Constraint::Length(20), Constraint::Length(1)].as_ref())
                    .split(f.area());
                self.app.view(&Id::SparklineAlfa, f, chunks[0]);
            })
            .expect("Drawing to the terminal failed");
    }

    /// Handle messages
    fn update(&mut self, msg: Msg) {
        self.redraw = true;
        match msg {
            Msg::AppClose => {
                self.quit = true;
            }
            Msg::Redraw => (),
        }
    }

    /// Mount all main components for initial app stage.
    fn mount_main(&mut self) -> Result<(), Box<dyn Error>> {
        self.app.mount(
            Id::SparklineAlfa,
            Box::new(SparklineAlfa::default()),
            vec![],
        )?;
        // We need to give focus to input then
        self.app.active(&Id::SparklineAlfa)?;

        Ok(())
    }
}

fn main() {
    let mut model = Model::new_ports([SyncPort::new(
        Box::new(DataGen::new(0, 64)),
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
                model.update(msg);
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

impl Poll<UserEvent> for DataGen<u64> {
    fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>> {
        Ok(Some(Event::User(UserEvent::DataGenerated(self.generate()))))
    }
}

// -- components

#[derive(Component)]
struct SparklineAlfa {
    component: Sparkline,
}

impl Default for SparklineAlfa {
    fn default() -> Self {
        Self {
            component: Sparkline::default()
                .title(
                    Title::from("bandwidth (Mbps) *data is fake*")
                        .alignment(HorizontalAlignment::Center),
                )
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Double)
                        .color(Color::Yellow),
                )
                .foreground(Color::LightYellow),
        }
    }
}

impl AppComponent<Msg, UserEvent> for SparklineAlfa {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            Event::User(UserEvent::DataGenerated(data)) => {
                let data: Vec<PropValue> = data.iter().copied().map(PropValue::U64).collect();
                self.attr(
                    Attribute::Dataset,
                    AttrValue::Payload(PropPayload::Vec(data)),
                );
                CmdResult::None
            }
            _ => CmdResult::None,
        };
        Some(Msg::Redraw)
    }
}
