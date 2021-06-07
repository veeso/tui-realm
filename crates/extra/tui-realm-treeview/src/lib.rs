//! # tui-realm-treeview
//!
//! [tui-realm-treeview](https://github.com/veeso/tui-realm-treeview) is a [tui-realm](https://github.com/veeso/tui-realm) implementation
//! of a treeview component
//!
//! ## Get Started
//!
//! ### Adding `tui-realm-treeview` as dependency
//!
//! ```toml
//! tui-realm-treeview = "0.2.0"
//! ```
//!
//! ## Setup a tree component
//!
//! ```rust,no_run
//! extern crate tui_realm_treeview;
//! extern crate tuirealm;
//!
//! use tuirealm::{props::Borders, PropsBuilder, Msg, Payload, Update, Value};
//! use tuirealm::tui::style::Color;
//! use tuirealm::tui::widgets::BorderType;
//! use tui_realm_treeview::{Node, Tree, TreeView, TreeViewPropsBuilder};
//!
//! const COMPONENT_TREEVIEW: &str = "TREEVIEW";
//!
//! pub struct Model;
//!
//! fn main() {
//!     let tree: Tree = Tree::new(
//!     Node::new("/", "/")
//!     .with_child(
//!         Node::new("/bin", "bin/")
//!             .with_child(Node::new("/bin/ls", "ls"))
//!             .with_child(Node::new("/bin/pwd", "pwd")),
//!     )
//!     .with_child(
//!         Node::new("/home", "home/").with_child(
//!             Node::new("/home/omar", "omar/")
//!                 .with_child(Node::new("/home/omar/readme.md", "readme.md"))
//!                 .with_child(Node::new("/home/omar/changelog.md", "changelog.md")),
//!         ),
//!     ),
//!     );
//!     let mut component: TreeView = TreeView::new(
//!         TreeViewPropsBuilder::default()
//!             .hidden()
//!             .visible()
//!             .with_borders(Borders::ALL, BorderType::Double, Color::LightYellow)
//!             .with_background(Color::Black)
//!             .with_foreground(Color::LightYellow)
//!             .with_title(Some(String::from("/dev/sda")))
//!             .with_highlighted_str("🚀")
//!             .with_tree(tree.root())
//!             .build(),
//!     );
//! }
//!
//! impl Update for Model {
//!     fn update(&mut self, msg: Option<(String, Msg)>) -> Option<(String, Msg)> {
//!         let ref_msg: Option<(&str, &Msg)> = msg.as_ref().map(|(s, msg)| (s.as_str(), msg));
//!         match ref_msg {
//!             None => None, // Exit after None
//!             Some(msg) => match msg {
//!                 (COMPONENT_TREEVIEW, Msg::OnChange(Payload::One(Value::Str(node_id)))) => {
//!                     println!("Moved to {}", node_id);
//!                     None
//!                 }
//!                 (COMPONENT_TREEVIEW, Msg::OnSubmit(Payload::One(Value::Str(node_id)))) => {
//!                     println!("Selected node {}", node_id);
//!                     None
//!                 }
//!                 _ => None,
//!             },
//!         }
//!     }
//! }
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
extern crate tui_tree_widget;
extern crate tuirealm;

// modules
mod serializer;
mod stateful_tree;
// internal
use stateful_tree::StatefulTree;
// deps
use tui_tree_widget::{Tree as TuiTree, TreeItem as TuiTreeItem, TreeState as TuiTreeState};
use tuirealm::tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
};
use tuirealm::{
    event::{Event, KeyCode},
    props::{BordersProps, TextParts, TextSpan},
    Canvas, Component, Msg, Payload, PropPayload, PropValue, Props, PropsBuilder, Value,
};

// -- structs

/// ## Tree
///
/// represent the tree data structure inside the component
#[derive(Debug)]
pub struct Tree {
    root: Node,
}

impl Tree {
    /// ### new
    ///
    /// Instantiates a new `Tree`
    pub fn new(root: Node) -> Self {
        Self { root }
    }

    /// ### root
    ///
    /// Returns a reference to the root node
    pub fn root(&self) -> &Node {
        &self.root
    }

    /// ### root_mut
    ///
    /// Returns a mutablen reference to the root node
    pub fn root_mut(&mut self) -> &mut Node {
        &mut self.root
    }

    /// ### query
    ///
    /// Query tree for a certain node
    pub fn query(&self, id: &str) -> Option<&Node> {
        self.root.query(id)
    }

    /// ### query_mut
    ///
    /// Query tree for a certain node and return it as a mutable reference
    pub fn query_mut(&mut self, id: &str) -> Option<&mut Node> {
        self.root.query_mut(id)
    }

