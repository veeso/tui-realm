//! ## BarChart
//!
//! A chart with bars

use std::collections::LinkedList;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, Style,
};
use tuirealm::ratatui::{layout::Rect, widgets::BarChart as TuiBarChart};
use tuirealm::{Frame, MockComponent, State};

// -- Props

use super::props::{
    BAR_CHART_BARS_GAP, BAR_CHART_BARS_STYLE, BAR_CHART_LABEL_STYLE, BAR_CHART_MAX_BARS,
    BAR_CHART_VALUES_STYLE,
};

// -- states

/// ### BarChartStates
///
/// Bar chart states
#[derive(Default)]
pub struct BarChartStates {
    pub cursor: usize,
}

impl BarChartStates {
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
#[derive(Default)]
pub struct BarChart {
    props: Props,
    pub states: BarChartStates,
}

impl BarChart {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
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
        self.attr(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    pub fn data(mut self, data: &[(&str, u64)]) -> Self {
        let mut list: LinkedList<PropPayload> = LinkedList::new();
        data.iter().for_each(|(a, b)| {
            list.push_back(PropPayload::Tup2((
                PropValue::Str(a.to_string()),
                PropValue::U64(*b),
            )))
        });
        self.attr(
            Attribute::Dataset,
            AttrValue::Payload(PropPayload::Linked(list)),
        );
        self
    }

    pub fn bar_gap(mut self, gap: u16) -> Self {
        self.attr(Attribute::Custom(BAR_CHART_BARS_GAP), AttrValue::Size(gap));
        self
    }

    pub fn bar_style(mut self, s: Style) -> Self {
        self.attr(Attribute::Custom(BAR_CHART_BARS_STYLE), AttrValue::Style(s));
        self
    }

    pub fn label_style(mut self, s: Style) -> Self {
        self.attr(
            Attribute::Custom(BAR_CHART_LABEL_STYLE),
            AttrValue::Style(s),
        );
        self
    }

    pub fn max_bars(mut self, l: usize) -> Self {
        self.attr(Attribute::Custom(BAR_CHART_MAX_BARS), AttrValue::Length(l));
        self
    }

    pub fn value_style(mut self, s: Style) -> Self {
        self.attr(
            Attribute::Custom(BAR_CHART_VALUES_STYLE),
            AttrValue::Style(s),
        );
        self
    }

    pub fn width(mut self, w: u16) -> Self {
        self.attr(Attribute::Width, AttrValue::Size(w));
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
            .map(|x| x.unwrap_payload().unwrap_linked().len())
            .unwrap_or(0)
    }

    fn get_data(&self, start: usize, len: usize) -> Vec<(String, u64)> {
        if let Some(PropPayload::Linked(list)) = self
            .props
            .get(Attribute::Dataset)
            .map(|x| x.unwrap_payload())
        {
            // Recalc len
            let len: usize = std::cmp::min(len, self.data_len() - start);
            // Prepare data storage
            let mut data: Vec<(String, u64)> = Vec::with_capacity(len);
            for (cursor, item) in list.iter().enumerate() {
                // If before start, continue
                if cursor < start {
                    continue;
                }
                // Push item
                if let PropPayload::Tup2((PropValue::Str(label), PropValue::U64(value))) = item {
                    data.push((label.clone(), *value));
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
            let mut div = crate::utils::get_block(borders, title, active, inactive_style);
            div = div.style(Style::default().bg(background).fg(foreground));
            // Get max elements
            let data_max_len: u64 = self
                .props
                .get(Attribute::Custom(BAR_CHART_MAX_BARS))
                .map(|x| x.unwrap_length() as u64)
                .unwrap_or(self.data_len() as u64);
            // Get data
            let data = self.get_data(self.states.cursor, data_max_len as usize);
            let data_ref: Vec<(&str, u64)> = data.iter().map(|x| (x.0.as_str(), x.1)).collect();
            // Create widget
            let mut widget: TuiBarChart =
                TuiBarChart::default().block(div).data(data_ref.as_slice());
            if let Some(gap) = self
                .props
                .get(Attribute::Custom(BAR_CHART_BARS_GAP))
                .map(|x| x.unwrap_size())
            {
                widget = widget.bar_gap(gap);
            }
            if let Some(width) = self.props.get(Attribute::Width).map(|x| x.unwrap_size()) {
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
                    self.states.move_cursor_right(self.data_len());
                }
                Cmd::GoTo(Position::Begin) => {
                    self.states.reset_cursor();
                }
                Cmd::GoTo(Position::End) => {
                    self.states.cursor_at_end(self.data_len());
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
        let mut states: BarChartStates = BarChartStates::default();
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
    fn test_components_bar_chart() {
        let mut component: BarChart = BarChart::default()
            .disabled(false)
            .title("my incomes", Alignment::Center)
            .label_style(Style::default().fg(Color::Yellow))
            .bar_style(Style::default().fg(Color::LightYellow))
            .bar_gap(2)
            .width(4)
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
    }
}
