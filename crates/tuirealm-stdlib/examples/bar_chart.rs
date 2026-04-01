use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::components::BarChart;
use tuirealm::application::PollStrategy;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{BorderType, Borders, Color, HorizontalAlignment, Style, Title};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::terminal::TerminalAdapter;

mod utils;
use utils::Model;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    ChartAlfaBlur,
    ChartBetaBlur,
    Redraw,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    ChartAlfa,
    ChartBeta,
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
                            Constraint::Length(10),
                            Constraint::Length(10),
                            Constraint::Length(1),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());
                self.app.view(&Id::ChartAlfa, f, chunks[0]);
                self.app.view(&Id::ChartBeta, f, chunks[1]);
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
            Msg::ChartAlfaBlur => {
                assert!(self.app.active(&Id::ChartBeta).is_ok());
            }
            Msg::ChartBetaBlur => {
                assert!(self.app.active(&Id::ChartAlfa).is_ok());
            }
            Msg::Redraw => (),
        }
    }

    /// Mount all main components for initial app stage.
    fn mount_main(&mut self) -> Result<(), Box<dyn Error>> {
        self.app
            .mount(Id::ChartAlfa, Box::new(ChartAlfa::default()), vec![])?;
        self.app
            .mount(Id::ChartBeta, Box::new(ChartBeta::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::ChartAlfa)?;

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
struct ChartAlfa {
    component: BarChart,
}

impl Default for ChartAlfa {
    fn default() -> Self {
        Self {
            component: BarChart::default()
                .title(Title::from("my incomes").alignment(HorizontalAlignment::Center))
                .label_style(Style::default().fg(Color::Yellow))
                .bar_style(Style::default().fg(Color::LightYellow))
                .bar_gap(6)
                .width(12)
                .borders(Borders::default().color(Color::LightBlue))
                .max_bars(4)
                .value_style(Style::default().fg(Color::LightBlue))
                .inactive(Style::new().fg(Color::Gray))
                .data(&[
                    ("january", 250),
                    ("february", 300),
                    ("march", 275),
                    ("april", 312),
                    ("may", 420),
                    ("june", 170),
                    ("july", 220),
                    ("august", 160),
                    ("september", 180),
                    ("october", 470),
                    ("november", 380),
                    ("december", 820),
                ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for ChartAlfa {
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
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::ChartAlfaBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}

// -- chart 2

#[derive(Component)]
struct ChartBeta {
    component: BarChart,
}

impl Default for ChartBeta {
    fn default() -> Self {
        Self {
            component: BarChart::default()
                .title(Title::from("my incomes").alignment(HorizontalAlignment::Left))
                .label_style(Style::default().fg(Color::Yellow))
                .bar_style(Style::default().fg(Color::LightYellow))
                .bar_gap(6)
                .width(12)
                .borders(
                    Borders::default()
                        .color(Color::LightBlue)
                        .modifiers(BorderType::Double),
                )
                .max_bars(12)
                .value_style(Style::default().fg(Color::LightBlue))
                .inactive(Style::new().fg(Color::Gray))
                .data(&[
                    ("january", 250),
                    ("february", 300),
                    ("march", 275),
                    ("april", 312),
                    ("may", 420),
                    ("june", 170),
                    ("july", 220),
                    ("august", 160),
                    ("september", 180),
                    ("october", 470),
                    ("november", 380),
                    ("december", 820),
                ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for ChartBeta {
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
            KeyEvent { code: Key::Tab, .. } => return Some(Msg::ChartBetaBlur),
            KeyEvent { code: Key::Esc, .. } => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}
