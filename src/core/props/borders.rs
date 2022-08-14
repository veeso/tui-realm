//! ## Borders
//!
//! `Borders` is the module which defines the border properties

use super::{Color, Style};

// Exports
pub use tui::widgets::{BorderType, Borders as BorderSides};

// -- Border

/// Defines the properties of the borders
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Borders {
    pub sides: BorderSides,
    pub modifiers: BorderType,
    pub color: Color,
}

impl Default for Borders {
    fn default() -> Self {
        Borders {
            sides: BorderSides::ALL,
            modifiers: BorderType::Plain,
            color: Color::Reset,
        }
    }
}

impl Borders {
    /// Set border sides
    pub fn sides(mut self, borders: BorderSides) -> Self {
        self.sides = borders;
        self
    }

    pub fn modifiers(mut self, modifiers: BorderType) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Get Border style
    pub fn style(&self) -> Style {
        Style::default().fg(self.color)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn borders() {
        // Default
        let props: Borders = Borders::default();
        assert_eq!(props.sides, BorderSides::ALL);
        assert_eq!(props.modifiers, BorderType::Plain);
        assert_eq!(props.color, Color::Reset);
        // Build
        let props = Borders::default()
            .sides(BorderSides::TOP)
            .modifiers(BorderType::Double)
            .color(Color::Yellow);
        assert_eq!(props.sides, BorderSides::TOP);
        assert_eq!(props.modifiers, BorderType::Double);
        assert_eq!(props.color, Color::Yellow);
        // Get style
        let style: Style = props.style();
        assert_eq!(*style.fg.as_ref().unwrap(), Color::Yellow);
    }
}
