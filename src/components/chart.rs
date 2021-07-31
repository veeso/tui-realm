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
use crate::event::KeyCode;
use crate::props::{BordersProps, Dataset, PropPayload, PropValue, Props, PropsBuilder};
use crate::tui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Axis, Block, BorderType, Borders, Chart as TuiChart, Dataset as TuiDataset},
};
use crate::{Component, Event, Frame, Msg, Payload};
use std::collections::LinkedList;

// -- Props
const PROP_X_BOUNDS: &str = "x-bounds";
const PROP_Y_BOUNDS: &str = "y-bounds";
const PROP_X_LABELS: &str = "x-labels";
const PROP_X_STYLE: &str = "x-style";
const PROP_Y_LABELS: &str = "y-labels";
const PROP_X_TITLE: &str = "x-title";
const PROP_Y_STYLE: &str = "y-style";
const PROP_Y_TITLE: &str = "y-title";
const PROP_DATA: &str = "data";
const PROP_DISABLED: &str = "disabled";
const PROP_TITLE: &str = "title";

pub struct ChartPropsBuilder {
    props: Option<Props>,
}

impl Default for ChartPropsBuilder {
    fn default() -> Self {
        Self {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for ChartPropsBuilder {
    fn build(&mut self) -> Props {
        self.props.take().unwrap()
    }

    fn hidden(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = false;
        }
        self
    }

    fn visible(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = true;
        }
        self
    }
}

impl From<Props> for ChartPropsBuilder {
    fn from(props: Props) -> Self {
        Self { props: Some(props) }
    }
}

impl ChartPropsBuilder {
    /// ### with_foreground
    ///
    /// Set foreground color for component
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    /// ### with_background
    ///
    /// Set background color for component
    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.background = color;
        }
        self
    }

    /// ### with_borders
    ///
    /// Set component borders style
    pub fn with_borders(
        &mut self,
        borders: Borders,
        variant: BorderType,
        color: Color,
    ) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.borders = BordersProps {
                borders,
                variant,
                color,
            }
        }
        self
    }

    /// ### with_title
    ///
    /// Set title
    pub fn with_title<S: AsRef<str>>(&mut self, title: S) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_TITLE,
                PropPayload::One(PropValue::Str(title.as_ref().to_string())),
            );
        }
        self
    }

    /// ### with_x_bounds
    ///
    /// Define x axis bounds
    pub fn with_x_bounds(&mut self, (floor, ceil): (f64, f64)) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_X_BOUNDS,
                PropPayload::Tup2((PropValue::F64(floor), PropValue::F64(ceil))),
            );
        }
        self
    }

    /// ### with_y_bounds
    ///
    /// Define y axis bounds
    pub fn with_y_bounds(&mut self, (floor, ceil): (f64, f64)) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_Y_BOUNDS,
                PropPayload::Tup2((PropValue::F64(floor), PropValue::F64(ceil))),
            );
        }
        self
    }

    /// ### with_x_labels
    ///
    /// Define x axis labels
    pub fn with_x_labels(&mut self, labels: &[&str]) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_X_LABELS,
                PropPayload::Vec(
                    labels
                        .iter()
                        .map(|x| PropValue::Str(x.to_string()))
                        .collect(),
                ),
            );
        }
        self
    }

    /// ### with_y_labels
    ///
    /// Define y axis labels
    pub fn with_y_labels(&mut self, labels: &[&str]) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_Y_LABELS,
                PropPayload::Vec(
                    labels
                        .iter()
                        .map(|x| PropValue::Str(x.to_string()))
                        .collect(),
                ),
            );
        }
        self
    }

    /// ### with_x_style
    ///
    /// Define x axis style
    pub fn with_x_style(&mut self, s: Style) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_X_STYLE, PropPayload::One(PropValue::Style(s)));
        }
        self
    }

    /// ### with_y_style
    ///
    /// Define y axis style
    pub fn with_y_style(&mut self, s: Style) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_Y_STYLE, PropPayload::One(PropValue::Style(s)));
        }
        self
    }

    /// ### with_x_title
    ///
    /// Define x axis title
    pub fn with_x_title(&mut self, title: &str) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_X_TITLE,
                PropPayload::One(PropValue::Str(title.to_string())),
            );
        }
        self
    }

    /// ### with_y_title
    ///
    /// Define y axis title
    pub fn with_y_title(&mut self, title: &str) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_Y_TITLE,
                PropPayload::One(PropValue::Str(title.to_string())),
            );
        }
        self
    }

    /// ### with_data
    ///
    /// Define chart data as a list of dataset
    pub fn with_data(&mut self, data: &[Dataset]) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            // Create linked list
            let mut list: LinkedList<PropPayload> = LinkedList::new();
            data.iter()
                .for_each(|x| list.push_back(PropPayload::One(PropValue::Dataset(x.clone()))));
            props.own.insert(PROP_DATA, PropPayload::Linked(list));
        }
        self
    }

    /// ### push_record
    ///
    /// Just pushes a record to the front of the data
    /// Set is the index of the chart where you want to push the point
    /// Panics if data is not initialized
    pub fn push_record(&mut self, set: usize, point: (f64, f64)) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            match props.own.get_mut(PROP_DATA) {
                Some(PropPayload::Linked(list)) => {
                    if let Some(PropPayload::One(PropValue::Dataset(dataset))) =
                        list.iter_mut().nth(set)
                    {
                        dataset.push(point);
                    }
                }
                _ => panic!("Data must be initialized first"),
            }
        }
        self
    }

    /// ### pop_record_back
    ///
    /// Pop last record on the back of the data list
    /// Set is the index of the chart where you want to pop from
    pub fn pop_record_back(&mut self, set: usize) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            if let Some(PropPayload::Linked(list)) = props.own.get_mut(PROP_DATA) {
                if let Some(PropPayload::One(PropValue::Dataset(dataset))) =
                    list.iter_mut().nth(set)
                {
                    dataset.pop();
                }
            }
        }
        self
    }

    /// ### pop_record_front
    ///
    /// Pop first record on the back of the data list
    /// Set is the index of the chart where you want to pop from
    pub fn pop_record_front(&mut self, set: usize) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            if let Some(PropPayload::Linked(list)) = props.own.get_mut(PROP_DATA) {
                if let Some(PropPayload::One(PropValue::Dataset(dataset))) =
                    list.iter_mut().nth(set)
                {
                    dataset.pop_front();
                }
            }
        }
        self
    }

    /// ### disabled
    ///
    /// If component is set to `disabled`, then input commands won't work, and colors will be rendered
    /// as if the component would have focus
    pub fn disabled(&mut self, disabled: bool) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_DISABLED, PropPayload::One(PropValue::Bool(disabled)));
        }
        self
    }
}

