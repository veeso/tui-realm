use tuirealm::props::{LineStatic, Style};
use tuirealm::ratatui::symbols::Marker;
use tuirealm::ratatui::widgets::{Dataset as TuiDataset, GraphType};

/// Dataset describes a set of data for a chart.
///
/// This mainly exists to map to ratatui's [`Dataset`](TuiDataset), which does not allow owned data.
#[derive(Clone, Debug)]
pub struct ChartDataset {
    pub name: LineStatic,
    pub marker: Marker,
    pub graph_type: GraphType,
    pub style: Style,
    data: Vec<(f64, f64)>,
}

impl Default for ChartDataset {
    fn default() -> Self {
        Self {
            name: LineStatic::default(),
            marker: Marker::Dot,
            graph_type: GraphType::Scatter,
            style: Style::default(),
            data: Vec::default(),
        }
    }
}

impl ChartDataset {
    /// Set a name for the dataset.
    pub fn name<S: Into<LineStatic>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    /// Set the [`Marker`] type for the dataset.
    pub fn marker(mut self, m: Marker) -> Self {
        self.marker = m;
        self
    }

    /// Set the [`GraphType`] for the dataset.
    pub fn graph_type(mut self, g: GraphType) -> Self {
        self.graph_type = g;
        self
    }

    /// Set Style for the dataset.
    ///
    /// This style is used for the data points and the legenend (if not overwritten by the text's style).
    ///
    /// Read more in [`Dataset::style`](TuiDataset::style).
    pub fn style(mut self, s: Style) -> Self {
        self.style = s;
        self
    }

    /// Set the data for this dataset.
    pub fn data(mut self, data: Vec<(f64, f64)>) -> Self {
        self.data = data;
        self
    }

    /// Push a new point to the back of this dataset.
    pub fn push(&mut self, point: (f64, f64)) {
        self.data.push(point);
    }

    /// Pop the last point from this dataset.
    pub fn pop(&mut self) {
        self.data.pop();
    }

    /// Pop the first point in this dataset.
    pub fn pop_front(&mut self) {
        if !self.data.is_empty() {
            self.data.remove(0);
        }
    }

    /// Get a reference to the data.
    pub fn get_data(&self) -> &[(f64, f64)] {
        &self.data
    }

    /// Create ratatui [`Dataset`](TuiDataset) from the current [`ChartDataset`].
    ///
    /// Only elements from `start` are included.
    pub fn as_tuichart<'a>(&'a self, start: usize) -> TuiDataset<'a> {
        // Prepare data storage
        TuiDataset::default()
            .name(self.name.clone())
            .marker(self.marker)
            .graph_type(self.graph_type)
            .style(self.style)
            .data(&self.get_data()[start..])
    }
}

impl PartialEq for ChartDataset {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.data == other.data
    }
}

impl<'a> From<&'a ChartDataset> for TuiDataset<'a> {
    fn from(data: &'a ChartDataset) -> TuiDataset<'a> {
        data.as_tuichart(0)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;
    use tuirealm::ratatui::style::Color;

    use super::*;

    #[test]
    fn dataset() {
        let mut dataset: ChartDataset = ChartDataset::default()
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
        assert_eq!(dataset.name.to_string(), "Avg temperatures");
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
