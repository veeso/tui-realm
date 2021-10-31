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

use tui_realm_stdlib::Canvas;
use tuirealm::props::{Alignment, Borders, Color, Shape};
use tuirealm::terminal::TerminalBridge;
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update, View,
};
// tui
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::tui::widgets::canvas::{Line, Map, MapResolution, Rectangle};

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Canvas,
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
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());
            app.view(&Id::Canvas, f, chunks[0]);
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
        .mount(Id::Canvas, Box::new(MyCanvas::default()), vec![])
        .is_ok());
    // We need to give focus to input then
    assert!(app.active(&Id::Canvas).is_ok());
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

// -- components

#[derive(MockComponent)]
struct MyCanvas {
    component: Canvas,
}

impl Default for MyCanvas {
    fn default() -> Self {
        Self {
            component: Canvas::default()
                .background(Color::Reset)
                .foreground(Color::LightYellow)
                .title("playing risiko", Alignment::Center)
                .borders(Borders::default().color(Color::LightBlue))
                .x_bounds((-180.0, 180.0))
                .y_bounds((-90.0, 90.0))
                .data(&[
                    Shape::Map(Map {
                        resolution: MapResolution::High,
                        color: Color::Rgb(240, 240, 240),
                    }),
                    Shape::Layer,
                    Shape::Line(Line {
                        x1: 0.0,
                        y1: 10.0,
                        x2: 10.0,
                        y2: 10.0,
                        color: Color::Red,
                    }),
                    Shape::Rectangle(Rectangle {
                        x: 60.0,
                        y: 20.0,
                        width: 70.0,
                        height: 20.0,
                        color: Color::Cyan,
                    }),
                    Shape::Points((
                        vec![
                            (21.0 as f64, 13.0 as f64),
                            (66.0 as f64, 77.0 as f64),
                            (34.0 as f64, 69.0 as f64),
                            (45.0 as f64, 76.0 as f64),
                            (120.0 as f64, 55.0 as f64),
                            (-32.0 as f64, -50.0 as f64),
                            (-4.0 as f64, 2.0 as f64),
                            (-32.0 as f64, -48.0 as f64),
                        ],
                        Color::Green,
                    )),
                ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for MyCanvas {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        if let Event::Keyboard(KeyEvent { code: Key::Esc, .. }) = ev {
            Some(Msg::AppClose)
        } else {
            None
        }
    }
}
