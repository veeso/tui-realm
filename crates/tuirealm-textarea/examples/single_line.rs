use std::time::Duration;

use tui_realm_textarea::{
    TEXTAREA_CMD_MOVE_WORD_BACK, TEXTAREA_CMD_MOVE_WORD_FORWARD, TEXTAREA_CMD_NEWLINE,
    TEXTAREA_CMD_REDO, TEXTAREA_CMD_UNDO, TextArea,
};
use tuirealm::application::{Application, PollStrategy};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::listener::EventListenerCfg;
use tuirealm::props::{
    AttrValue, Attribute, BorderType, Borders, Color, HorizontalAlignment, QueryResult, Style,
    TextModifiers, Title,
};
// tui
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::state::State;
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalResult};

// -- message
#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    Submit(Vec<String>),
    Redraw,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Input,
}

struct Model {
    app: Application<Id, Msg, NoUserEvent>,
    quit: bool,   // Becomes true when the user presses <ESC>
    redraw: bool, // Tells whether to refresh the UI; performance optimization
    terminal: CrosstermTerminalAdapter,
}

impl Model {
    /// Initialize the Terminal modes.
    fn init_adapter() -> TerminalResult<CrosstermTerminalAdapter> {
        let mut adapter = CrosstermTerminalAdapter::new()?;
        adapter.enable_raw_mode()?;
        adapter.enter_alternate_screen()?;
        adapter.enable_bracketed_paste()?;

        Ok(adapter)
    }

    fn new() -> Self {
        // Setup app
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default().crossterm_input_listener(Duration::from_millis(10), 10),
        );
        assert!(
            app.mount(Id::Input, Box::new(Input::default()), vec![])
                .is_ok()
        );
        assert!(app.active(&Id::Input).is_ok());
        Model {
            app,
            quit: false,
            redraw: true,
            terminal: Self::init_adapter().expect("Could not initialize terminal"),
        }
    }

    fn view(&mut self) {
        let _ = self.terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .constraints([Constraint::Length(6), Constraint::Min(0)].as_ref())
                .split(f.area());
            self.app.view(&Id::Input, f, chunks[0]);
        });
    }
}

fn main() {
    // Make model
    let mut model: Model = Model::new();
    // let's loop until quit is true
    while !model.quit {
        // Tick
        if let Ok(messages) = model
            .app
            .tick(PollStrategy::Once(Duration::from_millis(10)))
        {
            for msg in messages.into_iter() {
                model.update(msg);
            }
        }
        // Redraw
        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }
    // print content
    model
        .app
        .state(&Id::Input)
        .unwrap()
        .unwrap_vec()
        .into_iter()
        .for_each(|x| println!("{}", x.unwrap_string()));
}

// -- update

impl Model {
    pub fn update(&mut self, msg: Msg) {
        self.redraw = true;
        match msg {
            Msg::AppClose => {
                self.quit = true;
            }
            Msg::Submit(lines) => {
                println!("Got user text: {:?}", lines);
            }
            _ => (),
        }
    }
}

// -- components

pub struct Input {
    component: TextArea<'static>,
}

impl Component for Input {
    fn view(
        &mut self,
        frame: &mut tuirealm::ratatui::Frame,
        area: tuirealm::ratatui::layout::Rect,
    ) {
        self.component.view(frame, area);
    }

    fn query<'a>(&'a self, attr: Attribute) -> Option<QueryResult<'a>> {
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

impl Default for Input {
    fn default() -> Self {
        let textarea = TextArea::default().single_line(true);
        Self {
            component: textarea
                .borders(
                    Borders::default()
                        .color(Color::LightYellow)
                        .modifiers(BorderType::Plain),
                )
                .cursor_line_style(Style::default())
                .cursor_style(Style::default().add_modifier(TextModifiers::REVERSED))
                .footer_bar("Press <ESC> to quit", Style::default())
                .max_histories(64)
                .tab_length(4)
                .title(Title::from("Value").alignment(HorizontalAlignment::Left)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for Input {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let result = match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('h'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::Delete),
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => self.perform(Cmd::Cancel),
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::SHIFT,
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::SHIFT,
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers: KeyModifiers::SHIFT,
            }) => self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_BACK)),
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                modifiers: KeyModifiers::SHIFT,
            }) => self.perform(Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_FORWARD)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('e'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::GoTo(Position::End)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('m'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::Custom(TEXTAREA_CMD_NEWLINE)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('z'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::Custom(TEXTAREA_CMD_UNDO)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.perform(Cmd::Custom(TEXTAREA_CMD_REDO)),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => self.perform(Cmd::Type('\t')),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                ..
            }) => self.perform(Cmd::Type(*ch)),
            Event::Paste(text) => {
                self.component.paste(text);
                CmdResult::Changed(State::None)
            }
            _ => return None,
        };

        if matches!(result, CmdResult::None | CmdResult::Invalid(_)) {
            None
        } else {
            Some(Msg::Redraw)
        }
    }
}
