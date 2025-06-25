//! ## List
//!
//! `List` represents a read-only textual list component which can be scrollable through arrows or inactive

use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, Style, Table,
    TextModifiers,
};
use tuirealm::ratatui::text::Line as Spans;
use tuirealm::ratatui::{
    layout::Rect,
    text::Span,
    widgets::{List as TuiList, ListItem, ListState},
};
use tuirealm::{Frame, MockComponent, State, StateValue};

// -- States

#[derive(Default)]
pub struct ListStates {
    pub list_index: usize, // Index of selected item in list
    pub list_len: usize,   // Lines in text area
}

impl ListStates {
    /// ### set_list_len
    ///
    /// Set list length
    pub fn set_list_len(&mut self, len: usize) {
        self.list_len = len;
    }

    /// ### incr_list_index
    ///
    /// Incremenet list index
    pub fn incr_list_index(&mut self, rewind: bool) {
        // Check if index is at last element
        if self.list_index + 1 < self.list_len {
            self.list_index += 1;
        } else if rewind {
            self.list_index = 0;
        }
    }

    /// ### decr_list_index
    ///
    /// Decrement list index
    pub fn decr_list_index(&mut self, rewind: bool) {
        // Check if index is bigger than 0
        if self.list_index > 0 {
            self.list_index -= 1;
        } else if rewind && self.list_len > 0 {
            self.list_index = self.list_len - 1;
        }
    }

    /// ### fix_list_index
    ///
    /// Keep index if possible, otherwise set to lenght - 1
    pub fn fix_list_index(&mut self) {
        if self.list_index >= self.list_len && self.list_len > 0 {
            self.list_index = self.list_len - 1;
        } else if self.list_len == 0 {
            self.list_index = 0;
        }
    }

    /// ### list_index_at_first
    ///
    /// Set list index to the first item in the list
    pub fn list_index_at_first(&mut self) {
        self.list_index = 0;
    }

    /// ### list_index_at_last
    ///
    /// Set list index at the last item of the list
    pub fn list_index_at_last(&mut self) {
        if self.list_len > 0 {
            self.list_index = self.list_len - 1;
        } else {
            self.list_index = 0;
        }
    }

    /// ### calc_max_step_ahead
    ///
    /// Calculate the max step ahead to scroll list
    #[must_use]
    pub fn calc_max_step_ahead(&self, max: usize) -> usize {
        let remaining: usize = match self.list_len {
            0 => 0,
            len => len - 1 - self.list_index,
        };
        if remaining > max { max } else { remaining }
    }

    /// ### calc_max_step_ahead
    ///
    /// Calculate the max step ahead to scroll list
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

/// ## List
///
/// represents a read-only text component without any container.
#[derive(Default)]
#[must_use]
pub struct List {
    props: Props,
    pub states: ListStates,
}

impl List {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn title<S: Into<String>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(Attribute::Title, AttrValue::Title((t.into(), a)));
        self
    }

