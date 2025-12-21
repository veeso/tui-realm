//! ## Texts
//!
//! `Texts` is the module which defines the texts properties for components.
//! It also provides some helpers and builders to facilitate the use of builders.

pub type SpanStatic = crate::ratatui::text::Span<'static>;
pub type LineStatic = crate::ratatui::text::Line<'static>;
pub type TextStatic = crate::ratatui::text::Text<'static>;

/// Table represents a list of rows with a list of columns of text spans
pub type Table = Vec<Vec<LineStatic>>;

/// Table builder is a helper to make it easier to build text tables
pub struct TableBuilder {
    table: Option<Table>,
}

impl TableBuilder {
    /// Add a column to the last row
    pub fn add_col(&mut self, line: LineStatic) -> &mut Self {
        if let Some(table) = self.table.as_mut() {
            if let Some(row) = table.last_mut() {
                row.push(line);
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
}
