//! ## Dataset
//!
//! `Dataset` is a wrapper for tui dataset

use super::Style;
use crate::tui::{
    symbols::Marker,
    widgets::{Dataset as TuiDataset, GraphType},
};

/// Dataset describes a set of data for a chart
#[derive(Clone, Debug)]
pub struct Dataset {
    pub name: String,
    pub marker: Marker,
    pub graph_type: GraphType,
    pub style: Style,
    data: Vec<(f64, f64)>,
}

impl Default for Dataset {
    fn default() -> Self {
        Self {
            name: String::default(),
            marker: Marker::Dot,
            graph_type: GraphType::Scatter,
            style: Style::default(),
            data: Vec::default(),
        }
    }
}

impl Dataset {
    /// Set name for dataset
    pub fn name<S: AsRef<str>>(mut self, s: S) -> Self {
        self.name = s.as_ref().to_string();
        self
    }

    /// Set marker type for dataset
    pub fn marker(mut self, m: Marker) -> Self {
        self.marker = m;
        self
    }

    /// Set graphtype for dataset
    pub fn graph_type(mut self, g: GraphType) -> Self {
        self.graph_type = g;
        self
    }

    /// Set style for dataset
    pub fn style(mut self, s: Style) -> Self {
        self.style = s;
        self
    }

    /// Set data for dataset; must be a vec of (f64, f64)
    pub fn data(mut self, data: Vec<(f64, f64)>) -> Self {
        self.data = data;
        self
    }

    /// Push a record to the back of dataset
    pub fn push(&mut self, point: (f64, f64)) {
        self.data.push(point);
    }

    /// Pop last element of dataset
    pub fn pop(&mut self) {
        self.data.pop();
    }

    /// Pop last element of dataset
    pub fn pop_front(&mut self) {
        if !self.data.is_empty() {
            self.data.remove(0);
        }
    }

    /// Get a reference to data
    pub fn get_data(&self) -> &[(f64, f64)] {
        &self.data
    }
}

impl PartialEq for Dataset {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.data == other.data
    }
}

impl<'a> From<&'a Dataset> for TuiDataset<'a> {
    fn from(data: &'a Dataset) -> TuiDataset<'a> {
        TuiDataset::default()
            .name(data.name.clone())
            .marker(data.marker)
            .graph_type(data.graph_type)
            .style(data.style)
            .data(data.get_data())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::tui::style::Color;
    use pretty_assertions::assert_eq;

    #[test]
    fn dataset() {
        let mut dataset: Dataset = Dataset::default()
            .name("Avg temperatures")
            .graph_type(GraphType::Scatter)
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(vec![
                (0.0, -1.0),
                (1.0, 1.0),
                (2.0, 3.0),
                (3.0, 7.0),
                (4.0, 11.0),
                (5.0, 15.0),
                (6.0, 17.0),
                (7.0, 17.0),
                (8.0, 13.0),
                (9.0, 9.0),
                (10.0, 4.0),
                (11.0, 0.0),
            ]);
        assert_eq!(dataset.name.as_str(), "Avg temperatures");
        assert_eq!(dataset.style.fg.unwrap_or(Color::Reset), Color::Cyan);
        assert_eq!(dataset.get_data().len(), 12);
        // mut
        dataset.push((12.0, 1.0));
        assert_eq!(dataset.get_data().len(), 13);
        dataset.pop();
        assert_eq!(dataset.get_data().len(), 12);
        dataset.pop_front();
        assert_eq!(dataset.get_data().len(), 11);
        // From
        let _: TuiDataset = TuiDataset::from(&dataset);
    }
}
