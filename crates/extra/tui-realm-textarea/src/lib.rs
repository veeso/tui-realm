//! # tui-realm-textarea
//!
//! [tui-realm-textarea](https://github.com/veeso/tui-realm-textarea) is a
//! [tui-realm](https://github.com/veeso/tui-realm) implementation of a textarea component.
//! The tree engine is based on [Orange-trees](https://docs.rs/orange-trees/).
//!
//! ## Get Started
//!
//! ### Adding `tui-realm-textarea` as dependency
//!
//! ```toml
//! tui-realm-textarea = "^1.0.0"
//! ```
//!
//! Or if you don't use **Crossterm**, define the backend as you would do with tui-realm:
//!
//! ```toml
//! tui-realm-textarea = { version = "^1.0.0", default-features = false, features = [ "with-termion" ] }
//! ```
//!
//! ## Component API
//!
//! **Commands**:
//!
//! | Cmd                       | Result           | Behaviour                                            |
//! |---------------------------|------------------|------------------------------------------------------|
//! | `Custom($TREE_CMD_CLOSE)` | `None`           | Close selected node                                  |
//! | `Custom($TREE_CMD_OPEN)`  | `None`           | Open selected node                                   |
//! | `GoTo(Begin)`             | `Changed | None` | Move cursor to the top of the current tree node      |
//! | `GoTo(End)`               | `Changed | None` | Move cursor to the bottom of the current tree node   |
//! | `Move(Down)`              | `Changed | None` | Go to next element                                   |
//! | `Move(Up)`                | `Changed | None` | Go to previous element                               |
//! | `Scroll(Down)`            | `Changed | None` | Move cursor down by defined max steps or end of node |
//! | `Scroll(Up)`              | `Changed | None` | Move cursor up by defined max steps or begin of node |
//! | `Submit`                  | `Submit`         | Just returns submit result with current state        |
//!
//! **State**: the state returned is a `One(String)` containing the id of the selected node. If no node is selected `None` is returned.
//!
//! **Properties**:
//!
//! - `Background(Color)`: background color. The background color will be used as background for unselected entry, but will be used as foreground for the selected entry when focus is true
//! - `Borders(Borders)`: set borders properties for component
//! - `Custom($TREE_IDENT_SIZE, Size)`: Set space to render for each each depth level
//! - `Custom($TREE_INITIAL_NODE, String)`: Select initial node in the tree. This option has priority over `keep_state`
//! - `Custom($TREE_PRESERVE_STATE, Flag)`: If true, the selected entry will be kept after an update of the tree (obviously if the entry still exists in the tree).
//! - `FocusStyle(Style)`: inactive style
//! - `Foreground(Color)`: foreground color. The foreground will be used as foreground for the selected item, when focus is false, otherwise as background
//! - `HighlightedColor(Color)`: The provided color will be used to highlight the selected node. `Foreground` will be used if unset.
//! - `HighlightedStr(String)`: The provided string will be displayed on the left side of the selected entry in the tree
//! - `ScrollStep(Length)`: Defines the maximum amount of rows to scroll
//! - `TextProps(TextModifiers)`: set text modifiers
//! - `Title(Title)`: Set box title
//!
//! ### Updating the tree
//!
//! The tree in this component is not inside the `props`, but is a member of the `TreeView` mock component structure.
//! In order to update and work with the tree you've got basically two ways to do this.
//!
//! #### Remounting the component
//!
//! In situation where you need to update the tree on the update routine (as happens in the example),
//! the best way to update the tree is to remount the component from scratch.
//!
//! #### Updating the tree from the "on" method
//!
//! This method is probably better than remounting, but it is not always possible to use this.
//! When you implement `Component` for your textarea, you have a mutable reference to the component, and so here you can call these methods to operate on the tree:
//!
//! - `pub fn tree(&self) -> &Tree`: returns a reference to the tree
//! - `pub fn tree_mut(&mut self) -> &mut Tree`: returns a mutable reference to the tree; which allows you to operate on it
//! - `pub fn set_tree(&mut self, tree: Tree)`: update the current tree with another
//! - `pub fn tree_state(&self) -> &TreeState`: get a reference to the current tree state. (See tree state docs)
//!
//! You can access these methods from the `on()` method as said before. So these methods can be handy when you update the tree after a certain events or maybe even better, you can set the tree if you receive it from a `UserEvent` produced by a **Port**.
//!
//! ---
//!
//! ## Setup a tree component
//!
//! ```rust
//! extern crate tui_realm_textarea;
//! extern crate tuirealm;
//!
//! use tuirealm::{
//!     command::{Cmd, CmdResult, Direction, Position},
//!     event::{Event, Key, KeyEvent, KeyModifiers},
//!     props::{Alignment, BorderType, Borders, Color, Style},
//!     Component, MockComponent, NoUserEvent, State, StateValue,
//! };
//! // textarea
//! use tui_realm_textarea::{Node, Tree, TreeView, TREE_CMD_CLOSE, TREE_CMD_OPEN};
//!
//! #[derive(Debug, PartialEq)]
//! pub enum Msg {
//!     ExtendDir(String),
//!     GoToUpperDir,
//!     None,
//! }
//!
//! #[derive(MockComponent)]
//! pub struct FsTree {
//!     component: TreeView,
//! }
//!
//! impl FsTree {
//!     pub fn new(tree: Tree, initial_node: Option<String>) -> Self {
//!         // Preserve initial node if exists
//!         let initial_node = match initial_node {
//!             Some(id) if tree.root().query(&id).is_some() => id,
//!             _ => tree.root().id().to_string(),
//!         };
//!         FsTree {
//!             component: TreeView::default()
//!                 .foreground(Color::Reset)
//!                 .borders(
//!                     Borders::default()
//!                         .color(Color::LightYellow)
//!                         .modifiers(BorderType::Rounded),
//!                 )
//!                 .inactive(Style::default().fg(Color::Gray))
//!                 .indent_size(3)
//!                 .scroll_step(6)
//!                 .title(tree.root().id(), Alignment::Left)
//!                 .highlighted_color(Color::LightYellow)
//!                 .highlight_symbol("🦄")
//!                 .with_tree(tree)
//!                 .initial_node(initial_node),
//!         }
//!     }
//! }
//!
//! impl Component<Msg, NoUserEvent> for FsTree {
//!     fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
//!         let result = match ev {
//!             Event::Keyboard(KeyEvent {
//!                 code: Key::Left,
//!                 modifiers: KeyModifiers::NONE,
//!             }) => self.perform(Cmd::Custom(TREE_CMD_CLOSE)),
//!             Event::Keyboard(KeyEvent {
//!                 code: Key::Right,
//!                 modifiers: KeyModifiers::NONE,
//!             }) => self.perform(Cmd::Custom(TREE_CMD_OPEN)),
//!             Event::Keyboard(KeyEvent {
//!                 code: Key::PageDown,
//!                 modifiers: KeyModifiers::NONE,
//!             }) => self.perform(Cmd::Scroll(Direction::Down)),
//!             Event::Keyboard(KeyEvent {
//!                 code: Key::PageUp,
//!                 modifiers: KeyModifiers::NONE,
//!             }) => self.perform(Cmd::Scroll(Direction::Up)),
//!             Event::Keyboard(KeyEvent {
//!                 code: Key::Down,
//!                 modifiers: KeyModifiers::NONE,
//!             }) => self.perform(Cmd::Move(Direction::Down)),
//!             Event::Keyboard(KeyEvent {
//!                 code: Key::Up,
//!                 modifiers: KeyModifiers::NONE,
//!             }) => self.perform(Cmd::Move(Direction::Up)),
//!             Event::Keyboard(KeyEvent {
//!                 code: Key::Home,
//!                 modifiers: KeyModifiers::NONE,
//!             }) => self.perform(Cmd::GoTo(Position::Begin)),
//!             Event::Keyboard(KeyEvent {
//!                 code: Key::End,
//!                 modifiers: KeyModifiers::NONE,
//!             }) => self.perform(Cmd::GoTo(Position::End)),
//!             Event::Keyboard(KeyEvent {
//!                 code: Key::Enter,
//!                 modifiers: KeyModifiers::NONE,
//!             }) => self.perform(Cmd::Submit),
//!             Event::Keyboard(KeyEvent {
//!                 code: Key::Backspace,
//!                 modifiers: KeyModifiers::NONE,
//!             }) => return Some(Msg::GoToUpperDir),
//!             _ => return None,
//!         };
//!         match result {
//!             CmdResult::Submit(State::One(StateValue::String(node))) => Some(Msg::ExtendDir(node)),
//!             _ => Some(Msg::None),
//!         }
//!     }
//! }
//!
//! ```
//!
//! ---
//!
//! ## Tree widget
//!
//! If you want, you can also implement your own version of a tree view mock component using the `TreeWidget`
//! in order to render a tree.
//! Keep in mind that if you want to create a stateful tree (with highlighted item), you'll need to render it
//! as a stateful widget, passing to it a `TreeState`, which is provided by this library.
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]

