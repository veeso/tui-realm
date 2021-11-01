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

use tui_realm_stdlib::{Span, Spinner};
use tuirealm::command::CmdResult;
use tuirealm::props::{Alignment, Color, TextModifiers, TextSpan};
use tuirealm::terminal::TerminalBridge;
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update, View,
};
// tui
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};

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
    SpinnerAlfa,
    SpinnerBeta,
}

struct Model {
    quit: bool,   // Becomes true when the user presses <ESC>
    redraw: bool, // Tells whether to refresh the UI; performance optimization
    terminal: TerminalBridge,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Cannot create terminal bridge"),
        }
    }
}

impl Model {
    fn view(&mut self, app: &mut Application<Id, Msg, NoUserEvent>) {
        let _ = self.terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            let row1 = Layout::default()
                .direction(LayoutDirection::Horizontal)
                .margin(1)
                .constraints([Constraint::Length(2), Constraint::Min(10)].as_ref())
                .split(chunks[0]);
            app.view(&Id::SpinnerAlfa, f, row1[0]);
            app.view(&Id::SpanAlfa, f, row1[1]);
            let row2 = Layout::default()
                .direction(LayoutDirection::Horizontal)
                .margin(1)
                .constraints([Constraint::Length(2), Constraint::Min(10)].as_ref())
                .split(chunks[1]);
            app.view(&Id::SpinnerBeta, f, row2[0]);
            app.view(&Id::SpanBeta, f, row2[1]);
        });
    }
}

fn main() {
    let mut model = Model::default();
    let _ = model.terminal.enable_raw_mode();
    let _ = model.terminal.enter_alternate_screen();
    // Setup app
    let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
        EventListenerCfg::default().default_input_listener(Duration::from_millis(10)),
    );
    assert!(app
        .mount(Id::SpanAlfa, Box::new(SpanAlfa::default()), vec![])
        .is_ok());
    assert!(app
        .mount(Id::SpanBeta, Box::new(SpanBeta::default()), vec![])
        .is_ok());
    assert!(app
        .mount(Id::SpinnerAlfa, Box::new(SpinnerAlfa::default()), vec![])
        .is_ok());
    assert!(app
        .mount(Id::SpinnerBeta, Box::new(SpinnerBeta::default()), vec![])
        .is_ok());
    // We need to give focus to input then
    assert!(app.active(&Id::SpanAlfa).is_ok());
    // Now we use the Model struct to keep track of some states

    // let's loop until quit is true
    while !model.quit {
        // Tick
        if let Ok(sz) = app.tick(&mut model, PollStrategy::Once) {
            if sz > 0 {
                // NOTE: redraw if at least one msg has been processed
                model.redraw = true;
            }
        }
        // Redraw
        model.view(&mut app);
    }
    // Terminate terminal
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
}

impl Update<Id, Msg, NoUserEvent> for Model {
    fn update(&mut self, _: &mut View<Id, Msg, NoUserEvent>, msg: Option<Msg>) -> Option<Msg> {
        match msg.unwrap_or(Msg::None) {
            Msg::AppClose => {
                self.quit = true;
                None
            }
            Msg::None => None,
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
            component: Span::default().foreground(Color::Yellow).spans(&[
                TextSpan::new("Downloading tui-realm... ")
                    .underlined()
                    .fg(Color::Green),
                TextSpan::from("Please wait!"),
            ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for SpanAlfa {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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
                .alignment(Alignment::Right)
                .modifiers(TextModifiers::BOLD)
                .spans(&[
                    TextSpan::new("Downloading tui-realm-stdlib... ")
                        .underlined()
                        .fg(Color::Green),
                    TextSpan::from("Please wait!"),
                ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for SpanBeta {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

#[derive(MockComponent)]
struct SpinnerAlfa {
    component: Spinner,
}

impl Default for SpinnerAlfa {
    fn default() -> Self {
        Self {
            component: Spinner::default()
                .foreground(Color::LightBlue)
                .sequence("⣾⣽⣻⢿⡿⣟⣯⣷"),
        }
    }
}

impl Component<Msg, NoUserEvent> for SpinnerAlfa {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
struct SpinnerBeta {
    component: Spinner,
}

impl Default for SpinnerBeta {
    fn default() -> Self {
        Self {
            component: Spinner::default()
                .foreground(Color::LightBlue)
                .sequence("▉▊▋▌▍▎▏▎▍▌▋▊▉"),
        }
    }
}

impl Component<Msg, NoUserEvent> for SpinnerBeta {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
