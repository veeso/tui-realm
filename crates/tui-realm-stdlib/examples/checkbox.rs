//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::components::Checkbox;
use tuirealm::MockComponent;
use tuirealm::application::PollStrategy;
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::component::{Component, MockComponent};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{BorderType, Borders, Color, HorizontalAlignment, Title};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};

mod utils;
use utils::Model;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    CheckboxAlfaBlur,
    CheckboxBetaBlur,
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    CheckboxAlfa,
    CheckboxBeta,
}

impl Model<Id, Msg> {
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
                self.app.view(&Id::CheckboxAlfa, f, chunks[0]);
                self.app.view(&Id::CheckboxBeta, f, chunks[1]);
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
            Msg::CheckboxAlfaBlur => {
                assert!(self.app.active(&Id::CheckboxBeta).is_ok());
                None
            }
            Msg::CheckboxBetaBlur => {
                assert!(self.app.active(&Id::CheckboxAlfa).is_ok());
                None
            }
            Msg::None => None,
        }
    }

    /// Mount all main components for initial app stage.
    fn mount_main(&mut self) -> Result<(), Box<dyn Error>> {
        self.app
            .mount(Id::CheckboxAlfa, Box::new(CheckboxAlfa::default()), vec![])?;
        self.app
            .mount(Id::CheckboxBeta, Box::new(CheckboxBeta::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::CheckboxAlfa)?;

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

#[derive(MockComponent)]
struct CheckboxAlfa {
    component: Checkbox,
}

impl Default for CheckboxAlfa {
    fn default() -> Self {
        Self {
            component: Checkbox::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightGreen),
                )
                .foreground(Color::LightGreen)
                .background(Color::Black)
                .title(
                    Title::from("Select your ice cream flavours 🍦")
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

impl Component<Msg, NoUserEvent> for CheckboxAlfa {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => self.perform(Cmd::Submit),
            Event::Keyboard(KeyEvent {
                code: Key::Char(' '),
                ..
            }) => self.perform(Cmd::Toggle),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::CheckboxAlfaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

#[derive(MockComponent)]
struct CheckboxBeta {
    component: Checkbox,
}

impl Default for CheckboxBeta {
    fn default() -> Self {
        Self {
            component: Checkbox::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightYellow),
                )
                .foreground(Color::LightYellow)
                .background(Color::Black)
                .title(
                    Title::from("Select your toppings 🧁").alignment(HorizontalAlignment::Center),
                )
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

impl Component<Msg, NoUserEvent> for CheckboxBeta {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => self.perform(Cmd::Submit),
            Event::Keyboard(KeyEvent {
                code: Key::Char(' '),
                ..
            }) => self.perform(Cmd::Toggle),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::CheckboxBetaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
