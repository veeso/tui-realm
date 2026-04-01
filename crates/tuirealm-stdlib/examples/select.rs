//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::components::Select;
use tuirealm::application::PollStrategy;
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{
    BorderType, Borders, Color, HorizontalAlignment, Style, TextModifiers, Title,
};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::state::State;
use tuirealm::terminal::TerminalAdapter;

mod utils;
use utils::Model;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    SelectAlfaBlur,
    SelectBetaBlur,
    Redraw,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    SelectAlfa,
    SelectBeta,
}

impl Model<Id, Msg> {
    /// Draw all components.
    fn view(&mut self) {
        // Calc len
        let select_alfa_len = match self.app.state(&Id::SelectAlfa) {
            Ok(State::Single(_)) => 3,
            _ => 8,
        };
        let select_beta_len = match self.app.state(&Id::SelectBeta) {
            Ok(State::Single(_)) => 3,
            _ => 8,
        };
        self.terminal
            .draw(|f| {
                // Prepare chunks
                let chunks = Layout::default()
                    .direction(LayoutDirection::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(select_alfa_len),
                            Constraint::Length(select_beta_len),
                            Constraint::Length(1),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());
                self.app.view(&Id::SelectAlfa, f, chunks[0]);
                self.app.view(&Id::SelectBeta, f, chunks[1]);
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
            Msg::SelectAlfaBlur => {
                assert!(self.app.active(&Id::SelectBeta).is_ok());
            }
            Msg::SelectBetaBlur => {
                assert!(self.app.active(&Id::SelectAlfa).is_ok());
            }
            Msg::Redraw => (),
        }
    }

    /// Mount all main components for initial app stage.
    fn mount_main(&mut self) -> Result<(), Box<dyn Error>> {
        self.app
            .mount(Id::SelectAlfa, Box::new(SelectAlfa::default()), vec![])?;
        self.app
            .mount(Id::SelectBeta, Box::new(SelectBeta::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::SelectAlfa)?;

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
struct SelectAlfa {
    component: Select,
}

impl Default for SelectAlfa {
    fn default() -> Self {
        Self {
            component: Select::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightGreen),
                )
                .foreground(Color::LightGreen)
                .inactive(Style::new().fg(Color::Gray))
                .title(
                    Title::from("Select your ice cream flavour 🍦")
                        .alignment(HorizontalAlignment::Center),
                )
                .rewind(true)
                // No need to set highlight style as the default is "REVERSED"
                // .highlight_style(Style::new().add_modifier(TextModifiers::REVERSED))
                .highlight_str(">> ")
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

impl AppComponent<Msg, NoUserEvent> for SelectAlfa {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Down, ..
            } => self.perform(Cmd::Move(Direction::Down)),
            KeyEvent { code: Key::Up, .. } => self.perform(Cmd::Move(Direction::Up)),
            KeyEvent {
                code: Key::Enter, ..
            } => self.perform(Cmd::Submit),
            KeyEvent {
                code: Key::Delete | Key::Backspace,
                ..
            } => self.perform(Cmd::Cancel),
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::SelectAlfaBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}

#[derive(Component)]
struct SelectBeta {
    component: Select,
}

impl Default for SelectBeta {
    fn default() -> Self {
        Self {
            component: Select::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightYellow),
                )
                .foreground(Color::LightYellow)
                .inactive(Style::new().fg(Color::Gray))
                .title(Title::from("Select your topping 🧁").alignment(HorizontalAlignment::Center))
                .rewind(false)
                // Overwrite the default "REVERSED" style to be underlined instead
                .highlight_style(Style::new().add_modifier(TextModifiers::UNDERLINED))
                .highlight_str(">> ")
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

impl AppComponent<Msg, NoUserEvent> for SelectBeta {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Down, ..
            } => self.perform(Cmd::Move(Direction::Down)),
            KeyEvent { code: Key::Up, .. } => self.perform(Cmd::Move(Direction::Up)),
            KeyEvent {
                code: Key::Enter, ..
            } => self.perform(Cmd::Submit),
            KeyEvent {
                code: Key::Delete | Key::Backspace,
                ..
            } => self.perform(Cmd::Cancel),
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::SelectBetaBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}
