//! # tui-realm-treeview
//!
//! [tui-realm-treeview](https://github.com/veeso/tui-realm-treeview) is a
//! [tui-realm](https://github.com/veeso/tui-realm) implementation of a treeview component.
//! The tree engine is based on [Orange-trees](https://docs.rs/orange-trees/).
//!
//! ## Get Started
//!
//! ### Adding `tui-realm-treeview` as dependency
//!
//! ```toml
//! tui-realm-treeview = "2"
//! ```
//!
//! Or if you don't use **Crossterm**, define the backend as you would do with tui-realm:
//!
//! ```toml
//! tui-realm-treeview = { version = "2", default-features = false, features = [ "termion" ] }
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
//! When you implement `Component` for your treeview, you have a mutable reference to the component, and so here you can call these methods to operate on the tree:
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
//! extern crate tui_realm_treeview;
//! extern crate tuirealm;
//!
//! use tuirealm::{
//!     command::{Cmd, CmdResult, Direction, Position},
//!     event::{Event, Key, KeyEvent, KeyModifiers},
//!     props::{Alignment, BorderType, Borders, Color, Style},
//!     Component, MockComponent, NoUserEvent, State, StateValue,
//! };
//! // treeview
//! use tui_realm_treeview::{Node, Tree, TreeView, TREE_CMD_CLOSE, TREE_CMD_OPEN};
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
//!     component: TreeView<String>,
//! }
//!
//! impl FsTree {
//!     pub fn new(tree: Tree<String>, initial_node: Option<String>) -> Self {
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
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/tui-realm-treeview/main/docs/images/cargo/tui-realm-treeview-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/tui-realm-treeview/main/docs/images/cargo/tui-realm-treeview-512.png"
)]

// -- mock
#[cfg(test)]
pub(crate) mod mock;
// -- modules
mod tree_state;
mod widget;

use std::iter;
// internal
pub use tree_state::TreeState;
pub use widget::TreeWidget;
// deps
pub use orange_trees::{Node as OrangeNode, Tree as OrangeTree};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, Props, Style, TextModifiers, TextSpan,
};
use tuirealm::ratatui::{layout::Rect, widgets::Block};
use tuirealm::{Frame, MockComponent, State, StateValue};

/// Tree node value.
pub trait NodeValue: Default {
    /// Return iterator over render parts - text with it style.
    /// If style is `None`, then it will be inherited from widget style.
    fn render_parts_iter(&self) -> impl Iterator<Item = (&str, Option<Style>)>;
}

impl NodeValue for String {
    fn render_parts_iter(&self) -> impl Iterator<Item = (&str, Option<Style>)> {
        iter::once((self.as_str(), None))
    }
}

impl NodeValue for Vec<TextSpan> {
    fn render_parts_iter(&self) -> impl Iterator<Item = (&str, Option<Style>)> {
        self.iter().map(|span| {
            (
                span.content.as_str(),
                Some(
                    Style::new()
                        .fg(span.fg)
                        .bg(span.bg)
                        .add_modifier(span.modifiers),
                ),
            )
        })
    }
}

// -- type override
pub type Node<V> = OrangeNode<String, V>;
pub type Tree<V> = OrangeTree<String, V>;

// -- props

pub const TREE_INDENT_SIZE: &str = "indent-size";
pub const TREE_INITIAL_NODE: &str = "initial-mode";
pub const TREE_PRESERVE_STATE: &str = "preserve-state";

// -- Cmd

pub const TREE_CMD_OPEN: &str = "o";
pub const TREE_CMD_CLOSE: &str = "c";

// -- component

/// ## TreeView
///
/// Tree view Mock component for tui-realm
pub struct TreeView<V: NodeValue> {
    props: Props,
    states: TreeState,
    /// The actual Tree data structure. You can access this from your Component to operate on it
    /// for example after a certain events.
    tree: Tree<V>,
}

impl<V: NodeValue> Default for TreeView<V> {
    fn default() -> Self {
        Self {
            props: Props::default(),
            states: TreeState::default(),
            tree: Tree::new(Node::new(String::new(), V::default())),
        }
    }
}

impl<V: NodeValue> TreeView<V> {
    // -- constructors