// deps
use tui_textarea::TextArea as TextAreaWidget;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, Style,
    TextModifiers,
};
use tuirealm::tui::{layout::Rect, widgets::Block};
use tuirealm::{Frame, MockComponent, State, StateValue};

// -- props
pub const TEXTAREA_CURSOR_STYLE: &str = "cursor-style";
pub const TEXTAREA_CURSOR_LINE_STYLE: &str = "cursor-line-style";
pub const TEXTAREA_LINE_NUMBER_STYLE: &str = "line-number-style";
pub const TEXTAREA_MAX_HISTORY: &str = "max-history";
pub const TEXTAREA_TAB_SIZE: &str = "tab-size";

pub struct TextArea<'a> {
    props: Props,
    widget: TextAreaWidget<'a>,
}

impl<'a, I> From<I> for TextArea<'a>
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    fn from(i: I) -> Self {
        Self::new(i.into_iter().map(|s| s.into()).collect::<Vec<String>>())
    }
}

impl<'a> Default for TextArea<'a> {
    fn default() -> Self {
        Self::new(Vec::default())
    }
}

impl<'a> TextArea<'a> {
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            props: Props::default(),
            widget: TextAreaWidget::new(lines),
        }
    }

    /// Set widget foreground
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    /// Set widget background
    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    /// Set another style from default to use when component is inactive
    pub fn inactive(mut self, s: Style) -> Self {
        self.attr(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    /// Set widget border properties
    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    /// Set widget title
    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }

    /// Set how many modifications are remembered for undo/redo. Setting 0 disables undo/redo.
    pub fn max_histories(mut self, max: usize) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_MAX_HISTORY),
            AttrValue::Payload(PropPayload::One(PropValue::Usize(max))),
        );
        self
    }

    /// Set text editor cursor style
    pub fn cursor_style(mut self, s: Style) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_CURSOR_STYLE),
            AttrValue::Style(s),
        );
        self
    }

    /// Set text editor style for selected line
    pub fn cursor_line_style(mut self, s: Style) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_CURSOR_LINE_STYLE),
            AttrValue::Style(s),
        );
        self
    }

    /// Set text editor style for line numbers
    pub fn line_number_style(mut self, s: Style) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_LINE_NUMBER_STYLE),
            AttrValue::Style(s),
        );
        self
    }

    /// TODO: complete
    pub fn style(mut self, s: Style) -> Self {
        self.attr(Attribute::Style, AttrValue::Style(s));
        self
    }

    /// Set `<TAB>` size
    pub fn tab_length(mut self, l: u8) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_TAB_SIZE),
            AttrValue::Size(l as u16),
        );
        self
    }

    // TODO: status bar props (fmt?)

    // -- private
    fn get_block(&self) -> Block<'a> {
        todo!()
    }
}

