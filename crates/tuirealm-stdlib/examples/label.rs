//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::components::Label;
use tuirealm::application::PollStrategy;
use tuirealm::command::CmdResult;
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{Color, HorizontalAlignment, TextModifiers};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};

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
    LabelAlfa,
    LabelBeta,
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
                            Constraint::Length(1), // Label
                            Constraint::Length(1), // Label
                            Constraint::Length(1),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());
                self.app.view(&Id::LabelAlfa, f, chunks[0]);
                self.app.view(&Id::LabelBeta, f, chunks[1]);
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
        self.app
            .mount(Id::LabelAlfa, Box::new(LabelAlfa::default()), vec![])?;
        self.app
            .mount(Id::LabelBeta, Box::new(LabelBeta::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::LabelAlfa)?;

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
struct LabelAlfa {
    component: Label,
}

impl Default for LabelAlfa {
    fn default() -> Self {
        Self {
            component: Label::default()
                .alignment_horizontal(HorizontalAlignment::Center)
                .foreground(Color::Green)
                .modifiers(TextModifiers::BOLD)
                .text("This is a label"),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for LabelAlfa {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

#[derive(Component)]
struct LabelBeta {
    component: Label,
}

impl Default for LabelBeta {
    fn default() -> Self {
        Self {
            component: Label::default()
                .alignment_horizontal(HorizontalAlignment::Right)
                .foreground(Color::White)
                .background(Color::Blue)
                .modifiers(TextModifiers::ITALIC)
                .text("This is a label"),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for LabelBeta {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