// -- states

// -- states

/// ### OwnStates
///
/// chart states
struct OwnStates {
    cursor: usize,
    focus: bool,
}

impl Default for OwnStates {
    fn default() -> Self {
        Self {
            cursor: 0,
            focus: false,
        }
    }
}

impl OwnStates {
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
    states: OwnStates,
}

impl Chart {
    /// ### new
    ///
    /// Instantiates a new `Chart` component
    pub fn new(props: Props) -> Self {
        Self {
            props,
            states: OwnStates::default(),
        }
    }

    /// ### disabled
    ///
    /// Returns whether the component is in `disabled` mode
    fn disabled(&self) -> bool {
        self.props
            .own
            .get(PROP_DISABLED)
            .map(|x| match x {
                PropPayload::One(PropValue::Bool(disabled)) => *disabled,
                _ => false,
            })
            .unwrap_or(false)
    }

    /// ### data_len
    ///
    /// Retrieve current data len from properties
    fn data_len(&self) -> usize {
        self.props
            .own
            .get(PROP_DATA)
            .map(|x| match x {
                PropPayload::Linked(l) => l.len(),
                _ => 0,
            })
            .unwrap_or(0)
    }

    /// ### max_dataset_len
    ///
    /// Get the maximum len among the datasets
    fn max_dataset_len(&self) -> usize {
        self.props
            .own
            .get(PROP_DATA)
            .map(|x| {
                (match x {
                    PropPayload::Linked(l) => l
                        .iter()
                        .map(|x| match x {
                            PropPayload::One(PropValue::Dataset(dataset)) => {
                                dataset.get_data().len()
                            }
                            _ => 0,
                        })
                        .max(),
                    _ => Some(0),
                })
                .unwrap_or(0)
            })
            .unwrap_or(0)
    }

