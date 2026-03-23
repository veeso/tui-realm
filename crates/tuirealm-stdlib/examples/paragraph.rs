//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::components::Paragraph;
use tuirealm::application::PollStrategy;
use tuirealm::command::CmdResult;
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{BorderType, Borders, Color, HorizontalAlignment, Title};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::ratatui::style::Stylize;
use tuirealm::ratatui::text::Line;

mod utils;
use utils::Model;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    ParagraphAlfa,
    ParagraphBeta,
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
                            Constraint::Length(6),
                            Constraint::Length(6),
                            Constraint::Length(1),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());
                self.app.view(&Id::ParagraphAlfa, f, chunks[0]);
                self.app.view(&Id::ParagraphBeta, f, chunks[1]);
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
            Msg::None => None,
        }
    }

    /// Mount all main components for initial app stage.
    fn mount_main(&mut self) -> Result<(), Box<dyn Error>> {
        self.app.mount(
            Id::ParagraphAlfa,
            Box::new(ParagraphAlfa::default()),
            vec![],
        )?;
        self.app.mount(
            Id::ParagraphBeta,
            Box::new(ParagraphBeta::default()),
            vec![],
        )?;
        // We need to give focus to input then
        self.app.active(&Id::ParagraphAlfa)?;

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

#[derive(Component)]
struct ParagraphAlfa {
    component: Paragraph,
}

impl Default for ParagraphAlfa {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Yellow),
                )
                .foreground(Color::Yellow)
                .background(Color::Black)
                .title(Title::from("Lorem ipsum (wrap)").alignment(HorizontalAlignment::Center))
                .wrap_trim(true)
                .text(vec![
                    Line::raw("Lorem ipsum dolor sit amet,").underlined().fg(Color::Green),
                    Line::from("consectetur adipiscing elit. Praesent mauris est, vehicula et imperdiet sed, tincidunt sed est. Sed sed dui odio. Etiam nunc neque, sodales ut ex nec, tincidunt malesuada eros. Sed quis eros non felis sodales accumsan in ac risus"),
                    Line::from("                       Duis augue diam, tempor vitae posuere et, tempus mattis ligula.")
                ])
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for ParagraphAlfa {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

#[derive(Component)]
struct ParagraphBeta {
    component: Paragraph,
}

impl Default for ParagraphBeta {
    fn default() -> Self {
        Self {
            component: Paragraph::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Cyan),
                )
                .foreground(Color::Cyan)
                .background(Color::Black)
                .title(Title::from("Lorem ipsum (no wrap)").alignment(HorizontalAlignment::Center))
                .wrap_trim(false)
                .text(vec![
                    Line::raw("Lorem ipsum dolor sit amet,").underlined().fg(Color::Green),
                    Line::from("consectetur adipiscing elit. Praesent mauris est, vehicula et imperdiet sed, tincidunt sed est. Sed sed dui odio. Etiam nunc neque, sodales ut ex nec, tincidunt malesuada eros. Sed quis eros non felis sodales accumsan in ac risus"),
                    Line::from("                                        Duis augue diam, tempor vitae posuere et, tempus mattis ligula.")
                ])
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for ParagraphBeta {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
