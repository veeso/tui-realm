use std::time::Duration;

use tui_realm_stdlib::Canvas;
use tuirealm::props::{Alignment, Borders, Color, Shape};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalBridge};
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update,
};
// tui
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::ratatui::symbols::Marker;
use tuirealm::ratatui::widgets::canvas::{Line, Map, MapResolution, Rectangle};

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
    app: Application<Id, Msg, NoUserEvent>,
    quit: bool,   // Becomes true when the user presses <ESC>
    redraw: bool, // Tells whether to refresh the UI; performance optimization
}

impl Default for Model {
    fn default() -> Self {
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default().crossterm_input_listener(Duration::from_millis(10), 10),
        );
        assert!(app
            .mount(Id::Canvas, Box::new(MyCanvas::default()), vec![])
            .is_ok());
        // We need to give focus to input then
        assert!(app.active(&Id::Canvas).is_ok());
        Self {
            app,
            quit: false,
            redraw: true,
        }
    }
}

impl Model {
    fn view(&mut self, terminal: &mut TerminalBridge<CrosstermTerminalAdapter>) {
        let _ = terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.area());
            self.app.view(&Id::Canvas, f, chunks[0]);
        });
    }
}

fn main() {
    let mut terminal = TerminalBridge::init_crossterm().expect("Cannot create terminal bridge");
    let mut model = Model::default();
    let _ = terminal.enable_raw_mode();
    let _ = terminal.enter_alternate_screen();
    // let's loop until quit is true
    while !model.quit {
        // Tick
        if let Ok(messages) = model.app.tick(PollStrategy::Once) {
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
                .marker(Marker::Dot)
                .x_bounds((-180.0, 180.0))
                .y_bounds((-90.0, 90.0))
                .data(&[
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
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        if let Event::Keyboard(KeyEvent { code: Key::Esc, .. }) = ev {
            Some(Msg::AppClose)
        } else {
            None
        }
    }
}