    /// ### data
    ///
    /// Get data to be displayed, starting from provided index at `start` with a max length of `len`
    fn data(&self, start: usize, len: usize) -> Vec<TuiDataset> {
        if let Some(PropPayload::Linked(list)) = self.props.own.get(PROP_DATA) {
            let mut data: Vec<TuiDataset> = Vec::with_capacity(self.data_len());
            for item in list.iter() {
                if let PropPayload::One(PropValue::Dataset(dataset)) = item {
                    data.push(Self::get_tui_dataset(dataset, start, len));
                }
            }
            data
        } else {
            Vec::new()
        }
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

impl Component for Chart {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    fn render(&self, render: &mut Frame, area: Rect) {
        if self.props.visible {
            // If component is disabled, will be displayed as `active`; as focus state otherwise
            let active: bool = match self.disabled() {
                true => true,
                false => self.states.focus,
            };
            let title: Option<&str> = match self.props.own.get(PROP_TITLE).as_ref() {
                Some(PropPayload::One(PropValue::Str(t))) => Some(t),
                _ => None,
            };
            let block: Block = super::utils::get_block(&self.props.borders, title, active);
            // Get data
            let data: Vec<TuiDataset> = self.data(self.states.cursor, area.width as usize);
            // Create widget
            // -- x axis
            let mut x_axis: Axis = Axis::default();
            if let Some(PropPayload::Tup2((PropValue::F64(floor), PropValue::F64(ceil)))) =
                self.props.own.get(PROP_X_BOUNDS)
            {
                let why_using_vecs_when_you_can_use_useless_arrays: [f64; 2] = [*floor, *ceil];
                x_axis = x_axis.bounds(why_using_vecs_when_you_can_use_useless_arrays);
            }
            if let Some(PropPayload::Vec(labels)) = self.props.own.get(PROP_X_LABELS) {
                x_axis = x_axis.labels(
                    labels
                        .iter()
                        .map(|x| match x {
                            PropValue::Str(x) => Span::from(x.clone()),
                            _ => panic!("Label is not a string"),
                        })
                        .collect(),
                );
            }
            if let Some(PropPayload::One(PropValue::Style(s))) = self.props.own.get(PROP_X_STYLE) {
                x_axis = x_axis.style(*s);
            }
            if let Some(PropPayload::One(PropValue::Str(title))) = self.props.own.get(PROP_X_TITLE)
            {
                x_axis = x_axis.title(Span::styled(
                    title,
                    Style::default()
                        .fg(self.props.foreground)
                        .bg(self.props.background),
                ));
            }
            // -- y axis
            let mut y_axis: Axis = Axis::default();
            if let Some(PropPayload::Tup2((PropValue::F64(floor), PropValue::F64(ceil)))) =
                self.props.own.get(PROP_Y_BOUNDS)
            {
                let why_using_vecs_when_you_can_use_useless_arrays: [f64; 2] = [*floor, *ceil];
                y_axis = y_axis.bounds(why_using_vecs_when_you_can_use_useless_arrays);
            }
            if let Some(PropPayload::Vec(labels)) = self.props.own.get(PROP_Y_LABELS) {
                y_axis = y_axis.labels(
                    labels
                        .iter()
                        .map(|x| match x {
                            PropValue::Str(x) => Span::from(x.clone()),
                            _ => panic!("Label is not a string"),
                        })
                        .collect(),
                );
            }
            if let Some(PropPayload::One(PropValue::Style(s))) = self.props.own.get(PROP_Y_STYLE) {
                y_axis = y_axis.style(*s);
            }
            if let Some(PropPayload::One(PropValue::Str(title))) = self.props.own.get(PROP_Y_TITLE)
            {
                y_axis = y_axis.title(Span::styled(
                    title,
                    Style::default()
                        .fg(self.props.foreground)
                        .bg(self.props.background),
                ));
            }
            // Build widget
            let widget: TuiChart = TuiChart::new(data)
                .block(block)
                .x_axis(x_axis)
                .y_axis(y_axis);
            // Render
            render.render_widget(widget, area);
        }
    }

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update.
    /// Returns a Msg to the view
    fn update(&mut self, props: Props) -> Msg {
        self.props = props;
        // Reset cursor
        self.states.reset_cursor();
        Msg::None
    }

    /// ### get_props
    ///
    /// Returns a props builder starting from component properties.
    /// This returns a prop builder in order to make easier to create
    /// new properties for the element.
    fn get_props(&self) -> Props {
        self.props.clone()
    }

    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view
    fn on(&mut self, ev: Event) -> Msg {
        if let Event::Key(key) = ev {
            if self.disabled() {
                Msg::OnKey(key)
            } else {
                match key.code {
                    KeyCode::Left => {
                        // Move cursor left; msg None
                        self.states.move_cursor_left();
                        Msg::None
                    }
                    KeyCode::Right => {
                        // Move cursor right; Msg None
                        self.states.move_cursor_right(self.max_dataset_len());
                        Msg::None
                    }
                    KeyCode::End => {
                        // Cursor at last position
                        self.states.cursor_at_end(self.max_dataset_len());
                        Msg::None
                    }
                    KeyCode::Home => {
                        // Cursor at first positon
                        self.states.reset_cursor();
                        Msg::None
                    }
                    _ => Msg::OnKey(key),
                }
            }
        } else {
            Msg::None
        }
    }

    /// ### get_state
    ///
    /// Get current state from component
    /// This component always returns `None`
    fn get_state(&self) -> Payload {
        Payload::None
    }

    // -- events

    /// ### blur
    ///
    /// Blur component; basically remove focus
    fn blur(&mut self) {
        self.states.focus = false;
    }

    /// ### active
    ///
    /// Active component; basically give focus
    /// Works only if not disabled
    fn active(&mut self) {
        if !self.disabled() {
            self.states.focus = true;
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crate::tui::{symbols::Marker, widgets::GraphType};
    use crossterm::event::KeyEvent;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_chart_states() {
        let mut states: OwnStates = OwnStates::default();
        assert_eq!(states.cursor, 0);
        assert_eq!(states.focus, false);
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
        let mut component: Chart = Chart::new(
            ChartPropsBuilder::default()
                .hidden()
                .visible()
                .disabled(false)
                .with_background(Color::Reset)
                .with_foreground(Color::Reset)
                .with_borders(Borders::ALL, BorderType::Double, Color::Yellow)
                .with_title(String::from("average temperatures in Udine"))
                .with_x_bounds((0.0, 11.0))
                .with_x_labels(&[
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
                .with_x_style(Style::default().fg(Color::LightBlue))
                .with_x_title("Temperature (°C)")
                .with_y_bounds((-5.0, 35.0))
                .with_y_labels(&["-5", "0", "5", "10", "15", "20", "25", "30", "35"])
                .with_y_style(Style::default().fg(Color::LightYellow))
                .with_y_title("Month")
                .with_data(&[
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
                ])
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Reset);
        assert_eq!(component.props.background, Color::Reset);
        assert_eq!(component.props.visible, true);
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Double);
        assert_eq!(component.props.borders.color, Color::Yellow);
        assert_eq!(
            *component.props.own.get(PROP_X_BOUNDS).unwrap(),
            PropPayload::Tup2((PropValue::F64(0.0), PropValue::F64(11.0)))
        );
        assert!(component.props.own.get(PROP_X_LABELS).is_some());
        assert!(component.props.own.get(PROP_X_STYLE).is_some());
        assert!(component.props.own.get(PROP_X_TITLE).is_some());
        assert_eq!(
            *component.props.own.get(PROP_Y_BOUNDS).unwrap(),
            PropPayload::Tup2((PropValue::F64(-5.0), PropValue::F64(35.0)))
        );
        assert!(component.props.own.get(PROP_Y_LABELS).is_some());
        assert!(component.props.own.get(PROP_Y_STYLE).is_some());
        assert!(component.props.own.get(PROP_Y_TITLE).is_some());
        assert!(component.props.own.get(PROP_DATA).is_some());
        assert_eq!(
            *component.props.own.get(PROP_DISABLED).unwrap(),
            PropPayload::One(PropValue::Bool(false))
        );
        // focus
        component.active();
        assert_eq!(component.states.focus, true);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Commands
        assert_eq!(component.get_state(), Payload::None);
        // -> Right
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::None
        );
        assert_eq!(component.states.cursor, 1);
        // <- Left
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Left))),
            Msg::None
        );
        assert_eq!(component.states.cursor, 0);
        // End
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::End))),
            Msg::None
        );
        assert_eq!(component.states.cursor, 11);
        // Home
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Home))),
            Msg::None
        );
        assert_eq!(component.states.cursor, 0);
        // other keys
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::OnKey(KeyEvent::from(KeyCode::Char('a'))),
        );
        // component funcs
        assert_eq!(component.data_len(), 2);
        assert_eq!(component.max_dataset_len(), 12);
        assert_eq!(component.disabled(), false);
        assert_eq!(component.data(2, 4).len(), 2);
        // Update and test empty data
        component.states.cursor_at_end(12);
        assert_eq!(
            component.update(
                ChartPropsBuilder::from(component.get_props())
                    .with_data(&[])
                    .build()
            ),
            Msg::None
        );
        assert_eq!(component.max_dataset_len(), 0);
        // Cursor is reset
        assert_eq!(component.states.cursor, 0);
    }

    #[test]
    fn test_components_chart_disabled() {
        let mut component: Chart = Chart::new(
            ChartPropsBuilder::default()
                .hidden()
                .visible()
                .disabled(true)
                .with_background(Color::Reset)
                .with_foreground(Color::Reset)
                .with_title(String::from("average temperatures in Udine"))
                .with_borders(Borders::ALL, BorderType::Double, Color::Yellow)
                .with_x_bounds((0.0, 11.0))
                .with_x_labels(&[
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
                .with_x_style(Style::default().fg(Color::LightBlue))
                .with_x_title("Temperature (°C)")
                .with_y_bounds((-5.0, 35.0))
                .with_y_labels(&["-5", "0", "5", "10", "15", "20", "25", "30", "35"])
                .with_y_style(Style::default().fg(Color::LightYellow))
                .with_y_title("Month")
                .with_data(&[
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
                ])
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Reset);
        assert_eq!(component.props.background, Color::Reset);
        assert_eq!(component.props.visible, true);
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Double);
        assert_eq!(component.props.borders.color, Color::Yellow);
        assert_eq!(
            *component.props.own.get(PROP_X_BOUNDS).unwrap(),
            PropPayload::Tup2((PropValue::F64(0.0), PropValue::F64(11.0)))
        );
        assert!(component.props.own.get(PROP_X_LABELS).is_some());
        assert!(component.props.own.get(PROP_X_STYLE).is_some());
        assert!(component.props.own.get(PROP_X_TITLE).is_some());
        assert_eq!(
            *component.props.own.get(PROP_Y_BOUNDS).unwrap(),
            PropPayload::Tup2((PropValue::F64(-5.0), PropValue::F64(35.0)))
        );
        assert!(component.props.own.get(PROP_Y_LABELS).is_some());
        assert!(component.props.own.get(PROP_Y_STYLE).is_some());
        assert!(component.props.own.get(PROP_Y_TITLE).is_some());
        assert!(component.props.own.get(PROP_DATA).is_some());
        assert_eq!(
            *component.props.own.get(PROP_DISABLED).unwrap(),
            PropPayload::One(PropValue::Bool(true))
        );
        // focus
        component.active();
        assert_eq!(component.states.focus, false);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Commands
        assert_eq!(component.get_state(), Payload::None);
        // -> Right
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::OnKey(KeyEvent::from(KeyCode::Right))
        );
        assert_eq!(component.states.cursor, 0);
        // <- Left
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Left))),
            Msg::OnKey(KeyEvent::from(KeyCode::Left))
        );
        assert_eq!(component.states.cursor, 0);
        // End
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::End))),
            Msg::OnKey(KeyEvent::from(KeyCode::End))
        );
        assert_eq!(component.states.cursor, 0);
        // Home
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Home))),
            Msg::OnKey(KeyEvent::from(KeyCode::Home))
        );
        assert_eq!(component.states.cursor, 0);
        // other keys
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::OnKey(KeyEvent::from(KeyCode::Char('a'))),
        );
        // component funcs
        assert_eq!(component.data_len(), 2);
        assert_eq!(component.max_dataset_len(), 12);
        assert_eq!(component.disabled(), true);
        assert_eq!(component.data(2, 4).len(), 2);
        // Add a new record
        assert_eq!(
            component.update(
                ChartPropsBuilder::from(component.get_props())
                    .push_record(0, (0.5, 2.0))
                    .push_record(0, (2.5, 5.5))
                    .push_record(1, (10.2, 11.2))
                    .build()
            ),
            Msg::None
        );
        assert_eq!(component.max_dataset_len(), 14);
        // Pop records
        assert_eq!(
            component.update(
                ChartPropsBuilder::from(component.get_props())
                    .pop_record_back(0)
                    .pop_record_back(1)
                    .pop_record_front(1)
                    .build()
            ),
            Msg::None
        );
        assert_eq!(component.max_dataset_len(), 13);
        // Update and test empty data
        component.states.cursor_at_end(12);
        assert_eq!(
            component.update(
                ChartPropsBuilder::from(component.get_props())
                    .with_data(&[])
                    .build()
            ),
            Msg::None
        );
        assert_eq!(component.max_dataset_len(), 0);
        // Cursor is reset
        assert_eq!(component.states.cursor, 0);
    }

    #[test]
    #[should_panic]
    fn test_components_chart_uninitialized() {
        ChartPropsBuilder::default().push_record(2, (0.4, 0.7));
    }

    #[test]
    #[should_panic]
    fn test_components_chart_uninitialized_2() {
        ChartPropsBuilder::default().push_record(2, (0.4, 0.7));
    }
}
