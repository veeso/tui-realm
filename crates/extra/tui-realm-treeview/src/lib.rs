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

// modules
mod tree_state;
mod widget;
// internal
use tree_state::TreeState;
use widget::TreeWidget;
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

    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn inactive(mut self, s: Style) -> Self {
        self.attr(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }

    pub fn initial_node<S: AsRef<str>>(mut self, node: S) -> Self {
        self.attr(
            Attribute::Custom(TREE_INITIAL_NODE),
            AttrValue::String(node.as_ref().to_string()),
        );
        self
    }

    pub fn preserve_state(mut self, preserve: bool) -> Self {
        self.attr(
            Attribute::Custom(TREE_PRESERVE_STATE),
            AttrValue::Flag(preserve),
        );
        self
    }

    pub fn indent_size(mut self, sz: u16) -> Self {
        self.attr(Attribute::Custom(TREE_INDENT_SIZE), AttrValue::Size(sz));
        self
    }

    pub fn scroll_step(mut self, step: usize) -> Self {
        self.attr(Attribute::ScrollStep, AttrValue::Length(step));
        self
    }

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
            let (hg_fg, hg_bg): (Color, Color) = match focus {
                true => (background, hg_color),
                false => (hg_color, background),
            };
            let hg_str = self
                .props
                .get(Attribute::HighlightedStr)
                .map(|x| x.unwrap_string());
            let div = Self::get_block(borders, Some(title), focus, inactive_style);
            // Make widget
            let mut tree = TreeWidget::new(self.tree())
                .block(div)
                .highlight_style(Style::default().fg(hg_fg).bg(hg_bg).add_modifier(modifiers))
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
                self.states.select(node);
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
                // TODO: impl
                todo!();
                self.changed(prev.as_deref())
            }
            Cmd::GoTo(Position::End) => {
                let prev = self.states.selected().map(|x| x.to_string());
                // TODO: impl
                todo!();
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
            Cmd::Toggle => {
                // Open/close selected node
                if let Some(selected) = self.states.selected() {
                    if let Some(node) = self.tree.root().query(&selected.to_string()) {
                        if self.states.is_closed(node) {
                            self.states.open_node(node);
                        } else {
                            self.states.close_node(node);
                        }
                    }
                }
                CmdResult::None
            }
            _ => CmdResult::None,
        }
    }
}