    /// ### parent
    ///
    /// Get parent node of `id`
    pub fn parent(&self, id: &str) -> Option<&Node> {
        self.root().parent(id)
    }

    /// ### siblings
    ///
    /// Get siblings for provided node
    pub fn siblings(&self, id: &str) -> Option<Vec<&str>> {
        self.root().siblings(id)
    }

    /// ### node_by_route
    ///
    /// Get starting from root the node associated to the indexes.
    /// When starting from tree, the first element in route must be `0`
    pub fn node_by_route(&self, route: &[usize]) -> Option<&Node> {
        if route.is_empty() {
            None
        } else {
            self.root().node_by_route(&route[1..])
        }
    }

    /// ### route_by_node
    ///
    /// Calculate the route of a node by its id
    pub fn route_by_node(&self, id: &str) -> Option<Vec<usize>> {
        match self.root().route_by_node(id) {
            None => None,
            Some(route) => {
                let mut r: Vec<usize> = vec![0];
                r.extend(route);
                Some(r)
            }
        }
    }
}

/// ## Node
///
/// Describes a node inside the `Tree`
#[derive(Debug)]
pub struct Node {
    id: String,    // Must uniquely identify the node in the tree
    label: String, // The text to display associated to the node
    pub(crate) children: Vec<Node>,
}

impl Node {
    /// ### new
    ///
    /// Instantiates a new `Node`
    /// ATTENTION: id mustn't be empty nor duplicated
    pub fn new<S: AsRef<str>>(id: S, label: S) -> Self {
        Self {
            id: id.as_ref().to_string(),
            label: label.as_ref().to_string(),
            children: vec![],
        }
    }

    /// ### id
    ///
    /// Get reference to id
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// ### label
    ///
    /// Get reference to label
    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    /// ### with_children
    ///
    /// Sets Node children
    pub fn with_children(mut self, children: Vec<Node>) -> Self {
        self.children = children;
        self
    }

    /// ### with_child
    ///
    /// Create a new child in this Node
    pub fn with_child(mut self, child: Node) -> Self {
        self.add_child(child);
        self
    }

    // -- manipulation

    /// ### add_child
    ///
    /// Add a child to the node
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    /// ### clear
    ///
    /// Clear node children
    pub fn clear(&mut self) {
        self.children.clear();
    }

    /// ### truncate
    ///
    /// Truncate tree at depth.
    /// If depth is `0`, node's children will be cleared
    pub fn truncate(&mut self, depth: usize) {
        if depth == 0 {
            self.children.clear();
        } else {
            self.children.iter_mut().for_each(|x| x.truncate(depth - 1));
        }
    }

    // -- query

    /// ### is_leaf
    ///
    /// Returns whether this node is a leaf (which means it has no children)
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// ### query
    ///
    /// Search for `id` inside Node's children (or is itself)
    pub fn query(&self, id: &str) -> Option<&Self> {
        if self.id() == id {
            Some(&self)
        } else {
            // Recurse search
            self.children
                .iter()
                .map(|x| x.query(id))
                .filter(|x| x.is_some())
                .flatten()
                .next()
        }
    }

    /// ### query_mut
    ///
    /// Returns a mutable reference to a Node
    pub(self) fn query_mut(&mut self, id: &str) -> Option<&mut Self> {
        if self.id() == id {
            Some(self)
        } else {
            // Recurse search
            self.children
                .iter_mut()
                .map(|x| x.query_mut(id))
                .filter(|x| x.is_some())
                .flatten()
                .next()
        }
    }

    /// ### count
    ///
    /// Count items in tree
    pub fn count(&self) -> usize {
        self.children.iter().map(|x| x.count()).sum::<usize>() + 1
    }

    /// ### depth
    ///
    /// Calculate the maximum depth of the tree
    pub fn depth(&self) -> usize {
        /// ### depth_r
        ///
        /// Private recursive call for depth
        fn depth_r(ptr: &Node, depth: usize) -> usize {
            ptr.children
                .iter()
                .map(|x| depth_r(x, depth + 1))
                .max()
                .unwrap_or(depth)
        }
        depth_r(self, 1)
    }

    /// ### parent
    ///
    /// Get parent node of `id`
    pub fn parent(&self, id: &str) -> Option<&Self> {
        match self.route_by_node(id) {
            None => None,
            Some(route) => {
                // Get parent
                self.node_by_route(&route[0..route.len() - 1])
            }
        }
    }

    /// ### siblings
    ///
    /// Get siblings for provided node
    pub fn siblings(&self, id: &str) -> Option<Vec<&str>> {
        self.parent(id).map(|x| {
            x.children
                .iter()
                .filter(|&x| x.id() != id)
                .map(|x| x.id())
                .collect()
        })
    }

