//! ## Chart
//!
//! A component to plot one or more dataset in a cartesian coordinate system

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
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, Dataset, PropPayload, PropValue, Props, Style,
};
use tuirealm::tui::{
    layout::Rect,
    text::Span,
    widgets::{Axis, Chart as TuiChart, Dataset as TuiDataset},
};
use tuirealm::{Frame, MockComponent, State};

// -- Props
use super::props::{
    CHART_X_BOUNDS, CHART_X_LABELS, CHART_X_STYLE, CHART_X_TITLE, CHART_Y_BOUNDS, CHART_Y_LABELS,
    CHART_Y_STYLE, CHART_Y_TITLE,
};

/// ### ChartStates
///
/// chart states
pub struct ChartStates {
    cursor: usize,
    data: Vec<Dataset>,
}

impl Default for ChartStates {
    fn default() -> Self {
        Self {
            cursor: 0,
            data: Vec::default(),
        }
    }
}

impl ChartStates {
    /// ### move_cursor_left
    ///
    /// Move cursor to the left
    pub fn move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// ### move_cursor_right
    ///
    /// Move cursor to the right
    pub fn move_cursor_right(&mut self, data_len: usize) {
        if data_len > 0 && self.cursor + 1 < data_len {
            self.cursor += 1;
        }
    }

    /// ### reset_cursor
    ///
    /// Reset cursor to 0
    pub fn reset_cursor(&mut self) {
        self.cursor = 0;
    }

    /// ### cursor_at_end
    ///
    /// Move cursor to the end of the chart
    pub fn cursor_at_end(&mut self, data_len: usize) {
        if data_len > 0 {
            self.cursor = data_len - 1;
        } else {
            self.cursor = 0;
        }
    }
}

// -- component

/// ### Chart
///
/// A component to display a chart on a cartesian coordinate system.
/// The chart can work both in "active" and "disabled" mode.
///
/// #### Disabled mode
///
/// When in disabled mode, the chart won't be interactive, so you won't be able to move through data using keys.
/// If you have more data than the maximum amount of bars that can be displayed, you'll have to update data to display the remaining entries
///
/// #### Active mode
///
/// While in active mode (default) you can put as many entries as you wish. You can move with arrows and END/HOME keys
pub struct Chart {
    props: Props,
    pub states: ChartStates,
}

impl Default for Chart {
    fn default() -> Self {
        Self {
            props: Props::default(),
            states: ChartStates::default(),
        }
    }
}

