//! `Table` represents a read-only textual table component which can be scrollable through arrows or inactive.

use std::cmp::max;

use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::Component;
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, LineStatic, PropPayload, PropValue, Props, QueryResult,
    Style, Table as PropTable, TextModifiers, Title,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::{Constraint, Rect};
use tuirealm::ratatui::text::Line;
use tuirealm::ratatui::widgets::{Cell, Row, Table as TuiTable, TableState};
use tuirealm::state::{State, StateValue};

use super::props::TABLE_COLUMN_SPACING;
use crate::prop_ext::CommonProps;
use crate::utils::{self, borrow_clone_line};

// -- States

/// The state that has to be kept for the [`Table`] component.
#[derive(Default)]
pub struct TableStates {
    /// Index of selected item in textarea
    pub list_index: usize,
    /// Lines in text area
    pub list_len: usize,
}

impl TableStates {
    /// Set list length.
    pub fn set_list_len(&mut self, len: usize) {
        self.list_len = len;
    }

    /// Incremenet list index.
    pub fn incr_list_index(&mut self, rewind: bool) {
        // Check if index is at last element
        if self.list_index + 1 < self.list_len {
            self.list_index += 1;
        } else if rewind {
            self.list_index = 0;
        }
    }

    /// Decrement list index.
    pub fn decr_list_index(&mut self, rewind: bool) {
        // Check if index is bigger than 0
        if self.list_index > 0 {
            self.list_index -= 1;
        } else if rewind && self.list_len > 0 {
            self.list_index = self.list_len - 1;
        }
    }

    /// Keep index if possible, otherwise set to `lenght - 1`.
    pub fn fix_list_index(&mut self) {
        if self.list_index >= self.list_len && self.list_len > 0 {
            self.list_index = self.list_len - 1;
        } else if self.list_len == 0 {
            self.list_index = 0;
        }
    }

    /// Set list index to the first item in the list.
    pub fn list_index_at_first(&mut self) {
        self.list_index = 0;
    }

    /// Set list index at the last item of the list.
    pub fn list_index_at_last(&mut self) {
        if self.list_len > 0 {
            self.list_index = self.list_len - 1;
        } else {
            self.list_index = 0;
        }
    }

    /// Calculate the max step ahead to scroll list.
    #[must_use]
    pub fn calc_max_step_ahead(&self, max: usize) -> usize {
        let remaining: usize = match self.list_len {
            0 => 0,
            len => len - 1 - self.list_index,
        };
        if remaining > max { max } else { remaining }
    }

    /// Calculate the max step ahead to scroll list.
    #[must_use]
    pub fn calc_max_step_behind(&self, max: usize) -> usize {
        if self.list_index > max {
            max
        } else {
            self.list_index
        }
    }
}

// -- Component

/// `Table` represents a read-only textual table component which can be scrollable through arrows or inactive.
#[derive(Default)]
#[must_use]
pub struct Table {
    common: CommonProps,
    props: Props,
    pub states: TableStates,
}

impl Table {
    /// Set the main foreground color. This may get overwritten by individual text styles.
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    /// Set the main background color. This may get overwritten by individual text styles.
    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    /// Set the main text modifiers. This may get overwritten by individual text styles.
    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    /// Set the main style. This may get overwritten by individual text styles.
    ///
    /// This option will overwrite any previous [`foreground`](Self::foreground), [`background`](Self::background) and [`modifiers`](Self::modifiers)!
    pub fn style(mut self, style: Style) -> Self {
        self.attr(Attribute::Style, AttrValue::Style(style));
        self
    }

