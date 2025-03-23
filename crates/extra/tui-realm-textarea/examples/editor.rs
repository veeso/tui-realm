use std::fs;
use std::io::{self, BufRead};
use std::time::Duration;

// label
#[cfg(feature = "search")]
use tui_realm_stdlib::Input;
use tui_realm_stdlib::Label;
// textarea
#[cfg(feature = "clipboard")]
use tui_realm_textarea::TEXTAREA_CMD_PASTE;
use tui_realm_textarea::{
    TextArea, TEXTAREA_CMD_MOVE_WORD_BACK, TEXTAREA_CMD_MOVE_WORD_FORWARD, TEXTAREA_CMD_NEWLINE,
    TEXTAREA_CMD_REDO, TEXTAREA_CMD_UNDO,
};
#[cfg(feature = "search")]
use tui_realm_textarea::{
    TEXTAREA_CMD_SEARCH_BACK, TEXTAREA_CMD_SEARCH_FORWARD, TEXTAREA_SEARCH_PATTERN,
};
use tuirealm::application::PollStrategy;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, BorderType, Borders, Color, Style, TextModifiers,
};
// tui
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalBridge};
#[cfg(feature = "search")]
use tuirealm::StateValue;
use tuirealm::{
    Application, Component, EventListenerCfg, MockComponent, NoUserEvent, State, Update,
};

// -- message
#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    Submit(Vec<String>),
    ChangeFocus(Id),
    #[cfg(feature = "search")]
    Search(String),
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Editor,
    #[cfg(feature = "search")]
    Search,
    Label,
}

struct Model {
    app: Application<Id, Msg, NoUserEvent>,
    quit: bool,   // Becomes true when the user presses <ESC>
    redraw: bool, // Tells whether to refresh the UI; performance optimization
    terminal: TerminalBridge<CrosstermTerminalAdapter>,
}

impl Model {
    fn new() -> Self {
        // Setup app
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default().crossterm_input_listener(Duration::from_millis(10), 10),
        );
        assert!(app
            .mount(Id::Editor, Box::new(Editor::default()), vec![])
            .is_ok());
        assert!(app
            .mount(Id::Label, Box::new(DummyLabel::default()), vec![])
            .is_ok());
        #[cfg(feature = "search")]
        assert!(app
            .mount(Id::Search, Box::new(Search::default()), vec![])
            .is_ok());
        assert!(app.active(&Id::Editor).is_ok());
        Model {
            app,
            quit: false,
            redraw: true,
            terminal: TerminalBridge::init_crossterm().expect("Could not initialize terminal"),
        }
    }

    fn view(&mut self) {
        let _ = self.terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Min(5),
                        Constraint::Length(1),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.area());
            self.app.view(&Id::Editor, f, chunks[0]);
            self.app.view(&Id::Label, f, chunks[1]);
            #[cfg(feature = "search")]
            self.app.view(&Id::Search, f, chunks[2]);
        });
    }
}

fn main() {
    // Make model
    let mut model: Model = Model::new();
    let _ = model.terminal.enable_raw_mode();
    let _ = model.terminal.enter_alternate_screen();
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
            model.view();
            model.redraw = false;
        }
    }
    // Terminate terminal
    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
    // print content
    model
        .app
        .state(&Id::Editor)
        .unwrap()
        .unwrap_vec()
        .into_iter()
        .for_each(|x| println!("{}", x.unwrap_string()));
}

// -- update

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.redraw = true;
        match msg.unwrap_or(Msg::None) {
            Msg::AppClose => {
                self.quit = true;
                None
            }
            Msg::ChangeFocus(Id::Editor) => {
                let _ = self.app.active(&Id::Editor);
                None
            }
            Msg::ChangeFocus(Id::Label) => {
                let _ = self.app.active(&Id::Label);
                None
            }
            #[cfg(feature = "search")]
            Msg::ChangeFocus(Id::Search) => {
                let _ = self.app.active(&Id::Search);
                None
            }
            Msg::Submit(lines) => {
                println!("Got user text: {:?}", lines);
                None
            }
            #[cfg(feature = "search")]
            Msg::Search(pattern) => {
                assert!(self
                    .app
                    .attr(
                        &Id::Editor,
                        Attribute::Custom(TEXTAREA_SEARCH_PATTERN),
                        AttrValue::String(pattern)
                    )
                    .is_ok());
                None
            }
            Msg::None => None,
        }
    }
}

