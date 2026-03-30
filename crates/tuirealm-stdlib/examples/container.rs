//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::components::{Container, Table};
use tuirealm::application::PollStrategy;
use tuirealm::command::CmdResult;
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{
    BorderType, Borders, Color, HorizontalAlignment, Layout, TableBuilder, Title,
};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout as TuiLayout};
use tuirealm::ratatui::text::Line;
use tuirealm::terminal::TerminalAdapter;

mod utils;
use utils::Model;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    Redraw,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Container,
}

impl Model<Id, Msg> {
    /// Draw all components.
    fn view(&mut self) {
        self.terminal
            .draw(|f| {
                // Prepare chunks
                let chunks = TuiLayout::default()
                    .direction(LayoutDirection::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.area());
                self.app.view(&Id::Container, f, chunks[0]);
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
        self.app
            .mount(Id::Container, Box::new(MyContainer::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::Container)?;

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
struct MyContainer {
    component: Container,
}

impl Default for MyContainer {
    fn default() -> Self {
        Self {
            component: Container::default()
                .background(Color::Yellow)
                .foreground(Color::Yellow)
                .title(
                    Title::from("This is a div with two tables")
                        .alignment(HorizontalAlignment::Left),
                )
                .layout(
                    Layout::default()
                        .constraints(&[Constraint::Percentage(30), Constraint::Percentage(70)])
                        .direction(LayoutDirection::Horizontal)
                        .margin(1),
                )
                .children(vec![
                    Box::new(
                        Table::default()
                            .borders(
                                Borders::default()
                                    .modifiers(BorderType::Thick)
                                    .color(Color::Yellow),
                            )
                            .foreground(Color::Yellow)
                            .background(Color::Black)
                            .title(
                                Title::from("Keybindings").alignment(HorizontalAlignment::Center),
                            )
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
                    ),
                    Box::new(
                        Table::default()
                            .borders(
                                Borders::default()
                                    .modifiers(BorderType::Rounded)
                                    .color(Color::Green),
                            )
                            .foreground(Color::Green)
                            .background(Color::Black)
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
                    ),
                ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for MyContainer {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::NoChange,
        };
        Some(Msg::Redraw)
    }
}