    /// ### node_by_route
    ///
    /// Given a vector of indexes, returns the node associated to the route
    pub fn node_by_route(&self, route: &[usize]) -> Option<&Self> {
        if route.is_empty() {
            Some(self)
        } else {
            let next: &Node = self.children.get(route[0])?;
            let route = &route[1..];
            next.node_by_route(route)
        }
    }

    /// ### route_by_node
    ///
    /// Calculate the route of a node by its id
    pub fn route_by_node(&self, id: &str) -> Option<Vec<usize>> {
        // Recursive function
        fn route_by_node_r(
            node: &Node,
            id: &str,
            enumerator: Option<usize>,
            mut route: Vec<usize>,
        ) -> Option<Vec<usize>> {
            if let Some(enumerator) = enumerator {
                route.push(enumerator);
            }
            if node.id() == id {
                // Found!!!
                Some(route)
            } else if node.children.is_empty() {
                // No more children
                route.pop(); // Pop previous entry
                None
            } else {
                // Keep searching
                let mut result: Option<Vec<usize>> = None;
                node.children.iter().enumerate().for_each(|(i, x)| {
                    let this_route: Vec<usize> = route.clone();
                    if let Some(this_route) = route_by_node_r(x, id, Some(i), this_route) {
                        result = Some(this_route);
                    }
                });
                result
            }
        }
        // Call recursive function
        route_by_node_r(self, id, None, Vec::with_capacity(self.depth()))
    }
}

// -- states

/// ## OwnStates
///
/// TreeView states
#[derive(Debug)]
struct OwnStates<'a> {
    focus: bool,
    tree: Tree,
    tui_tree: StatefulTree<'a>,
}

impl<'a> OwnStates<'a> {
    /// ### new
    ///
    /// Instantiates a new OwnStates from tree data
    pub fn new(tree: Tree, initial_id: Option<&str>) -> Self {
        let mut component: Self = Self {
            focus: false,
            tui_tree: StatefulTree::from(&tree),
            tree,
        };
        component.open_first();
        // Init id
        if let Some(id) = initial_id {
            component.set_node(id);
        }
        component
    }

    // -- getters

    /// ### focus
    ///
    /// Get focus
    pub fn focus(&self) -> bool {
        self.focus
    }

    /// get_selected_node
    ///
    /// Get selected node in the tree
    pub fn selected_node(&self) -> Option<&Node> {
        let route: Vec<usize> = self.tui_tree.selected();
        self.tree.node_by_route(route.as_slice())
    }

    /// ### get_tui_tree
    ///
    /// Generate tui tree from self tree
    pub fn get_tui_tree(&self) -> TuiTree {
        TuiTree::new(self.tui_tree.items.clone())
    }

    /// ### get_tui_tree_state
    ///
    /// Get tui tree state
    pub fn get_tui_tree_state(&self) -> TuiTreeState {
        self.tui_tree.state.clone()
    }

    // -- api

    /// ### toggle_focus
    ///
    /// Set focus
    pub fn toggle_focus(&mut self, focus: bool) {
        self.focus = focus;
    }

    /// ### next
    ///
    /// Go to next the element in tree
    pub fn next(&mut self) {
        self.tui_tree.next();
    }

    /// ### previous
    ///
    /// Go to the previous element in the tree
    pub fn previous(&mut self) {
        self.tui_tree.previous();
    }

    /// ### open
    ///
    /// Open selected element in the tree
    pub fn open(&mut self) {
        self.tui_tree.open();
    }

    /// ### close
    ///
    /// Close seelected element in the tree
    pub fn close(&mut self) {
        self.tui_tree.close();
    }

    fn open_first(&mut self) {
        self.next();
        self.open();
    }

    // -- setters

    /// ### update_tree
    ///
    /// Update states tree
    pub fn update_tree(&mut self, tree: Tree, keep_state: bool, initial_id: Option<&str>) {
        // let route: Vec<usize> = self.tui_tree.selected(); NOTE: restore to track state
        let selected: Option<String> = self.selected_node().map(|x| x.id().to_string());
        self.tui_tree = StatefulTree::from(&tree);
        self.tree = tree;
        // Open first
        self.open_first();
        // Restore state if required (initial id mustn't be set)
        if keep_state && initial_id.is_none() {
            if let Some(selected) = selected {
                if let Some(route) = self.tree.route_by_node(selected.as_str()) {
                    self.tui_tree.set_state(route.as_slice());
                }
            }
        }
        // Set initial node
        if let Some(id) = initial_id {
            self.set_node(id);
        }
    }

