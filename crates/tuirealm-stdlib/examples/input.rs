//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::components::Input;
use tuirealm::application::PollStrategy;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{
    AttrValue, Attribute, BorderType, Borders, Color, HorizontalAlignment, InputType, Style, Title,
};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::state::{State, StateValue};
use tuirealm::terminal::TerminalAdapter;

mod utils;
use utils::Model;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    ColorBlur,
    EmailBlur,
    NumberBlur,
    PasswordBlur,
    PhoneBlur,
    TextBlur,
    Redraw,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Color,
    Email,
    Number,
    Password,
    Phone,
    Text,
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
                            Constraint::Length(3), // Text
                            Constraint::Length(3), // Email
                            Constraint::Length(3), // Number
                            Constraint::Length(3), // Password
                            Constraint::Length(3), // Phone
                            Constraint::Length(3), // Color
                            Constraint::Length(1),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());
                self.app.view(&Id::Text, f, chunks[0]);
                self.app.view(&Id::Email, f, chunks[1]);
                self.app.view(&Id::Number, f, chunks[2]);
                self.app.view(&Id::Password, f, chunks[3]);
                self.app.view(&Id::Phone, f, chunks[4]);
                self.app.view(&Id::Color, f, chunks[5]);
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
            Msg::TextBlur => {
                assert!(self.app.active(&Id::Email).is_ok());
            }
            Msg::EmailBlur => {
                assert!(self.app.active(&Id::Number).is_ok());
            }
            Msg::NumberBlur => {
                assert!(self.app.active(&Id::Password).is_ok());
            }
            Msg::PasswordBlur => {
                assert!(self.app.active(&Id::Phone).is_ok());
            }
            Msg::PhoneBlur => {
                assert!(self.app.active(&Id::Color).is_ok());
            }
            Msg::ColorBlur => {
                assert!(self.app.active(&Id::Text).is_ok());
            }
            Msg::Redraw => (),
        }
    }

    /// Mount all main components for initial app stage.
    fn mount_main(&mut self) -> Result<(), Box<dyn Error>> {
        self.app
            .mount(Id::Text, Box::new(InputText::default()), vec![])?;
        self.app
            .mount(Id::Email, Box::new(InputEmail::default()), vec![])?;
        self.app
            .mount(Id::Number, Box::new(InputNumber::default()), vec![])?;
        self.app
            .mount(Id::Password, Box::new(InputPassword::default()), vec![])?;
        self.app
            .mount(Id::Phone, Box::new(InputPhone::default()), vec![])?;
        self.app
            .mount(Id::Color, Box::new(InputColor::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::Text)?;

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

// -- components

#[derive(Component)]
struct InputText {
    component: Input,
}

impl Default for InputText {
    fn default() -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightYellow),
                )
                .foreground(Color::LightYellow)
                .input_type(InputType::Text)
                .title(Title::from("Username").alignment(HorizontalAlignment::Left))
                .value("veeso")
                .invalid_style(Style::default().fg(Color::Red)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputText {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Left, ..
            } => self.perform(Cmd::Move(Direction::Left)),
            KeyEvent {
                code: Key::Right, ..
            } => self.perform(Cmd::Move(Direction::Right)),
            KeyEvent {
                code: Key::Home, ..
            } => self.perform(Cmd::GoTo(Position::Begin)),
            KeyEvent { code: Key::End, .. } => self.perform(Cmd::GoTo(Position::End)),
            KeyEvent {
                code: Key::Delete, ..
            } => self.perform(Cmd::Cancel),
            KeyEvent {
                code: Key::Backspace,
                ..
            } => self.perform(Cmd::Delete),
            KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Type(*ch)),
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::TextBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}

#[derive(Component)]
struct InputEmail {
    component: Input,
}

impl Default for InputEmail {
    fn default() -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightCyan),
                )
                .foreground(Color::LightCyan)
                .input_type(InputType::Email)
                .title(Title::from("Email").alignment(HorizontalAlignment::Left))
                .placeholder(
                    "test@example.com",
                    Style::default().fg(Color::Rgb(120, 120, 120)),
                )
                .invalid_style(Style::default().fg(Color::Red)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputEmail {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Left, ..
            } => self.perform(Cmd::Move(Direction::Left)),
            KeyEvent {
                code: Key::Right, ..
            } => self.perform(Cmd::Move(Direction::Right)),
            KeyEvent {
                code: Key::Home, ..
            } => self.perform(Cmd::GoTo(Position::Begin)),
            KeyEvent { code: Key::End, .. } => self.perform(Cmd::GoTo(Position::End)),
            KeyEvent {
                code: Key::Delete, ..
            } => self.perform(Cmd::Cancel),
            KeyEvent {
                code: Key::Backspace,
                ..
            } => self.perform(Cmd::Delete),
            KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Type(*ch)),
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::EmailBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}

#[derive(Component)]
struct InputNumber {
    component: Input,
}