    /// Set a custom style for the border when the component is unfocused.
    pub fn inactive(mut self, s: Style) -> Self {
        self.attr(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    /// Add a border to the component.
    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    /// Add a title to the component.
    pub fn title<T: Into<Title>>(mut self, title: T) -> Self {
        self.attr(Attribute::Title, AttrValue::Title(title.into()));
        self
    }

    /// Set the scroll stepping to use on `Cmd::Scroll(Direction::Up)` or `Cmd::Scroll(Direction::Down)`.
    pub fn step(mut self, step: usize) -> Self {
        self.attr(Attribute::ScrollStep, AttrValue::Length(step));
        self
    }

    /// Should the list be scrollable or always show only the top (0th) element?
    pub fn scroll(mut self, scrollable: bool) -> Self {
        self.attr(Attribute::Scroll, AttrValue::Flag(scrollable));
        self
    }

    /// Set the Symbol and Style for the indicator of the current line.
    pub fn highlighted_str<S: Into<LineStatic>>(mut self, s: S) -> Self {
        self.attr(Attribute::HighlightedStr, AttrValue::TextLine(s.into()));
        self
    }

    /// Set a custom foreground color for the currently highlighted item.
    pub fn highlighted_color(mut self, c: Color) -> Self {
        // TODO: shouldnt this be a highlight style instead?
        self.attr(Attribute::HighlightedColor, AttrValue::Color(c));
        self
    }

    /// Set custom spacing between columns.
    pub fn column_spacing(mut self, w: u16) -> Self {
        self.attr(Attribute::Custom(TABLE_COLUMN_SPACING), AttrValue::Size(w));
        self
    }

    /// Set a custom height for all rows.
    ///
    /// Default: `1`
    pub fn row_height(mut self, h: u16) -> Self {
        self.attr(Attribute::Height, AttrValue::Size(h));
        self
    }

    /// Set the widths of each column.
    pub fn widths(mut self, w: &[u16]) -> Self {
        // TODO: should this maybe be "Layout"?
        self.attr(
            Attribute::Width,
            AttrValue::Payload(PropPayload::Vec(
                w.iter().map(|x| PropValue::U16(*x)).collect(),
            )),
        );
        self
    }

    /// Set headers for columns.
    pub fn headers<S: Into<String>>(mut self, headers: impl IntoIterator<Item = S>) -> Self {
        self.attr(
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(
                headers
                    .into_iter()
                    .map(|v| PropValue::Str(v.into()))
                    .collect(),
            )),
        );
        self
    }

    /// Set the data for the table.
    pub fn table(mut self, t: PropTable) -> Self {
        self.attr(Attribute::Content, AttrValue::Table(t));
        self
    }

    /// Set whether wraparound should be possible (down on the last choice wraps around to 0, and the other way around).
    pub fn rewind(mut self, r: bool) -> Self {
        self.attr(Attribute::Rewind, AttrValue::Flag(r));
        self
    }

    /// Set the initially selected line.
    pub fn selected_line(mut self, line: usize) -> Self {
        self.attr(
            Attribute::Value,
            AttrValue::Payload(PropPayload::Single(PropValue::Usize(line))),
        );
        self
    }

    /// Set the current component to be always active (show highligh even if unfocused)
    pub fn always_active(mut self) -> Self {
        self.attr(Attribute::AlwaysActive, AttrValue::Flag(true));
        self
    }

    /// ### scrollable
    ///
    /// returns the value of the scrollable flag; by default is false
    fn is_scrollable(&self) -> bool {
        self.props
            .get(Attribute::Scroll)
            .and_then(AttrValue::as_flag)
            .unwrap_or_default()
    }

    fn rewindable(&self) -> bool {
        self.props
            .get(Attribute::Rewind)
            .and_then(AttrValue::as_flag)
            .unwrap_or_default()
    }

    /// ### layout
    ///
    /// Returns layout based on properties.
    /// If layout is not set in properties, they'll be divided by rows number
    fn layout(&self) -> Vec<Constraint> {
        if let Some(widths) = self
            .props
            .get(Attribute::Width)
            .and_then(AttrValue::as_payload)
            .and_then(PropPayload::as_vec)
        {
            widths
                .iter()
                .cloned()
                .map(|x| x.unwrap_u16())
                .map(Constraint::Percentage)
                .collect()
        } else {
            // Get amount of columns (maximum len of row elements)
            let columns: usize = self
                .props
                .get(Attribute::Content)
                .and_then(AttrValue::as_table)
                .and_then(|rows| rows.iter().map(|col| col.len()).max())
                .unwrap_or(1);
            // Calc width in equal way, make sure not to divide by zero (this can happen when rows is [[]])
            let width: u16 = (100 / max(columns, 1)) as u16;
            (0..columns)
                .map(|_| Constraint::Percentage(width))
                .collect()
        }
    }

    /// Generate [`Row`]s from a 2d vector of [`TextSpan`](tuirealm::props::TextSpan)s in props [`Attribute::Content`].
    fn make_rows(&self, row_height: u16) -> Vec<Row<'_>> {
        let Some(table) = self
            .props
            .get(Attribute::Content)
            .and_then(|x| x.as_table())
        else {
            return Vec::new();
        };

        table
            .iter()
            .map(|row| {
                let columns: Vec<Cell> = row
                    .iter()
                    .map(|col| {
                        let line = Line::from(
                            col.spans
                                .iter()
                                .map(utils::borrow_clone_span)
                                .collect::<Vec<_>>(),
                        );
                        Cell::from(line)
                    })
                    .collect();
                Row::new(columns).height(row_height)
            })
            .collect() // Make List item from TextSpan
    }
}

