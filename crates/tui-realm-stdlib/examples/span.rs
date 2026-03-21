//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::Span;
use tuirealm::application::PollStrategy;
use tuirealm::command::CmdResult;
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Color, HorizontalAlignment, TextModifiers};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::ratatui::style::Stylize;
use tuirealm::ratatui::text::Span as RSpan;
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

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
    SpanAlfa,
    SpanBeta,
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
                            Constraint::Length(1),
                            Constraint::Length(1),
                            Constraint::Length(1),
                        ]
                        .as_ref(),
                    )
                    .split(f.area());
                self.app.view(&Id::SpanAlfa, f, chunks[0]);
                self.app.view(&Id::SpanBeta, f, chunks[1]);
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
            .mount(Id::SpanAlfa, Box::new(SpanAlfa::default()), vec![])?;
        self.app
            .mount(Id::SpanBeta, Box::new(SpanBeta::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::SpanAlfa)?;

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
struct SpanAlfa {
    component: Span,
}

impl Default for SpanAlfa {
    fn default() -> Self {
        Self {
            component: Span::default()
                .foreground(Color::Yellow)
                .spans([
                    RSpan::raw("Lorem ipsum dolor sit amet,").underlined().fg(Color::Green),
                    RSpan::from("consectetur adipiscing elit. Praesent mauris est, vehicula et imperdiet sed, tincidunt sed est."),
                ])
        }
    }
}

impl Component<Msg, NoUserEvent> for SpanAlfa {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

#[derive(MockComponent)]
struct SpanBeta {
    component: Span,
}

impl Default for SpanBeta {
    fn default() -> Self {
        Self {
            component: Span::default()
                .foreground(Color::Black)
                .background(Color::White)
                .alignment_horizontal(HorizontalAlignment::Right)
                .modifiers(TextModifiers::BOLD)
                .spans([
                    RSpan::raw("Lorem ipsum dolor sit amet,").underlined().fg(Color::Green).bg(Color::Red),
                    RSpan::from("consectetur adipiscing elit. Praesent mauris est, vehicula et imperdiet sed, tincidunt sed est."),
                ])
        }
    }
}

impl Component<Msg, NoUserEvent> for SpanBeta {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
