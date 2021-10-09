//! ## Borders
//!
//! `Borders` is the module which defines the border properties

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
use super::{Color, Style};

// Exports
pub use tui::widgets::{BorderType, Borders as BorderSides};

// -- Border

/// ## Borders
///
/// Defines the properties of the borders
#[derive(Clone, Debug, PartialEq)]
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
    /// ### sides
    ///
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

    /// ### style
    ///
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
