//! `Demo` shows how to use tui-realm in a real case

use std::error::Error;
use std::time::Duration;

use tui_realm_stdlib::Canvas;
use tuirealm::props::{Borders, Color, HorizontalAlignment, Shape, Title};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::ratatui::symbols::Marker;
use tuirealm::ratatui::widgets::canvas::{Line, Map, MapResolution, Rectangle};
use tuirealm::{
    Component, Event, MockComponent, NoUserEvent,
    application::PollStrategy,
    event::{Key, KeyEvent},
};

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
    Canvas,
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
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.area());
                self.app.view(&Id::Canvas, f, chunks[0]);
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
            .mount(Id::Canvas, Box::new(MyCanvas::default()), vec![])?;
        // We need to give focus to input then
        self.app.active(&Id::Canvas)?;

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
                model.redraw = true;
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
                .title(Title::from("playing risiko").alignment(HorizontalAlignment::Center))
                .borders(Borders::default().color(Color::LightBlue))
                .marker(Marker::Dot)
                .x_bounds((-180.0, 180.0))
                .y_bounds((-90.0, 90.0))
                .data([
                    Shape::Label((24.0, 34.0, String::from("Hello!"), Color::Cyan)),
                    Shape::Layer,
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
                            (21.0, 13.0),
                            (66.0, 77.0),
                            (34.0, 69.0),
                            (45.0, 76.0),
                            (120.0, 55.0),
                            (-32.0, -50.0),
                            (-4.0, 2.0),
                            (-32.0, -48.0),
                        ],
                        Color::Green,
                    )),
                ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for MyCanvas {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        if let Event::Keyboard(KeyEvent { code: Key::Esc, .. }) = ev {
            Some(Msg::AppClose)
        } else {
            None
        }
    }
}