    /// ### foreground
    ///
    /// Set widget foreground
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    /// ### background
    ///
    /// Set widget background
    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    /// ### inactive
    ///
    /// Set another style from default to use when component is inactive
    pub fn inactive(mut self, s: Style) -> Self {
        self.attr(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    /// ### borders
    ///
    /// Set widget border properties
    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    /// ### modifiers
    ///
    /// Set widget text modifiers
    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    /// ### title
    ///
    /// Set widget title
    pub fn title<S: Into<String>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(Attribute::Title, AttrValue::Title((t.into(), a)));
        self
    }

    /// ### highlight_symbol
    ///
    /// Set symbol to prepend to highlighted node
    pub fn highlight_symbol<S: Into<String>>(mut self, symbol: S) -> Self {
        self.attr(Attribute::HighlightedStr, AttrValue::String(symbol.into()));
        self
    }

    /// ### highlighted_color
    ///
    /// Set color to apply to highlighted item
    pub fn highlighted_color(mut self, color: Color) -> Self {
        self.attr(Attribute::HighlightedColor, AttrValue::Color(color));
        self
    }

    /// ### initial_node
    ///
    /// Set initial node for tree state.
    /// NOTE: this must be specified after `with_tree`
    pub fn initial_node<S: Into<String>>(mut self, node: S) -> Self {
        self.attr(
            Attribute::Custom(TREE_INITIAL_NODE),
            AttrValue::String(node.into()),
        );
        self
    }

    /// ### preserve_state
    ///
    /// Set whether to preserve state on tree change
    pub fn preserve_state(mut self, preserve: bool) -> Self {
        self.attr(
            Attribute::Custom(TREE_PRESERVE_STATE),
            AttrValue::Flag(preserve),
        );
        self
    }

    /// ### indent_size
    ///
    /// Set indent size for widget for each level of depth
    pub fn indent_size(mut self, sz: u16) -> Self {
        self.attr(Attribute::Custom(TREE_INDENT_SIZE), AttrValue::Size(sz));
        self
    }

    /// ### scroll_step
    ///
    /// Set scroll step for scrolling command
    pub fn scroll_step(mut self, step: usize) -> Self {
        self.attr(Attribute::ScrollStep, AttrValue::Length(step));
        self
    }

    /// ### with_tree
    ///
    /// Set tree to use as data
    pub fn with_tree(mut self, tree: Tree<V>) -> Self {
        self.tree = tree;
        self
    }

    // -- tree methods

    /// ### tree
    ///
    /// Get a reference to tree
    pub fn tree(&self) -> &Tree<V> {
        &self.tree
    }

    /// ### tree_mut
    ///
    /// Get mutable reference to tree
    pub fn tree_mut(&mut self) -> &mut Tree<V> {
        &mut self.tree
    }

    /// ### set_tree
    ///
    /// Set new tree in component.
    /// Current state is preserved if `PRESERVE_STATE` is set to `AttrValue::Flag(true)`
    pub fn set_tree(&mut self, tree: Tree<V>) {
        self.tree = tree;
        self.states.tree_changed(
            self.tree.root(),
            self.props
                .get_or(
                    Attribute::Custom(TREE_PRESERVE_STATE),
                    AttrValue::Flag(false),
                )
                .unwrap_flag(),
        );
    }

    /// ### tree_state
    ///
    /// Get a reference to the current tree state
    pub fn tree_state(&self) -> &TreeState {
        &self.states
    }

    // -- private

    /// ### changed
    ///
    /// Returns whether selectd node has changed
    fn changed(&self, prev: Option<&str>) -> CmdResult {
        match self.states.selected() {
            None => CmdResult::None,
            id if id != prev => CmdResult::Changed(self.state()),
            _ => CmdResult::None,
        }
    }

    fn get_block(
        props: Borders,
        title: (&str, Alignment),
        focus: bool,
        inactive_style: Option<Style>,
    ) -> Block<'_> {
        Block::default()
            .borders(props.sides)
            .border_style(match focus {
                true => props.style(),
                false => inactive_style
                    .unwrap_or_else(|| Style::default().fg(Color::Reset).bg(Color::Reset)),
            })
            .border_type(props.modifiers)
            .title(title.0)
            .title_alignment(title.1)
    }
}

// -- mock