    pub fn inactive(mut self, s: Style) -> Self {
        self.attr(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    pub fn rewind(mut self, r: bool) -> Self {
        self.attr(Attribute::Rewind, AttrValue::Flag(r));
        self
    }

    pub fn step(mut self, step: usize) -> Self {
        self.attr(Attribute::ScrollStep, AttrValue::Length(step));
        self
    }

    pub fn scroll(mut self, scrollable: bool) -> Self {
        self.attr(Attribute::Scroll, AttrValue::Flag(scrollable));
        self
    }

    pub fn highlighted_str<S: Into<String>>(mut self, s: S) -> Self {
        self.attr(Attribute::HighlightedStr, AttrValue::String(s.into()));
        self
    }

    pub fn highlighted_color(mut self, c: Color) -> Self {
        self.attr(Attribute::HighlightedColor, AttrValue::Color(c));
        self
    }

    pub fn rows(mut self, rows: Table) -> Self {
        self.attr(Attribute::Content, AttrValue::Table(rows));
        self
    }

    /// Set initial selected line
    /// This method must be called after `rows` and `scrollable` in order to work
    pub fn selected_line(mut self, line: usize) -> Self {
        self.attr(
            Attribute::Value,
            AttrValue::Payload(PropPayload::One(PropValue::Usize(line))),
        );
        self
    }

    fn scrollable(&self) -> bool {
        self.props
            .get_or(Attribute::Scroll, AttrValue::Flag(false))
            .unwrap_flag()
    }

    fn rewindable(&self) -> bool {
        self.props
            .get_or(Attribute::Rewind, AttrValue::Flag(false))
            .unwrap_flag()
    }
}

impl MockComponent for List {
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
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();
            let title = crate::utils::get_title_or_center(&self.props);
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            let inactive_style = self
                .props
                .get(Attribute::FocusStyle)
                .map(|x| x.unwrap_style());
            let active: bool = if self.scrollable() { focus } else { true };
            let div = crate::utils::get_block(borders, Some(&title), active, inactive_style);
            // Make list entries
            let list_items: Vec<ListItem> = match self
                .props
                .get_ref(Attribute::Content)
                .and_then(|x| x.as_table())
            {
                Some(table) => table
                    .iter()
                    .map(|row| {
                        let columns: Vec<Span> = row
                            .iter()
                            .map(|col| {
                                let (fg, bg, modifiers) =
                                    crate::utils::use_or_default_styles(&self.props, col);
                                Span::styled(
                                    &col.content,
                                    Style::default().add_modifier(modifiers).fg(fg).bg(bg),
                                )
                            })
                            .collect();
                        ListItem::new(Spans::from(columns))
                    })
                    .collect(), // Make List item from TextSpan
                _ => Vec::new(),
            };
            let highlighted_color = self
                .props
                .get(Attribute::HighlightedColor)
                .map(|x| x.unwrap_color());
            let modifiers = if focus {
                modifiers | TextModifiers::REVERSED
            } else {
                modifiers
            };
            // Make list

            let mut list = TuiList::new(list_items)
                .block(div)
                .style(Style::default().fg(foreground).bg(background))
                .direction(tuirealm::ratatui::widgets::ListDirection::TopToBottom);
            if let Some(highlighted_color) = highlighted_color {
                list = list.highlight_style(
                    Style::default()
                        .fg(highlighted_color)
                        .add_modifier(modifiers),
                );
            }
            // Highlighted symbol
            let hg_str = self
                .props
                .get_ref(Attribute::HighlightedStr)
                .and_then(|x| x.as_string());
            if let Some(hg_str) = hg_str {
                list = list.highlight_symbol(hg_str);
            }
            if self.scrollable() {
                let mut state: ListState = ListState::default();
                state.select(Some(self.states.list_index));
                render.render_stateful_widget(list, area, &mut state);
            } else {
                render.render_widget(list, area);
            }
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
        if matches!(attr, Attribute::Content) {
            // Update list len and fix index
            self.states.set_list_len(
                match self.props.get(Attribute::Content).map(|x| x.unwrap_table()) {
                    Some(spans) => spans.len(),
                    _ => 0,
                },
            );
            self.states.fix_list_index();
        } else if matches!(attr, Attribute::Value) && self.scrollable() {
            self.states.list_index = self
                .props
                .get(Attribute::Value)
                .map_or(0, |x| x.unwrap_payload().unwrap_one().unwrap_usize());
            self.states.fix_list_index();
        }
    }

    fn state(&self) -> State {
        if self.scrollable() {
            State::One(StateValue::Usize(self.states.list_index))
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
                    CmdResult::None
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            Cmd::Move(Direction::Up) => {
                let prev = self.states.list_index;
                self.states.decr_list_index(self.rewindable());
                if prev == self.states.list_index {
                    CmdResult::None
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            Cmd::Scroll(Direction::Down) => {
                let prev = self.states.list_index;
                let step = self
                    .props
                    .get_or(Attribute::ScrollStep, AttrValue::Length(8))
                    .unwrap_length();
                let step: usize = self.states.calc_max_step_ahead(step);
                (0..step).for_each(|_| self.states.incr_list_index(false));
                if prev == self.states.list_index {
                    CmdResult::None
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            Cmd::Scroll(Direction::Up) => {
                let prev = self.states.list_index;
                let step = self
                    .props
                    .get_or(Attribute::ScrollStep, AttrValue::Length(8))
                    .unwrap_length();
                let step: usize = self.states.calc_max_step_behind(step);
                (0..step).for_each(|_| self.states.decr_list_index(false));
                if prev == self.states.list_index {
                    CmdResult::None
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            Cmd::GoTo(Position::Begin) => {
                let prev = self.states.list_index;
                self.states.list_index_at_first();
                if prev == self.states.list_index {
                    CmdResult::None
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            Cmd::GoTo(Position::End) => {
                let prev = self.states.list_index;
                self.states.list_index_at_last();
                if prev == self.states.list_index {
                    CmdResult::None
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            _ => CmdResult::None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use tuirealm::props::{TableBuilder, TextSpan};

    #[test]
    fn list_states() {
        let mut states = ListStates::default();
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
    fn test_components_list_scrollable() {
        let mut component = List::default()
            .foreground(Color::Red)
            .background(Color::Blue)
            .highlighted_color(Color::Yellow)
            .highlighted_str("🚀")
            .modifiers(TextModifiers::BOLD)
            .scroll(true)
            .step(4)
            .borders(Borders::default())
            .title("events", Alignment::Center)
            .rewind(true)
            .rows(
                TableBuilder::default()
                    .add_col(TextSpan::from("KeyCode::Down"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor down"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::Up"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor up"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::PageDown"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor down by 8"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::PageUp"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("ove cursor up by 8"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::End"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor to last item"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::Home"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor to first item"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::Char(_)"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Return pressed key"))
                    .add_col(TextSpan::from("4th mysterious columns"))
                    .build(),
            );
        assert_eq!(component.states.list_len, 7);
        assert_eq!(component.states.list_index, 0);
        // Increment list index
        component.states.list_index += 1;
        assert_eq!(component.states.list_index, 1);
        // Check messages
        // Handle inputs
        assert_eq!(
            component.perform(Cmd::Move(Direction::Down)),
            CmdResult::Changed(State::One(StateValue::Usize(2)))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 2);
        // Index should be decremented
        assert_eq!(
            component.perform(Cmd::Move(Direction::Up)),
            CmdResult::Changed(State::One(StateValue::Usize(1)))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 1);
        // Index should be 2
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Down)),
            CmdResult::Changed(State::One(StateValue::Usize(5)))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 5);
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Down)),
            CmdResult::Changed(State::One(StateValue::Usize(6)))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 6);
        // Index should be 0
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Up)),
            CmdResult::Changed(State::One(StateValue::Usize(2)))
        );
        assert_eq!(component.states.list_index, 2);
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Up)),
            CmdResult::Changed(State::One(StateValue::Usize(0)))
        );
        assert_eq!(component.states.list_index, 0);
        // End
        assert_eq!(
            component.perform(Cmd::GoTo(Position::End)),
            CmdResult::Changed(State::One(StateValue::Usize(6)))
        );
        assert_eq!(component.states.list_index, 6);
        // Home
        assert_eq!(
            component.perform(Cmd::GoTo(Position::Begin)),
            CmdResult::Changed(State::One(StateValue::Usize(0)))
        );
        assert_eq!(component.states.list_index, 0);
        // Update
        component.attr(
            Attribute::Content,
            AttrValue::Table(
                TableBuilder::default()
                    .add_col(TextSpan::from("name"))
                    .add_col(TextSpan::from("age"))
                    .add_col(TextSpan::from("birthdate"))
                    .build(),
            ),
        );
        assert_eq!(component.states.list_len, 1);
        assert_eq!(component.states.list_index, 0);
        // Get value
        assert_eq!(component.state(), State::One(StateValue::Usize(0)));
    }

    #[test]
    fn test_components_list() {
        let component = List::default()
            .foreground(Color::Red)
            .background(Color::Blue)
            .highlighted_color(Color::Yellow)
            .highlighted_str("🚀")
            .modifiers(TextModifiers::BOLD)
            .borders(Borders::default())
            .title("events", Alignment::Center)
            .rows(
                TableBuilder::default()
                    .add_col(TextSpan::from("KeyCode::Down"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor down"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::Up"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor up"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::PageDown"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor down by 8"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::PageUp"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("ove cursor up by 8"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::End"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor to last item"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::Home"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor to first item"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::Char(_)"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Return pressed key"))
                    .build(),
            );
        // Get value (not scrollable)
        assert_eq!(component.state(), State::None);
    }

    #[test]
    fn should_init_list_value() {
        let mut component = List::default()
            .foreground(Color::Red)
            .background(Color::Blue)
            .highlighted_color(Color::Yellow)
            .highlighted_str("🚀")
            .modifiers(TextModifiers::BOLD)
            .borders(Borders::default())
            .title("events", Alignment::Center)
            .rows(
                TableBuilder::default()
                    .add_col(TextSpan::from("KeyCode::Down"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor down"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::Up"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor up"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::PageDown"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor down by 8"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::PageUp"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("ove cursor up by 8"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::End"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor to last item"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::Home"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Move cursor to first item"))
                    .add_row()
                    .add_col(TextSpan::from("KeyCode::Char(_)"))
                    .add_col(TextSpan::from("OnKey"))
                    .add_col(TextSpan::from("Return pressed key"))
                    .build(),
            )
            .scroll(true)
            .selected_line(2);
        assert_eq!(component.states.list_index, 2);
        // Index out of bounds
        component.attr(
            Attribute::Value,
            AttrValue::Payload(PropPayload::One(PropValue::Usize(50))),
        );
        assert_eq!(component.states.list_index, 6);
    }
}
