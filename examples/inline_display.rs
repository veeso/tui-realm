use std::time::Duration;

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::ratatui::layout::{Constraint, Direction, Layout, Rect};
use tuirealm::ratatui::style::{Color, Style};
use tuirealm::ratatui::widgets::{LineGauge, Paragraph};
use tuirealm::ratatui::{TerminalOptions, Viewport};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter};
use tuirealm::{
    Application, AttrValue, Attribute, Component, Event, EventListenerCfg, Frame, MockComponent,
    NoUserEvent, PollStrategy, State, StdClock, Update,
};

/// This Example Showcases tui-realm can be used Inline too
///
/// The most important changes:
/// 1. Create the Terminal with options [`Viewport::Inline`]
/// 2. Dont enter Alternate Screen
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app: Application<Id, Msg, NoUserEvent, StdClock> = Application::init(
        EventListenerCfg::default()
            .crossterm_input_listener(Duration::from_millis(10), 3)
            .tick_interval(Duration::from_millis(500)),
    );

    // subscribe component to clause
    app.mount(Id::Info, Box::new(Label::default()), Vec::new())?;
    app.mount(Id::Display, Box::new(ProgressBar::default()), Vec::new())?;

    app.active(&Id::Display).expect("failed to active");

    // The most important difference: Create the Terminal with specific options
    let mut model = Model::new(
        app,
        CrosstermTerminalAdapter::new_with_options(TerminalOptions {
            viewport: Viewport::Inline(5), // Info + Line + Spacing + Top Margin
        })?,
    );

    // draw the initial state
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
                // NOTE: redraw if at least one msg has been processed
                model.redraw = true;
                for msg in messages {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
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

    model.restore()?;

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

pub struct Model<T>
where
    T: TerminalAdapter,
{
    /// Application
    pub app: Application<Id, Msg, NoUserEvent, StdClock>,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: T,
}

impl<T> Model<T>
where
    T: TerminalAdapter,
{
    pub fn new(app: Application<Id, Msg, NoUserEvent, StdClock>, mut adapter: T) -> Self {
        // Second most important change compared to normal: Dont enter Alternate Screen
        adapter.enable_raw_mode().expect("Enabling rawmode");
        Self {
            app,
            quit: false,
            redraw: true,
            terminal: adapter,
        }
    }

    pub fn view(&mut self) {
        self.terminal
            .draw(|f| {
                let [info, display] = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .spacing(1)
                    .constraints(
                        [
                            Constraint::Length(1), // Info
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

    pub fn restore(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.disable_raw_mode()?;

        Ok(())
    }
}

// Let's implement Update for model

impl<T> Update<Msg> for Model<T>
where
    T: TerminalAdapter,
{
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            // Set redraw
            self.redraw = true;
            // Match message
            match msg {
                Msg::AppClose => {
                    self.quit = true; // Terminate
                    None
                }
                Msg::Redraw => None,
            }
        } else {
            None
        }
    }
}

/// Display basic information
#[derive(Default)]
pub struct Label {}

impl MockComponent for Label {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Get properties
        let text = "Demo to showcase tui-realm can be used inline too. Press ESCAPE to exit";
        frame.render_widget(Paragraph::new(text), area);
    }

    fn query(&self, _attr: Attribute) -> Option<AttrValue> {
        None
    }

    fn attr(&mut self, _attr: Attribute, _value: AttrValue) {}

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Msg, NoUserEvent> for Label {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(Default)]
pub struct ProgressBar {
    state: f64,
}

impl MockComponent for ProgressBar {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let progressbar = LineGauge::default()
            .filled_style(Style::default().fg(Color::Blue))
            .unfilled_style(Style::default().fg(Color::Gray))
            .ratio(self.state);
        frame.render_widget(progressbar, area);
    }

    fn query(&self, _attr: Attribute) -> Option<AttrValue> {
        None
    }

    fn attr(&mut self, _attr: Attribute, _value: AttrValue) {
        // not implemented
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Msg, NoUserEvent> for ProgressBar {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        if let Event::Keyboard(KeyEvent {
            code: Key::Esc,
            modifiers: _,
        }) = ev
        {
            return Some(Msg::AppClose);
        }

        if let Event::Tick = ev {
            self.state += 0.01;
            if self.state >= 1.0 {
                self.state = 0.0;
            }
        }

        Some(Msg::Redraw)
    }
}