impl<'a> MockComponent for TextArea<'a> {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        todo!()
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value.clone());
        match (attr, value) {
            (
                Attribute::Custom(TEXTAREA_MAX_HISTORY),
                AttrValue::Payload(PropPayload::One(PropValue::Usize(max))),
            ) => {
                self.widget.set_max_histories(max);
            }
            (Attribute::Custom(TEXTAREA_TAB_SIZE), AttrValue::Size(size)) => {
                self.widget.set_tab_length(size as u8);
            }
            (Attribute::Custom(TEXTAREA_CURSOR_STYLE), AttrValue::Style(s)) => {
                self.widget.set_cursor_style(s);
            }
            (Attribute::Custom(TEXTAREA_CURSOR_LINE_STYLE), AttrValue::Style(s)) => {
                self.widget.set_cursor_line_style(s);
            }
            (Attribute::Custom(TEXTAREA_LINE_NUMBER_STYLE), AttrValue::Style(s)) => {
                self.widget.set_line_number_style(s);
            }
            (Attribute::Style, AttrValue::Style(s)) => {
                self.widget.set_style(s);
            }
            (_, _) => {
                self.widget.set_block(self.get_block());
            }
        }
    }

    fn state(&self) -> State {
        State::Vec(
            self.widget
                .lines()
                .into_iter()
                .map(|x| StateValue::String(x.to_string()))
                .collect(),
        )
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        todo!()
    }
}
