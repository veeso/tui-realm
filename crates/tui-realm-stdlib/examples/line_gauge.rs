//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::LineGauge;
use tuirealm::command::CmdResult;
use tuirealm::listener::{Poll, PortResult, SyncPort};
use tuirealm::props::{
    AttrValue, Attribute, BorderType, Borders, Color, HorizontalAlignment, PropPayload, PropValue,
    Style, Title,
};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::ratatui::symbols::line::{HORIZONTAL, THICK_HORIZONTAL};
use tuirealm::ratatui::text::Span;
use tuirealm::{
    Component, Event, MockComponent,
    application::PollStrategy,
    event::{Key, KeyEvent},
};

mod utils;
use utils::{Loader, Model};

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

    /// Mount all main components for initial app stage.
    fn mount_main(&mut self) -> Result<(), Box<dyn Error>> {
        self.app
            .mount(Id::GaugeAlfa, Box::new(GaugeAlfa::default()), vec![])?;
        self.app
            .mount(Id::GaugeBeta, Box::new(GaugeBeta::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::GaugeAlfa)?;

        Ok(())
    }
}

fn main() {
    let mut model = Model::new_ports([SyncPort::new(
        Box::new(Loader::default()),
        Duration::from_millis(50),
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

impl Poll<UserEvent> for Loader {
    fn poll(&mut self) -> PortResult<Option<Event<UserEvent>>> {
        Ok(Some(Event::User(UserEvent::Loaded(self.load()))))
    }
}

// -- components

#[derive(MockComponent)]
struct GaugeAlfa {
    component: LineGauge,
}

impl Default for GaugeAlfa {
    fn default() -> Self {
        Self {
            component: LineGauge::default()
                .borders(
                    Borders::default()
                        .color(Color::Green)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::Green)
                .label("0%")
                .title(Title::from("Loading...").alignment(HorizontalAlignment::Center))
                .line_style(THICK_HORIZONTAL, HORIZONTAL)
                .progress(0.0),
        }
    }
}

impl Component<Msg, UserEvent> for GaugeAlfa {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::User(UserEvent::Loaded(prog)) => {
                // Update
                let label = format!("{:02}%", (prog * 100.0) as usize);
                self.attr(
                    Attribute::Value,
                    AttrValue::Payload(PropPayload::Single(PropValue::F64(*prog))),
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
    component: LineGauge,
}

impl Default for GaugeBeta {
    fn default() -> Self {
        Self {
            component: LineGauge::default()
                .borders(
                    Borders::default()
                        .color(Color::Blue)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::Blue)
                .label("0%")
                .title(Title::from("Loading...").alignment(HorizontalAlignment::Center))
                .line_style(
                    Span::styled(HORIZONTAL, Style::new().fg(Color::Red)),
                    Span::styled(HORIZONTAL, Style::new().fg(Color::Gray)),
                )
                .progress(0.0),
        }
    }
}

impl Component<Msg, UserEvent> for GaugeBeta {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::User(UserEvent::Loaded(prog)) => {
                // Update
                let label = format!("{:02}%", (prog * 100.0) as usize);
                self.attr(
                    Attribute::Value,
                    AttrValue::Payload(PropPayload::Single(PropValue::F64(*prog))),
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