// -- components

pub struct Editor<'a> {
    component: TextArea<'a>,
}

impl MockComponent for Editor<'_> {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::ratatui::layout::Rect) {
        self.component.view(frame, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, query: Attribute, attr: AttrValue) {
        self.component.attr(query, attr)
    }

    fn state(&self) -> State {
        self.component.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}

impl Default for Editor<'_> {
    fn default() -> Self {
        let textarea = match fs::File::open("README.md") {
            Ok(reader) => TextArea::new(
                io::BufReader::new(reader)
                    .lines()
                    .map(|l| l.unwrap())
                    .collect::<_>(),
            ),
            Err(_) => TextArea::default(),
        };
        Self {
            component: textarea
                .borders(
                    Borders::default()
                        .color(Color::LightYellow)
                        .modifiers(BorderType::Double),
                )
                .cursor_line_style(Style::default())
                .cursor_style(Style::default().add_modifier(TextModifiers::REVERSED))
                .footer_bar("Press <ESC> to quit", Style::default())
                .line_number_style(
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(TextModifiers::ITALIC),
                )
                .max_histories(64)
                .scroll_step(4)
                .status_bar(
                    "README.md Ln {ROW}, Col {COL}",
                    Style::default().add_modifier(TextModifiers::REVERSED),
                )
                .tab_length(4)
                .title("Editing README.md", Alignment::Left),
        }
    }
}

impl Component<Msg, NoUserEvent> for Editor<'_> {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => Some(Msg::AppClose),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::SHIFT,
            }) => {
                self.perform(Cmd::Scroll(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::SHIFT,
            }) => {
                self.perform(Cmd::Scroll(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => {
                self.perform(Cmd::Move(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers: KeyModifiers::SHIFT,
            }) => {
                self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_BACK));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                modifiers: KeyModifiers::SHIFT,
            }) => {
                self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_FORWARD));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('e'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('m'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(Cmd::Custom(TEXTAREA_CMD_NEWLINE));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            #[cfg(feature = "search")]
            Event::Keyboard(KeyEvent {
                code: Key::Char('s'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(Cmd::Custom(TEXTAREA_CMD_SEARCH_BACK));
                Some(Msg::None)
            }
            #[cfg(feature = "search")]
            Event::Keyboard(KeyEvent {
                code: Key::Char('d'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(Cmd::Custom(TEXTAREA_CMD_SEARCH_FORWARD));
                Some(Msg::None)
            }
            #[cfg(feature = "clipboard")]
            Event::Keyboard(KeyEvent {
                code: Key::Char('v'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(Cmd::Custom(TEXTAREA_CMD_PASTE));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('z'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(Cmd::Custom(TEXTAREA_CMD_UNDO));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.perform(Cmd::Custom(TEXTAREA_CMD_REDO));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                self.perform(Cmd::Type('\t'));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                ..
            }) => {
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Function(2),
                ..
            }) => Some(Msg::ChangeFocus(Id::Label)),
            #[cfg(feature = "search")]
            Event::Keyboard(KeyEvent {
                code: Key::Function(3),
                ..
            }) => Some(Msg::ChangeFocus(Id::Search)),
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct DummyLabel {
    component: Label,
}

impl Default for DummyLabel {
    fn default() -> Self {
        Self {
            component: Label::default().text("text editor demo"),
        }
    }
}

impl Component<Msg, NoUserEvent> for DummyLabel {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Function(1),
                ..
            }) => Some(Msg::ChangeFocus(Id::Editor)),
            _ => None,
        }
    }
}

#[cfg(feature = "search")]
#[derive(MockComponent)]
pub struct Search {
    component: Input,
}

#[cfg(feature = "search")]
impl Default for Search {
    fn default() -> Self {
        Self {
            component: Input::default()
                .title("Search text", Alignment::Left)
                .foreground(Color::LightYellow)
                .invalid_style(Style::default().fg(Color::Red)),
        }
    }
}

#[cfg(feature = "search")]
impl Component<Msg, NoUserEvent> for Search {
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
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => self.perform(Cmd::Cancel),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => self.perform(Cmd::Delete),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => {
                if let CmdResult::Changed(State::One(StateValue::String(pattern))) =
                    self.perform(Cmd::Type(ch))
                {
                    return Some(Msg::Search(pattern));
                }
                CmdResult::None
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                return Some(Msg::ChangeFocus(Id::Editor))
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
