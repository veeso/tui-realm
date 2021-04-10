//! ## Borders
//!
//! `Borders` is the module which defines the border properties

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
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
use tui::style::{Color, Style};

// Exports
pub use tui::widgets::{BorderType, Borders};

// -- Border

/// ## BordersProps
///
/// Defines the properties of the borders
#[derive(Clone)]
pub struct BordersProps {
    pub borders: Borders,
    pub variant: BorderType,
    pub(super) color: Color,
}

impl Default for BordersProps {
    fn default() -> Self {
        BordersProps {
            borders: Borders::ALL,
            variant: BorderType::Plain,
            color: Color::Reset,
        }
    }
}

impl BordersProps {
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

    #[test]
    fn test_props_borders() {
        // Default
        let mut props: BordersProps = BordersProps::default();
        assert_eq!(props.borders, Borders::ALL);
        assert_eq!(props.variant, BorderType::Plain);
        assert_eq!(props.color, Color::Reset);
        // Get style
        props.color = Color::Yellow;
        let style: Style = props.style();
        assert_eq!(*style.fg.as_ref().unwrap(), Color::Yellow);
    }
}
