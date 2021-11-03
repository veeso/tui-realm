//! # Widget
//!
//! This module implements the tui widget for rendering a treeview

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
use super::{Node, Tree, TreeState};

use tuirealm::tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, StatefulWidget, Widget},
};

/// ## TreeWidget
///
/// tui-rs widget implementation of a tree view
pub struct TreeWidget<'a> {
    /// Block properties
    block: Option<Block<'a>>,
    /// Style for tree
    style: Style,
    /// Highlight style
    highlight_style: Style,
    /// Symbol to display on the side of the current highlighted
    highlight_symbol: Option<&'a str>,
    /// Spaces to use for indentation
    indent_size: usize,
    /// Tree to render
    tree: &'a Tree,
}

impl<'a> TreeWidget<'a> {
    /// ### new
    ///
    /// Setup a new `TreeWidget`
    pub fn new(tree: &'a Tree) -> Self {
        Self {
            block: None,
            style: Style::default(),
            highlight_style: Style::default(),
            highlight_symbol: None,
            indent_size: 4,
            tree,
        }
    }

    /// ### block
    ///
    /// Set block to render around the tree view
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// ### style
    ///
    /// Set style for tree view
    pub fn style(mut self, s: Style) -> Self {
        self.style = s;
        self
    }

    /// ### highlight_style
    ///
    /// Set highlighted entry style
    pub fn highlight_style(mut self, s: Style) -> Self {
        self.highlight_style = s;
        self
    }

    /// ### highlight_symbol
    ///
    /// Set symbol to prepend to highlighted entry
    pub fn highlight_symbol(mut self, s: &'a str) -> Self {
        self.highlight_symbol = Some(s);
        self
    }

    /// ### indent_size
    ///
    /// Size for indentation
    pub fn indent_size(mut self, sz: usize) -> Self {
        self.indent_size = sz;
        self
    }
}

// -- render

struct Render {
    depth: usize,
}

impl<'a> Widget for TreeWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = TreeState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl<'a> StatefulWidget for TreeWidget<'a> {
    type State = TreeState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Set style for area
        buf.set_style(area, self.style);
        // Build block
        let area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };
        // Return if too small
        if area.width < 1 || area.height < 1 {
            return;
        }
        // Recurse render
        let mut render = Render { depth: 0 };
        self.iter_nodes(self.tree.root(), area, buf, state, &mut render);
    }
}

impl<'a> TreeWidget<'a> {
    fn iter_nodes(
        &self,
        node: &Node,
        area: Rect,
        buf: &mut Buffer,
        state: &TreeState,
        render: &mut Render,
    ) {
        // Render self
        let mut area = self.render_node(node, area, buf, state, render);
        // Increment depth
        render.depth += 1;
        // Render children
        for child in node.iter() {
            if area.height == 0 {
                break;
            }
            area = self.render_node(child, area, buf, state, render);
        }
        // Decrement depth
        render.depth -= 1;
    }

    fn render_node(
        &self,
        node: &Node,
        area: Rect,
        buf: &mut Buffer,
        state: &TreeState,
        render: &mut Render,
    ) -> Rect {
        let highlight_symbol = match state.is_selected(node) {
            true => Some(self.highlight_symbol.unwrap_or("")),
            false => None,
        };
        // Get area for current node
        let node_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        };
        // Apply style
        match state.is_selected(node) {
            false => buf.set_style(node_area, self.style),
            true => buf.set_style(node_area, self.highlight_style),
        };
        // Calc depth for node (is selected?)
        let depth = match state.is_selected(node) {
            true => render.depth - 1,
            false => render.depth,
        } * self.indent_size;
        let width: usize = area.width as usize;
        // Write depth
        let (start_x, start_y) =
            buf.set_stringn(area.x, area.y, " ".repeat(depth), width - depth, self.style);
        // Write highlight symbol
        let (start_x, start_y) = highlight_symbol
            .map(|x| {
                buf.set_stringn(
                    start_x,
                    start_y,
                    x,
                    width - start_x as usize,
                    self.highlight_style,
                )
            })
            .unwrap_or((start_x, start_y));
        // Write node name
        let (start_x, start_y) = buf.set_stringn(
            start_x,
            start_y,
            node.value(),
            width - start_x as usize,
            self.style,
        );
        // Write arrow based on node
        let write_after = if state.is_open(node) {
            // Is open
            "\u{25bc}" // Arrow down
        } else if node.is_leaf() {
            // Is leaf (has no children)
            " "
        } else {
            // Has children, but is closed
            "\u{25b6}" // Arrow to right
        };
        let _ = buf.set_stringn(
            start_x,
            start_y,
            write_after,
            width - start_x as usize,
            self.style,
        );
        // Return new area
        Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: area.height - 1,
        }
    }
}

// <https://docs.rs/tui-tree-widget/0.7.0/tui_tree_widget/>
// <https://docs.rs/tui-tree-widget/0.7.0/src/tui_tree_widget/lib.rs.html#136-146>
