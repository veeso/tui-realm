use std::path::{Path, PathBuf};
use std::time::Duration;

use tui_realm_stdlib::components::{Input, Phantom};
use tui_realm_treeview::{Node, TREE_CMD_CLOSE, TREE_CMD_OPEN, Tree, TreeView};
use tuirealm::application::{Application, PollStrategy};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::listener::EventListenerCfg;
use tuirealm::props::{
    AttrValue, Attribute, BorderType, Borders, Color, HorizontalAlignment, InputType, Style,
    TextModifiers, Title,
};
use tuirealm::ratatui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::ratatui::text::Line;
use tuirealm::state::{State, StateValue};
use tuirealm::subscription::{EventClause as SubEventClause, Sub, SubClause};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter, TerminalResult};

const MAX_DEPTH: usize = 3;

// -- message
#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    ExtendDir(String),
    FsTreeBlur,
    GoToBlur,
    GoTo(PathBuf),
    GoToUpperDir,
    Redraw,
}

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    FsTree,
    GlobalListener,
    GoTo,
}

struct Model {
    app: Application<Id, Msg, NoUserEvent>,
    path: PathBuf,
    tree: Tree<String>, // You can choose a Tree<Vec<TextSpan>> for more flexible rendering
    quit: bool,         // Becomes true when the user presses <ESC>
    redraw: bool,       // Tells whether to refresh the UI; performance optimization
    terminal: CrosstermTerminalAdapter,
}

impl Model {
    /// Initialize the Terminal modes.
    fn init_adapter() -> TerminalResult<CrosstermTerminalAdapter> {
        let mut adapter = CrosstermTerminalAdapter::new()?;
        adapter.enable_raw_mode()?;
        adapter.enter_alternate_screen()?;

        Ok(adapter)
    }

    fn new(p: &Path) -> Self {
        // Setup app
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default().crossterm_input_listener(Duration::from_millis(10), 10),
        );
        assert!(
            app.mount(
                Id::FsTree,
                Box::new(FsTree::new(Tree::new(Self::dir_tree(p, MAX_DEPTH)), None)),
                vec![]
            )
            .is_ok()
        );
        assert!(
            app.mount(Id::GoTo, Box::new(GoTo::default()), vec![])
                .is_ok()
        );
        // Mount global listener which will listen for <ESC>
        assert!(
            app.mount(
                Id::GlobalListener,
                Box::new(GlobalListener::default()),
                vec![Sub::new(
                    SubEventClause::Keyboard(KeyEvent {
                        code: Key::Esc,
                        modifiers: KeyModifiers::NONE,
                    }),
                    SubClause::Always
                )]
            )
            .is_ok()
        );
        // We need to give focus to input then
        assert!(app.active(&Id::FsTree).is_ok());
        Model {
            app,
            quit: false,
            redraw: true,
            tree: Tree::new(Self::dir_tree(p, MAX_DEPTH)),
            path: p.to_path_buf(),
            terminal: Self::init_adapter().expect("Could not initialize terminal"),
        }
    }

    pub fn scan_dir(&mut self, p: &Path) {
        self.path = p.to_path_buf();
        self.tree = Tree::new(Self::dir_tree(p, MAX_DEPTH));
    }

    pub fn upper_dir(&self) -> Option<PathBuf> {
        self.path.parent().map(|x| x.to_path_buf())
    }

    pub fn extend_dir(&mut self, id: &String, p: &Path, depth: usize) {
        if let Some(node) = self.tree.root_mut().query_mut(id)
            && depth > 0
            && p.is_dir()
        {
            // Clear node
            node.clear();
            // Scan dir
            if let Ok(e) = std::fs::read_dir(p) {
                e.flatten()
                    .for_each(|x| node.add_child(Self::dir_tree(x.path().as_path(), depth - 1)));
            }
        }
    }

    fn dir_tree(p: &Path, depth: usize) -> Node<String> {
        let name: String = match p.file_name() {
            None => "/".to_string(),
            Some(n) => n.to_string_lossy().into_owned().to_string(),
        };
        let mut node: Node<String> = Node::new(p.to_string_lossy().into_owned(), name);
        if depth > 0
            && p.is_dir()
            && let Ok(e) = std::fs::read_dir(p)
        {
            e.flatten()
                .for_each(|x| node.add_child(Self::dir_tree(x.path().as_path(), depth - 1)));
        }
        node
    }

    fn view(&mut self) {
        let _ = self.terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(1)
                .constraints([Constraint::Min(5), Constraint::Length(3)].as_ref())
                .split(f.area());
            self.app.view(&Id::FsTree, f, chunks[0]);
            self.app.view(&Id::GoTo, f, chunks[1]);
        });
    }

    fn reload_tree(&mut self) {
        let current_node = match self.app.state(&Id::FsTree).ok().unwrap() {
            State::Single(StateValue::String(id)) => Some(id),
            _ => None,
        };
        // Remount tree
        assert!(self.app.umount(&Id::FsTree).is_ok());
        assert!(
            self.app
                .mount(
                    Id::FsTree,
                    Box::new(FsTree::new(self.tree.clone(), current_node)),
                    vec![]
                )
                .is_ok()
        );
        assert!(self.app.active(&Id::FsTree).is_ok());
    }
}

