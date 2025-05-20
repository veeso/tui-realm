//! ## Demo
//!
//! `Demo` shows how to use tui-realm in a real case

use std::time::Duration;

use tui_realm_stdlib::Table;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalBridge};
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update,
};
// tui
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    TableAlfaBlur,
    TableBetaBlur,
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    TableAlfa,
    TableBeta,
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
            EventListenerCfg::default().crossterm_input_listener(Duration::from_millis(10), 10),
        );
        assert!(app
            .mount(Id::TableAlfa, Box::new(TableAlfa::default()), vec![])
            .is_ok());
        assert!(app
            .mount(Id::TableBeta, Box::new(TableBeta::default()), vec![])
            .is_ok());
        // We need to give focus to input then
        assert!(app.active(&Id::TableAlfa).is_ok());
        Self {
            quit: false,
            redraw: true,
            app,
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
                .constraints(
                    [
                        Constraint::Length(10),
                        Constraint::Length(6),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.area());
            self.app.view(&Id::TableAlfa, f, chunks[0]);
            self.app.view(&Id::TableBeta, f, chunks[1]);
        });
    }
}

fn main() {
    let mut model = Model::default();
    let mut terminal = TerminalBridge::init_crossterm().expect("Cannot create terminal bridge");
    let _ = terminal.enable_raw_mode();
    let _ = terminal.enter_alternate_screen();
    // Now we use the Model struct to keep track of some states

    // let's loop until quit is true
    while !model.quit {
        // Tick
        if let Ok(messages) = model.app.tick(PollStrategy::Once) {
            for msg in messages {
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
            Msg::TableAlfaBlur => {
                assert!(self.app.active(&Id::TableBeta).is_ok());
                None
            }
            Msg::TableBetaBlur => {
                assert!(self.app.active(&Id::TableAlfa).is_ok());
                None
            }
            Msg::None => None,
        }
    }
}

#[derive(MockComponent)]
struct TableAlfa {
    component: Table,
}

impl Default for TableAlfa {
    fn default() -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Thick)
                        .color(Color::Yellow),
                )
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
        }
    }
}

impl Component<Msg, NoUserEvent> for TableAlfa {
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
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::TableAlfaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

#[derive(MockComponent)]
struct TableBeta {
    component: Table,
}

impl Default for TableBeta {
    fn default() -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Green),
                )
                .foreground(Color::Green)
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
        }
    }
}

impl Component<Msg, NoUserEvent> for TableBeta {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::TableBetaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
