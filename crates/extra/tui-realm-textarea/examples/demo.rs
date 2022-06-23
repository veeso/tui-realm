use std::{
    fs,
    io::{self, BufRead},
    time::Duration,
};
use tuirealm::{
    application::PollStrategy,
    command::{Cmd, CmdResult, Direction, Position},
    event::{Event, Key, KeyEvent, KeyModifiers},
    props::{Alignment, AttrValue, Attribute, BorderType, Borders, Color, Style, TextModifiers},
    terminal::TerminalBridge,
    Application, Component, EventListenerCfg, MockComponent, NoUserEvent, State, StateValue,
    Update,
};
// tui
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};
// label
use tui_realm_stdlib::Label;
// textarea
#[cfg(feature = "clipboard")]
use tui_realm_textarea::TEXTAREA_CMD_PASTE;
use tui_realm_textarea::{
    TextArea, TEXTAREA_CMD_MOVE_WORD_BACK, TEXTAREA_CMD_MOVE_WORD_FORWARD, TEXTAREA_CMD_NEWLINE,
    TEXTAREA_CMD_REDO, TEXTAREA_CMD_UNDO,
};

// -- message
#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    Submit(Vec<String>),
    ChangeFocus(Id),
    None,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Editor,
    Label,
}

struct Model {
    app: Application<Id, Msg, NoUserEvent>,
    quit: bool,   // Becomes true when the user presses <ESC>
    redraw: bool, // Tells whether to refresh the UI; performance optimization
    terminal: TerminalBridge,
}

impl Model {
    fn new() -> Self {
        // Setup app
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default().default_input_listener(Duration::from_millis(10)),
        );
        assert!(app
            .mount(Id::Editor, Box::new(Editor::default()), vec![])
            .is_ok());
        assert!(app
            .mount(Id::Label, Box::new(DummyLabel::default()), vec![])
            .is_ok());
        assert!(app.active(&Id::Editor).is_ok());
        Model {
            app,
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Could not initialize terminal"),
        }
    }

    fn view(&mut self) {
        let _ = self.terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(1)
                .constraints([Constraint::Min(5), Constraint::Length(1)].as_ref())
                .split(f.size());
            self.app.view(&Id::Editor, f, chunks[0]);
            self.app.view(&Id::Label, f, chunks[1]);
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
    if let Ok(State::Vec(lines)) = model.app.state(&Id::Editor) {
        lines.into_iter().for_each(|x| {
            if let StateValue::String(x) = x {
                println!("{}", x)
            }
        });
    }
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
            Msg::Submit(lines) => {
                println!("Got user text: {:?}", lines);
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

impl<'a> MockComponent for Editor<'a> {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
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

impl<'a> Default for Editor<'a> {
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

impl<'a> Component<Msg, NoUserEvent> for Editor<'a> {
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
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('d'),
                modifiers: KeyModifiers::CONTROL,
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