fn main() {
    // Make model
    let mut model: Model = Model::new(std::env::current_dir().ok().unwrap().as_path());
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
    // Terminate terminal
    let _ = model.terminal.restore();
}

// -- update

impl Model {
    pub fn update(&mut self, msg: Msg) {
        self.redraw = true;
        match msg {
            Msg::AppClose => {
                self.quit = true;
            }
            Msg::ExtendDir(path) => {
                self.extend_dir(&path, PathBuf::from(path.as_str()).as_path(), MAX_DEPTH);
                self.reload_tree();
            }
            Msg::GoTo(path) => {
                // Go to and reload tree
                self.scan_dir(path.as_path());
                self.reload_tree();
            }
            Msg::GoToUpperDir => {
                if let Some(parent) = self.upper_dir() {
                    self.scan_dir(parent.as_path());
                    self.reload_tree();
                }
            }
            Msg::FsTreeBlur => {
                assert!(self.app.active(&Id::GoTo).is_ok());
            }
            Msg::GoToBlur => {
                assert!(self.app.active(&Id::FsTree).is_ok());
            }
            _ => (),
        }
    }
}

// -- components

#[derive(Component)]
pub struct FsTree {
    component: TreeView<String>,
}

impl FsTree {
    pub fn new(tree: Tree<String>, initial_node: Option<String>) -> Self {
        // Preserve initial node if exists
        let initial_node = match initial_node {
            Some(id) if tree.root().query(&id).is_some() => id,
            _ => tree.root().id().to_string(),
        };
        FsTree {
            component: TreeView::default()
                .foreground(Color::Reset)
                .borders(
                    Borders::default()
                        .color(Color::LightYellow)
                        .modifiers(BorderType::Rounded),
                )
                .inactive(Style::new().fg(Color::Gray))
                .indent_size(3)
                .scroll_step(6)
                .title(
                    Title::from(tree.root().id().to_string()).alignment(HorizontalAlignment::Left),
                )
                .highlight_style(
                    Style::new()
                        .fg(Color::LightYellow)
                        .add_modifier(TextModifiers::REVERSED),
                )
                .highlight_str("🦄")
                .with_tree(tree)
                .initial_node(initial_node),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for FsTree {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let result = match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Left,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Custom(TREE_CMD_CLOSE)),
            KeyEvent {
                code: Key::Right,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Custom(TREE_CMD_OPEN)),
            KeyEvent {
                code: Key::PageDown,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Scroll(Direction::Down)),
            KeyEvent {
                code: Key::PageUp,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Scroll(Direction::Up)),
            KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Move(Direction::Down)),
            KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Move(Direction::Up)),
            KeyEvent {
                code: Key::Home,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::GoTo(Position::Begin)),
            KeyEvent {
                code: Key::End,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::GoTo(Position::End)),
            KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Submit),
            KeyEvent {
                code: Key::Backspace,
                modifiers: KeyModifiers::NONE,
            } => return Some(Msg::GoToUpperDir),
            KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            } => return Some(Msg::FsTreeBlur),
            _ => return None,
        };
        match result {
            CmdResult::Submit(State::Single(StateValue::String(node))) => {
                Some(Msg::ExtendDir(node))
            }
            _ => Some(Msg::Redraw),
        }
    }
}

// -- global listener

#[derive(Default, Component)]
pub struct GlobalListener {
    component: Phantom,
}

impl AppComponent<Msg, NoUserEvent> for GlobalListener {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::AppClose),
            _ => None,
        }
    }
}

// -- goto input

#[derive(Component)]
pub struct GoTo {
    component: Input,
}

impl Default for GoTo {
    fn default() -> Self {
        Self {
            component: Input::default()
                .foreground(Color::LightBlue)
                .inactive(Style::new().fg(Color::Gray))
                .borders(
                    Borders::default()
                        .color(Color::LightBlue)
                        .modifiers(BorderType::Rounded),
                )
                .input_type(InputType::Text)
                .placeholder(Line::styled(
                    "/foo/bar/buzz",
                    Style::new().fg(Color::Rgb(120, 120, 120)),
                ))
                .title(Title::from("Go to...").alignment(HorizontalAlignment::Left)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for GoTo {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        let result = match ev.as_keyboard()? {
            KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
            } => {
                let res = self.perform(Cmd::Submit);
                // Clear value
                self.attr(Attribute::Value, AttrValue::String(String::new()));
                res
            }
            KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Type(*ch)),
            KeyEvent {
                code: Key::Left,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Move(Direction::Left)),
            KeyEvent {
                code: Key::Right,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Move(Direction::Right)),
            KeyEvent {
                code: Key::Home,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::GoTo(Position::Begin)),
            KeyEvent {
                code: Key::End,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::GoTo(Position::End)),
            KeyEvent {
                code: Key::Delete,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Cancel),
            KeyEvent {
                code: Key::Backspace,
                modifiers: KeyModifiers::NONE,
            } => self.perform(Cmd::Delete),
            KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            } => return Some(Msg::GoToBlur),
            _ => return None,
        };
        match result {
            CmdResult::Submit(State::Single(StateValue::String(path))) => {
                Some(Msg::GoTo(PathBuf::from(path.as_str())))
            }
            _ => Some(Msg::Redraw),
        }
    }
}
