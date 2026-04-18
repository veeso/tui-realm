//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::components::Textarea;
use tuirealm::application::PollStrategy;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{BorderType, Borders, Color, HorizontalAlignment, Style, Title};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::ratatui::text::{Line, Span};
use tuirealm::terminal::TerminalAdapter;

mod utils;
use utils::Model;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    TextareaAlfaBlur,
    TextareaBetaBlur,
    Redraw,
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
    fn update(&mut self, msg: Msg) {
        self.redraw = true;
        match msg {
            Msg::AppClose => {
                self.quit = true;
            }
            Msg::TextareaAlfaBlur => {
                assert!(self.app.active(&Id::TextareaBeta).is_ok());
            }
            Msg::TextareaBetaBlur => {
                assert!(self.app.active(&Id::TextareaAlfa).is_ok());
            }
            Msg::Redraw => (),
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
                .inactive(Style::new().fg(Color::Gray))
                .title(
                    Title::from("Night Moves (Bob Seger)").alignment(HorizontalAlignment::Center),
                )
                .step(4)
                .highlight_str("🎵")
                .text_rows([
                    Line::from(Span::styled(
                        "I was a little too tall, could've used a few pounds,",
                        Style::new().underlined().fg(Color::Green),
                    )),
                    Line::from_iter([
                        Span::raw("Tight"),
                        Span::styled(" pants ", Style::new().italic()),
                        Span::styled("points,", Style::new().crossed_out()),
                        Span::raw(" hardly renowned"),
                    ]),
                    Line::from("She was a black-haired beauty with big dark eyes"),
                    Line::from("And points of her own, sittin' way up high"),
                    Line::from("Way up firm and high"),
                    Line::from("Out past the cornfields where the woods got heavy"),
                    Line::from("Out in the back seat of my '60 Chevy"),
                    Line::from("Workin' on mysteries without any clues"),
                    Line::from("Workin' on our night moves"),
                    Line::from("Tryin' to make some front page drive-in news"),
                    Line::from("Workin' on our night moves"),
                    Line::from("In the summertime"),
                    Line::from("Umm, in the sweet summertime"),
                ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for TextareaAlfa {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Down, ..
            } => self.perform(Cmd::Move(Direction::Down)),
            KeyEvent { code: Key::Up, .. } => self.perform(Cmd::Move(Direction::Up)),
            KeyEvent {
                code: Key::PageDown,
                ..
            } => self.perform(Cmd::Scroll(Direction::Down)),
            KeyEvent {
                code: Key::PageUp, ..
            } => self.perform(Cmd::Scroll(Direction::Up)),
            KeyEvent {
                code: Key::Home, ..
            } => self.perform(Cmd::GoTo(Position::Begin)),
            KeyEvent { code: Key::End, .. } => self.perform(Cmd::GoTo(Position::End)),
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::TextareaAlfaBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}

#[derive(Component)]
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
                .inactive(Style::new().fg(Color::Gray))
                .title(Title::from("Roxanne (The Police)").alignment(HorizontalAlignment::Center))
                .step(4)
                .highlight_str("🎵")
                .text_rows([
                    Line::from(Span::styled(
                        "Roxanne",
                        Style::new().underlined().fg(Color::Red),
                    )),
                    Line::from("You don't have to put on the red light"),
                    Line::from("Those days are over"),
                    Line::from("You don't have to sell your body to the night"),
                    Line::from("Roxanne"),
                    Line::from("You don't have to wear that dress tonight"),
                    Line::from("Walk the streets for money"),
                    Line::from("You don't care if it's wrong or if it's right"),
                    Line::from("Roxanne"),
                    Line::from("You don't have to put on the red light"),
                    Line::from("Roxanne"),
                    Line::from("You don't have to put on the red light"),
                ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for TextareaBeta {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Down, ..
            } => self.perform(Cmd::Move(Direction::Down)),
            KeyEvent { code: Key::Up, .. } => self.perform(Cmd::Move(Direction::Up)),
            KeyEvent {
                code: Key::PageDown,
                ..
            } => self.perform(Cmd::Scroll(Direction::Down)),
            KeyEvent {
                code: Key::PageUp, ..
            } => self.perform(Cmd::Scroll(Direction::Up)),
            KeyEvent {
                code: Key::Home, ..
            } => self.perform(Cmd::GoTo(Position::Begin)),
            KeyEvent { code: Key::End, .. } => self.perform(Cmd::GoTo(Position::End)),
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::TextareaBetaBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}
