//! ## BarChart
//!
//! A chart with bars

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
use crate::props::{BordersProps, PropPayload, PropValue, Props, PropsBuilder, TextParts};
use crate::tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{BarChart as TuiBarChart, Block, BorderType, Borders},
};
use crate::{Canvas, Component, Event, Msg, Payload};
use std::collections::LinkedList;

// -- Props
const PROP_BAR_WIDTH: &str = "bar-width";
const PROP_BAR_GAP: &str = "bar-gap";
const PROP_BAR_STYLE: &str = "bar-style";
const PROP_DATA: &str = "data";
const PROP_DISABLED: &str = "disabled";
const PROP_LABEL_STYLE: &str = "label-style";
const PROP_MAX_BARS: &str = "max-bars";
const PROP_VALUE_STYLE: &str = "value-style";

pub struct BarChartPropsBuilder {
    props: Option<Props>,
}

impl Default for BarChartPropsBuilder {
    fn default() -> Self {
        Self {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for BarChartPropsBuilder {
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

impl From<Props> for BarChartPropsBuilder {
    fn from(props: Props) -> Self {
        Self { props: Some(props) }
    }
}

impl BarChartPropsBuilder {
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

    /// ### with_label
    ///
    /// Set input label
    pub fn with_label(&mut self, label: String) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.texts = TextParts::new(Some(label), None);
        }
        self
    }

    /// ### with_bar_gap
    ///
    /// Define bar gap for chart
    pub fn with_bar_gap(&mut self, gap: u16) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_BAR_GAP, PropPayload::One(PropValue::U16(gap)));
        }
        self
    }

    /// ### with_bar_style
    ///
    /// Define bar style for chart
    pub fn with_bar_style(&mut self, style: Style) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_BAR_STYLE, PropPayload::One(PropValue::Style(style)));
        }
        self
    }

    /// ### with_bar_width
    ///
    /// Define bar width for chart
    pub fn with_bar_width(&mut self, width: u16) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_BAR_WIDTH, PropPayload::One(PropValue::U16(width)));
        }
        self
    }

    /// ### with_label_style
    ///
    /// Define bar style for chart
    pub fn with_label_style(&mut self, style: Style) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_LABEL_STYLE, PropPayload::One(PropValue::Style(style)));
        }
        self
    }

    /// ### with_max_bars
    ///
    /// Define maximum amount of bars to be displayed
    pub fn with_max_bars(&mut self, max: u64) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_MAX_BARS, PropPayload::One(PropValue::U64(max)));
        }
        self
    }

    /// ### with_value_style
    ///
    /// Define style for values
    pub fn with_value_style(&mut self, style: Style) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_VALUE_STYLE, PropPayload::One(PropValue::Style(style)));
        }
        self
    }

    /// ### with_data
    ///
    /// Define chart data
    pub fn with_data(&mut self, data: &[(&str, u64)]) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            // Create linked list
            let mut list: LinkedList<PropPayload> = LinkedList::new();
            data.iter().for_each(|(label, value)| {
                list.push_back(PropPayload::Tup2((
                    PropValue::Str(label.to_string()),
                    PropValue::U64(*value),
                )))
            });
            props.own.insert(PROP_DATA, PropPayload::Linked(list));
        }
        self
    }

    /// ### push_record_back
    ///
    /// Just pushes a record to the back of the data
    pub fn push_record_back(&mut self, (label, value): (&str, u64)) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            match props.own.get_mut(PROP_DATA) {
                Some(PropPayload::Linked(list)) => {
                    list.push_back(PropPayload::Tup2((
                        PropValue::Str(label.to_string()),
                        PropValue::U64(value),
                    )));
                }
                _ => {
                    // Create list
                    let mut l: LinkedList<PropPayload> = LinkedList::new();
                    l.push_back(PropPayload::Tup2((
                        PropValue::Str(label.to_string()),
                        PropValue::U64(value),
                    )));
                    props.own.insert(PROP_DATA, PropPayload::Linked(l));
                }
            }
        }
        self
    }

    /// ### push_record_front
    ///
    /// Just pushes a record to the front of the data
    pub fn push_record_front(&mut self, (label, value): (&str, u64)) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            match props.own.get_mut(PROP_DATA) {
                Some(PropPayload::Linked(list)) => {
                    list.push_front(PropPayload::Tup2((
                        PropValue::Str(label.to_string()),
                        PropValue::U64(value),
                    )));
                }
                _ => {
                    // Create list
                    let mut l: LinkedList<PropPayload> = LinkedList::new();
                    l.push_front(PropPayload::Tup2((
                        PropValue::Str(label.to_string()),
                        PropValue::U64(value),
                    )));
                    props.own.insert(PROP_DATA, PropPayload::Linked(l));
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

/// ### OwnStates
///
/// Bar chart states
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

/// ### BarChart
///
/// A component to display a chart with bars.
/// The bar chart can work both in "active" and "disabled" mode.
///
/// #### Disabled mode
///
/// When in disabled mode, the chart won't be interactive, so you won't be able to move through data using keys.
/// If you have more data than the maximum amount of bars that can be displayed, you'll have to update data to display the remaining entries
///
/// #### Active mode
///
/// While in active mode (default) you can put as many entries as you wish. You can move with arrows and END/HOME keys
pub struct BarChart {
    props: Props,
    states: OwnStates,
}

impl BarChart {
    /// ### new
    ///
    /// Instantiates a new `BarChart` component
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

    /// ### data
    ///
    /// Get data to be displayed, starting from provided index at `start` with a max length of `len`
    fn data(&self, start: usize, len: usize) -> Vec<(&str, u64)> {
        if let Some(PropPayload::Linked(list)) = self.props.own.get(PROP_DATA) {
            // Recalc len
            let len: usize = std::cmp::min(len, self.data_len() - start);
            // Prepare data storage
            let mut data: Vec<(&str, u64)> = Vec::with_capacity(len);
            for (cursor, item) in list.iter().enumerate() {
                // If before start, continue
                if cursor < start {
                    continue;
                }
                // Push item
                if let PropPayload::Tup2((PropValue::Str(label), PropValue::U64(value))) = item {
                    data.push((label, *value));
                }
                // Break
                if data.len() >= len {
                    break;
                }
            }

            data
        } else {
            Vec::new()
        }
    }
}

impl Component for BarChart {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    #[cfg(not(tarpaulin_include))]
    fn render(&self, render: &mut Canvas, area: Rect) {
        if self.props.visible {
            // If component is disabled, will be displayed as `active`; as focus state otherwise
            let active: bool = match self.disabled() {
                true => true,
                false => self.states.focus,
            };
            let block: Block =
                super::utils::get_block(&self.props.borders, &self.props.texts.title, active);
            // Get max elements
            let data_max_len: u64 = self
                .props
                .own
                .get(PROP_MAX_BARS)
                .map(|x| match x {
                    PropPayload::One(PropValue::U64(l)) => *l,
                    _ => panic!("Max-bars is not u64"),
                })
                .unwrap_or(self.data_len() as u64);
            // Get data
            let data: Vec<(&str, u64)> = self.data(self.states.cursor, data_max_len as usize);
            // Create widget
            let mut widget: TuiBarChart = TuiBarChart::default()
                .block(block)
                .data(data.as_slice())
                .max(data_max_len);
            if let Some(PropPayload::One(PropValue::U16(gap))) = self.props.own.get(PROP_BAR_GAP) {
                widget = widget.bar_gap(*gap);
            }
            if let Some(PropPayload::One(PropValue::U16(width))) =
                self.props.own.get(PROP_BAR_WIDTH)
            {
                widget = widget.bar_width(*width);
            }
            if let Some(PropPayload::One(PropValue::Style(style))) =
                self.props.own.get(PROP_BAR_STYLE)
            {
                widget = widget.bar_style(*style);
            }
            if let Some(PropPayload::One(PropValue::Style(style))) =
                self.props.own.get(PROP_LABEL_STYLE)
            {
                widget = widget.label_style(*style);
            }
            if let Some(PropPayload::One(PropValue::Style(style))) =
                self.props.own.get(PROP_VALUE_STYLE)
            {
                widget = widget.value_style(*style);
            }
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
                        self.states.move_cursor_right(self.data_len());
                        Msg::None
                    }
                    KeyCode::End => {
                        // Cursor at last position
                        self.states.cursor_at_end(self.data_len());
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

    use crossterm::event::KeyEvent;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_bar_chart_states() {
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
    fn test_components_bar_chart() {
        let mut component: BarChart = BarChart::new(
            BarChartPropsBuilder::default()
                .hidden()
                .visible()
                .disabled(false)
                .with_background(Color::White)
                .with_foreground(Color::Black)
                .with_label(String::from("my incomes"))
                .with_label_style(Style::default().fg(Color::Yellow))
                .with_bar_style(Style::default().fg(Color::LightYellow))
                .with_bar_gap(2)
                .with_bar_width(4)
                .with_borders(Borders::ALL, BorderType::Double, Color::Yellow)
                .with_max_bars(6)
                .with_value_style(Style::default().fg(Color::LightBlue))
                .with_data(&[
                    ("january", 250),
                    ("february", 300),
                    ("march", 275),
                    ("april", 312),
                    ("may", 420),
                    ("june", 170),
                    ("july", 220),
                    ("august", 160),
                    ("september", 180),
                    ("october", 470),
                    ("november", 380),
                    ("december", 820),
                ])
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Black);
        assert_eq!(component.props.background, Color::White);
        assert_eq!(component.props.visible, true);
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Double);
        assert_eq!(component.props.borders.color, Color::Yellow);
        assert_eq!(
            *component.props.own.get(PROP_BAR_GAP).unwrap(),
            PropPayload::One(PropValue::U16(2))
        );
        assert_eq!(
            *component.props.own.get(PROP_BAR_WIDTH).unwrap(),
            PropPayload::One(PropValue::U16(4))
        );
        assert_eq!(
            *component.props.own.get(PROP_MAX_BARS).unwrap(),
            PropPayload::One(PropValue::U64(6))
        );
        assert!(component.props.own.get(PROP_DATA).is_some());
        assert!(component.props.own.get(PROP_BAR_STYLE).is_some());
        assert!(component.props.own.get(PROP_LABEL_STYLE).is_some());
        assert!(component.props.own.get(PROP_VALUE_STYLE).is_some());
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
        assert_eq!(component.data_len(), 12);
        assert_eq!(component.disabled(), false);
        assert_eq!(
            component.data(2, 4),
            vec![("march", 275), ("april", 312), ("may", 420), ("june", 170),]
        );
        // Update and test empty data
        component.states.cursor_at_end(12);
        assert_eq!(
            component.update(
                BarChartPropsBuilder::from(component.get_props())
                    .with_data(&[])
                    .build()
            ),
            Msg::None
        );
        assert_eq!(component.data(0, 4), vec![]);
        // Cursor is reset
        assert_eq!(component.states.cursor, 0);
    }

    #[test]
    fn test_components_bar_chart_disabled() {
        let mut component: BarChart = BarChart::new(
            BarChartPropsBuilder::default()
                .hidden()
                .visible()
                .disabled(true)
                .with_background(Color::White)
                .with_foreground(Color::Black)
                .with_label(String::from("my incomes"))
                .with_label_style(Style::default().fg(Color::Yellow))
                .with_bar_style(Style::default().fg(Color::LightYellow))
                .with_bar_gap(2)
                .with_bar_width(4)
                .with_borders(Borders::ALL, BorderType::Double, Color::Yellow)
                .with_max_bars(12)
                .with_value_style(Style::default().fg(Color::LightBlue))
                .with_data(&[
                    ("january", 250),
                    ("february", 300),
                    ("march", 275),
                    ("april", 312),
                    ("may", 420),
                    ("june", 170),
                    ("july", 220),
                    ("august", 160),
                    ("september", 180),
                    ("october", 470),
                    ("november", 380),
                    ("december", 820),
                ])
                .build(),
        );
        // focus
        component.active();
        assert_eq!(component.states.focus, false); // NOTE: never enabled
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
        assert_eq!(component.data_len(), 12);
        assert_eq!(component.disabled(), true);
        assert_eq!(
            component.data(2, 4),
            vec![("march", 275), ("april", 312), ("may", 420), ("june", 170),]
        );
        // Add a new record
        assert_eq!(
            component.update(
                BarChartPropsBuilder::from(component.get_props())
                    .push_record_back(("january", 983))
                    .push_record_front(("december", 187))
                    .build()
            ),
            Msg::None
        );
        assert_eq!(component.data_len(), 14);
        // Update and test empty data
        component.states.cursor_at_end(14);
        assert_eq!(
            component.update(
                BarChartPropsBuilder::from(component.get_props())
                    .with_data(&[])
                    .build()
            ),
            Msg::None
        );
        assert_eq!(component.data(0, 4), vec![]);
        // Cursor is reset
        assert_eq!(component.states.cursor, 0);
    }
}
