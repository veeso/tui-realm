/**
 * MIT License
 *
 * tui-realm-treeview - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use std::path::{Path, PathBuf};
use std::time::Duration;
use tui_realm_stdlib::{Input, Phantom};
use tuirealm::{
    application::PollStrategy,
    command::{Cmd, CmdResult, Direction, Position},
    event::{Event, Key, KeyEvent, KeyModifiers},
    props::{Alignment, AttrValue, Attribute, BorderType, Borders, Color, InputType, Style},
    terminal::TerminalBridge,
    Application, Component, EventListenerCfg, MockComponent, NoUserEvent, State, StateValue, Sub,
    SubClause, SubEventClause, Update,
};
// tui
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};
// treeview
use tui_realm_treeview::{Node, Tree, TreeView, TREE_CMD_CLOSE, TREE_CMD_OPEN};

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
    None,
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
    tree: Tree,
    quit: bool,   // Becomes true when the user presses <ESC>
    redraw: bool, // Tells whether to refresh the UI; performance optimization
    terminal: TerminalBridge,
}

impl Model {
    fn new(p: &Path) -> Self {
        // Setup app
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default().default_input_listener(Duration::from_millis(10)),
        );
        assert!(app
            .mount(
                Id::FsTree,
                Box::new(FsTree::new(Tree::new(Self::dir_tree(p, MAX_DEPTH)), None)),
                vec![]
            )
            .is_ok());
        assert!(app
            .mount(Id::GoTo, Box::new(GoTo::default()), vec![])
            .is_ok());
        // Mount global listener which will listen for <ESC>
        assert!(app
            .mount(
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
            .is_ok());
        // We need to give focus to input then
        assert!(app.active(&Id::FsTree).is_ok());
        Model {
            app,
            quit: false,
            redraw: true,
            tree: Tree::new(Self::dir_tree(p, MAX_DEPTH)),
            path: p.to_path_buf(),
            terminal: TerminalBridge::new().expect("Could not initialize terminal"),
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
        if let Some(node) = self.tree.root_mut().query_mut(id) {
            if depth > 0 && p.is_dir() {
                // Clear node
                node.clear();
                // Scan dir
                if let Ok(e) = std::fs::read_dir(p) {
                    e.flatten().for_each(|x| {
                        node.add_child(Self::dir_tree(x.path().as_path(), depth - 1))
                    });
                }
            }
        }
    }

    fn dir_tree(p: &Path, depth: usize) -> Node {
        let name: String = match p.file_name() {
            None => "/".to_string(),
            Some(n) => n.to_string_lossy().into_owned().to_string(),
        };
        let mut node: Node = Node::new(p.to_string_lossy().into_owned(), name);
        if depth > 0 && p.is_dir() {
            if let Ok(e) = std::fs::read_dir(p) {
                e.flatten()
                    .for_each(|x| node.add_child(Self::dir_tree(x.path().as_path(), depth - 1)));
            }
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
                .split(f.size());
            self.app.view(&Id::FsTree, f, chunks[0]);
            self.app.view(&Id::GoTo, f, chunks[1]);
        });
    }

    fn reload_tree(&mut self) {
        let current_node = match self.app.state(&Id::FsTree).ok().unwrap() {
            State::One(StateValue::String(id)) => Some(id),
            _ => None,
        };
        // Remount tree
        assert!(self.app.umount(&Id::FsTree).is_ok());
        assert!(self
            .app
            .mount(
                Id::FsTree,
                Box::new(FsTree::new(self.tree.clone(), current_node)),
                vec![]
            )
            .is_ok());
        assert!(self.app.active(&Id::FsTree).is_ok());
    }
}

fn main() {
    // Make model
    let mut model: Model = Model::new(std::env::current_dir().ok().unwrap().as_path());
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
            Msg::ExtendDir(path) => {
                self.extend_dir(&path, PathBuf::from(path.as_str()).as_path(), MAX_DEPTH);
                self.reload_tree();
                None
            }
            Msg::GoTo(path) => {
                // Go to and reload tree
                self.scan_dir(path.as_path());
                self.reload_tree();
                None
            }
            Msg::GoToUpperDir => {
                if let Some(parent) = self.upper_dir() {
                    self.scan_dir(parent.as_path());
                    self.reload_tree();
                }
                None
            }
            Msg::FsTreeBlur => {
                assert!(self.app.active(&Id::GoTo).is_ok());
                None
            }
            Msg::GoToBlur => {
                assert!(self.app.active(&Id::FsTree).is_ok());
                None
            }
            Msg::None => None,
        }
    }
}

// -- components

#[derive(MockComponent)]
pub struct FsTree {
    component: TreeView,
}

impl FsTree {
    pub fn new(tree: Tree, initial_node: Option<String>) -> Self {
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
                .inactive(Style::default().fg(Color::Gray))
                .indent_size(3)
                .scroll_step(6)
                .title(tree.root().id(), Alignment::Left)
                .highlighted_color(Color::LightYellow)
                .highlight_symbol("🦄")
                .with_tree(tree)
                .initial_node(initial_node),
        }
    }
}

impl Component<Msg, NoUserEvent> for FsTree {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let result = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Custom(TREE_CMD_CLOSE)),
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Custom(TREE_CMD_OPEN)),
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Move(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Home,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent {
                code: Key::End,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::GoTo(Position::End)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Submit),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::GoToUpperDir),
            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::FsTreeBlur),
            _ => return None,
        };
        match result {
            CmdResult::Submit(State::One(StateValue::String(node))) => Some(Msg::ExtendDir(node)),
            _ => Some(Msg::None),
        }
    }
}

// -- global listener

#[derive(Default, MockComponent)]
pub struct GlobalListener {
    component: Phantom,
}

impl Component<Msg, NoUserEvent> for GlobalListener {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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

#[derive(MockComponent)]
pub struct GoTo {
    component: Input,
}

impl Default for GoTo {
    fn default() -> Self {
        Self {
            component: Input::default()
                .foreground(Color::LightBlue)
                .borders(
                    Borders::default()
                        .color(Color::LightBlue)
                        .modifiers(BorderType::Rounded),
                )
                .input_type(InputType::Text)
                .placeholder(
                    "/foo/bar/buzz",
                    Style::default().fg(Color::Rgb(120, 120, 120)),
                )
                .title("Go to...", Alignment::Left),
        }
    }
}

impl Component<Msg, NoUserEvent> for GoTo {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let result = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
            }) => {
                let res = self.perform(Cmd::Submit);
                // Clear value
                self.attr(Attribute::Value, AttrValue::String(String::new()));
                res
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Type(ch)),
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Home,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent {
                code: Key::End,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::GoTo(Position::End)),
            Event::Keyboard(KeyEvent {
                code: Key::Delete,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Cancel),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Delete),
            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::GoToBlur),
            _ => return None,
        };
        match result {
            CmdResult::Submit(State::One(StateValue::String(path))) => {
                Some(Msg::GoTo(PathBuf::from(path.as_str())))
            }
            _ => Some(Msg::None),
        }
    }
}
