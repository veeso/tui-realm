//! ## Texts
//!
//! `Texts` is the module which defines the texts properties for components.
//! It also provides some helpers and builders to facilitate the use of builders.

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
use tui::style::{Color, Modifier};

// -- Text parts

/// ## Table
///
/// Table represents a list of rows with a list of columns of text spans
pub type Table = Vec<Vec<TextSpan>>;

/// ## TextParts
///
/// TextParts holds optional component for the text displayed by a component
#[derive(Clone)]
pub struct TextParts {
    pub title: Option<String>,
    pub spans: Option<Vec<TextSpan>>,
    pub table: Option<Table>, // First vector is rows, inner vec is column
}

impl TextParts {
    /// ### new
    ///
    /// Instantiates a new TextParts entity
    pub fn new(title: Option<String>, spans: Option<Vec<TextSpan>>) -> Self {
        TextParts {
            title,
            spans,
            table: None,
        }
    }

    /// ### table
    ///
    /// Instantiates a new TextParts as a Table
    pub fn table(title: Option<String>, table: Table) -> Self {
        TextParts {
            title,
            spans: None,
            table: Some(table),
        }
    }
}

impl Default for TextParts {
    fn default() -> Self {
        TextParts {
            title: None,
            spans: None,
            table: None,
        }
    }
}

/// ## TableBuilder
///
/// Table builder is a helper to make it easier to build text tables
pub struct TableBuilder {
    table: Option<Table>,
}

impl TableBuilder {
    /// ### add_col
    ///
    /// Add a column to the last row
    pub fn add_col(&mut self, span: TextSpan) -> &mut Self {
        if let Some(table) = self.table.as_mut() {
            if let Some(row) = table.last_mut() {
                row.push(span);
            }
        }
        self
    }

    /// ### add_row
    ///
    /// Add a new row to the table
    pub fn add_row(&mut self) -> &mut Self {
        if let Some(table) = self.table.as_mut() {
            table.push(vec![]);
        }
        self
    }

    /// ### build
    ///
    /// Take table out of builder
    /// Don't call this method twice for any reasons!
    pub fn build(&mut self) -> Table {
        self.table.take().unwrap()
    }
}

impl Default for TableBuilder {
    fn default() -> Self {
        TableBuilder {
            table: Some(vec![vec![]]),
        }
    }
}

/// ### TextSpan
///
/// TextSpan is a "cell" of text with its attributes
#[derive(Clone, std::fmt::Debug)]
pub struct TextSpan {
    pub content: String,
    pub fg: Color,
    pub bg: Color,
    pub modifiers: Modifier,
}

impl From<&str> for TextSpan {
    fn from(txt: &str) -> Self {
        TextSpan {
            content: txt.to_string(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifiers: Modifier::empty(),
        }
    }
}

impl From<String> for TextSpan {
    fn from(content: String) -> Self {
        TextSpan {
            content,
            fg: Color::Reset,
            bg: Color::Reset,
            modifiers: Modifier::empty(),
        }
    }
}

// -- TextSpan builder

/// ## TextSpanBuilder
///
/// TextSpanBuilder is a struct which helps building quickly a TextSpan
pub struct TextSpanBuilder {
    text: Option<TextSpan>,
}

impl TextSpanBuilder {
    /// ### new
    ///
    /// Instantiate a new TextSpanBuilder
    pub fn new(text: &str) -> Self {
        TextSpanBuilder {
            text: Some(TextSpan::from(text)),
        }
    }

    /// ### with_foreground
    ///
    /// Set foreground for text span
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(text) = self.text.as_mut() {
            text.fg = color;
        }
        self
    }

