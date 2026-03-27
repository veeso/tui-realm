//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::components::Table;
use tuirealm::application::PollStrategy;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{BorderType, Borders, Color, HorizontalAlignment, TableBuilder, Title};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::ratatui::text::Line;
use tuirealm::terminal::TerminalAdapter;

mod utils;
use utils::Model;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    TableAlfaBlur,
    TableBetaBlur,
    Redraw,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    TableAlfa,
    TableBeta,
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
                            Constraint::Length(15),
                            Constraint::Length(6),
                            Constraint::Length(1),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());
                self.app.view(&Id::TableAlfa, f, chunks[0]);
                self.app.view(&Id::TableBeta, f, chunks[1]);
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
            Msg::TableAlfaBlur => {
                assert!(self.app.active(&Id::TableBeta).is_ok());
            }
            Msg::TableBetaBlur => {
                assert!(self.app.active(&Id::TableAlfa).is_ok());
            }
            Msg::Redraw => (),
        }
    }

    /// Mount all main components for initial app stage.
    fn mount_main(&mut self) -> Result<(), Box<dyn Error>> {
        self.app
            .mount(Id::TableAlfa, Box::new(TableAlfa::default()), vec![])?;
        self.app
            .mount(Id::TableBeta, Box::new(TableBeta::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::TableAlfa)?;

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
struct TableAlfa {
    component: Table,
}

impl Default for TableAlfa {
    fn default() -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Thick)
                        .color(Color::Yellow),
                )
                .foreground(Color::Yellow)
                .background(Color::Black)
                .title(Title::from("Keybindings").alignment(HorizontalAlignment::Center))
                .scroll(true)
                .highlighted_color(Color::LightYellow)
                .highlighted_str("🚀")
                .rewind(true)
                .step(4)
                .row_height(1)
                .headers(["Key", "Msg", "Description"])
                .column_spacing(3)
                .widths(&[30, 20, 50])
                .table(
                    TableBuilder::default()
                        .add_col(Line::from("KeyCode::Down"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("Move cursor down"))
                        .add_row()
                        .add_col(Line::from("KeyCode::Up"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("Move cursor up"))
                        .add_row()
                        .add_col(Line::from("KeyCode::PageDown"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("Move cursor down by 8"))
                        .add_row()
                        .add_col(Line::from("KeyCode::PageUp"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("ove cursor up by 8"))
                        .add_row()
                        .add_col(Line::from("KeyCode::End"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("Move cursor to last item"))
                        .add_row()
                        .add_col(Line::from("KeyCode::Home"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("Move cursor to first item"))
                        .add_row()
                        .add_col(Line::from("KeyCode::Char(_)"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("Return pressed key"))
                        .build(),
                ),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for TableAlfa {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
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
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::TableAlfaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::Redraw)
    }
}

#[derive(Component)]
struct TableBeta {
    component: Table,
}

impl Default for TableBeta {
    fn default() -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Green),
                )
                .foreground(Color::Green)
                .background(Color::Gray)
                .title(
                    Title::from("Keybindings (not scrollable)")
                        .alignment(HorizontalAlignment::Center),
                )
                .scroll(false)
                .highlighted_color(Color::Green)
                .highlighted_str(">> ")
                .row_height(1)
                .headers(["Key", "Msg", "Description"])
                .column_spacing(3)
                .widths(&[30, 20, 50])
                .table(
                    TableBuilder::default()
                        .add_col(Line::from("KeyCode::Down"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("Move cursor down"))
                        .add_row()
                        .add_col(Line::from("KeyCode::Up"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("Move cursor up"))
                        .add_row()
                        .add_col(Line::from("KeyCode::PageDown"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("Move cursor down by 8"))
                        .add_row()
                        .add_col(Line::from("KeyCode::PageUp"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("Move cursor up by 8"))
                        .add_row()
                        .add_col(Line::from("KeyCode::End"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("Move cursor to last item"))
                        .add_row()
                        .add_col(Line::from("KeyCode::Home"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("Move cursor to first item"))
                        .add_row()
                        .add_col(Line::from("KeyCode::Char(_)"))
                        .add_col(Line::from("OnKey"))
                        .add_col(Line::from("Return pressed key"))
                        .build(),
                ),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for TableBeta {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::TableBetaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::Redraw)
    }
}
