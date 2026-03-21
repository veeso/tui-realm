//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::Textarea;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{BorderType, Borders, Color, HorizontalAlignment, Title};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::ratatui::style::Stylize;
use tuirealm::ratatui::text::Span;
use tuirealm::{
    Component, Event, MockComponent, NoUserEvent,
    application::PollStrategy,
    event::{Key, KeyEvent},
};

mod utils;
use utils::Model;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    TextareaAlfaBlur,
    TextareaBetaBlur,
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    TextareaAlfa,
    TextareaBeta,
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
                            Constraint::Length(12),
                            Constraint::Length(12),
                            Constraint::Length(1),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());
                self.app.view(&Id::TextareaAlfa, f, chunks[0]);
                self.app.view(&Id::TextareaBeta, f, chunks[1]);
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
            Msg::TextareaAlfaBlur => {
                assert!(self.app.active(&Id::TextareaBeta).is_ok());
                None
            }
            Msg::TextareaBetaBlur => {
                assert!(self.app.active(&Id::TextareaAlfa).is_ok());
                None
            }
            Msg::None => None,
        }
    }

    /// Mount all main components for initial app stage.
    fn mount_main(&mut self) -> Result<(), Box<dyn Error>> {
        self.app
            .mount(Id::TextareaAlfa, Box::new(TextareaAlfa::default()), vec![])?;
        self.app
            .mount(Id::TextareaBeta, Box::new(TextareaBeta::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::TextareaAlfa)?;

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
struct TextareaAlfa {
    component: Textarea,
}

impl Default for TextareaAlfa {
    fn default() -> Self {
        Self {
            component: Textarea::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Yellow),
                )
                .foreground(Color::Yellow)
                .title(
                    Title::from("Night Moves (Bob Seger)").alignment(HorizontalAlignment::Center),
                )
                .step(4)
                .highlighted_str("🎵")
                .text_rows([
                    Span::raw("I was a little too tall, could've used a few pounds,")
                        .underlined()
                        .fg(Color::Green),
                    Span::from("Tight pants points, hardly renowned"),
                    Span::from("She was a black-haired beauty with big dark eyes"),
                    Span::from("And points of her own, sittin' way up high"),
                    Span::from("Way up firm and high"),
                    Span::from("Out past the cornfields where the woods got heavy"),
                    Span::from("Out in the back seat of my '60 Chevy"),
                    Span::from("Workin' on mysteries without any clues"),
                    Span::from("Workin' on our night moves"),
                    Span::from("Tryin' to make some front page drive-in news"),
                    Span::from("Workin' on our night moves"),
                    Span::from("In the summertime"),
                    Span::from("Umm, in the sweet summertime"),
                ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for TextareaAlfa {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::TextareaAlfaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

#[derive(MockComponent)]
struct TextareaBeta {
    component: Textarea,
}

impl Default for TextareaBeta {
    fn default() -> Self {
        Self {
            component: Textarea::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightBlue),
                )
                .foreground(Color::LightBlue)
                .title(Title::from("Roxanne (The Police)").alignment(HorizontalAlignment::Center))
                .step(4)
                .highlighted_str("🎵")
                .text_rows([
                    Span::raw("Roxanne").underlined().fg(Color::Red),
                    Span::from("You don't have to put on the red light"),
                    Span::from("Those days are over"),
                    Span::from("You don't have to sell your body to the night"),
                    Span::from("Roxanne"),
                    Span::from("You don't have to wear that dress tonight"),
                    Span::from("Walk the streets for money"),
                    Span::from("You don't care if it's wrong or if it's right"),
                    Span::from("Roxanne"),
                    Span::from("You don't have to put on the red light"),
                    Span::from("Roxanne"),
                    Span::from("You don't have to put on the red light"),
                ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for TextareaBeta {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::TextareaBetaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
