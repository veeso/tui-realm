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
use std::collections::LinkedList;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, Style,
};
use tuirealm::tui::{layout::Rect, widgets::BarChart as TuiBarChart};
use tuirealm::{Frame, MockComponent, State};

// -- Props

use super::props::{
    BAR_CHART_BARS_GAP, BAR_CHART_BARS_STYLE, BAR_CHART_LABEL_STYLE, BAR_CHART_MAX_BARS,
    BAR_CHART_VALUES_STYLE,
};

// -- states

/// ### OwnStates
///
/// Bar chart states
struct OwnStates {
    cursor: usize,
}

impl Default for OwnStates {
    fn default() -> Self {
        Self { cursor: 0 }
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

impl Default for BarChart {
    fn default() -> Self {
        Self {
            props: Props::default(),
            states: OwnStates::default(),
        }
    }
}

impl BarChart {
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
            AttrValue::Title(t.as_ref().to_string(), a),
        );
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props
            .set(Attribute::Disabled, AttrValue::Flag(disabled));
        self
    }

    pub fn data(mut self, data: &[(&str, u64)]) -> Self {
        let mut list: LinkedList<PropPayload> = LinkedList::new();
        data.into_iter().for_each(|(a, b)| {
            list.push_back(PropPayload::Tup2(
                PropValue::Str(a.to_string()),
                PropValue::U64(*b),
            ))
        });
        self.props.set(
            Attribute::Dataset,
            AttrValue::Payload(PropPayload::Linked(list)),
        );
        self
    }

    // TODO: missing custom attributes

    pub fn width(mut self, w: u16) -> Self {
        self.props.set(Attribute::Width, AttrValue::Size(w));
        self
    }

    fn is_disabled(&self) -> bool {
        self.props
            .get_or(Attribute::Disabled, AttrValue::Flag(false))
            .unwrap_flag()
    }

    /// ### data_len
    ///
    /// Retrieve current data len from properties
    fn data_len(&self) -> usize {
        self.props
            .get(Attribute::Dataset)
            .map(|x| {
                x.unwrap_payload()
                    .unwrap_vec()
                    .iter()
                    .map(|x| x.unwrap_dataset().get_data().len())
                    .max()
            })
            .unwrap_or(0)
    }

    fn get_data(&self, start: usize, len: usize) -> Vec<(&str, u64)> {
        if let Some(PropPayload::Linked(list)) = self
            .props
            .get(Attribute::Dataset)
            .map(|x| x.unwrap_payload())
        {
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

impl MockComponent for BarChart {
    fn view(&self, render: &mut Frame, area: Rect) {
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
            let div = crate::utils::get_block(borders, title, focus, inactive_style);
            let active: bool = match self.is_disabled() {
                true => true,
                false => focus,
            };
            // Get max elements
            let data_max_len: u64 = self
                .props
                .get(Attribute::Custom(BAR_CHART_MAX_BARS))
                .map(|x| *x.unwrap_length() as u64)
                .unwrap_or(self.data_len() as u64);
            // Get data
            let data: Vec<(&str, u64)> = self.get_data(self.states.cursor, data_max_len as usize);
            // Create widget
            let mut widget: TuiBarChart = TuiBarChart::default()
                .block(div)
                .get_data(data.as_slice())
                .max(data_max_len);
            if let Some(PropPayload::One(PropValue::U16(gap))) = self
                .props
                .get(Attribute::Custom(BAR_CHART_BARS_GAP))
                .map(|x| x.unwrap_size())
            {
                widget = widget.bar_gap(gap);
            }
            if let Some(PropPayload::One(PropValue::U16(width))) =
                self.props.get(Attribute::Width).map(|x| x.unwrap_size())
            {
                widget = widget.bar_width(width);
            }
            if let Some(style) = self
                .props
                .get(Attribute::Custom(BAR_CHART_BARS_STYLE))
                .map(|x| x.unwrap_style())
            {
                widget = widget.bar_style(style);
            }
            if let Some(style) = self
                .props
                .get(Attribute::Custom(BAR_CHART_LABEL_STYLE))
                .map(|x| x.unwrap_style())
            {
                widget = widget.label_style(style);
            }
            if let Some(style) = self
                .props
                .get(Attribute::Custom(BAR_CHART_VALUES_STYLE))
                .map(|x| x.unwrap_style())
            {
                widget = widget.value_style(style);
            }
            // Render
            render.render_widget(widget, area);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value)
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
        let mut component: BarChart = BarChart::default()
            .disabled(false)
            .background(Color::White)
            .foreground(Color::Black)
            .title("my incomes", Alignment::Center)
            .label_style(Style::default().fg(Color::Yellow))
            .bar_style(Style::default().fg(Color::LightYellow))
            .bar_gap(2)
            .bar_width(4)
            .borders(Borders::default())
            .max_bars(6)
            .value_style(Style::default().fg(Color::LightBlue))
            .data(&[
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
            ]);
        // Commands
        assert_eq!(component.state(), State::None);
        // -> Right
        assert_eq!(component.on(Cmd::Move(Direction::Right)), CmdResult::None);
        assert_eq!(component.states.cursor, 1);
        // <- Left
        assert_eq!(component.on(Cmd::Move(Direction::Left)), CmdResult::None);
        assert_eq!(component.states.cursor, 0);
        // End
        assert_eq!(component.on(Cmd::GoTo(Position::End)), CmdResult::None);
        assert_eq!(component.states.cursor, 11);
        // Home
        assert_eq!(component.on(Cmd::GoTo(Position::Begin)), CmdResult::None);
        assert_eq!(component.states.cursor, 0);
    }
}
