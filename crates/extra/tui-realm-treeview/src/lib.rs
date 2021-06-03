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

use tui_tree_widget::{Tree as TuiTree, TreeItem as TuiTreeItem, TreeState as TuiTreeState};
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
}

/// ## Node
///
/// Describes a node inside the `Tree`
#[derive(Debug)]
pub struct Node {
    id: String,    // Must uniquely identify the node in the tree
    label: String, // The text to display associated to the node
    children: Vec<Node>,
}

impl Node {
    /// ### new
    ///
    /// Instantiates a new `Node`
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
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
        self.query_mut(id).as_deref()
    }

    /// ### query_mut
    ///
    /// Returns a mutable reference to a Node
    pub(self) fn query_mut(&mut self, id: &str) -> Option<&mut Self> {
        if self.id.as_str() == id {
            Some(&mut self)
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
}

// TODO: to propPayload
// TODO: from prop payload

// -- states

/// ## OwnStates
///
/// TreeView states
#[derive(Debug)]
struct OwnStates<'a> {
    focus: bool,
    tree: TuiTree<'a>,
}

impl<'a> OwnStates<'a> {
    /// ### new
    ///
    /// Instantiates a new OwnStates from tree data
    pub fn new(tree: PropPayload) -> Self {
        Self {
            focus: false,
            tree: TuiTree::from(tree),
        }
    }
}

impl<'a> From<PropPayload> for TuiTree<'a> {
    /// ### PropPayload to TuiTree
    ///
    /// The PropPayload is a series of `Linked` where item is a Tuple made up of `(id, label, parent)`
    /// and next element is the following element in root
    fn from(props: PropPayload) -> Self {}
}

// -- props

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
        // -- Query
        assert_eq!(
            tree.query("/home/omar/changelog.md").unwrap().id.as_str(),
            "/home/omar/changelog.md"
        );
        assert!(tree.query("ommlar").is_none());
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
    }
}
