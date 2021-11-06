//! # tui-realm-treeview
//!
//! [tui-realm-treeview](https://github.com/veeso/tui-realm-treeview) is a [tui-realm](https://github.com/veeso/tui-realm) implementation
//! of a treeview component.
//! The tree engine is based on [Orange-trees](https://docs.rs/orange-trees/).
//!
//! ## Get Started
//!
//! ### Adding `tui-realm-treeview` as dependency
//!
//! ```toml
//! tui-realm-treeview = "^1.0.0"
//! ```
//!
//! Or if you don't use **Crossterm**, define the backend as you do with tui-realm:
//!
//! ```toml
//! tui-realm-treeview = { version = "^1.0.0", default-features = false, features = [ "with-termion" ] }
//! ```
//!
//! ## Setup a tree component
//!
//! ```rust,no_run
//! extern crate tui_realm_treeview;
//! extern crate tuirealm;
//!
//! // TODO: example
//!
//! ```
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/tui-realm-treeview/main/docs/images/cargo/tui-realm-treeview-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/tui-realm-treeview/main/docs/images/cargo/tui-realm-treeview-512.png"
)]

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
extern crate orange_trees;
extern crate tuirealm;

// -- mock
#[cfg(test)]
pub(crate) mod mock;
// -- modules
mod tree_state;
mod widget;
// internal
pub use tree_state::TreeState;
pub use widget::TreeWidget;
// deps
pub use orange_trees::{Node as OrangeNode, Tree as OrangeTree};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, Props, Style, TextModifiers,
};
use tuirealm::tui::{layout::Rect, widgets::Block};
use tuirealm::{Frame, MockComponent, State, StateValue};

// -- type override
pub type Node = OrangeNode<String, String>;
pub type Tree = OrangeTree<String, String>;

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
pub struct TreeView {
    props: Props,
    states: TreeState,
    /// The actual Tree data structure. You can access this from your Component to operate on it
    /// for example after a certain events.
    tree: Tree,
}

impl Default for TreeView {
    fn default() -> Self {
        Self {
            props: Props::default(),
            states: TreeState::default(),
            tree: Tree::new(Node::new(String::new(), String::new())),
        }
    }
}

impl TreeView {
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
    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }

    /// ### highlight_symbol
    ///
    /// Set symbol to prepend to highlighted node
    pub fn highlight_symbol<S: AsRef<str>>(mut self, symbol: S) -> Self {
        self.attr(
            Attribute::HighlightedStr,
            AttrValue::String(symbol.as_ref().to_string()),
        );
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
    pub fn initial_node<S: AsRef<str>>(mut self, node: S) -> Self {
        self.attr(
            Attribute::Custom(TREE_INITIAL_NODE),
            AttrValue::String(node.as_ref().to_string()),
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
    pub fn with_tree(mut self, tree: Tree) -> Self {
        self.tree = tree;
        self
    }

    // -- tree methods

    /// ### tree
    ///
    /// Get a reference to tree
    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    /// ### tree_mut
    ///
    /// Get mutable reference to tree
    pub fn tree_mut(&mut self) -> &mut Tree {
        &mut self.tree
    }

    /// ### set_tree
    ///
    /// Set new tree in component.
    /// Current state is preserved if `PRESERVE_STATE` is set to `AttrValue::Flag(true)`
    pub fn set_tree(&mut self, tree: Tree) {
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

    fn get_block<'a>(
        props: Borders,
        title: Option<(String, Alignment)>,
        focus: bool,
        inactive_style: Option<Style>,
    ) -> Block<'a> {
        let title = title.unwrap_or((String::default(), Alignment::Left));
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

impl MockComponent for TreeView {
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
                .get_or(
                    Attribute::Title,
                    AttrValue::Title((String::default(), Alignment::Center)),
                )
                .unwrap_title();
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
                .get(Attribute::HighlightedStr)
                .map(|x| x.unwrap_string());
            let div = Self::get_block(borders, Some(title), focus, inactive_style);
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
                tree = tree.highlight_symbol(hg_str);
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
