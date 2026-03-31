use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::Component;
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, LineStatic, Props, QueryResult, Style, TextModifiers,
    TextStatic, Title,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::{List, ListItem, ListState};
use tuirealm::state::{State, StateValue};

use crate::prop_ext::CommonProps;
use crate::utils::borrow_clone_line;

// -- States

/// The state that has to be kept for the [`Textarea`] component.
#[derive(Default)]
pub struct TextareaStates {
    /// Index of selected item in textarea
    pub list_index: usize,
    /// Lines in text area
    pub list_len: usize,
}

impl TextareaStates {
    /// Set list length and fix list index.
    pub fn set_list_len(&mut self, len: usize) {
        self.list_len = len;
        self.fix_list_index();
    }

    /// Incremenet list index.
    pub fn incr_list_index(&mut self) {
        // Check if index is at last element
        if self.list_index + 1 < self.list_len {
            self.list_index += 1;
        }
    }

    /// Decrement list index.
    pub fn decr_list_index(&mut self) {
        // Check if index is bigger than 0
        if self.list_index > 0 {
            self.list_index -= 1;
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
    fn calc_max_step_ahead(&self, max: usize) -> usize {
        let remaining: usize = match self.list_len {
            0 => 0,
            len => len - 1 - self.list_index,
        };
        if remaining > max { max } else { remaining }
    }

    /// Calculate the max step ahead to scroll list.
    fn calc_max_step_behind(&self, max: usize) -> usize {
        if self.list_index > max {
            max
        } else {
            self.list_index
        }
    }
}

/// A Textarea represents multi-line, multi-style, automatically wrapped text, with container and scroll support.
///
/// If scroll is not necessary, use [`Paragrapg`](super::Paragraph) instead.
///
/// If single-style, single-line text is wanted, use [`Label`](super::Label).
/// If multi-style, single-line text is wanted, use [`Span`](super::Span).
#[derive(Default)]
#[must_use]
pub struct Textarea {
    common: CommonProps,
    props: Props,
    pub states: TextareaStates,
}

impl Textarea {
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
        self.attr(Attribute::UnfocusedBorderStyle, AttrValue::Style(s));
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

    /// Set the Symbol and Style for the indicator of the current line.
    pub fn highlight_str<S: Into<LineStatic>>(mut self, s: S) -> Self {
        self.attr(Attribute::HighlightedStr, AttrValue::TextLine(s.into()));
        self
    }

    /// Set the Text content via a array or iterator.
    ///
    /// # Example
    ///
    /// ```
    /// # use tui_realm_stdlib::components::Textarea;
    /// # use tuirealm::ratatui::text::Line;
    /// Textarea::default()
    ///     .text_rows([
    ///         Line::raw("line1"),
    ///         Line::raw("line2")
    ///     ]);
    /// ```
    pub fn text_rows<T>(self, text: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<LineStatic>,
    {
        let text = TextStatic::from_iter(text);
        self.text(text)
    }

    /// Set the Text content via a single struct.
    ///
    /// # Example
    ///
    /// ```
    /// # use tui_realm_stdlib::components::Textarea;
    /// # use tuirealm::ratatui::text::{Line, Text};
    /// Textarea::default()
    ///     .text(Line::raw("line"));
    /// Textarea::default()
    ///     .text(Text::raw("another line"));
    /// ```
    pub fn text(mut self, text: impl Into<TextStatic>) -> Self {
        let text = text.into();
        self.states.set_list_len(text.lines.len());
        self.attr(Attribute::Text, AttrValue::Text(text));
        self
    }
}

impl Component for Textarea {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if !self.common.display {
            return;
        }

        // Highlighted symbol
        let hg_str = self
            .props
            .get(Attribute::HighlightedStr)
            .and_then(|x| x.as_textline());
        // NOTE: wrap width is width of area minus 2 (block) minus width of highlighting string
        let wrap_width = (area.width as usize) - hg_str.as_ref().map_or(0, |x| x.width()) - 2;
        // TODO: refactor to use "Text"?
        let lines: Vec<ListItem> = self
            .props
            .get(Attribute::Text)
            .and_then(AttrValue::as_text)
            .map(|text| {
                text.iter()
                    .map(|x| crate::utils::wrap_lines(&[x], wrap_width))
                    .map(ListItem::new)
                    .collect()
            })
            .unwrap_or_default();

        let mut state: ListState = ListState::default();
        state.select(Some(self.states.list_index));
        // Make component

        let mut list = List::new(lines)
            .direction(tuirealm::ratatui::widgets::ListDirection::TopToBottom)
            .style(self.common.style);

        if let Some(block) = self.common.get_block() {
            list = list.block(block);
        }
        if let Some(hg_str) = hg_str {
            list = list.highlight_symbol(borrow_clone_line(hg_str));
        }

        render.render_stateful_widget(list, area, &mut state);
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
            // Update list len and fix index
            self.states.set_list_len(
                self.props
                    .get(Attribute::Text)
                    .and_then(AttrValue::as_text)
                    .map_or(0, |text| text.lines.len()),
            );
            self.states.fix_list_index();
        }
    }

    fn state(&self) -> State {
        State::Single(StateValue::Usize(self.states.list_index))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        let prev = self.states.list_index;
        match cmd {
            Cmd::Move(Direction::Down) => {
                self.states.incr_list_index();
            }
            Cmd::Move(Direction::Up) => {
                self.states.decr_list_index();
            }
            Cmd::Scroll(Direction::Down) => {
                let step = self
                    .props
                    .get(Attribute::ScrollStep)
                    .and_then(AttrValue::as_length)
                    .unwrap_or(8);
                let step = self.states.calc_max_step_ahead(step);
                (0..step).for_each(|_| self.states.incr_list_index());
            }
            Cmd::Scroll(Direction::Up) => {
                let step = self
                    .props
                    .get(Attribute::ScrollStep)
                    .and_then(AttrValue::as_length)
                    .unwrap_or(8);
                let step = self.states.calc_max_step_behind(step);
                (0..step).for_each(|_| self.states.decr_list_index());
            }
            Cmd::GoTo(Position::Begin) => {
                self.states.list_index_at_first();
            }
            Cmd::GoTo(Position::End) => {
                self.states.list_index_at_last();
            }
            _ => return CmdResult::Invalid(cmd),
        }
        if prev != self.states.list_index {
            CmdResult::Changed(self.state())
        } else {
            CmdResult::NoChange
        }
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;
    use tuirealm::props::HorizontalAlignment;
    use tuirealm::ratatui::text::{Line, Span, Text};
    use tuirealm::state::StateValue;

    use super::*;

    #[test]
    fn test_components_textarea() {
        // Make component
        let mut component = Textarea::default()
            .foreground(Color::Red)
            .background(Color::Blue)
            .modifiers(TextModifiers::BOLD)
            .borders(Borders::default())
            .highlight_str("🚀")
            .step(4)
            .title(Title::from("textarea").alignment(HorizontalAlignment::Center))
            .text_rows([Line::from("welcome to "), Line::from("tui-realm")]);
        // Increment list index
        component.states.list_index += 1;
        assert_eq!(component.states.list_index, 1);
        // Add one row
        component.attr(
            Attribute::Text,
            AttrValue::Text(TextStatic::from_iter([
                Line::from("welcome"),
                Line::from("to"),
                Line::from("tui-realm"),
            ])),
        );
        // Verify states
        assert_eq!(component.states.list_index, 1); // Kept
        assert_eq!(component.states.list_len, 3);
        // get value
        assert_eq!(component.state(), State::Single(StateValue::Usize(1)));
        // Render
        assert_eq!(component.states.list_index, 1);
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
        // Index should be decremented
        assert_eq!(component.states.list_index, 1);
        // Index should be 2
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Down)),
            CmdResult::Changed(State::Single(StateValue::Usize(2)))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 2);
        // Index should be 0
        assert_eq!(
            component.perform(Cmd::Scroll(Direction::Up)),
            CmdResult::Changed(State::Single(StateValue::Usize(0)))
        );
        assert_eq!(component.states.list_index, 0);
        // End
        assert_eq!(
            component.perform(Cmd::GoTo(Position::End)),
            CmdResult::Changed(State::Single(StateValue::Usize(2)))
        );
        assert_eq!(component.states.list_index, 2);
        // Home
        assert_eq!(
            component.perform(Cmd::GoTo(Position::Begin)),
            CmdResult::Changed(State::Single(StateValue::Usize(0)))
        );
        assert_eq!(component.states.list_index, 0);
        // No-op when already at beginning
        assert_eq!(
            component.perform(Cmd::GoTo(Position::Begin)),
            CmdResult::NoChange
        );
        // Unhandled command
        assert_eq!(
            component.perform(Cmd::Delete),
            CmdResult::Invalid(Cmd::Delete)
        );
    }

    #[test]
    fn various_textrows_types() {
        // Vec
        let _ = Textarea::default().text_rows(vec![Span::raw("hello")]);
        // static array
        let _ = Textarea::default().text_rows([Span::raw("hello")]);
        // boxed array
        let _ = Textarea::default().text_rows(vec![Span::raw("hello")].into_boxed_slice());
        // already a iterator
        let _ = Textarea::default().text_rows(["Hello"].map(Span::raw));

        // Vec
        let _ = Textarea::default().text_rows(vec![Line::raw("hello")]);
        // static array
        let _ = Textarea::default().text_rows([Line::raw("hello")]);
        // boxed array
        let _ = Textarea::default().text_rows(vec![Line::raw("hello")].into_boxed_slice());
        // already a iterator
        let _ = Textarea::default().text_rows(["Hello"].map(Line::raw));
    }

    #[test]
    fn various_text_types() {
        // Line
        let _ = Textarea::default().text(Text::raw("hello"));
        // Line
        let _ = Textarea::default().text(Line::raw("hello"));
        // Span
        let _ = Textarea::default().text(Span::raw("hello"));
        // str
        let _ = Textarea::default().text("hello");
        // String
        let _ = Textarea::default().text("hello".to_string());
    }
}
