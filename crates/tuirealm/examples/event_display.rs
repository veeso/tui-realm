use std::time::Duration;

use tuirealm::application::{Application, PollStrategy};
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent};
use tuirealm::listener::EventListenerCfg;
use tuirealm::props::{AttrValue, Attribute, QueryResult};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::{Constraint, Direction, Layout, Rect};
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::state::State;
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalResult};

// /// Enable the crossterm-async event listener.
// ///
// /// Required `crossterm` and `async-ports`.
// fn crossterm_async() -> EventListenerCfg<UserEvent> {
//     let handle = tokio::runtime::Handle::current();
//     EventListenerCfg::default()
//         .with_handle(handle)
//         .async_crossterm_input_listener(Duration::default(), 3)
// }

/// Enable the crossterm event listener.
///
/// Required `crossterm`.
fn crossterm() -> EventListenerCfg<UserEvent> {
    EventListenerCfg::default().crossterm_input_listener(Duration::from_millis(10), 3)
}

// /// Enable the termion event listener.
// ///
// /// Required `termion`.
// fn termion() -> EventListenerCfg<UserEvent> {
//     EventListenerCfg::default().termion_input_listener(Duration::from_millis(10), 3)
// }

// /// Enable the termwiz event listener.
// ///
// /// Required `termwiz`.
// fn termwiz() -> EventListenerCfg<UserEvent> {
//     EventListenerCfg::default().termwiz_input_listener(Duration::from_millis(10), 3)
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let event_listener = crossterm_async();
    let event_listener = crossterm();
    // let event_listener = termion();
    // let event_listener = termwiz();

    let mut app: Application<Id, Msg, UserEvent> = Application::init(event_listener);

    // subscribe component to clause
    app.mount(Id::Info, Box::new(Label::default()), Vec::new())?;
    app.mount(Id::Display, Box::new(EventDisplay::default()), Vec::new())?;

    app.active(&Id::Display).expect("failed to active");

    let mut model = Model::new(app)?;

    tokio::task::block_in_place(|| {
        // draw the initial state, as there is no ticking here
        model.view();

        // Main loop
        // NOTE: loop until quit; quit is set in update if AppClose is received from counter
        while !model.quit {
            // Tick
            match model.app.tick(PollStrategy::BlockCollectUpTo(5)) {
                Err(err) => {
                    panic!("application error {err}");
                }
                Ok(messages) if !messages.is_empty() => {
                    for msg in messages {
                        model.update(msg);
                    }
                }
                _ => {}
            }
            // Redraw
            if model.redraw {
                model.view();
                model.redraw = false;
            }
        }
    });

    model.terminal.restore()?;

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    Redraw,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Info,
    Display,
}

#[derive(Debug, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum UserEvent {}

impl PartialEq for UserEvent {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

pub struct Model {
    /// Application
    pub app: Application<Id, Msg, UserEvent>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: CrosstermTerminalAdapter,
}

impl Model {
    fn init_adapter() -> TerminalResult<CrosstermTerminalAdapter> {
        let mut adapter = CrosstermTerminalAdapter::new()?;
        adapter.enable_raw_mode()?;
        adapter.enter_alternate_screen()?;
        adapter.enable_mouse_capture()?;

        Ok(adapter)
    }

    pub fn new(app: Application<Id, Msg, UserEvent>) -> TerminalResult<Self> {
        Ok(Self {
            app,
            quit: false,
            redraw: true,
            terminal: Self::init_adapter()?,
        })
    }

    pub fn view(&mut self) {
        self.terminal
            .draw(|f| {
                let [info, display] = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3), // Info
                            Constraint::Fill(1),
                        ]
                        .as_ref(),
                    )
                    .areas(f.area());
                self.app.view(&Id::Info, f, info);
                self.app.view(&Id::Display, f, display);
            })
            .expect("Draw correctly");
    }
}

// Let's implement Update for model

impl Model {
    fn update(&mut self, msg: Msg) {
        // Set redraw
        self.redraw = true;
        // Match message
        match msg {
            Msg::AppClose => {
                self.quit = true; // Terminate
            }
            Msg::Redraw => (),
        }
    }
}

/// Display basic information
#[derive(Default)]
pub struct Label {}

impl Component for Label {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Get properties
        let text = "Demo to display Pressed Keys. Press ESCAPE to exit";
        frame.render_widget(Paragraph::new(text), area);
    }

    fn query<'a>(&'a self, _attr: Attribute) -> Option<QueryResult<'a>> {
        None
    }

    fn attr(&mut self, _attr: Attribute, _value: AttrValue) {}

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        CmdResult::Invalid(cmd)
    }
}

impl AppComponent<Msg, UserEvent> for Label {
    fn on(&mut self, _ev: &Event<UserEvent>) -> Option<Msg> {
        None
    }
}

/// Display basic information
#[derive(Default)]
pub struct EventDisplay {
    last_event: Option<Event<UserEvent>>,
    is_same: bool,
}

impl Component for EventDisplay {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let text = format!(
            "Is Same as previous: {}\nEvent: {:#?}",
            self.is_same, self.last_event
        );

        frame.render_widget(Paragraph::new(text), area);
    }

    fn query<'a>(&'a self, _attr: Attribute) -> Option<QueryResult<'a>> {
        None
    }

    fn attr(&mut self, _attr: Attribute, _value: AttrValue) {}

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        CmdResult::Invalid(cmd)
    }
}

impl AppComponent<Msg, UserEvent> for EventDisplay {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Msg> {
        if let Event::Keyboard(KeyEvent {
            code: Key::Esc,
            modifiers: _,
        }) = ev
        {
            return Some(Msg::AppClose);
        }

        let is_same = Some(ev) == self.last_event.as_ref();
        self.last_event = Some(ev.clone());
        self.is_same = is_same;
        Some(Msg::Redraw)
    }
}
