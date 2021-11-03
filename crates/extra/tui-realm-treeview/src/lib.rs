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
use orange_trees::{Node as OrangeNode, Tree as OrangeTree};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, InputType, Props, Style, TextModifiers,
};
use tuirealm::tui::layout::Rect;
use tuirealm::{Frame, MockComponent, State, StateValue};

// -- type override
pub(crate) type Node = OrangeNode<String, String>;
pub(crate) type Tree = OrangeTree<String, String>;

// -- props

// -- component

/// ## TreeView
///
/// Tree view Mock component for tui-realm
pub struct TreeView {
    props: Props,
    states: TreeState,
    /// The actual Tree data structure. You can access this from your Component to operate on it
    /// for example after a certain events.
    pub tree: Tree,
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

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }

    // TODO: custom properties

    // TODO: indent_spaces

    pub fn tree(mut self, tree: Tree) -> Self {
        self.tree = tree;
        self
    }
}
