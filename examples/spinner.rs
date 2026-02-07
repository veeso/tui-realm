//! ## Demo
//!
//! `Demo` shows how to use tui-realm in a real case

use std::time::Duration;

use tui_realm_stdlib::{Span, Spinner};
use tuirealm::command::CmdResult;
use tuirealm::props::{Color, HorizontalAlignment, TextModifiers};
use tuirealm::ratatui::style::Stylize;
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter};
use tuirealm::{
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update,
    application::PollStrategy,
    event::{Key, KeyEvent},
};
use tuirealm::{Sub, SubClause, SubEventClause};
// tui
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::ratatui::text::Span as RSpan;

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
    app: Application<Id, Msg, NoUserEvent>,
}

impl Default for Model {
    fn default() -> Self {
        // Setup app
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(10), 10)
                .tick_interval(Duration::from_millis(500)),
        );
        assert!(
            app.mount(Id::SpanAlfa, Box::new(SpanAlfa::default()), vec![])
                .is_ok()
        );
        assert!(
            app.mount(Id::SpanBeta, Box::new(SpanBeta::default()), vec![])
                .is_ok()
        );
        assert!(
            app.mount(
                Id::SpinnerAlfa,
                Box::new(SpinnerAlfa::default()),
                vec![Sub::new(SubEventClause::Tick, SubClause::Always)]
            )
            .is_ok()
        );
        assert!(
            app.mount(Id::SpinnerBeta, Box::new(SpinnerBeta::default()), vec![])
                .is_ok()
        );
        // We need to give focus to input then
        assert!(app.active(&Id::SpanAlfa).is_ok());
        Self {
            quit: false,
            redraw: true,
            app,
        }
    }
}

impl Model {
    fn view(&mut self, terminal: &mut CrosstermTerminalAdapter) {
        let _ = terminal.raw_mut().draw(|f| {
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
                .split(f.area());
            let row1 = Layout::default()
                .direction(LayoutDirection::Horizontal)
                .margin(1)
                .constraints([Constraint::Length(2), Constraint::Min(10)].as_ref())
                .split(chunks[0]);
            self.app.view(&Id::SpinnerAlfa, f, row1[0]);
            self.app.view(&Id::SpanAlfa, f, row1[1]);
            let row2 = Layout::default()
                .direction(LayoutDirection::Horizontal)
                .margin(1)
                .constraints([Constraint::Length(2), Constraint::Min(10)].as_ref())
                .split(chunks[1]);
            self.app.view(&Id::SpinnerBeta, f, row2[0]);
            self.app.view(&Id::SpanBeta, f, row2[1]);
        });
    }
}

fn main() {
    let mut model = Model::default();
    let mut terminal = CrosstermTerminalAdapter::new().expect("Cannot create terminal bridge");
    let _ = terminal.enable_raw_mode();
    let _ = terminal.enter_alternate_screen();
    // Now we use the Model struct to keep track of some states

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
        model.view(&mut terminal);
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

#[derive(MockComponent)]
struct SpanAlfa {
    component: Span,
}

impl Default for SpanAlfa {
    fn default() -> Self {
        Self {
            component: Span::default().foreground(Color::Yellow).spans([
                RSpan::raw("Downloading tui-realm...")
                    .underlined()
                    .fg(Color::Green),
                RSpan::raw(" "),
                RSpan::from("Please wait!"),
            ]),
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
                // fallback colors if the spans dont have their own
                .foreground(Color::White)
                .background(Color::DarkGray)
                .alignment(HorizontalAlignment::Right)
                .modifiers(TextModifiers::BOLD)
                .spans([
                    RSpan::raw("Downloading tui-realm-stdlib...")
                        .underlined()
                        .fg(Color::Yellow)
                        .bg(Color::Black),
                    RSpan::raw(" "),
                    RSpan::from("Please wait!"),
                ]),
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

#[derive(MockComponent)]
struct SpinnerAlfa {
    component: Spinner,
}

impl Default for SpinnerAlfa {
    fn default() -> Self {
        Self {
            component: Spinner::default()
                .foreground(Color::LightBlue)
                .sequence("⣾⣽⣻⢿⡿⣟⣯⣷")
                .manual_step(),
        }
    }
}

impl Component<Msg, NoUserEvent> for SpinnerAlfa {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        if *ev == Event::Tick {
            self.component.states.step();
        }

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
    fn on(&mut self, _: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}
