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
use super::{Node, NodeValue, Tree, TreeState};

use tuirealm::tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthStr;

/// ## TreeWidget
///
/// tui-rs widget implementation of a tree view
pub struct TreeWidget<'a, V: NodeValue> {
    /// Block properties
    block: Option<Block<'a>>,
    /// Style for tree
    style: Style,
    /// Highlight style
    highlight_style: Style,
    /// Symbol to display on the side of the current highlighted
    highlight_symbol: Option<String>,
    /// Spaces to use for indentation
    indent_size: usize,
    /// Tree to render
    tree: &'a Tree<V>,
}

impl<'a, V: NodeValue> TreeWidget<'a, V> {
    /// ### new
    ///
    /// Setup a new `TreeWidget`
    pub fn new(tree: &'a Tree<V>) -> Self {
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
    pub fn highlight_symbol(mut self, s: String) -> Self {
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
    skip_rows: usize,
}

impl<'a, V: NodeValue> Widget for TreeWidget<'a, V> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = TreeState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl<'a, V: NodeValue> StatefulWidget for TreeWidget<'a, V> {
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
        let mut render = Render {
            depth: 1,
            skip_rows: self.calc_rows_to_skip(state, area.height),
        };
        self.iter_nodes(self.tree.root(), area, buf, state, &mut render);
    }
}

impl<'a, V: NodeValue> TreeWidget<'a, V> {
    fn iter_nodes(
        &self,
        node: &Node<V>,
        mut area: Rect,
        buf: &mut Buffer,
        state: &TreeState,
        render: &mut Render,
    ) -> Rect {
        // Render self
        area = self.render_node(node, area, buf, state, render);
        // Render children if node is open
        if state.is_open(node) {
            // Increment depth
            render.depth += 1;
            for child in node.iter() {
                if area.height == 0 {
                    break;
                }
                area = self.iter_nodes(child, area, buf, state, render);
            }
            // Decrement depth
            render.depth -= 1;
        }
        area
    }

    fn render_node(
        &self,
        node: &Node<V>,
        area: Rect,
        buf: &mut Buffer,
        state: &TreeState,
        render: &mut Render,
    ) -> Rect {
        // If row should skip, then skip
        if render.skip_rows > 0 {
            render.skip_rows -= 1;
            return area;
        }
        let highlight_symbol = match state.is_selected(node) {
            true => Some(self.highlight_symbol.clone().unwrap_or_default()),
            false => None,
        };
        // Get area for current node
        let node_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        };
        // Get style to use
        let style = match state.is_selected(node) {
            false => self.style,
            true => self.highlight_style,
        };
        // Apply style
        buf.set_style(node_area, style);
        // Calc depth for node (is selected?)
        let indent_size = render.depth * self.indent_size;
        let indent_size = match state.is_selected(node) {
            true if highlight_symbol.is_some() => {
                indent_size.saturating_sub(highlight_symbol.as_deref().unwrap().width() + 1)
            }
            _ => indent_size,
        };
        let width: usize = area.width as usize;
        // Write indentation
        let (start_x, start_y) = buf.set_stringn(
            area.x,
            area.y,
            " ".repeat(indent_size),
            width - indent_size,
            style,
        );
        // Write highlight symbol
        let (start_x, start_y) = highlight_symbol
            .map(|x| buf.set_stringn(start_x, start_y, x, width - start_x as usize, style))
            .map(|(x, y)| buf.set_stringn(x, y, " ", width - start_x as usize, style))
            .unwrap_or((start_x, start_y));

