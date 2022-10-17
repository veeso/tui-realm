//! ## Demo
//!
//! `Demo` shows how to use tui-realm in a real case

use std::time::Duration;

use tui_realm_stdlib::Label;
use tuirealm::command::CmdResult;
use tuirealm::props::{Alignment, Color, TextModifiers};
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
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    LabelAlfa,
    LabelBeta,
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
            .mount(Id::LabelAlfa, Box::new(LabelAlfa::default()), vec![])
            .is_ok());
        assert!(app
            .mount(Id::LabelBeta, Box::new(LabelBeta::default()), vec![])
            .is_ok());
        // We need to give focus to input then
        assert!(app.active(&Id::LabelAlfa).is_ok());
        Self {
            quit: false,
            redraw: true,
            app,
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
                        Constraint::Length(1), // Label
                        Constraint::Length(1), // Label
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            self.app.view(&Id::LabelAlfa, f, chunks[0]);
            self.app.view(&Id::LabelBeta, f, chunks[1]);
        });
    }
}

fn main() {
    let mut terminal = TerminalBridge::new().expect("Cannot create terminal bridge");
    let mut model = Model::default();
    let _ = terminal.enable_raw_mode();
    let _ = terminal.enter_alternate_screen();
    // Setup ap
    // Now we use the Model struct to keep track of some states

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
            Msg::None => None,
        }
    }
}

#[derive(MockComponent)]
struct LabelAlfa {
    component: Label,
}

impl Default for LabelAlfa {
    fn default() -> Self {
        Self {
            component: Label::default()
                .alignment(Alignment::Center)
                .foreground(Color::Green)
                .modifiers(TextModifiers::BOLD)
                .text("This is a label"),
        }
    }
}

impl Component<Msg, NoUserEvent> for LabelAlfa {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

#[derive(MockComponent)]
struct LabelBeta {
    component: Label,
}

impl Default for LabelBeta {
    fn default() -> Self {
        Self {
            component: Label::default()
                .alignment(Alignment::Right)
                .foreground(Color::White)
                .background(Color::Blue)
                .modifiers(TextModifiers::ITALIC)
                .text("This is a label"),
        }
    }
}

impl Component<Msg, NoUserEvent> for LabelBeta {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
