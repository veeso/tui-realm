//! ## Layout
//!
//! This module exposes the layout type

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
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
use crate::tui::layout::{Constraint, Direction, Layout as TuiLayout, Margin, Rect};

/// ## Layout
///
/// Defines how a layout has to be rendered
#[derive(Debug, PartialEq, Clone)]
pub struct Layout {
    constraints: Vec<Constraint>,
    direction: Direction,
    margin: Margin,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            constraints: Vec::new(),
            direction: Direction::Vertical,
            margin: Margin {
                horizontal: 0,
                vertical: 0,
            },
        }
    }
}

impl Layout {
    // -- constructors

    pub fn constraints(mut self, constraints: &[Constraint]) -> Self {
        self.constraints = constraints.to_vec();
        self
    }

    pub fn margin(mut self, margin: u16) -> Self {
        self.margin = Margin {
            horizontal: margin,
            vertical: margin,
        };
        self
    }

    pub fn horizontal_margin(mut self, margin: u16) -> Self {
        self.margin.horizontal = margin;
        self
    }

    pub fn vertical_margin(mut self, margin: u16) -> Self {
        self.margin.vertical = margin;
        self
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    // -- chunks

    /// ### chunks
    ///
    /// Split an `Area` into chunks using the current layout configuration
    pub fn chunks(&self, area: Rect) -> Vec<Rect> {
        TuiLayout::default()
            .direction(self.direction.clone())
            .horizontal_margin(self.margin.horizontal)
            .vertical_margin(self.margin.vertical)
            .constraints::<&[Constraint]>(self.constraints.as_ref())
            .split(area)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_build_a_layout() {
        let area = Rect::default();
        let layout = Layout::default()
            .margin(10)
            .horizontal_margin(15)
            .vertical_margin(12)
            .direction(Direction::Vertical)
            .constraints(&[
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(1),
            ]);
        assert_eq!(layout.chunks(area).len(), 3);
    }
}