    /// ### set_node
    ///
    /// Try to set node to id
    fn set_node(&mut self, id: &str) {
        if let Some(route) = self.tree.route_by_node(id) {
            self.tui_tree.set_state(route.as_slice());
        }
    }
}

// -- props

const PROP_TREE: &str = "tree";
const PROP_INITIAL_NODE: &str = "initial_node";
const PROP_KEEP_STATE: &str = "keep_state";

/// ## TreeViewPropsBuilder
///
/// Tree View properties builder
pub struct TreeViewPropsBuilder {
    props: Option<Props>,
}

impl Default for TreeViewPropsBuilder {
    fn default() -> Self {
        Self {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for TreeViewPropsBuilder {
    fn build(&mut self) -> Props {
        self.props.take().unwrap()
    }

    fn hidden(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = false;
        }
        self
    }

    fn visible(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = true;
        }
        self
    }
}

impl From<Props> for TreeViewPropsBuilder {
    fn from(props: Props) -> Self {
        TreeViewPropsBuilder { props: Some(props) }
    }
}

impl TreeViewPropsBuilder {
    /// ### with_foreground
    ///
    /// Set foreground. The foreground will be used as foreground for the selected item, when focus is false, otherwise as background
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    /// ### with_background
    ///
    /// Set background. The background color will be used as background for unselected entry, but will be used as foreground for the selected entry
    /// when focus is true
    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.background = color;
        }
        self
    }

    /// ### with_borders
    ///
    /// Set component borders style
    pub fn with_borders(
        &mut self,
        borders: Borders,
        variant: BorderType,
        color: Color,
    ) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.borders = BordersProps {
                borders,
                variant,
                color,
            }
        }
        self
    }

    /// ### with_title
    ///
    /// Set box title
    pub fn with_title(&mut self, title: Option<String>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            let spans = props.texts.spans.clone();
            props.texts = TextParts::new(title, spans);
        }
        self
    }

    /// ### with_highlighted_str
    ///
    /// The provided string will be displayed on the left side of the selected entry in the tree
    pub fn with_highlighted_str(&mut self, s: &str) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            let title = props.texts.title.clone();
            let spans = vec![TextSpan::from(s)];
            props.texts = TextParts::new(title, Some(spans));
        }
        self
    }

    /// ### with_tree_and_depth
    ///
    /// Sets the tree and its max depth for Props builder
    pub fn with_tree_and_depth(&mut self, root: &Node, depth: usize) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(PROP_TREE, root.to_prop_payload(depth, ""));
        }
        self
    }

    /// ### with_tree
    ///
    /// Sets the tree for Props builder
    pub fn with_tree(&mut self, root: &Node) -> &mut Self {
        self.with_tree_and_depth(root, usize::MAX)
    }

    /// ### with_node
    ///
    /// Select initial node in the tree.
    /// NOTE: this option has priority over `keep_state`
    pub fn with_node(&mut self, id: Option<&str>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            match id {
                Some(id) => props.own.insert(
                    PROP_INITIAL_NODE,
                    PropPayload::One(PropValue::Str(id.to_string())),
                ),
                None => props.own.remove(PROP_INITIAL_NODE),
            };
        }
        self
    }

    /// ### keep_state
    ///
    /// If keep is true, the selected entry will be kept after an update of the tree (obviously if the entry still exists in the tree).
    /// NOTE: this property has lower property than `with_node`
    pub fn keep_state(&mut self, keep: bool) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_KEEP_STATE, PropPayload::One(PropValue::Bool(keep)));
        }
        self
    }
}

// -- component

/// ## TreeView
///
/// Tree view tui-realm component
pub struct TreeView<'a> {
    props: Props,
    states: OwnStates<'a>,
}

impl<'a> TreeView<'a> {
    /// ### new
    ///
    /// Instantiate a new Checkbox Group component
    pub fn new(props: Props) -> Self {
        // Make states
        let tree: Tree = match props.own.get(PROP_TREE) {
            Some(tree) => Tree::from(tree),
            None => Tree::new(Node::new("", "")),
        };
        let initial_id: Option<&str> = match props.own.get(PROP_INITIAL_NODE) {
            Some(PropPayload::One(PropValue::Str(id))) => Some(id.as_str()),
            _ => None,
        };
        let states: OwnStates = OwnStates::new(tree, initial_id);
        TreeView { props, states }
    }
    /// ### get_block
    ///
    /// Get block
    pub fn get_block(&self) -> Block<'a> {
        let div: Block = Block::default()
            .borders(self.props.borders.borders)
            .border_style(match self.states.focus() {
                true => self.props.borders.style(),
                false => Style::default(),
            })
            .border_type(self.props.borders.variant);
        // Set title
        match self.props.texts.title.as_ref() {
            Some(t) => div.title(t.to_string()),
            None => div,
        }
    }
}

