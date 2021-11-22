//! ## Demo
//!
//! `Demo` shows how to use tui-realm in a real case

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use std::time::Duration;

use tui_realm_stdlib::Textarea;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextSpan};
use tuirealm::terminal::TerminalBridge;
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update,
};
// tui
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};

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

struct Model {
    quit: bool,   // Becomes true when the user presses <ESC>
    redraw: bool, // Tells whether to refresh the UI; performance optimization
    app: Application<Id, Msg, NoUserEvent>,
}

impl Default for Model {
    fn default() -> Self {
        // Setup app
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default().default_input_listener(Duration::from_millis(10)),
        );
        assert!(app
            .mount(Id::TextareaAlfa, Box::new(TextareaAlfa::default()), vec![])
            .is_ok());
        assert!(app
            .mount(Id::TextareaBeta, Box::new(TextareaBeta::default()), vec![])
            .is_ok());
        // We need to give focus to input then
        assert!(app.active(&Id::TextareaAlfa).is_ok());
        Self {
            quit: false,
            redraw: true,
            app,
        }
    }
}

impl Model {
    fn view(&mut self, terminal: &mut TerminalBridge) {
        let _ = terminal.raw_mut().draw(|f| {
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
                .split(f.size());
            self.app.view(&Id::TextareaAlfa, f, chunks[0]);
            self.app.view(&Id::TextareaBeta, f, chunks[1]);
        });
    }
}

fn main() {
    let mut model = Model::default();
    let mut terminal = TerminalBridge::new().expect("Cannot create terminal bridge");
    let _ = terminal.enable_raw_mode();
    let _ = terminal.enter_alternate_screen();
    // Now we use the Model struct to keep track of some states

    // let's loop until quit is true
    while !model.quit {
        // Tick
        if let Ok(messages) = model.app.tick(PollStrategy::Once) {
            for msg in messages.into_iter() {
                let mut msg = Some(msg);
                while msg.is_some() {
                    msg = model.update(msg);
                }
            }
        }
        // Redraw
        if model.redraw {
            model.view(&mut terminal);
            model.redraw = false;
        }
    }
    // Terminate terminal
    let _ = terminal.leave_alternate_screen();
    let _ = terminal.disable_raw_mode();
    let _ = terminal.clear_screen();
}

impl Update<Msg> for Model {
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
                .background(Color::Black)
                .title("Night Moves (Bob Seger)", Alignment::Center)
                .step(4)
                .highlighted_str("🎵")
                .text_rows(&[
                    TextSpan::new("I was a little too tall, could've used a few pounds,")
                        .underlined()
                        .fg(Color::Green),
                    TextSpan::from("Tight pants points, hardly renowned"),
                    TextSpan::from("She was a black-haired beauty with big dark eyes"),
                    TextSpan::from("And points of her own, sittin' way up high"),
                    TextSpan::from("Way up firm and high"),
                    TextSpan::from("Out past the cornfields where the woods got heavy"),
                    TextSpan::from("Out in the back seat of my '60 Chevy"),
                    TextSpan::from("Workin' on mysteries without any clues"),
                    TextSpan::from("Workin' on our night moves"),
                    TextSpan::from("Tryin' to make some front page drive-in news"),
                    TextSpan::from("Workin' on our night moves"),
                    TextSpan::from("In the summertime"),
                    TextSpan::from("Umm, in the sweet summertime"),
                ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for TextareaAlfa {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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
                .background(Color::Black)
                .title("Roxanne (The Police)", Alignment::Center)
                .step(4)
                .highlighted_str("🎵")
                .text_rows(&[
                    TextSpan::new("Roxanne").underlined().fg(Color::Red),
                    TextSpan::from("You don't have to put on the red light"),
                    TextSpan::from("Those days are over"),
                    TextSpan::from("You don't have to sell your body to the night"),
                    TextSpan::from("Roxanne"),
                    TextSpan::from("You don't have to wear that dress tonight"),
                    TextSpan::from("Walk the streets for money"),
                    TextSpan::from("You don't care if it's wrong or if it's right"),
                    TextSpan::from("Roxanne"),
                    TextSpan::from("You don't have to put on the red light"),
                    TextSpan::from("Roxanne"),
                    TextSpan::from("You don't have to put on the red light"),
                ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for TextareaBeta {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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
