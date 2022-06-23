//! ## Texts
//!
//! `Texts` is the module which defines the texts properties for components.
//! It also provides some helpers and builders to facilitate the use of builders.

use tui::style::{Color, Modifier};

// -- Text parts

/// ### TextSpan
///
/// TextSpan is a "cell" of text with its attributes
#[derive(Clone, Debug, PartialEq)]
pub struct TextSpan {
    pub content: String,
    pub fg: Color,
    pub bg: Color,
    pub modifiers: Modifier,
}

impl TextSpan {
    /// Instantiate a new `TextSpan`
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            content: text.as_ref().to_string(),
            fg: Color::Reset,
            bg: Color::Reset,
            modifiers: Modifier::empty(),
        }
    }

    pub fn fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }

    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    /// Set bold property for text
    pub fn bold(mut self) -> Self {
        self.modifiers |= Modifier::BOLD;
        self
    }

    /// Set italic property for text
    pub fn italic(mut self) -> Self {
        self.modifiers |= Modifier::ITALIC;
        self
    }

    /// Set underlined property for text
    pub fn underlined(mut self) -> Self {
        self.modifiers |= Modifier::UNDERLINED;
        self
    }

    /// Set slow_blink property for text
    pub fn slow_blink(mut self) -> Self {
        self.modifiers |= Modifier::SLOW_BLINK;
        self
    }

    /// Set rapid_blink property for text
    pub fn rapid_blink(mut self) -> Self {
        self.modifiers |= Modifier::RAPID_BLINK;
        self
    }

    /// Set reversed property for text
    pub fn reversed(mut self) -> Self {
        self.modifiers |= Modifier::REVERSED;
        self
    }

    /// Set strikethrough property for text
    pub fn strikethrough(mut self) -> Self {
        self.modifiers |= Modifier::CROSSED_OUT;
        self
    }
}

impl Default for TextSpan {
    fn default() -> Self {
        Self::new(String::default())
    }
}

impl<S> From<S> for TextSpan
where
    S: AsRef<str>,
{
    fn from(txt: S) -> Self {
        TextSpan::new(txt)
    }
}

/// Table represents a list of rows with a list of columns of text spans
pub type Table = Vec<Vec<TextSpan>>;

/// Table builder is a helper to make it easier to build text tables
pub struct TableBuilder {
    table: Option<Table>,
}

impl TableBuilder {
    /// Add a column to the last row
    pub fn add_col(&mut self, span: TextSpan) -> &mut Self {
        if let Some(table) = self.table.as_mut() {
            if let Some(row) = table.last_mut() {
                row.push(span);
            }
        }
        self
    }

    /// Add a new row to the table
    pub fn add_row(&mut self) -> &mut Self {
        if let Some(table) = self.table.as_mut() {
            table.push(vec![]);
        }
        self
    }

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

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn tables() {
        let table: Table = TableBuilder::default()
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
            .build();
        // Verify table
        assert_eq!(table.len(), 5); // 5 spans
        assert_eq!(table.get(0).unwrap().len(), 2); // 2 cols
        assert_eq!(table.get(1).unwrap().len(), 2); // 2 cols
        assert_eq!(
            table.get(1).unwrap().get(0).unwrap().content.as_str(),
            "christian"
        ); // check content
        assert_eq!(table.get(2).unwrap().len(), 2); // 2 cols
        assert_eq!(table.get(3).unwrap().len(), 0); // 0 cols
        assert_eq!(table.get(4).unwrap().len(), 1); // 1 cols
    }

    #[test]
    fn text_span() {
        // default
        let span: TextSpan = TextSpan::default();
        assert_eq!(span.content.as_str(), "");
        assert_eq!(span.modifiers, Modifier::empty());
        assert_eq!(span.fg, Color::Reset);
        assert_eq!(span.bg, Color::Reset);
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
        let span: TextSpan = TextSpan::new("Error")
            .bg(Color::Red)
            .fg(Color::Black)
            .bold()
            .italic()
            .underlined()
            .rapid_blink()
            .rapid_blink()
            .slow_blink()
            .strikethrough()
            .reversed();
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