impl Component for Table {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if !self.common.display {
            return;
        }

        let row_height = self
            .props
            .get(Attribute::Height)
            .and_then(AttrValue::as_size)
            .unwrap_or(1);
        // Make rows
        let rows: Vec<Row> = self.make_rows(row_height);
        let widths: Vec<Constraint> = self.layout();

        let mut widget = TuiTable::new(rows, &widths).style(self.common.style);

        if let Some(block) = self.common.get_block() {
            widget = widget.block(block);
        }

        let highlighted_color = self
            .props
            .get(Attribute::HighlightedColor)
            .and_then(AttrValue::as_color);

        if let Some(highlighted_color) = highlighted_color {
            widget =
                widget.row_highlight_style(Style::default().fg(highlighted_color).add_modifier(
                    if self.common.is_active() {
                        TextModifiers::REVERSED
                    } else {
                        TextModifiers::empty()
                    },
                ));
        }
        // Highlighted symbol
        let hg_str = self
            .props
            .get(Attribute::HighlightedStr)
            .and_then(|x| x.as_textline());
        if let Some(hg_str) = hg_str {
            widget = widget.highlight_symbol(borrow_clone_line(hg_str));
        }
        // Col spacing
        if let Some(spacing) = self
            .props
            .get(Attribute::Custom(TABLE_COLUMN_SPACING))
            .and_then(AttrValue::as_size)
        {
            widget = widget.column_spacing(spacing);
        }
        // Header
        let headers: Vec<&str> = self
            .props
            .get(Attribute::Text)
            .and_then(|v| v.as_payload())
            .and_then(|v| v.as_vec())
            .map(|v| {
                v.iter()
                    .filter_map(|v| v.as_str().map(|v| v.as_str()))
                    .collect()
            })
            .unwrap_or_default();
        if !headers.is_empty() {
            widget = widget.header(
                Row::new(headers)
                    .style(self.common.style)
                    .height(row_height),
            );
        }
        if self.is_scrollable() {
            let mut state: TableState = TableState::default();
            state.select(Some(self.states.list_index));
            render.render_stateful_widget(widget, area, &mut state);
        } else {
            render.render_widget(widget, area);
        }
    }

    fn query<'a>(&'a self, attr: Attribute) -> Option<QueryResult<'a>> {
        if let Some(value) = self.common.get_for_query(attr) {
            return Some(value);
        }

        self.props.get_for_query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if let Some(value) = self.common.set(attr, value) {
            self.props.set(attr, value);
            if matches!(attr, Attribute::Content) {
                // Update list len and fix index
                self.states.set_list_len(
                    self.props
                        .get(Attribute::Content)
                        .and_then(AttrValue::as_table)
                        .map(|spans| spans.len())
                        .unwrap_or_default(),
                );
                self.states.fix_list_index();
            } else if matches!(attr, Attribute::Value) && self.is_scrollable() {
                self.states.list_index = self
                    .props
                    .get(Attribute::Value)
                    .and_then(AttrValue::as_payload)
                    .and_then(PropPayload::as_single)
                    .and_then(PropValue::as_usize)
                    .unwrap_or_default();
                self.states.fix_list_index();
            }
        }
    }

    fn state(&self) -> State {
        if self.is_scrollable() {
            State::Single(StateValue::Usize(self.states.list_index))
        } else {
            State::None
        }
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Down) => {
                let prev = self.states.list_index;
                self.states.incr_list_index(self.rewindable());
                if prev == self.states.list_index {
                    CmdResult::NoChange
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            Cmd::Move(Direction::Up) => {
                let prev = self.states.list_index;
                self.states.decr_list_index(self.rewindable());
                if prev == self.states.list_index {
                    CmdResult::NoChange
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            Cmd::Scroll(Direction::Down) => {
                let prev = self.states.list_index;
                let step = self
                    .props
                    .get(Attribute::ScrollStep)
                    .and_then(AttrValue::as_length)
                    .unwrap_or(8);
                let step: usize = self.states.calc_max_step_ahead(step);
                (0..step).for_each(|_| self.states.incr_list_index(false));
                if prev == self.states.list_index {
                    CmdResult::NoChange
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            Cmd::Scroll(Direction::Up) => {
                let prev = self.states.list_index;
                let step = self
                    .props
                    .get(Attribute::ScrollStep)
                    .and_then(AttrValue::as_length)
                    .unwrap_or(8);
                let step: usize = self.states.calc_max_step_behind(step);
                (0..step).for_each(|_| self.states.decr_list_index(false));
                if prev == self.states.list_index {
                    CmdResult::NoChange
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            Cmd::GoTo(Position::Begin) => {
                let prev = self.states.list_index;
                self.states.list_index_at_first();
                if prev == self.states.list_index {
                    CmdResult::NoChange
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            Cmd::GoTo(Position::End) => {
                let prev = self.states.list_index;
                self.states.list_index_at_last();
                if prev == self.states.list_index {
                    CmdResult::NoChange
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            _ => CmdResult::Invalid(cmd),
        }
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;
    use tuirealm::props::{HorizontalAlignment, TableBuilder};

    use super::*;

    #[test]
    fn table_states() {
        let mut states = TableStates::default();
        assert_eq!(states.list_index, 0);
        assert_eq!(states.list_len, 0);
        states.set_list_len(5);
        assert_eq!(states.list_index, 0);
        assert_eq!(states.list_len, 5);
        // Incr
        states.incr_list_index(true);
        assert_eq!(states.list_index, 1);
        states.list_index = 4;
        states.incr_list_index(false);
        assert_eq!(states.list_index, 4);
        states.incr_list_index(true);
        assert_eq!(states.list_index, 0);
        // Decr
        states.decr_list_index(false);
        assert_eq!(states.list_index, 0);
        states.decr_list_index(true);
        assert_eq!(states.list_index, 4);
        states.decr_list_index(true);
        assert_eq!(states.list_index, 3);
        // Begin
        states.list_index_at_first();
        assert_eq!(states.list_index, 0);
        states.list_index_at_last();
        assert_eq!(states.list_index, 4);
        // Fix
        states.set_list_len(3);
        states.fix_list_index();
        assert_eq!(states.list_index, 2);
    }

    #[test]
    fn test_component_table_scrolling() {
        // Make component
        let mut component = Table::default()
            .foreground(Color::Red)
            .background(Color::Blue)
            .highlighted_color(Color::Yellow)
            .highlighted_str("🚀")
            .modifiers(TextModifiers::BOLD)
            .scroll(true)
            .step(4)
            .borders(Borders::default())
            .title(Title::from("events").alignment(HorizontalAlignment::Center))
            .column_spacing(4)
            .widths(&[25, 25, 25, 25])
            .row_height(3)
            .headers(["Event", "Message", "Behaviour", "???"])
            .table(
                TableBuilder::default()
                    .add_col(Line::from("KeyCode::Down"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor down"))
                    .add_row()
                    .add_col(Line::from("KeyCode::Up"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor up"))
                    .add_row()
                    .add_col(Line::from("KeyCode::PageDown"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor down by 8"))
                    .add_row()
                    .add_col(Line::from("KeyCode::PageUp"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("ove cursor up by 8"))
                    .add_row()
                    .add_col(Line::from("KeyCode::End"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor to last item"))
                    .add_row()
                    .add_col(Line::from("KeyCode::Home"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor to first item"))
                    .add_row()
                    .add_col(Line::from("KeyCode::Char(_)"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Return pressed key"))
                    .add_col(Line::from("4th mysterious columns"))
                    .build(),
            );
        assert_eq!(component.states.list_len, 7);
        assert_eq!(component.states.list_index, 0);
        // Own funcs
        assert_eq!(component.layout().len(), 4);
        // Increment list index
        component.states.list_index += 1;
        assert_eq!(component.states.list_index, 1);
        // Check messages
        // Handle inputs
        assert_eq!(
            component.perform(Cmd::Move(Direction::Down)),
            CmdResult::Changed(State::Single(StateValue::Usize(2)))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 2);
        // Index should be decremented
        assert_eq!(
            component.perform(Cmd::Move(Direction::Up)),
            CmdResult::Changed(State::Single(StateValue::Usize(1)))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 1);
        // Index should be 2
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Down)),
            CmdResult::Changed(State::Single(StateValue::Usize(5)))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 5);
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Down)),
            CmdResult::Changed(State::Single(StateValue::Usize(6)))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 6);
        // Index should be 0
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Up)),
            CmdResult::Changed(State::Single(StateValue::Usize(2)))
        );
        assert_eq!(component.states.list_index, 2);
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Up)),
            CmdResult::Changed(State::Single(StateValue::Usize(0)))
        );
        assert_eq!(component.states.list_index, 0);
        // End
        assert_eq!(
            component.perform(Cmd::GoTo(Position::End)),
            CmdResult::Changed(State::Single(StateValue::Usize(6)))
        );
        assert_eq!(component.states.list_index, 6);
        // Home
        assert_eq!(
            component.perform(Cmd::GoTo(Position::Begin)),
            CmdResult::Changed(State::Single(StateValue::Usize(0)))
        );
        assert_eq!(component.states.list_index, 0);
        // Update
        component.attr(
            Attribute::Content,
            AttrValue::Table(
                TableBuilder::default()
                    .add_col(Line::from("name"))
                    .add_col(Line::from("age"))
                    .add_col(Line::from("birthdate"))
                    .build(),
            ),
        );
        assert_eq!(component.states.list_len, 1);
        assert_eq!(component.states.list_index, 0);
        // Get value
        assert_eq!(component.state(), State::Single(StateValue::Usize(0)));
    }

    #[test]
    fn test_component_table_with_empty_rows_and_no_width_set() {
        // Make component
        let component = Table::default().table(TableBuilder::default().build());

        assert_eq!(component.states.list_len, 1);
        assert_eq!(component.states.list_index, 0);
        // calculating layout would fail if no widths and using "empty" TableBuilder
        assert_eq!(component.layout().len(), 0);
    }

    #[test]
    fn test_components_table() {
        // Make component
        let component = Table::default()
            .foreground(Color::Red)
            .background(Color::Blue)
            .highlighted_color(Color::Yellow)
            .highlighted_str("🚀")
            .modifiers(TextModifiers::BOLD)
            .borders(Borders::default())
            .title(Title::from("events").alignment(HorizontalAlignment::Center))
            .column_spacing(4)
            .widths(&[33, 33, 33])
            .row_height(3)
            .headers(["Event", "Message", "Behaviour"])
            .table(
                TableBuilder::default()
                    .add_col(Line::from("KeyCode::Down"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor down"))
                    .add_row()
                    .add_col(Line::from("KeyCode::Up"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor up"))
                    .add_row()
                    .add_col(Line::from("KeyCode::PageDown"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor down by 8"))
                    .add_row()
                    .add_col(Line::from("KeyCode::PageUp"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("ove cursor up by 8"))
                    .add_row()
                    .add_col(Line::from("KeyCode::End"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor to last item"))
                    .add_row()
                    .add_col(Line::from("KeyCode::Home"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor to first item"))
                    .add_row()
                    .add_col(Line::from("KeyCode::Char(_)"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Return pressed key"))
                    .build(),
            );
        // Get value (not scrollable)
        assert_eq!(component.state(), State::None);
    }

    #[test]
    fn should_init_list_value() {
        let mut component = Table::default()
            .foreground(Color::Red)
            .background(Color::Blue)
            .highlighted_color(Color::Yellow)
            .highlighted_str("🚀")
            .modifiers(TextModifiers::BOLD)
            .borders(Borders::default())
            .title(Title::from("events").alignment(HorizontalAlignment::Center))
            .table(
                TableBuilder::default()
                    .add_col(Line::from("KeyCode::Down"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor down"))
                    .add_row()
                    .add_col(Line::from("KeyCode::Up"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor up"))
                    .add_row()
                    .add_col(Line::from("KeyCode::PageDown"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor down by 8"))
                    .add_row()
                    .add_col(Line::from("KeyCode::PageUp"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("ove cursor up by 8"))
                    .add_row()
                    .add_col(Line::from("KeyCode::End"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor to last item"))
                    .add_row()
                    .add_col(Line::from("KeyCode::Home"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Move cursor to first item"))
                    .add_row()
                    .add_col(Line::from("KeyCode::Char(_)"))
                    .add_col(Line::from("OnKey"))
                    .add_col(Line::from("Return pressed key"))
                    .build(),
            )
            .scroll(true)
            .selected_line(2);
        assert_eq!(component.states.list_index, 2);
        // Index out of bounds
        component.attr(
            Attribute::Value,
            AttrValue::Payload(PropPayload::Single(PropValue::Usize(50))),
        );
        assert_eq!(component.states.list_index, 6);
    }

    #[test]
    fn various_header_types() {
        // static array of static strings
        let _ = Table::default().headers(["hello"]);
        // static array of strings
        let _ = Table::default().headers(["hello".to_string()]);
        // vec of static strings
        let _ = Table::default().headers(vec!["hello"]);
        // vec of strings
        let _ = Table::default().headers(vec!["hello".to_string()]);
        // boxed array of static strings
        let _ = Table::default().headers(vec!["hello"].into_boxed_slice());
        // boxed array of strings
        let _ = Table::default().headers(vec!["hello".to_string()].into_boxed_slice());
    }
}
