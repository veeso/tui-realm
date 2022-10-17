//! ## Demo
//!
//! `Demo` shows how to use tui-realm in a real case

use std::time::Duration;

use tui_realm_stdlib::List;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan};
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
    ListAlfaBlur,
    ListBetaBlur,
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    ListAlfa,
    ListBeta,
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
            EventListenerCfg::default().default_input_listener(Duration::from_millis(10)),
        );
        assert!(app
            .mount(Id::ListAlfa, Box::new(ListAlfa::default()), vec![])
            .is_ok());
        assert!(app
            .mount(Id::ListBeta, Box::new(ListBeta::default()), vec![])
            .is_ok());
        // We need to give focus to input then
        assert!(app.active(&Id::ListAlfa).is_ok());
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
                        Constraint::Length(10),
                        Constraint::Length(6),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            self.app.view(&Id::ListAlfa, f, chunks[0]);
            self.app.view(&Id::ListBeta, f, chunks[1]);
        });
    }
}

fn main() {
    let mut terminal = TerminalBridge::new().expect("Cannot create terminal bridge");
    let mut model = Model::default();
    let _ = terminal.enable_raw_mode();
    let _ = terminal.enter_alternate_screen();
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
            Msg::ListAlfaBlur => {
                assert!(self.app.active(&Id::ListBeta).is_ok());
                None
            }
            Msg::ListBetaBlur => {
                assert!(self.app.active(&Id::ListAlfa).is_ok());
                None
            }
            Msg::None => None,
        }
    }
}

#[derive(MockComponent)]
struct ListAlfa {
    component: List,
}

impl Default for ListAlfa {
    fn default() -> Self {
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Yellow),
                )
                .title("Lorem ipsum (scrollable)", Alignment::Center)
                .scroll(true)
                .highlighted_color(Color::LightYellow)
                .highlighted_str("🚀")
                .rewind(true)
                .step(4)
                .rows(
                    TableBuilder::default()
                        .add_col(TextSpan::from("01").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit"))
                        .add_row()
                        .add_col(TextSpan::from("02").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Maecenas tincidunt dui ut gravida fringilla"))
                        .add_row()
                        .add_col(TextSpan::from("03").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Duis est neque, fringilla sit amet enim id, congue hendrerit mauris"))
                        .add_row()
                        .add_col(TextSpan::from("04").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Nulla facilisi. Vestibulum tincidunt tempor orci, in pellentesque lacus placerat id."))
                        .add_row()
                        .add_col(TextSpan::from("05").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Integer at nisl scelerisque, egestas ipsum in, iaculis tellus. Pellentesque tincidunt vestibulum nisi, ut vehicula augue scelerisque at"))
                        .add_row()
                        .add_col(TextSpan::from("06").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Quisque quis tincidunt tellus. Nam accumsan leo non nunc finibus feugiat."))
                        .add_row()
                        .add_col(TextSpan::from("07").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("non lacus ac orci fermentum aliquam ut feugiat libero. Suspendisse eget nunc in erat molestie egestas eu at massa"))
                        .add_row()
                        .add_col(TextSpan::from("08").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Donec feugiat dui quis libero ornare, vel sodales mauris ornare."))
                        .add_row()
                        .add_col(TextSpan::from("09").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Aenean tempor porta nisi, at sodales eros semper ut. Vivamus sit amet commodo risus"))
                        .add_row()
                        .add_col(TextSpan::from("10").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Etiam urna nisi, ullamcorper at justo et, rhoncus pellentesque dui. Nunc ante velit, ultrices a ornare sit amet, sagittis in ex. Nam pulvinar tellus tortor. Praesent ac accumsan nunc, ac consectetur nisi."))
                        .add_row()
                        .add_col(TextSpan::from("11").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Proin non elit fermentum, pretium diam eget, facilisis mi"))
                        .add_row()
                        .add_col(TextSpan::from("12").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Duis suscipit nibh lacus, quis porta enim accumsan vel"))
                        .add_row()
                        .add_col(TextSpan::from("13").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Etiam volutpat magna tortor, a laoreet ex accumsan sit amet"))
                        .build()
                )
                .selected_line(2),
        }
    }
}

impl Component<Msg, NoUserEvent> for ListAlfa {
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
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::ListAlfaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}

#[derive(MockComponent)]
struct ListBeta {
    component: List,
}

impl Default for ListBeta {
    fn default() -> Self {
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::Green),
                )
                .foreground(Color::Green)
                .title("Lorem ipsum (unscrollable)", Alignment::Center)
                .scroll(false)
                .rows(
                    TableBuilder::default()
                        .add_col(TextSpan::from("01").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit"))
                        .add_row()
                        .add_col(TextSpan::from("02").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Maecenas tincidunt dui ut gravida fringilla"))
                        .add_row()
                        .add_col(TextSpan::from("03").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Duis est neque, fringilla sit amet enim id, congue hendrerit mauris"))
                        .add_row()
                        .add_col(TextSpan::from("04").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Nulla facilisi. Vestibulum tincidunt tempor orci, in pellentesque lacus placerat id."))
                        .add_row()
                        .add_col(TextSpan::from("05").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Integer at nisl scelerisque, egestas ipsum in, iaculis tellus. Pellentesque tincidunt vestibulum nisi, ut vehicula augue scelerisque at"))
                        .add_row()
                        .add_col(TextSpan::from("06").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Quisque quis tincidunt tellus. Nam accumsan leo non nunc finibus feugiat."))
                        .add_row()
                        .add_col(TextSpan::from("07").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("non lacus ac orci fermentum aliquam ut feugiat libero. Suspendisse eget nunc in erat molestie egestas eu at massa"))
                        .add_row()
                        .add_col(TextSpan::from("08").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Donec feugiat dui quis libero ornare, vel sodales mauris ornare."))
                        .add_row()
                        .add_col(TextSpan::from("09").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Aenean tempor porta nisi, at sodales eros semper ut. Vivamus sit amet commodo risus"))
                        .add_row()
                        .add_col(TextSpan::from("10").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Etiam urna nisi, ullamcorper at justo et, rhoncus pellentesque dui. Nunc ante velit, ultrices a ornare sit amet, sagittis in ex. Nam pulvinar tellus tortor. Praesent ac accumsan nunc, ac consectetur nisi."))
                        .add_row()
                        .add_col(TextSpan::from("11").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Proin non elit fermentum, pretium diam eget, facilisis mi"))
                        .add_row()
                        .add_col(TextSpan::from("12").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Duis suscipit nibh lacus, quis porta enim accumsan vel"))
                        .add_row()
                        .add_col(TextSpan::from("13").fg(Color::Cyan).italic())
                        .add_col(TextSpan::from(" "))
                        .add_col(TextSpan::from("Etiam volutpat magna tortor, a laoreet ex accumsan sit amet"))
                        .build()
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for ListBeta {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::ListBetaBlur),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
