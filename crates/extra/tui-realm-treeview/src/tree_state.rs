//! # Tree State
//!
//! This module implements the tree state.

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
use super::Node;

/// ## TreeState
///
/// Tree state tracks the current state for the component tree.
pub struct TreeState {
    /// Tracks open nodes
    open: Vec<String>,
}

impl Default for TreeState {
    fn default() -> Self {
        Self {
            open: Vec::default(),
        }
    }
}

impl TreeState {
    /// ### tree_changed
    ///
    /// The tree has changed, so this method must check whether to keep states or not
    pub fn tree_changed(&mut self, root: &Node) {
        todo!()
    }

    /// ### open_node
    ///
    /// Open `node`
    pub fn open_node(&mut self, node: &Node) {
        todo!()
    }

    /// ### close_node
    ///
    /// Close `node`
    pub fn close_node(&mut self, node: &Node) {
        todo!()
    }

    /// ### is_open
    ///
    /// Returns whether `node` is open
    pub fn is_open(&self, node: &Node) -> bool {
        todo!()
    }

    // -- private

    /// ### open_children
    ///
    /// Open all node children recursively
    fn open_children(&mut self, node: &Node) {
        todo!()
    }

    /// ### close_children
    ///
    /// Close all node children recursively
    fn close_children(&mut self, node: &Node) {
        todo!()
    }
}