impl Default for InputNumber {
    fn default() -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightGreen),
                )
                .foreground(Color::LightGreen)
                .input_type(InputType::UnsignedInteger)
                .input_len(2)
                .title(Title::from("What's your age").alignment(HorizontalAlignment::Left))
                .invalid_style(Style::default().fg(Color::Red)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputNumber {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Left, ..
            } => self.perform(Cmd::Move(Direction::Left)),
            KeyEvent {
                code: Key::Right, ..
            } => self.perform(Cmd::Move(Direction::Right)),
            KeyEvent {
                code: Key::Home, ..
            } => self.perform(Cmd::GoTo(Position::Begin)),
            KeyEvent { code: Key::End, .. } => self.perform(Cmd::GoTo(Position::End)),
            KeyEvent {
                code: Key::Delete, ..
            } => self.perform(Cmd::Cancel),
            KeyEvent {
                code: Key::Backspace,
                ..
            } => self.perform(Cmd::Delete),
            KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Type(*ch)),
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::NumberBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}

#[derive(Component)]
struct InputPassword {
    component: Input,
}

impl Default for InputPassword {
    fn default() -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightMagenta),
                )
                .foreground(Color::LightMagenta)
                .input_type(InputType::Password('●'))
                .title(Title::from("Password").alignment(HorizontalAlignment::Left))
                .invalid_style(Style::default().fg(Color::Red)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputPassword {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Left, ..
            } => self.perform(Cmd::Move(Direction::Left)),
            KeyEvent {
                code: Key::Right, ..
            } => self.perform(Cmd::Move(Direction::Right)),
            KeyEvent {
                code: Key::Home, ..
            } => self.perform(Cmd::GoTo(Position::Begin)),
            KeyEvent { code: Key::End, .. } => self.perform(Cmd::GoTo(Position::End)),
            KeyEvent {
                code: Key::Delete, ..
            } => self.perform(Cmd::Cancel),
            KeyEvent {
                code: Key::Backspace,
                ..
            } => self.perform(Cmd::Delete),
            KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Type(*ch)),
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::PasswordBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}

#[derive(Component)]
struct InputPhone {
    component: Input,
}

impl Default for InputPhone {
    fn default() -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightBlue),
                )
                .foreground(Color::LightBlue)
                .input_type(InputType::Telephone)
                .input_len(14)
                .title(Title::from("Phone number").alignment(HorizontalAlignment::Left))
                .placeholder(
                    "+39366123123",
                    Style::default().fg(Color::Rgb(120, 120, 120)),
                )
                .invalid_style(Style::default().fg(Color::Red)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputPhone {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Left, ..
            } => self.perform(Cmd::Move(Direction::Left)),
            KeyEvent {
                code: Key::Right, ..
            } => self.perform(Cmd::Move(Direction::Right)),
            KeyEvent {
                code: Key::Home, ..
            } => self.perform(Cmd::GoTo(Position::Begin)),
            KeyEvent { code: Key::End, .. } => self.perform(Cmd::GoTo(Position::End)),
            KeyEvent {
                code: Key::Delete, ..
            } => self.perform(Cmd::Cancel),
            KeyEvent {
                code: Key::Backspace,
                ..
            } => self.perform(Cmd::Delete),
            KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Type(*ch)),
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::PhoneBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}

#[derive(Component)]
struct InputColor {
    component: Input,
}

impl Default for InputColor {
    fn default() -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::White),
                )
                .foreground(Color::White)
                .input_type(InputType::Color)
                .title(
                    Title::from("What's your favourite color").alignment(HorizontalAlignment::Left),
                )
                .invalid_style(Style::default().fg(Color::Red)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InputColor {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Left, ..
            } => self.perform(Cmd::Move(Direction::Left)),
            KeyEvent {
                code: Key::Right, ..
            } => self.perform(Cmd::Move(Direction::Right)),
            KeyEvent {
                code: Key::Home, ..
            } => self.perform(Cmd::GoTo(Position::Begin)),
            KeyEvent { code: Key::End, .. } => self.perform(Cmd::GoTo(Position::End)),
            KeyEvent {
                code: Key::Delete, ..
            } => self.perform(Cmd::Cancel),
            KeyEvent {
                code: Key::Backspace,
                ..
            } => self.perform(Cmd::Delete),
            KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            } => {
                if let CmdResult::Changed(State::Single(StateValue::String(color))) =
                    self.perform(Cmd::Type(*ch))
                {
                    let color = tuirealm::utils::parser::parse_color(&color).unwrap();
                    self.attr(Attribute::Foreground, AttrValue::Color(color));
                    self.attr(
                        Attribute::Borders,
                        AttrValue::Borders(
                            Borders::default()
                                .modifiers(BorderType::Rounded)
                                .color(color),
                        ),
                    );
                }
                CmdResult::NoChange
            }
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::ColorBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}
