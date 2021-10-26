//! ## List
//!
//! `List` represents a read-only textual list component which can be scrollable through arrows or inactive

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
    Alignment, AttrValue, Attribute, Borders, Color, Props, Style, Table, TextModifiers,
};
use tuirealm::tui::{
    layout::{Corner, Rect},
    text::{Span, Spans},
    widgets::{List as TuiList, ListItem, ListState},
};
use tuirealm::{Frame, MockComponent, State, StateValue};

// -- States

struct OwnStates {
    list_index: usize, // Index of selected item in list
    list_len: usize,   // Lines in text area
}

impl Default for OwnStates {
    fn default() -> Self {
        Self {
            list_index: 0,
            list_len: 0,
        }
    }
}

impl OwnStates {
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
    pub fn calc_max_step_ahead(&self, max: usize) -> usize {
        let remaining: usize = match self.list_len {
            0 => 0,
            len => len - 1 - self.list_index,
        };
        if remaining > max {
            max
        } else {
            remaining
        }
    }

    /// ### calc_max_step_ahead
    ///
    /// Calculate the max step ahead to scroll list
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
pub struct List {
    props: Props,
    states: OwnStates,
}

impl Default for List {
    fn default() -> Self {
        Self {
            props: Props::default(),
            states: OwnStates::default(),
        }
    }
}

impl List {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.props.set(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.props.set(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.props
            .set(Attribute::TextProps, AttrValue::TextModifiers(m));
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

    pub fn inactive(mut self, s: Style) -> Self {
        self.props.set(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    pub fn rewind(mut self, r: bool) -> Self {
        self.props.set(Attribute::Rewind, AttrValue::Flag(r));
        self
    }

    pub fn step(mut self, step: usize) -> Self {
        self.props
            .set(Attribute::ScrollStep, AttrValue::Length(step));
        self
    }

    pub fn scroll(mut self, scrollable: bool) -> Self {
        self.props
            .set(Attribute::Scroll, AttrValue::Flag(scrollable));
        self
    }

    pub fn highlighted_str<S: AsRef<str>>(mut self, s: S) -> Self {
        self.props.set(
            Attribute::HighlightedStr,
            AttrValue::String(s.as_ref().to_string()),
        );
        self
    }

    pub fn highlighted_color(mut self, c: Color) -> Self {
        self.props
            .set(Attribute::HighlightedColor, AttrValue::Color(c));
        self
    }

    pub fn rows(mut self, rows: Table) -> Self {
        self.states.set_list_len(rows.len());
        self.states.fix_list_index();
        self.props.set(Attribute::Content, AttrValue::Table(rows));
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
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();
            let title = self
                .props
                .get_or(
                    Attribute::Title,
                    AttrValue::Title((String::default(), Alignment::Center)),
                )
                .unwrap_title();
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
            let active: bool = match self.scrollable() {
                true => focus,
                false => true,
            };
            let div = crate::utils::get_block(borders, title.as_ref(), active);
            // Make list entries
            let list_items: Vec<ListItem> =
                match self.props.get(Attribute::Content).map(|x| x.unwrap_table()) {
                    Some(table) => table
                        .iter()
                        .map(|row| {
                            let columns: Vec<Span> = row
                                .iter()
                                .map(|col| {
                                    let (fg, bg, modifiers) =
                                        crate::utils::use_or_default_styles(&self.props, col);
                                    Span::styled(
                                        col.content.clone(),
                                        Style::default().add_modifier(modifiers).fg(fg).bg(bg),
                                    )
                                })
                                .collect();
                            ListItem::new(Spans::from(columns))
                        })
                        .collect(), // Make List item from TextSpan
                    _ => Vec::new(),
                };
            let highlighted_color: Color = match self
                .props
                .get(Attribute::HighlightedColor)
                .map(|x| x.unwrap_color())
            {
                None => match focus {
                    true => background,
                    false => foreground,
                },
                Some(color) => color,
            };
            let (fg, bg): (Color, Color) = match active {
                true => (background, highlighted_color),
                false => (highlighted_color, background),
            };
            // Make list
            let mut list = TuiList::new(list_items)
                .block(div)
                .start_corner(Corner::TopLeft)
                .highlight_style(
                    Style::default()
                        .fg(fg)
                        .bg(bg)
                        .add_modifier(self.props.modifiers),
                );
            // Highlighted symbol
            if let Some(hg_str) = self
                .props
                .get(Attribute::HighlightedStr)
                .map(|x| x.unwrap_string())
            {
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
        // Update list len and fix index
        self.set_list_len(
            match self.props.get(Attribute::Content).map(|x| x.unwrap_table()) {
                Some(spans) => spans.len(),
                _ => 0,
            },
        );
        self.states.fix_list_index();
    }

    fn state(&self) -> State {
        match self.is_scrollable() {
            true => State::One(StateValue::Usize(self.states.list_index)),
            false => State::None,
        }
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Down) => {
                let prev = self.states.list_index;
                self.states.incr_list_index(self.rewindable());
                if prev != self.states.list_index {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::Move(Direction::Up) => {
                let prev = self.states.list_index;
                self.states.decr_list_index(self.rewindable());
                if prev != self.states.list_index {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
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
                if prev != self.states.list_index {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::Scroll(Direction::Up) => {
                let prev = self.states.list_index;
                let step = self
                    .props
                    .get_or(Attribute::ScrollStep, AttrValue::Length(8))
                    .unwrap_length();
                let step: usize = self.states.calc_max_step_ahead(step);
                (0..step).for_each(|_| self.states.decr_list_index(false));
                if prev != self.states.list_index {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::GoTo(Position::Begin) => {
                let prev = self.states.list_index;
                self.states.list_index_at_first();
                if prev != self.states.list_index {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::GoTo(Position::End) => {
                let prev = self.states.list_index;
                self.states.list_index_at_last();
                if prev != self.states.list_index {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            _ => CmdResult::None,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::io::Seek;

    use super::*;
    use pretty_assertions::assert_eq;
    use tuirealm::props::{TableBuilder, TextSpan};

    #[test]
    fn list_states() {
        let mut states = OwnStates::default();
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
        // Own funcs
        assert_eq!(component.layout().len(), 4);
        // Increment list index
        component.states.list_index += 1;
        assert_eq!(component.states.list_index, 1);
        // Check messages
        // Handle inputs
        assert_eq!(
            component.on(Cmd::Move(Direction::Down)),
            CmdResult::Changed(State::One(StateValue::Usize(2)))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 2);
        // Index should be decremented
        assert_eq!(
            component.on(Cmd::Move(Direction::Up)),
            CmdResult::Changed(State::One(StateValue::Usize(1)))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 1);
        // Index should be 2
        assert_eq!(
            component.on(Cmd::Scroll(Direction::Down)),
            CmdResult::Changed(State::One(StateValue::Usize(5)))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 5);
        assert_eq!(
            component.on(Cmd::Scroll(Direction::Down)),
            CmdResult::Changed(State::One(StateValue::Usize(6)))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 6);
        // Index should be 0
        assert_eq!(
            component.on(Cmd::Scroll(Direction::Up)),
            CmdResult::Changed(State::One(StateValue::Usize(2)))
        );
        assert_eq!(component.states.list_index, 2);
        assert_eq!(
            component.on(Cmd::Scroll(Direction::Up)),
            CmdResult::Changed(State::One(StateValue::Usize(0)))
        );
        assert_eq!(component.states.list_index, 0);
        // End
        assert_eq!(
            component.on(Cmd::GoTo(Position::End)),
            CmdResult::Changed(State::One(StateValue::Usize(6)))
        );
        assert_eq!(component.states.list_index, 6);
        // Home
        assert_eq!(
            component.on(Cmd::GoTo(Position::Begin)),
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
            );
        // Get value (not scrollable)
        assert_eq!(component.state(), State::None);
    }
}