impl<V: NodeValue> MockComponent for TreeView<V> {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();
            let title = self
                .props
                .get_ref(Attribute::Title)
                .and_then(|v| v.as_title())
                .map(|v| (v.0.as_str(), v.1))
                .unwrap_or(("", Alignment::Center));
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            let inactive_style = self
                .props
                .get(Attribute::FocusStyle)
                .map(|x| x.unwrap_style());
            let indent_size = self
                .props
                .get_or(Attribute::Custom(TREE_INDENT_SIZE), AttrValue::Size(4))
                .unwrap_size();
            let hg_color = self
                .props
                .get_or(Attribute::HighlightedColor, AttrValue::Color(foreground))
                .unwrap_color();
            let hg_style = match focus {
                true => Style::default().bg(hg_color).fg(Color::Black),
                false => Style::default().fg(hg_color),
            }
            .add_modifier(modifiers);
            let hg_str = self
                .props
                .get_ref(Attribute::HighlightedStr)
                .and_then(|x| x.as_string());
            let div = Self::get_block(borders, title, focus, inactive_style);
            // Make widget
            let mut tree = TreeWidget::new(self.tree())
                .block(div)
                .highlight_style(hg_style)
                .indent_size(indent_size.into())
                .style(
                    Style::default()
                        .fg(foreground)
                        .bg(background)
                        .add_modifier(modifiers),
                );
            if let Some(hg_str) = hg_str {
                tree = tree.highlight_symbol(hg_str.as_str());
            }
            let mut state = self.states.clone();
            frame.render_stateful_widget(tree, area, &mut state);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        // Initial node
        if matches!(attr, Attribute::Custom(TREE_INITIAL_NODE)) {
            // Select node if exists
            if let Some(node) = self.tree.root().query(&value.unwrap_string()) {
                self.states.select(self.tree.root(), node);
            }
        } else {
            self.props.set(attr, value);
        }
    }

    fn state(&self) -> State {
        match self.states.selected() {
            None => State::None,
            Some(id) => State::One(StateValue::String(id.to_string())),
        }
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::GoTo(Position::Begin) => {
                let prev = self.states.selected().map(|x| x.to_string());
                // Get first sibling of current node
                if let Some(first) = self.states.first_sibling(self.tree.root()) {
                    self.states.select(self.tree.root(), first);
                }
                self.changed(prev.as_deref())
            }
            Cmd::GoTo(Position::End) => {
                let prev = self.states.selected().map(|x| x.to_string());
                // Get first sibling of current node
                if let Some(last) = self.states.last_sibling(self.tree.root()) {
                    self.states.select(self.tree.root(), last);
                }
                self.changed(prev.as_deref())
            }
            Cmd::Move(Direction::Down) => {
                let prev = self.states.selected().map(|x| x.to_string());
                self.states.move_down(self.tree.root());
                self.changed(prev.as_deref())
            }
            Cmd::Move(Direction::Up) => {
                let prev = self.states.selected().map(|x| x.to_string());
                self.states.move_up(self.tree.root());
                self.changed(prev.as_deref())
            }
            Cmd::Scroll(Direction::Down) => {
                let prev = self.states.selected().map(|x| x.to_string());
                let step = self
                    .props
                    .get_or(Attribute::ScrollStep, AttrValue::Length(8))
                    .unwrap_length();
                (0..step).for_each(|_| self.states.move_down(self.tree.root()));
                self.changed(prev.as_deref())
            }
            Cmd::Scroll(Direction::Up) => {
                let prev = self.states.selected().map(|x| x.to_string());
                let step = self
                    .props
                    .get_or(Attribute::ScrollStep, AttrValue::Length(8))
                    .unwrap_length();
                (0..step).for_each(|_| self.states.move_up(self.tree.root()));
                self.changed(prev.as_deref())
            }
            Cmd::Submit => CmdResult::Submit(self.state()),
            Cmd::Custom(TREE_CMD_CLOSE) => {
                // close selected node
                self.states.close(self.tree.root());
                CmdResult::None
            }
            Cmd::Custom(TREE_CMD_OPEN) => {
                // close selected node
                self.states.open(self.tree.root());
                CmdResult::None
            }
            _ => CmdResult::None,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::mock_tree;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_initialize_component() {
        let mut component = TreeView::default()
            .background(Color::White)
            .foreground(Color::Cyan)
            .borders(Borders::default())
            .inactive(Style::default())
            .indent_size(4)
            .modifiers(TextModifiers::all())
            .preserve_state(true)
            .scroll_step(4)
            .title("My tree", Alignment::Center)
            .with_tree(mock_tree())
            .initial_node("aB1");
        // Check tree
        assert_eq!(component.tree_state().selected().unwrap(), "aB1");
        assert!(component.tree().root().query(&String::from("aB")).is_some());
        component
            .tree_mut()
            .root_mut()
            .add_child(Node::new(String::from("d"), String::from("d")));
    }

    #[test]
    fn should_return_consistent_state() {
        let component = TreeView::default().with_tree(mock_tree());
        assert_eq!(component.state(), State::None);
        let component = TreeView::default()
            .with_tree(mock_tree())
            .initial_node("aA");
        assert_eq!(
            component.state(),
            State::One(StateValue::String(String::from("aA")))
        );
    }

    #[test]
    fn should_perform_go_to_begin() {
        let mut component = TreeView::default()
            .with_tree(mock_tree())
            .initial_node("bB3");
        // GoTo begin (changed)
        assert_eq!(
            component.perform(Cmd::GoTo(Position::Begin)),
            CmdResult::Changed(State::One(StateValue::String(String::from("bB0"))))
        );
        // GoTo begin (unchanged)
        assert_eq!(
            component.perform(Cmd::GoTo(Position::Begin)),
            CmdResult::None
        );
    }

    #[test]
    fn should_perform_go_to_end() {
        let mut component = TreeView::default()
            .with_tree(mock_tree())
            .initial_node("bB1");
        // GoTo end (changed)
        assert_eq!(
            component.perform(Cmd::GoTo(Position::End)),
            CmdResult::Changed(State::One(StateValue::String(String::from("bB5"))))
        );
        // GoTo end (unchanged)
        assert_eq!(component.perform(Cmd::GoTo(Position::End)), CmdResult::None);
    }

    #[test]
    fn should_perform_move_down() {
        let mut component = TreeView::default()
            .with_tree(mock_tree())
            .initial_node("cA1");
        // Move down (changed)
        assert_eq!(
            component.perform(Cmd::Move(Direction::Down)),
            CmdResult::Changed(State::One(StateValue::String(String::from("cA2"))))
        );
        // Move down (unchanged)
        assert_eq!(
            component.perform(Cmd::Move(Direction::Down)),
            CmdResult::None
        );
    }

    #[test]
    fn should_perform_move_up() {
        let mut component = TreeView::default().with_tree(mock_tree()).initial_node("a");
        // Move up (changed)
        assert_eq!(
            component.perform(Cmd::Move(Direction::Up)),
            CmdResult::Changed(State::One(StateValue::String(String::from("/"))))
        );
        // Move up (unchanged)
        assert_eq!(component.perform(Cmd::Move(Direction::Up)), CmdResult::None);
    }

    #[test]
    fn should_perform_scroll_down() {
        let mut component = TreeView::default()
            .scroll_step(2)
            .with_tree(mock_tree())
            .initial_node("cA0");
        // Scroll down (changed)
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Down)),
            CmdResult::Changed(State::One(StateValue::String(String::from("cA2"))))
        );
        // Scroll down (unchanged)
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Down)),
            CmdResult::None
        );
    }

    #[test]
    fn should_perform_scroll_up() {
        let mut component = TreeView::default()
            .scroll_step(4)
            .with_tree(mock_tree())
            .initial_node("aA1");
        // Scroll Up (changed)
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Up)),
            CmdResult::Changed(State::One(StateValue::String(String::from("/"))))
        );
        // Scroll Up (unchanged)
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Up)),
            CmdResult::None
        );
    }

    #[test]
    fn should_perform_submit() {
        let mut component = TreeView::default()
            .with_tree(mock_tree())
            .initial_node("aA1");
        assert_eq!(
            component.perform(Cmd::Submit),
            CmdResult::Submit(State::One(StateValue::String(String::from("aA1"))))
        );
    }

    #[test]
    fn should_perform_close() {
        let mut component = TreeView::default()
            .with_tree(mock_tree())
            .initial_node("aA1");
        component.states.open(component.tree.root());
        assert_eq!(
            component.perform(Cmd::Custom(TREE_CMD_CLOSE)),
            CmdResult::None
        );
        assert!(component
            .tree_state()
            .is_closed(component.tree().root().query(&String::from("aA1")).unwrap()));
    }

    #[test]
    fn should_perform_open() {
        let mut component = TreeView::default()
            .with_tree(mock_tree())
            .initial_node("aA");
        assert_eq!(
            component.perform(Cmd::Custom(TREE_CMD_OPEN)),
            CmdResult::None
        );
        assert!(component
            .tree_state()
            .is_open(component.tree().root().query(&String::from("aA")).unwrap()));
    }

    #[test]
    fn should_update_tree() {
        let mut component = TreeView::default()
            .with_tree(mock_tree())
            .preserve_state(true)
            .initial_node("aA");
        // open 'bB'
        component.states.select(
            component.tree.root(),
            component.tree.root().query(&String::from("bB")).unwrap(),
        );
        component.states.open(component.tree.root());
        // re-selecte 'aA'
        component.states.select(
            component.tree.root(),
            component.tree.root().query(&String::from("aA")).unwrap(),
        );
        // Create new tree
        let mut new_tree = mock_tree();
        new_tree.root_mut().remove_child(&String::from("a"));
        // Set new tree
        component.set_tree(new_tree);
        // selected item should be root
        assert_eq!(component.states.selected().unwrap(), "/");
    }
}
