//! `List` represents a read-only textual list component which can be scrollable through arrows or inactive.

use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::Component;
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, LineStatic, PropPayload, PropValue, Props, QueryResult,
    Style, TextModifiers, Title,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::{List as TuiList, ListItem, ListState};
use tuirealm::state::{State, StateValue};

use crate::prop_ext::CommonProps;
use crate::utils::{self, borrow_clone_line};

// -- States

/// The state that needs to be kept for the [`List`] componennt:
#[derive(Default)]
pub struct ListStates {
    /// Index of selected item in list
    pub list_index: usize,
    /// Lines in text area
    pub list_len: usize,
}

impl ListStates {
    /// Set the list length.
    pub fn set_list_len(&mut self, len: usize) {
        self.list_len = len;
    }

    /// Incremenet the list index.
    pub fn incr_list_index(&mut self, rewind: bool) {
        // Check if index is at last element
        if self.list_index + 1 < self.list_len {
            self.list_index += 1;
        } else if rewind {
            self.list_index = 0;
        }
    }

    /// Decrement the list index.
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

    /// Set the list index to the first item in the list.
    pub fn list_index_at_first(&mut self) {
        self.list_index = 0;
    }

    /// Set the list index at the last item of the list.
    pub fn list_index_at_last(&mut self) {
        if self.list_len > 0 {
            self.list_index = self.list_len - 1;
        } else {
            self.list_index = 0;
        }
    }

    /// Calculate the max step ahead to scroll the list.
    #[must_use]
    pub fn calc_max_step_ahead(&self, max: usize) -> usize {
        let remaining: usize = match self.list_len {
            0 => 0,
            len => len - 1 - self.list_index,
        };
        if remaining > max { max } else { remaining }
    }

    /// Calculate the max step ahead to scroll the list.
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

/// `List` represents a read-only textual list component which can be scrollable through arrows or inactive.
#[derive(Default)]
#[must_use]
pub struct List {
    common: CommonProps,
    props: Props,
    pub states: ListStates,
}

impl List {
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

    /// Set whether wraparound should be possible (down on the last choice wraps around to 0, and the other way around).
    pub fn rewind(mut self, r: bool) -> Self {
        self.attr(Attribute::Rewind, AttrValue::Flag(r));
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

    /// Set the rows of items the list should contain
    pub fn rows<T>(mut self, rows: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<LineStatic>,
    {
        self.attr(
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(
                rows.into_iter()
                    .map(Into::into)
                    .map(PropValue::TextLine)
                    .collect(),
            )),
        );
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

    fn scrollable(&self) -> bool {
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
}

impl Component for List {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if !self.common.display {
            return;
        }

        // Make list entries
        let payload = self.props.get(Attribute::Text).and_then(|x| x.as_payload());
        let list_items: Vec<ListItem> = match payload {
            Some(PropPayload::Vec(lines)) => {
                lines
                    .iter()
                    // this will skip any "PropValue" that is not a "TextLine", instead of panicing
                    .filter_map(|x| x.as_textline())
                    .map(utils::borrow_clone_line)
                    .map(ListItem::from)
                    .collect()
            }
            _ => Vec::new(),
        };
        let highlighted_color = self
            .props
            .get(Attribute::HighlightedColor)
            .and_then(AttrValue::as_color);

        // Make list
        let mut widget = TuiList::new(list_items)
            .style(self.common.style)
            .direction(tuirealm::ratatui::widgets::ListDirection::TopToBottom);

        if let Some(block) = self.common.get_block() {
            widget = widget.block(block);
        }

        if let Some(highlighted_color) = highlighted_color {
            widget = widget.highlight_style(Style::default().fg(highlighted_color).add_modifier(
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
        if self.scrollable() {
            let mut state: ListState = ListState::default();
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
            if matches!(attr, Attribute::Text) {
                // Update list len and fix index
                self.states.set_list_len(
                    match self
                        .props
                        .get(Attribute::Text)
                        .and_then(AttrValue::as_payload)
                        .and_then(PropPayload::as_vec)
                    {
                        Some(rows) => rows.len(),
                        _ => 0,
                    },
                );
                self.states.fix_list_index();
            } else if matches!(attr, Attribute::Value) && self.scrollable() {
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
        if self.scrollable() {
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
                    .get(Attribute::ScrollStep)
                    .and_then(AttrValue::as_length)
                    .unwrap_or(8);
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
                    .get(Attribute::ScrollStep)
                    .and_then(AttrValue::as_length)
                    .unwrap_or(8);
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
            _ => CmdResult::Invalid(cmd),
        }
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;
    use tuirealm::props::HorizontalAlignment;
    use tuirealm::ratatui::text::{Line, Span};

    use super::*;

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
            .title(Title::from("events").alignment(HorizontalAlignment::Center))
            .rewind(true)
            .rows([
                // Note: this could be improved if ratatui implements "From<[X; _]> for Line"
                // will get automatically converted to lines
                vec![
                    Span::from("KeyCode::Down"),
                    Span::from("OnKey"),
                    Span::from("Move cursor down"),
                ],
                vec![
                    Span::from("KeyCode::Up"),
                    Span::from("OnKey"),
                    Span::from("Move cursor up"),
                ],
                vec![
                    Span::from("KeyCode::PageDown"),
                    Span::from("OnKey"),
                    Span::from("Move cursor down by 8"),
                ],
                vec![
                    Span::from("KeyCode::PageUp"),
                    Span::from("OnKey"),
                    Span::from("ove cursor up by 8"),
                ],
                vec![
                    Span::from("KeyCode::End"),
                    Span::from("OnKey"),
                    Span::from("Move cursor to last item"),
                ],
                vec![
                    Span::from("KeyCode::Home"),
                    Span::from("OnKey"),
                    Span::from("Move cursor to first item"),
                ],
                vec![
                    Span::from("KeyCode::Char(_)"),
                    Span::from("OnKey"),
                    Span::from("Return pressed key"),
                    Span::from("4th mysterious columns"),
                ],
            ]);
        assert_eq!(component.states.list_len, 7);
        assert_eq!(component.states.list_index, 0);
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
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(vec![PropValue::TextLine(Line::from(
                "name age birthdate",
            ))])),
        );
        assert_eq!(component.states.list_len, 1);
        assert_eq!(component.states.list_index, 0);
        // Get value
        assert_eq!(component.state(), State::Single(StateValue::Usize(0)));
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
            .title(Title::from("events").alignment(HorizontalAlignment::Center))
            .rows([
                Line::from("KeyCode::Down OnKey Move cursor down"),
                Line::from("KeyCode::Up OnKey Move cursor up"),
                Line::from("KeyCode::PageDown OnKey Move cursor down by 8"),
                Line::from("KeyCode::PageUp OnKey Move cursor up by 8"),
                Line::from("KeyCode::End OnKey Move cursor to last item"),
                Line::from("KeyCode::Home OnKey Move cursor to first item"),
                Line::from("KeyCode::Char(_) OnKey Return pressed key"),
            ]);
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
            .title(Title::from("events").alignment(HorizontalAlignment::Center))
            .rows([
                "KeyCode::Down OnKey Move cursor down",
                "KeyCode::Up OnKey Move cursor up",
                "KeyCode::PageDown OnKey Move cursor down by 8",
                "KeyCode::PageUp OnKey Move cursor up by 8",
                "KeyCode::End OnKey Move cursor to last item",
                "KeyCode::Home OnKey Move cursor to first item",
                "KeyCode::Char(_) OnKey Return pressed key",
            ])
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
}
