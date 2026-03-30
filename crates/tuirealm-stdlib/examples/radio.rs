//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::components::Radio;
use tuirealm::application::PollStrategy;
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{BorderType, Borders, Color, HorizontalAlignment, Title};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::terminal::TerminalAdapter;

mod utils;
use utils::Model;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    RadioAlfaBlur,
    RadioBetaBlur,
    Redraw,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    RadioAlfa,
    RadioBeta,
    RadioCeta,
}

impl Model<Id, Msg> {
    /// Draw all components.
    fn view(&mut self) {
        self.terminal
            .draw(|f| {
                // Prepare chunks
                let chunks = Layout::default()
                    .direction(LayoutDirection::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(1),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());
                self.app.view(&Id::RadioAlfa, f, chunks[0]);
                self.app.view(&Id::RadioBeta, f, chunks[1]);
                self.app.view(&Id::RadioCeta, f, chunks[2]);
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
            Msg::RadioAlfaBlur => {
                assert!(self.app.active(&Id::RadioBeta).is_ok());
            }
            Msg::RadioBetaBlur => {
                assert!(self.app.active(&Id::RadioAlfa).is_ok());
            }
            // Ceta is not focusable
            Msg::Redraw => (),
        }
    }

    /// Mount all main components for initial app stage.
    fn mount_main(&mut self) -> Result<(), Box<dyn Error>> {
        self.app
            .mount(Id::RadioAlfa, Box::new(RadioAlfa::default()), vec![])?;
        self.app
            .mount(Id::RadioBeta, Box::new(RadioBeta::default()), vec![])?;
        self.app
            .mount(Id::RadioCeta, Box::new(RadioCeta::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::RadioAlfa)?;

        Ok(())
    }
}

fn main() {
    let mut model = Model::new();
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

#[derive(Component)]
struct RadioAlfa {
    component: Radio,
}

impl Default for RadioAlfa {
    fn default() -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightGreen),
                )
                .foreground(Color::LightGreen)
                .title(
                    Title::from("Select your ice cream flavour 🍦")
                        .alignment(HorizontalAlignment::Center),
                )
                .rewind(true)
                .choices([
                    "vanilla",
                    "chocolate",
                    "coconut",
                    "mint",
                    "strawberry",
                    "lemon",
                    "hazelnut",
                    "coffee",
                ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for RadioAlfa {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Left, ..
            } => self.perform(Cmd::Move(Direction::Left)),
            KeyEvent {
                code: Key::Right, ..
            } => self.perform(Cmd::Move(Direction::Right)),
            KeyEvent {
                code: Key::Enter, ..
            } => self.perform(Cmd::Submit),
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::RadioAlfaBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}

#[derive(Component)]
struct RadioBeta {
    component: Radio,
}

impl Default for RadioBeta {
    fn default() -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightYellow),
                )
                .foreground(Color::LightYellow)
                .title(Title::from("Select your topping 🧁").alignment(HorizontalAlignment::Center))
                .rewind(false)
                .choices([
                    "hazelnuts",
                    "chocolate",
                    "maple cyrup",
                    "smarties",
                    "raspberries",
                ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for RadioBeta {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Left, ..
            } => self.perform(Cmd::Move(Direction::Left)),
            KeyEvent {
                code: Key::Right, ..
            } => self.perform(Cmd::Move(Direction::Right)),
            KeyEvent {
                code: Key::Enter, ..
            } => self.perform(Cmd::Submit),
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::RadioBetaBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}

#[derive(Component)]
struct RadioCeta {
    component: Radio,
}

impl Default for RadioCeta {
    fn default() -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightYellow),
                )
                .foreground(Color::LightYellow)
                .title(Title::from("Choice of the day").alignment(HorizontalAlignment::Center))
                .rewind(false)
                .choices([
                    "hazelnuts",
                    "chocolate",
                    "maple cyrup",
                    "smarties",
                    "raspberries",
                ])
                .value(3)
                .always_active(),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for RadioCeta {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