        let mut start_x = start_x;
        let mut start_y = start_y;
        for (text, part_style) in node.value().render_parts_iter() {
            let part_style = part_style.unwrap_or(style);
            // Write node name
            (start_x, start_y) = buf.set_stringn(
                start_x,
                start_y,
                text,
                width - start_x as usize,
                part_style,
            );
        }
        // Write arrow based on node
        let write_after = if state.is_open(node) {
            // Is open
            " \u{25bc}" // Arrow down
        } else if node.is_leaf() {
            // Is leaf (has no children)
            "  "
        } else {
            // Has children, but is closed
            " \u{25b6}" // Arrow to right
        };
        let _ = buf.set_stringn(
            start_x,
            start_y,
            write_after,
            width - start_x as usize,
            style,
        );
        // Return new area
        Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: area.height - 1,
        }
    }

    /// ### calc_rows_to__skip
    ///
    /// Calculate rows to skip before starting rendering the current tree
    fn calc_rows_to_skip(&self, state: &TreeState, height: u16) -> usize {
        // if no node is selected, return 0
        let selected = match state.selected() {
            Some(s) => s,
            None => return 0,
        };
        /// ### calc_rows_to_skip_r
        ///
        /// Inner recursive call to calc rows to skip.
        /// Returns the rows to skip and whether the item has been found (this last oneshould be ignored)
        fn calc_rows_to_skip_r<V: NodeValue>(
            node: &Node<V>,
            state: &TreeState,
            height: u16,
            selected: &str,
            mut acc: usize,
        ) -> (usize, bool) {
            // If node is selected, return `acc`
            if node.id().as_str() == selected {
                (acc + 1, true)
            } else if state.is_closed(node) {
                // If node is closed, then return acc + 1
                (acc + 1, false)
            } else {
                // is open and is not selected
                // I increment the accumulator by one
                acc += 1;
                // For each child, let's call this function
                for child in node.iter() {
                    let (ret, found) = calc_rows_to_skip_r(child, state, height, selected, acc);
                    // Set acc to ret
                    acc = ret;
                    // If found, return
                    if found {
                        return (acc, true);
                    }
                }
                (acc, false)
            }
        }
        // Return the result of recursive call;
        // if the result is less than area height, then return 0; otherwise subtract the height to result
        match calc_rows_to_skip_r(self.tree.root(), state, height, selected, 0).0 {
            x if x < (height as usize) => 0,
            x => x - (height as usize),
        }
    }
}

// <https://docs.rs/tui-tree-widget/0.7.0/tui_tree_widget/>
// <https://docs.rs/tui-tree-widget/0.7.0/src/tui_tree_widget/lib.rs.html#136-146>

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::mock_tree;

    use pretty_assertions::assert_eq;
    use tuirealm::tui::style::Color;

    #[test]
    fn should_construct_default_widget() {
        let tree = mock_tree();
        let widget = TreeWidget::new(&tree);
        assert_eq!(widget.block, None);
        assert_eq!(widget.highlight_style, Style::default());
        assert_eq!(widget.highlight_symbol, None);
        assert_eq!(widget.indent_size, 4);
        assert_eq!(widget.style, Style::default());
    }

    #[test]
    fn should_construct_widget() {
        let tree = mock_tree();
        let widget = TreeWidget::new(&tree)
            .block(Block::default())
            .highlight_style(Style::default().fg(Color::Red))
            .highlight_symbol(String::from(">"))
            .indent_size(8)
            .style(Style::default().fg(Color::LightRed));
        assert!(widget.block.is_some());
        assert_eq!(widget.highlight_style.fg.unwrap(), Color::Red);
        assert_eq!(widget.indent_size, 8);
        assert_eq!(widget.highlight_symbol.as_deref().unwrap(), ">");
        assert_eq!(widget.style.fg.unwrap(), Color::LightRed);
    }

    #[test]
    fn should_have_no_row_to_skip_when_in_first_height_elements() {
        let tree = mock_tree();
        let mut state = TreeState::default();
        // Select aA2
        let aa2 = tree.root().query(&String::from("aA2")).unwrap();
        state.select(tree.root(), aa2);
        // Get rows to skip (no block)
        let widget = TreeWidget::new(&tree);
        // Before end
        assert_eq!(widget.calc_rows_to_skip(&state, 8), 0);
        // At end
        assert_eq!(widget.calc_rows_to_skip(&state, 6), 0);
    }

    #[test]
    fn should_have_rows_to_skip_when_out_of_viewport() {
        let tree = mock_tree();
        let mut state = TreeState::default();
        // Open all previous nodes
        state.force_open(&["/", "a", "aA", "aB", "aC", "b", "bA", "bB"]);
        // Select bB2
        let bb2 = tree.root().query(&String::from("bB2")).unwrap();
        state.select(tree.root(), bb2);
        // Get rows to skip (no block)
        let widget = TreeWidget::new(&tree);
        // 20th element - height (12) + 1
        assert_eq!(widget.calc_rows_to_skip(&state, 8), 13);
    }
}
