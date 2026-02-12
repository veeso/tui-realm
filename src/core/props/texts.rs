//! `Texts` is the module which defines the texts properties for components.
//! It also provides some helpers and builders to facilitate the use of builders.
use alloc::vec;
use alloc::vec::Vec;

use crate::ratatui::layout::Alignment;
use crate::ratatui::widgets::TitlePosition;

/// Simple alias for [`Span<'static>`](ratatui::text::Span).
pub type SpanStatic = crate::ratatui::text::Span<'static>;
/// Simple alias for [`Line<'static>`](ratatui::text::Line).
pub type LineStatic = crate::ratatui::text::Line<'static>;
/// Simple alias for [`Text<'static>`](ratatui::text::Text).
pub type TextStatic = crate::ratatui::text::Text<'static>;

// Note that we cannot use "ratatui::widgets::block::Title" as that is deprecated.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Title {
    /// The text and styling content of the title
    pub content: LineStatic,
    /// The Position the title should be in.
    ///
    /// This will determine if [`Block::title_top`](crate::ratatui::widgets::Block::title_top) or [`Block::title_bottom`](crate::ratatui::widgets::Block::title_bottom) is called.
    pub position: TitlePosition,
}

impl Title {
    /// Set a specific [`Alignment`] on the underlying [`Line`](crate::ratatui::text::Line).
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.content.alignment = Some(alignment);

        self
    }

    /// Set a specific position the title should be in.
    pub fn position(mut self, position: TitlePosition) -> Self {
        self.position = position;

        self
    }

    /// Overwrite the content of the title.
    pub fn content(mut self, line: LineStatic) -> Self {
        self.content = line;

        self
    }
}

impl<T> From<T> for Title
where
    T: Into<LineStatic>,
{
    fn from(value: T) -> Self {
        Self {
            content: value.into(),
            ..Default::default()
        }
    }
}

/// Table represents a list of rows with a list of columns of text spans
pub type Table = Vec<Vec<LineStatic>>;

/// Table builder is a helper to make it easier to build text tables
pub struct TableBuilder {
    table: Option<Table>,
}

impl TableBuilder {
    /// Add a column to the last row
    pub fn add_col<L>(&mut self, line: L) -> &mut Self
    where
        L: Into<LineStatic>,
    {
        if let Some(table) = self.table.as_mut() {
            if let Some(row) = table.last_mut() {
                row.push(line.into());
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

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn tables() {
        let table: Table = TableBuilder::default()
            .add_col(LineStatic::from("name"))
            .add_col(LineStatic::from("age"))
            .add_row()
            .add_col(LineStatic::from("christian"))
            .add_col(LineStatic::from("23"))
            .add_row()
            .add_col(LineStatic::from("omar"))
            .add_col(LineStatic::from("25"))
            .add_row()
            .add_row()
            .add_col(LineStatic::from("pippo"))
            .build();
        // Verify table
        assert_eq!(table.len(), 5); // 5 spans
        assert_eq!(table.first().unwrap().len(), 2); // 2 cols
        assert_eq!(table.get(1).unwrap().len(), 2); // 2 cols
        assert_eq!(
            table.get(1).unwrap().first().unwrap().to_string(),
            "christian"
        ); // check content
        assert_eq!(table.get(2).unwrap().len(), 2); // 2 cols
        assert_eq!(table.get(3).unwrap().len(), 0); // 0 cols
        assert_eq!(table.get(4).unwrap().len(), 1); // 1 cols
    }

    #[test]
    fn from_col_multi_value() {
        let _ = TableBuilder::default()
            .add_col(LineStatic::from("Line"))
            .add_col("simple str")
            .add_col(SpanStatic::from("span"));
    }

    #[test]
    fn title_from() {
        assert_eq!(
            Title::from("simple str").content,
            LineStatic::from("simple str")
        );
        assert_eq!(
            Title::from(String::from("owned string")).content,
            LineStatic::from(String::from("owned string"))
        );
        assert_eq!(
            Title::from(LineStatic::from("Line")).content,
            LineStatic::from("Line")
        );
    }

    #[test]
    fn title_builder() {
        assert_eq!(Title::from("test").position, TitlePosition::Top);
        assert_eq!(
            Title::from("test").position(TitlePosition::Bottom).position,
            TitlePosition::Bottom
        );

        assert_eq!(Title::from("test").content.alignment, None);
        assert_eq!(
            Title::from("test")
                .alignment(Alignment::Left)
                .content
                .alignment,
            Some(Alignment::Left)
        );
        assert_eq!(
            Title::from("test")
                .alignment(Alignment::Right)
                .content
                .alignment,
            Some(Alignment::Right)
        );
        assert_eq!(
            Title::from("test")
                .alignment(Alignment::Center)
                .content
                .alignment,
            Some(Alignment::Center)
        );
    }
}