impl<'a> Component for TreeView<'a> {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    fn render(&self, render: &mut Canvas, area: Rect) {
        if self.props.visible {
            // Make colors
            let (bg, fg): (Color, Color) = match &self.states.focus {
                true => (self.props.foreground, self.props.background),
                false => (Color::Reset, self.props.foreground),
            };
            let block: Block = self.get_block();
            let mut tree: TuiTree = self
                .states
                .get_tui_tree()
                .block(block)
                .highlight_style(Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD));
            // Highlighted symbol
            if let Some(spans) = self.props.texts.spans.as_ref() {
                if let Some(span) = spans.get(0) {
                    tree = tree.highlight_symbol(&span.content);
                }
            }
            render.render_stateful_widget(tree, area, &mut self.states.get_tui_tree_state());
        }
    }

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update.
    /// Returns a Msg to the view
    fn update(&mut self, props: Props) -> Msg {
        let prev_selection: Option<String> =
            self.states.selected_node().map(|x| x.id().to_string());
        // make tree
        let tree: Tree = match props.own.get(PROP_TREE) {
            Some(tree) => Tree::from(tree),
            None => Tree::new(Node::new("", "")),
        };
        // Update
        let keep_state: bool = matches!(
            props.own.get(PROP_KEEP_STATE),
            Some(PropPayload::One(PropValue::Bool(true)))
        );
        let initial_id: Option<&str> = match props.own.get(PROP_INITIAL_NODE) {
            Some(PropPayload::One(PropValue::Str(id))) => Some(id.as_str()),
            _ => None,
        };
        self.states.update_tree(tree, keep_state, initial_id);
        let new_selection: Option<String> = self.states.selected_node().map(|x| x.id().to_string());
        // Set props
        self.props = props;
        // Msg none
        if prev_selection != new_selection {
            Msg::OnChange(self.get_state())
        } else {
            Msg::None
        }
    }

    /// ### get_props
    ///
    /// Returns a copy of the component properties.
    fn get_props(&self) -> Props {
        self.props.clone()
    }

    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view.
    fn on(&mut self, ev: Event) -> Msg {
        // Match event
        if let Event::Key(key) = ev {
            match key.code {
                KeyCode::Right => {
                    // Open
                    self.states.open();
                    // Return Msg On Change
                    Msg::OnChange(self.get_state())
                }
                KeyCode::Left => {
                    // Close
                    self.states.close();
                    // Return Msg On Change
                    Msg::OnChange(self.get_state())
                }
                KeyCode::Up => {
                    // Previous
                    self.states.previous();
                    // Return Msg On Change
                    Msg::OnChange(self.get_state())
                }
                KeyCode::Down => {
                    // Next
                    self.states.next();
                    // Return Msg On Change
                    Msg::OnChange(self.get_state())
                }
                KeyCode::Enter => {
                    // Return Submit
                    Msg::OnSubmit(self.get_state())
                }
                _ => {
                    // Return key event to activity
                    Msg::OnKey(key)
                }
            }
        } else {
            // Ignore event
            Msg::None
        }
    }

    /// ### get_state
    ///
    /// Get current state from component
    /// For this component `Payload::One(Value::Str(node_id))` if a node is selected,
    /// None otherwise
    fn get_state(&self) -> Payload {
        match self.states.selected_node() {
            Some(node) => Payload::One(Value::Str(node.id().to_string())),
            None => Payload::None,
        }
    }

    // -- events

    /// ### blur
    ///
    /// Blur component
    fn blur(&mut self) {
        self.states.toggle_focus(false);
    }

    /// ### active
    ///
    /// Active component
    fn active(&mut self) {
        self.states.toggle_focus(true);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;
    use tuirealm::event::KeyEvent;

    #[test]
    fn test_tree() {
        // -- Build
        let mut tree: Tree = Tree::new(
            Node::new("/", "/")
                .with_child(
                    Node::new("/bin", "bin/")
                        .with_child(Node::new("/bin/ls", "ls"))
                        .with_child(Node::new("/bin/pwd", "pwd")),
                )
                .with_child(
                    Node::new("/home", "home/").with_child(
                        Node::new("/home/omar", "omar/")
                            .with_child(Node::new("/home/omar/readme.md", "readme.md"))
                            .with_child(Node::new("/home/omar/changelog.md", "changelog.md")),
                    ),
                ),
        );
        let root: &Node = tree.root();
        assert_eq!(root.id(), "/");
        assert_eq!(root.label(), "/");
        assert_eq!(root.children.len(), 2);
        let bin: &Node = &root.children[0];
        assert_eq!(bin.id(), "/bin");
        assert_eq!(bin.label(), "bin/");
        assert_eq!(bin.children.len(), 2);
        let bin_ids: Vec<&str> = bin.children.iter().map(|x| x.id()).collect();
        assert_eq!(bin_ids, vec!["/bin/ls", "/bin/pwd"]);
        let home: &Node = &tree.root.children[1];
        assert_eq!(home.id(), "/home");
        assert_eq!(home.label(), "home/");
        assert_eq!(home.children.len(), 1);
        let omar_home: &Node = &home.children[0];
        let omar_home_ids: Vec<&str> = omar_home.children.iter().map(|x| x.id()).collect();
        assert_eq!(
            omar_home_ids,
            vec!["/home/omar/readme.md", "/home/omar/changelog.md"]
        );
        // count
        assert_eq!(root.count(), 8);
        // depth
        assert_eq!(root.depth(), 4);
        // -- Query
        assert_eq!(
            tree.query("/home/omar/changelog.md").unwrap().id(),
            "/home/omar/changelog.md"
        );
        assert!(tree.query("ommlar").is_none());
        // is leaf
        assert_eq!(tree.query("/home/omar").unwrap().is_leaf(), false);
        assert_eq!(
            tree.query("/home/omar/changelog.md").unwrap().is_leaf(),
            true
        );
        // parent
        assert_eq!(
            tree.parent("/home/omar/changelog.md").unwrap().id(),
            "/home/omar"
        );
        assert!(tree.parent("/homer").is_none());
        // siblings
        assert_eq!(
            tree.siblings("/home/omar/changelog.md").unwrap(),
            vec!["/home/omar/readme.md"]
        );
        assert_eq!(tree.siblings("/home/omar").unwrap().len(), 0);
        assert!(tree.siblings("/homer").is_none());
        // Mutable
        let _ = tree.root_mut();
        // Push node
        tree.query_mut("/home/omar")
            .unwrap()
            .add_child(Node::new("/home/omar/Cargo.toml", "Cargo.toml"));
        assert_eq!(
            tree.query("/home/omar/Cargo.toml").unwrap().id(),
            "/home/omar/Cargo.toml"
        );
        // -- node_by_route
        assert_eq!(
            tree.node_by_route(&[0, 1, 0, 1]).unwrap().id(),
            "/home/omar/changelog.md"
        );
        assert_eq!(
            tree.root().node_by_route(&[1, 0, 1]).unwrap().id(),
            "/home/omar/changelog.md"
        );
        assert!(tree.root().node_by_route(&[1, 0, 3]).is_none());
        // -- Route by node
        assert_eq!(
            tree.route_by_node("/home/omar/changelog.md").unwrap(),
            vec![0, 1, 0, 1]
        );
        assert_eq!(
            tree.root()
                .route_by_node("/home/omar/changelog.md")
                .unwrap(),
            vec![1, 0, 1]
        );
        assert!(tree.root().route_by_node("ciccio-pasticcio").is_none());
        // Clear node
        tree.query_mut("/home/omar").unwrap().clear();
        assert_eq!(tree.query("/home/omar").unwrap().children.len(), 0);
        // -- With children
        let tree: Tree = Tree::new(
            Node::new("a", "a").with_children(vec![Node::new("a1", "a1"), Node::new("a2", "a2")]),
        );
        assert!(tree.query("a").is_some());
        assert!(tree.query("a1").is_some());
        assert!(tree.query("a2").is_some());
        // -- truncate
        let mut tree: Tree = Tree::new(
            Node::new("/", "/")
                .with_child(
                    Node::new("/bin", "bin/")
                        .with_child(Node::new("/bin/ls", "ls"))
                        .with_child(Node::new("/bin/pwd", "pwd")),
                )
                .with_child(
                    Node::new("/home", "home/").with_child(
                        Node::new("/home/omar", "omar/")
                            .with_child(Node::new("/home/omar/readme.md", "readme.md"))
                            .with_child(Node::new("/home/omar/changelog.md", "changelog.md")),
                    ),
                ),
        );
        let root: &mut Node = &mut tree.root;
        root.truncate(1);
        assert_eq!(root.children.len(), 2);
        assert_eq!(root.children[0].children.len(), 0);
        assert_eq!(root.children[0].id(), "/bin");
        assert_eq!(root.children[1].children.len(), 0);
        assert_eq!(root.children[1].id(), "/home");
    }

    #[test]
    fn test_states() {
        // -- Build
        let tree: Tree = Tree::new(
            Node::new("/", "/")
                .with_child(
                    Node::new("/bin", "bin/")
                        .with_child(Node::new("/bin/ls", "ls"))
                        .with_child(Node::new("/bin/pwd", "pwd")),
                )
                .with_child(
                    Node::new("/home", "home/").with_child(
                        Node::new("/home/omar", "omar/")
                            .with_child(Node::new("/home/omar/readme.md", "readme.md"))
                            .with_child(Node::new("/home/omar/changelog.md", "changelog.md")),
                    ),
                ),
        );
        let mut states: OwnStates = OwnStates::new(tree, None);
        assert_eq!(states.focus(), false);
        states.toggle_focus(true);
        assert_eq!(states.focus(), true);
        let _ = states.get_tui_tree();
        let _ = states.get_tui_tree_state();
        // Get selected
        assert_eq!(states.selected_node().unwrap().id(), "/");
        // Open
        states.open();
        states.next();
        assert_eq!(states.selected_node().unwrap().id(), "/bin");
        states.next();
        assert_eq!(states.selected_node().unwrap().id(), "/home");
        states.previous();
        assert_eq!(states.selected_node().unwrap().id(), "/bin");
        // Open
        states.open();
        assert_eq!(states.selected_node().unwrap().id(), "/bin");
        states.next();
        assert_eq!(states.selected_node().unwrap().id(), "/bin/ls");
        states.next();
        assert_eq!(states.selected_node().unwrap().id(), "/bin/pwd");
        states.close();
        assert_eq!(states.selected_node().unwrap().id(), "/bin");
        // -- Update
        let tree: Tree = Tree::new(
            Node::new("/", "/")
                .with_child(Node::new("/bin", "bin/").with_child(Node::new("/bin/ls", "ls"))),
        );
        // Verify state kept
        states.update_tree(tree, true, None);
        assert_eq!(states.selected_node().unwrap().id(), "/bin");
        // State not kept
        let tree: Tree = Tree::new(
            Node::new("/", "/").with_child(
                Node::new("/home", "home/").with_child(
                    Node::new("/home/omar", "omar/")
                        .with_child(Node::new("/home/omar/readme.md", "readme.md"))
                        .with_child(Node::new("/home/omar/changelog.md", "changelog.md")),
                ),
            ),
        );
        states.update_tree(tree, true, None);
        assert_eq!(states.selected_node().unwrap().id(), "/");
        // Update but with node
        let tree: Tree = Tree::new(
            Node::new("/", "/")
                .with_child(
                    Node::new("/bin", "bin/")
                        .with_child(Node::new("/bin/ls", "ls"))
                        .with_child(Node::new("/bin/pwd", "pwd")),
                )
                .with_child(
                    Node::new("/home", "home/").with_child(
                        Node::new("/home/omar", "omar/")
                            .with_child(Node::new("/home/omar/readme.md", "readme.md"))
                            .with_child(Node::new("/home/omar/changelog.md", "changelog.md")),
                    ),
                ),
        );
        states.update_tree(tree, true, Some("/home/omar"));
        assert_eq!(states.selected_node().unwrap().id(), "/home/omar");
        // new with state
        let tree: Tree = Tree::new(
            Node::new("/", "/")
                .with_child(
                    Node::new("/bin", "bin/")
                        .with_child(Node::new("/bin/ls", "ls"))
                        .with_child(Node::new("/bin/pwd", "pwd")),
                )
                .with_child(
                    Node::new("/home", "home/").with_child(
                        Node::new("/home/omar", "omar/")
                            .with_child(Node::new("/home/omar/readme.md", "readme.md"))
                            .with_child(Node::new("/home/omar/changelog.md", "changelog.md")),
                    ),
                ),
        );
        let states: OwnStates = OwnStates::new(tree, Some("/home/omar/readme.md"));
        assert_eq!(states.selected_node().unwrap().id(), "/home/omar/readme.md");
    }

    #[test]
    fn test_treeview_component() {
        let tree: Tree = Tree::new(
            Node::new("/", "/")
                .with_child(
                    Node::new("/bin", "bin/")
                        .with_child(Node::new("/bin/ls", "ls"))
                        .with_child(Node::new("/bin/pwd", "pwd")),
                )
                .with_child(
                    Node::new("/home", "home/").with_child(
                        Node::new("/home/omar", "omar/")
                            .with_child(Node::new("/home/omar/readme.md", "readme.md"))
                            .with_child(Node::new("/home/omar/changelog.md", "changelog.md")),
                    ),
                ),
        );
        let mut component: TreeView = TreeView::new(
            TreeViewPropsBuilder::default()
                .hidden()
                .visible()
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .with_background(Color::White)
                .with_foreground(Color::Red)
                .with_title(Some(String::from("C:\\")))
                .with_highlighted_str(">>")
                .with_tree(tree.root())
                .keep_state(false)
                .with_node(Some("/home"))
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.background, Color::White);
        assert_eq!(component.props.visible, true);
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Double);
        assert_eq!(component.props.borders.color, Color::Red);
        // Verify with_node
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("/home")))
        );
        // Block
        let _ = component.get_block();
        // Focus
        assert_eq!(component.states.focus, false);
        component.active();
        assert_eq!(component.states.focus, true);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Update
        let props = TreeViewPropsBuilder::from(component.get_props())
            .with_foreground(Color::Yellow)
            .with_title(Some(String::from("aaa")))
            .hidden()
            .with_node(None)
            .build();
        assert_eq!(
            component.update(props),
            Msg::OnChange(Payload::One(Value::Str("/".to_string())))
        );
        assert_eq!(component.props.visible, false);
        assert_eq!(component.props.foreground, Color::Yellow);
        assert_eq!(component.props.texts.title.as_ref().unwrap(), "aaa");
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("/")))
        );
        assert_eq!(
            component.props.texts.spans.as_ref().unwrap()[0]
                .content
                .as_str(),
            ">>"
        );
        // Events
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("/")))
        );
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::OnChange(Payload::One(Value::Str(String::from("/"))))
        );
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("/")))
        );
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Enter))),
            Msg::OnSubmit(Payload::One(Value::Str(String::from("/"))))
        );
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Down))),
            Msg::OnChange(Payload::One(Value::Str(String::from("/bin"))))
        );
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::OnChange(Payload::One(Value::Str(String::from("/bin"))))
        );
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Down))),
            Msg::OnChange(Payload::One(Value::Str(String::from("/bin/ls"))))
        );
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Left))),
            Msg::OnChange(Payload::One(Value::Str(String::from("/bin"))))
        );
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Up))),
            Msg::OnChange(Payload::One(Value::Str(String::from("/"))))
        );
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::OnKey(KeyEvent::from(KeyCode::Char('a'))),
        );
        assert_eq!(component.on(Event::Resize(0, 0)), Msg::None,);
        // Go to a directory
        component.states.next();
        component.states.open();
        component.states.next();
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("/bin/ls")))
        );
        // Update with on change (KEEP STATE)
        let tree: Tree = Tree::new(
            Node::new("/", "/")
                .with_child(Node::new("/bin", "bin/").with_child(Node::new("/bin/ls", "ls")))
                .with_child(
                    Node::new("/home", "home/").with_child(
                        Node::new("/home/omar", "omar/")
                            .with_child(Node::new("/home/omar/readme.md", "readme.md"))
                            .with_child(Node::new("/home/omar/changelog.md", "changelog.md")),
                    ),
                ),
        );
        let props = TreeViewPropsBuilder::from(component.get_props())
            .with_tree(tree.root())
            .keep_state(true)
            .build();
        assert_eq!(component.update(props), Msg::None);
        // Verify state kept
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("/bin/ls")))
        );
        // Update without keeping state
        let tree: Tree = Tree::new(
            Node::new("/", "/")
                .with_child(Node::new("/bin", "bin/").with_child(Node::new("/bin/ls", "ls")))
                .with_child(
                    Node::new("/home", "home/").with_child(
                        Node::new("/home/omar", "omar/")
                            .with_child(Node::new("/home/omar/changelog.md", "changelog.md")),
                    ),
                ),
        );
        let props = TreeViewPropsBuilder::from(component.get_props())
            .with_tree(tree.root())
            .keep_state(false)
            .build();
        assert_eq!(
            component.update(props),
            Msg::OnChange(Payload::One(Value::Str(String::from("/"))))
        );
        // Update with depth
        let tree: Tree = Tree::new(
            Node::new("/", "/")
                .with_child(Node::new("/bin", "bin/").with_child(Node::new("/bin/pwd", "pwd")))
                .with_child(
                    Node::new("/home", "home/").with_child(
                        Node::new("/home/omar", "omar/")
                            .with_child(Node::new("/home/omar/readme.md", "readme.md"))
                            .with_child(Node::new("/home/omar/changelog.md", "changelog.md")),
                    ),
                ),
        );
        let props = TreeViewPropsBuilder::from(component.get_props())
            .with_tree_and_depth(tree.root(), 1)
            .build();
        assert_eq!(component.update(props), Msg::None);
        // Reset state
        component.states.tui_tree = StatefulTree::new();
        assert_eq!(component.get_state(), Payload::None);
    }
}
