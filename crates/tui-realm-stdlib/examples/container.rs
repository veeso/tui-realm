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

use tui_realm_stdlib::{Container, Table};
use tuirealm::command::CmdResult;
use tuirealm::props::{Alignment, BorderType, Borders, Color, Layout, TableBuilder, TextSpan};
use tuirealm::terminal::TerminalBridge;
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update, View,
};
// tui
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout as TuiLayout};

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Container,
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
            let chunks = TuiLayout::default()
                .direction(LayoutDirection::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());
            app.view(&Id::Container, f, chunks[0]);
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
        .mount(Id::Container, Box::new(MyContainer::default()), vec![])
        .is_ok());
    // We need to give focus to input then
    assert!(app.active(&Id::Container).is_ok());
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
        if model.redraw {
            model.view(&mut app);
            model.redraw = false;
        }
    }
    // Terminate terminal
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
}

impl Update<Id, Msg, NoUserEvent> for Model {
    fn update(&mut self, _view: &mut View<Id, Msg, NoUserEvent>, msg: Option<Msg>) -> Option<Msg> {
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
struct MyContainer {
    component: Container,
}

impl Default for MyContainer {
    fn default() -> Self {
        Self {
            component: Container::default()
                .background(Color::Yellow)
                .foreground(Color::Yellow)
                .title("This is a div with two tables", Alignment::Left)
                .layout(
                    Layout::default()
                        .constraints(&[Constraint::Percentage(30), Constraint::Percentage(70)])
                        .direction(LayoutDirection::Horizontal)
                        .margin(2),
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
                            .title("Keybindings", Alignment::Center)
                            .scroll(true)
                            .highlighted_color(Color::LightYellow)
                            .highlighted_str("🚀")
                            .rewind(true)
                            .step(4)
                            .row_height(1)
                            .headers(&["Key", "Msg", "Description"])
                            .column_spacing(3)
                            .widths(&[30, 20, 50])
                            .table(
                                TableBuilder::default()
                                    .add_col(TextSpan::from("KeyCode::Down"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("Move cursor down"))
                                    .add_row()
                                    .add_col(TextSpan::from("KeyCode::Up"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("Move cursor up"))
                                    .add_row()
                                    .add_col(TextSpan::from("KeyCode::PageDown"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("Move cursor down by 8"))
                                    .add_row()
                                    .add_col(TextSpan::from("KeyCode::PageUp"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("ove cursor up by 8"))
                                    .add_row()
                                    .add_col(TextSpan::from("KeyCode::End"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("Move cursor to last item"))
                                    .add_row()
                                    .add_col(TextSpan::from("KeyCode::Home"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("Move cursor to first item"))
                                    .add_row()
                                    .add_col(TextSpan::from("KeyCode::Char(_)"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("Return pressed key"))
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
                            .title("Keybindings (not scrollable)", Alignment::Center)
                            .scroll(false)
                            .highlighted_color(Color::Green)
                            .highlighted_str(">> ")
                            .row_height(1)
                            .headers(&["Key", "Msg", "Description"])
                            .column_spacing(3)
                            .widths(&[30, 20, 50])
                            .table(
                                TableBuilder::default()
                                    .add_col(TextSpan::from("KeyCode::Down"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("Move cursor down"))
                                    .add_row()
                                    .add_col(TextSpan::from("KeyCode::Up"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("Move cursor up"))
                                    .add_row()
                                    .add_col(TextSpan::from("KeyCode::PageDown"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("Move cursor down by 8"))
                                    .add_row()
                                    .add_col(TextSpan::from("KeyCode::PageUp"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("ove cursor up by 8"))
                                    .add_row()
                                    .add_col(TextSpan::from("KeyCode::End"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("Move cursor to last item"))
                                    .add_row()
                                    .add_col(TextSpan::from("KeyCode::Home"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("Move cursor to first item"))
                                    .add_row()
                                    .add_col(TextSpan::from("KeyCode::Char(_)"))
                                    .add_col(TextSpan::from("OnKey"))
                                    .add_col(TextSpan::from("Return pressed key"))
                                    .build(),
                            ),
                    ),
                ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for MyContainer {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