    /// ### with_background
    ///
    /// Set background for text span
    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(text) = self.text.as_mut() {
            text.bg = color;
        }
        self
    }

    /// ### bold
    ///
    /// Set bold property for text
    pub fn bold(&mut self) -> &mut Self {
        if let Some(props) = self.text.as_mut() {
            props.modifiers |= Modifier::BOLD;
        }
        self
    }

    /// ### italic
    ///
    /// Set italic property for text
    pub fn italic(&mut self) -> &mut Self {
        if let Some(props) = self.text.as_mut() {
            props.modifiers |= Modifier::ITALIC;
        }
        self
    }

    /// ### underlined
    ///
    /// Set underlined property for text
    pub fn underlined(&mut self) -> &mut Self {
        if let Some(props) = self.text.as_mut() {
            props.modifiers |= Modifier::UNDERLINED;
        }
        self
    }

    /// ### slow_blink
    ///
    /// Set slow_blink property for text
    pub fn slow_blink(&mut self) -> &mut Self {
        if let Some(props) = self.text.as_mut() {
            props.modifiers |= Modifier::SLOW_BLINK;
        }
        self
    }

    /// ### rapid_blink
    ///
    /// Set rapid_blink property for text
    pub fn rapid_blink(&mut self) -> &mut Self {
        if let Some(props) = self.text.as_mut() {
            props.modifiers |= Modifier::RAPID_BLINK;
        }
        self
    }

    /// ### reversed
    ///
    /// Set reversed property for text
    pub fn reversed(&mut self) -> &mut Self {
        if let Some(props) = self.text.as_mut() {
            props.modifiers |= Modifier::REVERSED;
        }
        self
    }

    /// ### strikethrough
    ///
    /// Set strikethrough property for text
    pub fn strikethrough(&mut self) -> &mut Self {
        if let Some(props) = self.text.as_mut() {
            props.modifiers |= Modifier::CROSSED_OUT;
        }
        self
    }

    /// ### build
    ///
    /// Make TextSpan out of builder
    /// Don't call this method twice for any reasons!
    pub fn build(&mut self) -> TextSpan {
        self.text.take().unwrap()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_props_text_parts_with_values() {
        let parts: TextParts = TextParts::new(
            Some(String::from("Hello world!")),
            Some(vec![TextSpan::from("row1"), TextSpan::from("row2")]),
        );
        assert_eq!(parts.title.as_ref().unwrap().as_str(), "Hello world!");
        assert_eq!(
            parts
                .spans
                .as_ref()
                .unwrap()
                .get(0)
                .unwrap()
                .content
                .as_str(),
            "row1"
        );
        assert_eq!(
            parts
                .spans
                .as_ref()
                .unwrap()
                .get(1)
                .unwrap()
                .content
                .as_str(),
            "row2"
        );
    }

    #[test]
    fn test_props_text_parts_default() {
        let parts: TextParts = TextParts::default();
        assert!(parts.title.is_none());
        assert!(parts.spans.is_none());
    }

    #[test]
    fn test_props_text_parts_table() {
        let table: TextParts = TextParts::table(
            Some(String::from("my data")),
            TableBuilder::default()
                .add_col(TextSpan::from("name"))
                .add_col(TextSpan::from("age"))
                .add_row()
                .add_col(TextSpan::from("christian"))
                .add_col(TextSpan::from("23"))
                .add_row()
                .add_col(TextSpan::from("omar"))
                .add_col(TextSpan::from("25"))
                .add_row()
                .add_row()
                .add_col(TextSpan::from("pippo"))
                .build(),
        );
        // Verify table
        assert_eq!(table.title.as_ref().unwrap().as_str(), "my data");
        assert!(table.spans.is_none());
        assert_eq!(table.table.as_ref().unwrap().len(), 5); // 5 spans
        assert_eq!(table.table.as_ref().unwrap().get(0).unwrap().len(), 2); // 2 cols
        assert_eq!(table.table.as_ref().unwrap().get(1).unwrap().len(), 2); // 2 cols
        assert_eq!(
            table
                .table
                .as_ref()
                .unwrap()
                .get(1)
                .unwrap()
                .get(0)
                .unwrap()
                .content
                .as_str(),
            "christian"
        ); // check content
        assert_eq!(table.table.as_ref().unwrap().get(2).unwrap().len(), 2); // 2 cols
        assert_eq!(table.table.as_ref().unwrap().get(3).unwrap().len(), 0); // 0 cols
        assert_eq!(table.table.as_ref().unwrap().get(4).unwrap().len(), 1); // 1 cols
    }

    #[test]
    fn test_props_text_span() {
        // from str
        let span: TextSpan = TextSpan::from("Hello!");
        assert_eq!(span.content.as_str(), "Hello!");
        assert_eq!(span.modifiers, Modifier::empty());
        assert_eq!(span.fg, Color::Reset);
        assert_eq!(span.bg, Color::Reset);
        // From String
        let span: TextSpan = TextSpan::from(String::from("omar"));
        assert_eq!(span.content.as_str(), "omar");
        assert_eq!(span.fg, Color::Reset);
        assert_eq!(span.bg, Color::Reset);
        // With attributes
        let span: TextSpan = TextSpanBuilder::new("Error")
            .with_background(Color::Red)
            .with_foreground(Color::Black)
            .bold()
            .italic()
            .underlined()
            .rapid_blink()
            .rapid_blink()
            .slow_blink()
            .strikethrough()
            .reversed()
            .build();
        assert_eq!(span.content.as_str(), "Error");
        assert_eq!(span.fg, Color::Black);
        assert_eq!(span.bg, Color::Red);
        assert!(span.modifiers.intersects(Modifier::BOLD));
        assert!(span.modifiers.intersects(Modifier::ITALIC));
        assert!(span.modifiers.intersects(Modifier::UNDERLINED));
        assert!(span.modifiers.intersects(Modifier::SLOW_BLINK));
        assert!(span.modifiers.intersects(Modifier::RAPID_BLINK));
        assert!(span.modifiers.intersects(Modifier::REVERSED));
        assert!(span.modifiers.intersects(Modifier::CROSSED_OUT));
    }
}
