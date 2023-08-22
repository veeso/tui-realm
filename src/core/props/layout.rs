//! ## Layout
//!
//! This module exposes the layout type

use crate::tui::layout::{Constraint, Direction, Layout as TuiLayout, Margin, Rect};

/// Defines how a layout has to be rendered
#[derive(Debug, PartialEq, Clone, Eq)]
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

    /// Split an `Area` into chunks using the current layout configuration
    pub fn chunks(&self, area: Rect) -> Vec<Rect> {
        TuiLayout::default()
            .direction(self.direction.clone())
            .horizontal_margin(self.margin.horizontal)
            .vertical_margin(self.margin.vertical)
            .constraints::<&[Constraint]>(self.constraints.as_ref())
            .split(area)
            .to_vec()
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