impl Chart {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.props.set(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.props.set(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.props.set(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.props.set(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.attr(Attribute::Disabled, AttrValue::Flag(disabled));
        self
    }

    pub fn inactive(mut self, s: Style) -> Self {
        self.props.set(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    pub fn data(mut self, data: &[Dataset]) -> Self {
        self.props.set(
            Attribute::Dataset,
            AttrValue::Payload(PropPayload::Vec(
                data.iter().cloned().map(PropValue::Dataset).collect(),
            )),
        );
        self
    }

    pub fn x_bounds(mut self, bounds: (f64, f64)) -> Self {
        self.props.set(
            Attribute::Custom(CHART_X_BOUNDS),
            AttrValue::Payload(PropPayload::Tup2((
                PropValue::F64(bounds.0),
                PropValue::F64(bounds.1),
            ))),
        );
        self
    }

    pub fn y_bounds(mut self, bounds: (f64, f64)) -> Self {
        self.props.set(
            Attribute::Custom(CHART_Y_BOUNDS),
            AttrValue::Payload(PropPayload::Tup2((
                PropValue::F64(bounds.0),
                PropValue::F64(bounds.1),
            ))),
        );
        self
    }

    pub fn x_labels(mut self, labels: &[&str]) -> Self {
        self.attr(
            Attribute::Custom(CHART_X_LABELS),
            AttrValue::Payload(PropPayload::Vec(
                labels
                    .iter()
                    .map(|x| PropValue::Str(x.to_string()))
                    .collect(),
            )),
        );
        self
    }

    pub fn y_labels(mut self, labels: &[&str]) -> Self {
        self.attr(
            Attribute::Custom(CHART_Y_LABELS),
            AttrValue::Payload(PropPayload::Vec(
                labels
                    .iter()
                    .map(|x| PropValue::Str(x.to_string()))
                    .collect(),
            )),
        );
        self
    }

    pub fn x_style(mut self, s: Style) -> Self {
        self.attr(Attribute::Custom(CHART_X_STYLE), AttrValue::Style(s));
        self
    }

    pub fn y_style(mut self, s: Style) -> Self {
        self.attr(Attribute::Custom(CHART_Y_STYLE), AttrValue::Style(s));
        self
    }

    pub fn x_title<S: AsRef<str>>(mut self, t: S) -> Self {
        self.props.set(
            Attribute::Custom(CHART_X_TITLE),
            AttrValue::String(t.as_ref().to_string()),
        );
        self
    }

    pub fn y_title<S: AsRef<str>>(mut self, t: S) -> Self {
        self.props.set(
            Attribute::Custom(CHART_Y_TITLE),
            AttrValue::String(t.as_ref().to_string()),
        );
        self
    }

    fn is_disabled(&self) -> bool {
        self.props
            .get_or(Attribute::Disabled, AttrValue::Flag(false))
            .unwrap_flag()
    }

    /// ### max_dataset_len
    ///
    /// Get the maximum len among the datasets
    fn max_dataset_len(&self) -> usize {
        self.props
            .get(Attribute::Dataset)
            .map(|x| {
                x.unwrap_payload()
                    .unwrap_vec()
                    .iter()
                    .cloned()
                    .map(|x| x.unwrap_dataset().get_data().len())
                    .max()
            })
            .unwrap_or(None)
            .unwrap_or(0)
    }

    /// ### data
    ///
    /// Get data to be displayed, starting from provided index at `start` with a max length of `len`
    fn get_data(&mut self, start: usize, len: usize) -> Vec<TuiDataset> {
        self.states.data = self
            .props
            .get(Attribute::Dataset)
            .map(|x| {
                x.unwrap_payload()
                    .unwrap_vec()
                    .into_iter()
                    .map(|x| x.unwrap_dataset())
                    .collect()
            })
            .unwrap_or_default();
        self.states
            .data
            .iter()
            .map(|x| Self::get_tui_dataset(x, start, len))
            .collect()
    }
}

impl<'a> Chart {
    /// ### get_tui_dataset
    ///
    /// Create tui_dataset from dataset
    /// Only elements from `start` to `len` are preserved from dataset
    fn get_tui_dataset(dataset: &'a Dataset, start: usize, len: usize) -> TuiDataset<'a> {
        // Recalc len
        let points = dataset.get_data();
        let len: usize = match points.len() > start {
            true => std::cmp::min(len, points.len() - start),
            false => 0,
        };
        // Prepare data storage
        let end: usize = points.len() - len;
        TuiDataset::default()
            .name(dataset.name.clone())
            .marker(dataset.marker)
            .graph_type(dataset.graph_type)
            .style(dataset.style)
            .data(&points[start..end])
    }
}

impl MockComponent for Chart {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let title = self.props.get(Attribute::Title).map(|x| x.unwrap_title());
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            let inactive_style = self
                .props
                .get(Attribute::FocusStyle)
                .map(|x| x.unwrap_style());
            let active: bool = match self.is_disabled() {
                true => true,
                false => focus,
            };
            let div = crate::utils::get_block(borders, title, active, inactive_style);
            // Create widget
            // -- x axis
            let mut x_axis: Axis = Axis::default();
            if let Some((PropValue::F64(floor), PropValue::F64(ceil))) = self
                .props
                .get(Attribute::Custom(CHART_X_BOUNDS))
                .map(|x| x.unwrap_payload().unwrap_tup2())
            {
                let why_using_vecs_when_you_can_use_useless_arrays: [f64; 2] = [floor, ceil];
                x_axis = x_axis.bounds(why_using_vecs_when_you_can_use_useless_arrays);
            }
            if let Some(PropPayload::Vec(labels)) = self
                .props
                .get(Attribute::Custom(CHART_X_LABELS))
                .map(|x| x.unwrap_payload())
            {
                x_axis = x_axis.labels(
                    labels
                        .iter()
                        .cloned()
                        .map(|x| Span::from(x.unwrap_str()))
                        .collect(),
                );
            }
            if let Some(s) = self
                .props
                .get(Attribute::Custom(CHART_X_STYLE))
                .map(|x| x.unwrap_style())
            {
                x_axis = x_axis.style(s);
            }
            if let Some(title) = self
                .props
                .get(Attribute::Custom(CHART_X_TITLE))
                .map(|x| x.unwrap_string())
            {
                x_axis = x_axis.title(Span::styled(
                    title,
                    Style::default().fg(foreground).bg(background),
                ));
            }
            // -- y axis
            let mut y_axis: Axis = Axis::default();
            if let Some((PropValue::F64(floor), PropValue::F64(ceil))) = self
                .props
                .get(Attribute::Custom(CHART_Y_BOUNDS))
                .map(|x| x.unwrap_payload().unwrap_tup2())
            {
                let why_using_vecs_when_you_can_use_useless_arrays: [f64; 2] = [floor, ceil];
                y_axis = y_axis.bounds(why_using_vecs_when_you_can_use_useless_arrays);
            }
            if let Some(PropPayload::Vec(labels)) = self
                .props
                .get(Attribute::Custom(CHART_Y_LABELS))
                .map(|x| x.unwrap_payload())
            {
                y_axis = y_axis.labels(
                    labels
                        .iter()
                        .cloned()
                        .map(|x| Span::from(x.unwrap_str()))
                        .collect(),
                );
            }
            if let Some(s) = self
                .props
                .get(Attribute::Custom(CHART_Y_STYLE))
                .map(|x| x.unwrap_style())
            {
                y_axis = y_axis.style(s);
            }
            if let Some(title) = self
                .props
                .get(Attribute::Custom(CHART_Y_TITLE))
                .map(|x| x.unwrap_string())
            {
                y_axis = y_axis.title(Span::styled(
                    title,
                    Style::default().fg(foreground).bg(background),
                ));
            }
            // Get data
            let data: Vec<TuiDataset> = self.get_data(self.states.cursor, area.width as usize);
            // Build widget
            let widget: TuiChart = TuiChart::new(data).block(div).x_axis(x_axis).y_axis(y_axis);
            // Render
            render.render_widget(widget, area);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
        self.states.reset_cursor();
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        if !self.is_disabled() {
            match cmd {
                Cmd::Move(Direction::Left) => {
                    self.states.move_cursor_left();
                }
                Cmd::Move(Direction::Right) => {
                    self.states.move_cursor_right(self.max_dataset_len());
                }
                Cmd::GoTo(Position::Begin) => {
                    self.states.reset_cursor();
                }
                Cmd::GoTo(Position::End) => {
                    self.states.cursor_at_end(self.max_dataset_len());
                }
                _ => {}
            }
        }
        CmdResult::None
    }

    fn state(&self) -> State {
        State::None
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use tuirealm::tui::{symbols::Marker, widgets::GraphType};

    #[test]
    fn test_components_chart_states() {
        let mut states: ChartStates = ChartStates::default();
        assert_eq!(states.cursor, 0);
        // Incr
        states.move_cursor_right(2);
        assert_eq!(states.cursor, 1);
        // At end
        states.move_cursor_right(2);
        assert_eq!(states.cursor, 1);
        // Decr
        states.move_cursor_left();
        assert_eq!(states.cursor, 0);
        // At begin
        states.move_cursor_left();
        assert_eq!(states.cursor, 0);
        // Move at end
        states.cursor_at_end(3);
        assert_eq!(states.cursor, 2);
        states.reset_cursor();
        assert_eq!(states.cursor, 0);
    }

    #[test]
    fn test_components_chart() {
        let mut component: Chart = Chart::default()
            .disabled(false)
            .background(Color::Reset)
            .foreground(Color::Reset)
            .borders(Borders::default())
            .title("average temperatures in Udine", Alignment::Center)
            .x_bounds((0.0, 11.0))
            .x_labels(&[
                "january",
                "february",
                "march",
                "april",
                "may",
                "june",
                "july",
                "august",
                "september",
                "october",
                "november",
                "december",
            ])
            .x_style(Style::default().fg(Color::LightBlue))
            .x_title("Temperature (°C)")
            .y_bounds((-5.0, 35.0))
            .y_labels(&["-5", "0", "5", "10", "15", "20", "25", "30", "35"])
            .y_style(Style::default().fg(Color::LightYellow))
            .y_title("Month")
            .data(&[
                Dataset::default()
                    .name("Minimum")
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
                    ]),
                Dataset::default()
                    .name("Maximum")
                    .graph_type(GraphType::Line)
                    .marker(Marker::Dot)
                    .style(Style::default().fg(Color::LightRed))
                    .data(vec![
                        (0.0, 7.0),
                        (1.0, 9.0),
                        (2.0, 13.0),
                        (3.0, 17.0),
                        (4.0, 22.0),
                        (5.0, 25.0),
                        (6.0, 28.0),
                        (7.0, 28.0),
                        (8.0, 24.0),
                        (9.0, 19.0),
                        (10.0, 13.0),
                        (11.0, 8.0),
                    ]),
            ]);
        // Commands
        assert_eq!(component.state(), State::None);
        // -> Right
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::None
        );
        assert_eq!(component.states.cursor, 1);
        // <- Left
        assert_eq!(
            component.perform(Cmd::Move(Direction::Left)),
            CmdResult::None
        );
        assert_eq!(component.states.cursor, 0);
        // End
        assert_eq!(component.perform(Cmd::GoTo(Position::End)), CmdResult::None);
        assert_eq!(component.states.cursor, 11);
        // Home
        assert_eq!(
            component.perform(Cmd::GoTo(Position::Begin)),
            CmdResult::None
        );
        assert_eq!(component.states.cursor, 0);
        // component funcs
        assert_eq!(component.max_dataset_len(), 12);
        assert_eq!(component.is_disabled(), false);
        assert_eq!(component.get_data(2, 4).len(), 2);
        // Update and test empty data
        component.states.cursor_at_end(12);
        component.attr(
            Attribute::Dataset,
            AttrValue::Payload(PropPayload::Vec(vec![])),
        );
        assert_eq!(component.max_dataset_len(), 0);
        // Cursor is reset
        assert_eq!(component.states.cursor, 0);
    }
}
