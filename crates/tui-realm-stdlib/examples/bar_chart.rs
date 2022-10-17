use std::time::Duration;

use tui_realm_stdlib::BarChart;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{Alignment, BorderType, Borders, Color, Style};
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
    ChartAlfaBlur,
    ChartBetaBlur,
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    ChartAlfa,
    ChartBeta,
}

struct Model {
    app: Application<Id, Msg, NoUserEvent>,
    quit: bool,   // Becomes true when the user presses <ESC>
    redraw: bool, // Tells whether to refresh the UI; performance optimization
}

impl Default for Model {
    fn default() -> Self {
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default().default_input_listener(Duration::from_millis(10)),
        );
        assert!(app
            .mount(Id::ChartAlfa, Box::new(ChartAlfa::default()), vec![])
            .is_ok());
        assert!(app
            .mount(Id::ChartBeta, Box::new(ChartBeta::default()), vec![])
            .is_ok());
        // We need to give focus to input then
        assert!(app.active(&Id::ChartAlfa).is_ok());
        Self {
            app,
            quit: false,
            redraw: true,
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
                        Constraint::Length(10),
                        Constraint::Length(10),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            self.app.view(&Id::ChartAlfa, f, chunks[0]);
            self.app.view(&Id::ChartBeta, f, chunks[1]);
        });
    }
}

fn main() {
    let mut terminal = TerminalBridge::new().expect("Cannot create terminal bridge");
    let mut model = Model::default();
    let _ = terminal.enable_raw_mode();
    let _ = terminal.enter_alternate_screen();
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
            Msg::ChartAlfaBlur => {
                assert!(self.app.active(&Id::ChartBeta).is_ok());
                None
            }
            Msg::ChartBetaBlur => {
                assert!(self.app.active(&Id::ChartAlfa).is_ok());
                None
            }
            Msg::None => None,
        }
    }
}

// -- components

#[derive(MockComponent)]
struct ChartAlfa {
    component: BarChart,
}

impl Default for ChartAlfa {
    fn default() -> Self {
        Self {
            component: BarChart::default()
                .disabled(false)
                .title("my incomes", Alignment::Center)
                .label_style(Style::default().fg(Color::Yellow))
                .bar_style(Style::default().fg(Color::LightYellow))
                .bar_gap(6)
                .width(12)
                .borders(Borders::default().color(Color::LightBlue))
                .max_bars(4)
                .value_style(Style::default().fg(Color::LightBlue))
                .data(&[
                    ("january", 250),
                    ("february", 300),
                    ("march", 275),
                    ("april", 312),
                    ("may", 420),
                    ("june", 170),
                    ("july", 220),
                    ("august", 160),
                    ("september", 180),
                    ("october", 470),
                    ("november", 380),
                    ("december", 820),
                ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for ChartAlfa {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::ChartAlfaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

// -- chart 2

#[derive(MockComponent)]
struct ChartBeta {
    component: BarChart,
}

impl Default for ChartBeta {
    fn default() -> Self {
        Self {
            component: BarChart::default()
                .disabled(false)
                .title("my incomes", Alignment::Left)
                .label_style(Style::default().fg(Color::Yellow))
                .bar_style(Style::default().fg(Color::LightYellow))
                .bar_gap(6)
                .width(12)
                .borders(
                    Borders::default()
                        .color(Color::LightBlue)
                        .modifiers(BorderType::Double),
                )
                .max_bars(12)
                .value_style(Style::default().fg(Color::LightBlue))
                .data(&[
                    ("january", 250),
                    ("february", 300),
                    ("march", 275),
                    ("april", 312),
                    ("may", 420),
                    ("june", 170),
                    ("july", 220),
                    ("august", 160),
                    ("september", 180),
                    ("october", 470),
                    ("november", 380),
                    ("december", 820),
                ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for ChartBeta {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::ChartBetaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
