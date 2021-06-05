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
//! tui-realm-treeview = "0.1.0"
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

use stateful_tree::StatefulTree;

// deps
use tui_tree_widget::{TreeItem as TuiTreeItem, TreeState as TuiTreeState};
use tuirealm::{Component, PropPayload, PropValue, Props, PropsBuilder};

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
    /// Returns a mutable reference to the root node
    pub(self) fn root_mut(&mut self) -> &mut Node {
        &mut self.root
    }

    /// ### query
    ///
    /// Query tree for a certain node
    pub fn query(&self, id: &str) -> Option<&Node> {
        self.root.query(id)
    }

    /// ### node_by_route
    ///
    /// Get starting from root the node associated to the indexes.
    /// When starting from tree, the first element in route must be `0`
    pub fn node_by_route(&self, route: &[usize]) -> Option<&Node> {
        self.root().node_by_route(&route[1..])
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

    /// ### with_children
    ///
    /// Sets Node children
    pub fn with_children(mut self, children: Vec<Node>) -> Self {
        self.children = children;
        self
    }

    /// ### add_child
    ///
    /// Create a new child in this Node
    pub fn add_child(mut self, child: Node) -> Self {
        self.children.push(child);
        self
    }

    /// ### query
    ///
    /// Search for `id` inside Node's children (or is itself)
    pub fn query(&self, id: &str) -> Option<&Self> {
        if self.id.as_str() == id {
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
        if self.id.as_str() == id {
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

    /// ### count
    ///
    /// Count items in tree
    pub fn count(&self) -> usize {
        self.children.iter().map(|x| x.count()).sum::<usize>() + 1
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
    pub fn new(tree: PropPayload) -> Self {
        let tree: Tree = Tree::from(tree);
        Self {
            focus: false,
            tui_tree: StatefulTree::from(&tree),
            tree,
        }
    }
}

// -- props

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
    /// ### with_tree_and_depth
    ///
    /// Sets the tree and its max depth for Props builder
    pub fn with_tree_and_depth(&mut self, root: &Node, depth: usize) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.value = root.to_prop_payload(depth, "")
        }
        self
    }

    /// ### with_tree
    ///
    /// Sets the tree for Props builder
    pub fn with_tree(&mut self, root: &Node) -> &mut Self {
        self.with_tree_and_depth(root, usize::MAX)
    }
}

// -- component

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_tree() {
        // -- Build
        let tree: Tree = Tree::new(
            Node::new("/", "/")
                .add_child(
                    Node::new("/bin", "bin/")
                        .add_child(Node::new("/bin/ls", "ls"))
                        .add_child(Node::new("/bin/pwd", "pwd")),
                )
                .add_child(
                    Node::new("/home", "home/").add_child(
                        Node::new("/home/omar", "omar/")
                            .add_child(Node::new("/home/omar/readme.md", "readme.md"))
                            .add_child(Node::new("/home/omar/changelog.md", "changelog.md")),
                    ),
                ),
        );
        let root: &Node = tree.root();
        assert_eq!(root.id.as_str(), "/");
        assert_eq!(root.label.as_str(), "/");
        assert_eq!(root.children.len(), 2);
        let bin: &Node = &root.children[0];
        assert_eq!(bin.id.as_str(), "/bin");
        assert_eq!(bin.label.as_str(), "bin/");
        assert_eq!(bin.children.len(), 2);
        let bin_ids: Vec<&str> = bin.children.iter().map(|x| x.id.as_str()).collect();
        assert_eq!(bin_ids, vec!["/bin/ls", "/bin/pwd"]);
        let home: &Node = &tree.root.children[1];
        assert_eq!(home.id.as_str(), "/home");
        assert_eq!(home.label.as_str(), "home/");
        assert_eq!(home.children.len(), 1);
        let omar_home: &Node = &home.children[0];
        let omar_home_ids: Vec<&str> = omar_home.children.iter().map(|x| x.id.as_str()).collect();
        assert_eq!(
            omar_home_ids,
            vec!["/home/omar/readme.md", "/home/omar/changelog.md"]
        );
        // count
        assert_eq!(root.count(), 8);
        // -- Query
        assert_eq!(
            tree.query("/home/omar/changelog.md").unwrap().id.as_str(),
            "/home/omar/changelog.md"
        );
        assert!(tree.query("ommlar").is_none());
        // -- node_by_route
        assert_eq!(
            tree.node_by_route(&[0, 1, 0, 1]).unwrap().id.as_str(),
            "/home/omar/changelog.md"
        );
        assert_eq!(
            tree.root().node_by_route(&[1, 0, 1]).unwrap().id.as_str(),
            "/home/omar/changelog.md"
        );
        assert!(tree.root().node_by_route(&[1, 0, 2]).is_none());
        // -- With children
        let mut tree: Tree = Tree::new(
            Node::new("a", "a").with_children(vec![Node::new("a1", "a1"), Node::new("a2", "a2")]),
        );
        assert!(tree.query("a").is_some());
        assert!(tree.query("a1").is_some());
        assert!(tree.query("a2").is_some());
        // mut
        assert!(tree.root_mut().query_mut("a1").is_some());
        assert_eq!(tree.root_mut().id.as_str(), "a");
        // -- truncate
        let mut tree: Tree = Tree::new(
            Node::new("/", "/")
                .add_child(
                    Node::new("/bin", "bin/")
                        .add_child(Node::new("/bin/ls", "ls"))
                        .add_child(Node::new("/bin/pwd", "pwd")),
                )
                .add_child(
                    Node::new("/home", "home/").add_child(
                        Node::new("/home/omar", "omar/")
                            .add_child(Node::new("/home/omar/readme.md", "readme.md"))
                            .add_child(Node::new("/home/omar/changelog.md", "changelog.md")),
                    ),
                ),
        );
        let root: &mut Node = tree.root_mut();
        root.truncate(1);
        assert_eq!(root.children.len(), 2);
        assert_eq!(root.children[0].children.len(), 0);
        assert_eq!(root.children[0].id.as_str(), "/bin");
        assert_eq!(root.children[1].children.len(), 0);
        assert_eq!(root.children[1].id.as_str(), "/home");
    }
}
