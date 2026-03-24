//! `Input` represents a read-write input field. This component supports different input types, input length
//! and handles input events related to cursor position, backspace, canc, ...

use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::Component;
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, InputType, Props, Style, TextModifiers, Title,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::state::{State, StateValue};

use super::props::{INPUT_INVALID_STYLE, INPUT_PLACEHOLDER, INPUT_PLACEHOLDER_STYLE};
use crate::prop_ext::CommonProps;
use crate::utils::calc_utf8_cursor_position;

// -- states

/// The number of characters [`InputStates::display_offset`] will keep in view in a single direction.
const PREVIEW_DISTANCE: usize = 2;

/// The state that needs to be kept for the [`Input`] component.
#[derive(Default, Debug)]
pub struct InputStates {
    /// The current input text
    pub input: Vec<char>,
    /// The cursor into "input", used as a index on where a character gets added next
    pub cursor: usize,
    /// The display offset for scrolling, always tries to keep the cursor within bounds
    pub display_offset: usize,
    /// The last drawn width of the component that displays "input".
    ///
    /// This is necessary to keep "display_offset" from jumping around on width changes.
    pub last_width: Option<u16>,
}

impl InputStates {
    /// Append a character, if possible according to input type.
    pub fn append(&mut self, ch: char, itype: &InputType, max_len: Option<usize>) {
        // Check if max length has been reached
        if self.input.len() < max_len.unwrap_or(usize::MAX) {
            // Check whether can push
            if itype.char_valid(self.input.iter().collect::<String>().as_str(), ch) {
                self.input.insert(self.cursor, ch);
                self.incr_cursor();
            }
        }
    }

    /// Delete the element at `cursor - 1`, then decrement cursor by 1.
    pub fn backspace(&mut self) {
        if self.cursor > 0 && !self.input.is_empty() {
            self.input.remove(self.cursor - 1);
            // Decrement cursor
            self.cursor -= 1;

            if self.cursor < self.display_offset.saturating_add(PREVIEW_DISTANCE) {
                self.display_offset = self.display_offset.saturating_sub(1);
            }
        }
    }

    /// Delete element at cursor position.
    pub fn delete(&mut self) {
        if self.cursor < self.input.len() {
            self.input.remove(self.cursor);
        }
    }

    /// Increment cursor by one if possible. (also known as moving RIGHT)
    pub fn incr_cursor(&mut self) {
        if self.cursor < self.input.len() {
            self.cursor += 1;

            if let Some(last_width) = self.last_width {
                let input_with_width = self.input.len().saturating_sub(
                    usize::from(self.last_width.unwrap_or_default())
                        .saturating_sub(PREVIEW_DISTANCE),
                );
                // only increase the offset IF cursor is higher than last_width
                // and the remaining text does not fit within the last_width
                if self.cursor
                    > usize::from(last_width).saturating_sub(PREVIEW_DISTANCE) + self.display_offset
                    && self.display_offset < input_with_width
                {
                    self.display_offset += 1;
                }
            }
        }
    }

    /// Decrement cursor value by one if possible. (also known as moving LEFT)
    pub fn decr_cursor(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;

            if self.cursor < self.display_offset.saturating_add(PREVIEW_DISTANCE) {
                self.display_offset = self.display_offset.saturating_sub(1);
            }
        }
    }

    /// Move the cursor to the beginning of the input.
    pub fn cursor_at_begin(&mut self) {
        self.cursor = 0;
        self.display_offset = 0;
    }

    /// Move the cursor the the end of the input.
    pub fn cursor_at_end(&mut self) {
        self.cursor = self.input.len();
        self.display_offset = self.input.len().saturating_sub(
            usize::from(self.last_width.unwrap_or_default()).saturating_sub(PREVIEW_DISTANCE),
        );
    }

    /// Update the last width used to display [`InputStates::input`].
    ///
    /// This is necessary to update [`InputStates::display_offset`] correctly and keep it
    /// from jumping around on width changes.
    ///
    /// Without using this function, no scrolling will effectively be applied.
    pub fn update_width(&mut self, new_width: u16) {
        let old_width = self.last_width;
        self.last_width = Some(new_width);

        // if the cursor would now be out-of-bounds, adjust the display offset to keep the cursor within bounds
        if self.cursor
            > (self.display_offset + usize::from(new_width)).saturating_sub(PREVIEW_DISTANCE)
        {
            let diff = if let Some(old_width) = old_width {
                usize::from(old_width.saturating_sub(new_width))
            } else {
                // there was no previous width, use new_width minus cursor.
                // this happens if "update_width" had never been called (like before the first draw)
                // but the value is longer than the current display width and the cursor is not within bounds.
                self.cursor.saturating_sub(usize::from(new_width))
            };
            self.display_offset += diff;
        }
    }

    /// Get the full text to render in the input, according to the [`InputType`].
    #[must_use]
    pub fn render_value(&self, itype: &InputType) -> String {
        self.render_value_chars(itype).iter().collect::<String>()
    }

    /// Get the partial text to render, according to the [`InputType`].
    ///
    /// Unlike [`InputStates::render_value`], this will only collect the actually displayed text in the returned String.
    #[must_use]
    pub fn render_value_offset(&self, itype: &InputType) -> String {
        self.render_value_chars(itype)
            .iter()
            .skip(self.display_offset)
            .collect()
    }

    /// Get the characters to render, according to [`InputType`].
    ///
    /// It is recommended to use [`render_value`](Self::render_value) or [`render_value_offset`](Self::render_value_offset) over this function.
    #[must_use]
    pub fn render_value_chars(&self, itype: &InputType) -> Vec<char> {
        // TODO: can we return a iterator or something to prevent this intermediary Vec?
        match itype {
            InputType::Password(ch) | InputType::CustomPassword(ch, _, _) => {
                (0..self.input.len()).map(|_| *ch).collect()
            }
            _ => self.input.clone(),
        }
    }

    /// Get the current input as a String in full, without any [`InputType`] modifications.
    #[must_use]
    pub fn get_value(&self) -> String {
        self.input.iter().collect()
    }
}

// -- Component

/// `Input` represents a read-write input field. This component supports different input types, input length
/// and handles input events related to cursor position, backspace, canc, ...
#[derive(Default)]
#[must_use]
pub struct Input {
    common: CommonProps,
    props: Props,
    pub states: InputStates,
}

impl Input {
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

    /// Set the type of input this Input Component is for. Specific types may have different display or validate methods.
    pub fn input_type(mut self, itype: InputType) -> Self {
        self.attr(Attribute::InputType, AttrValue::InputType(itype));
        self
    }

    /// Set the max length of the input.
    pub fn input_len(mut self, ilen: usize) -> Self {
        self.attr(Attribute::InputLength, AttrValue::Length(ilen));
        self
    }

    /// Set the inital value of the Input.
    pub fn value<S: Into<String>>(mut self, s: S) -> Self {
        self.attr(Attribute::Value, AttrValue::String(s.into()));
        self
    }

    /// Set a style for when the input fails validation.
    pub fn invalid_style(mut self, s: Style) -> Self {
        self.attr(Attribute::Custom(INPUT_INVALID_STYLE), AttrValue::Style(s));
        self
    }

    /// Set a placeholder text and stylew for when the Input is empty.
    pub fn placeholder<S: Into<String>>(mut self, placeholder: S, style: Style) -> Self {
        // TODO: Span / Line?
        self.attr(
            Attribute::Custom(INPUT_PLACEHOLDER),
            AttrValue::String(placeholder.into()),
        );
        self.attr(
            Attribute::Custom(INPUT_PLACEHOLDER_STYLE),
            AttrValue::Style(style),
        );
        self
    }

    fn get_input_len(&self) -> Option<usize> {
        self.props
            .get(Attribute::InputLength)
            .and_then(AttrValue::as_length)
    }

    fn get_input_type(&self) -> &InputType {
        self.props
            .get(Attribute::InputType)
            .and_then(AttrValue::as_input_type)
            .unwrap_or(&InputType::Text)
    }

    /// Checks whether current input is valid according to the set [`InputType`].
    fn is_valid(&self) -> bool {
        let value = self.states.get_value();
        self.get_input_type().validate(value.as_str())
    }
}

impl Component for Input {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if !self.common.display {
            return;
        }

        let mut normal_style = self.common.style;

        let mut block = self.common.get_block();
        // Apply invalid style
        // TODO: invalid style should likely still be applied even if unfocused
        if self.common.focused
            && !self.is_valid()
            && let Some(invalid_style) = self
                .props
                .get(Attribute::Custom(INPUT_INVALID_STYLE))
                .and_then(AttrValue::as_style)
        {
            if let Some(block) = &mut block {
                let border_style = self
                    .common
                    .border
                    .unwrap_or_default()
                    .style()
                    .patch(invalid_style);
                // i dont like this, but ratatui does not offer a non-self taking method to change the style
                *block = std::mem::take(block).border_style(border_style);
            }

            normal_style = normal_style.patch(invalid_style);
        }

        let mut area_for_bounds = area;

        if let Some(block) = &block {
            // Create input's area
            let block_inner_area = block.inner(area);

            self.states.update_width(block_inner_area.width);

            area_for_bounds = block_inner_area;
        }

        let text_to_display = self.states.render_value_offset(self.get_input_type());

        let show_placeholder = text_to_display.is_empty();
        // Choose whether to show placeholder; if placeholder is unset, show nothing
        let text_to_display = if show_placeholder {
            self.states.cursor = 0;
            self.props
                .get(Attribute::Custom(INPUT_PLACEHOLDER))
                .and_then(AttrValue::as_string)
                .map(String::as_str)
                .unwrap_or_default()
        } else {
            &text_to_display
        };
        // Choose paragraph style based on whether is valid or not and if has focus and if should show placeholder
        let paragraph_style = if self.common.focused {
            normal_style
        } else {
            // TODO: this should likely be a different property
            self.common.border_unfocused_style
        };
        let paragraph_style = if show_placeholder {
            self.props
                .get(Attribute::Custom(INPUT_PLACEHOLDER_STYLE))
                .and_then(AttrValue::as_style)
                .unwrap_or(paragraph_style)
        } else {
            paragraph_style
        };

        let mut widget = Paragraph::new(text_to_display).style(paragraph_style);

        if let Some(block) = block {
            widget = widget.block(block);
        }

        render.render_widget(widget, area);

        // Set cursor, if focus
        if self.common.focused && !area_for_bounds.is_empty() {
            let x: u16 = area_for_bounds.x
                + calc_utf8_cursor_position(
                    &self.states.render_value_chars(self.get_input_type())[0..self.states.cursor],
                )
                .saturating_sub(u16::try_from(self.states.display_offset).unwrap_or(u16::MAX));
            let x = x.min(area_for_bounds.x + area_for_bounds.width);
            render.set_cursor_position(tuirealm::ratatui::prelude::Position {
                x,
                y: area_for_bounds.y,
            });
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        if let Some(value) = self.common.get(attr) {
            return Some(value);
        }

        self.props.get(attr).cloned()
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if let Some(value) = self.common.set(attr, value) {
            let sanitize_input = matches!(
                attr,
                Attribute::InputLength | Attribute::InputType | Attribute::Value
            );
            // Check if new input
            let new_input = match attr {
                Attribute::Value => Some(value.clone().unwrap_string()),
                _ => None,
            };
            self.props.set(attr, value);
            if sanitize_input {
                let input = match new_input {
                    None => self.states.input.clone(),
                    Some(v) => v.chars().collect(),
                };
                self.states.input = Vec::new();
                self.states.cursor = 0;
                let itype = self.get_input_type().clone();
                let max_len = self.get_input_len();
                for ch in input {
                    self.states.append(ch, &itype, max_len);
                }
            }
        }
    }

    fn state(&self) -> State {
        // Validate input
        if self.is_valid() {
            State::Single(StateValue::String(self.states.get_value()))
        } else {
            State::None
        }
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Delete => {
                // Backspace and None
                let prev_input = self.states.input.clone();
                self.states.backspace();
                if prev_input == self.states.input {
                    CmdResult::None
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            Cmd::Cancel => {
                // Delete and None
                let prev_input = self.states.input.clone();
                self.states.delete();
                if prev_input == self.states.input {
                    CmdResult::None
                } else {
                    CmdResult::Changed(self.state())
                }
            }
            Cmd::Submit => CmdResult::Submit(self.state()),
            Cmd::Move(Direction::Left) => {
                self.states.decr_cursor();
                CmdResult::None
            }
            Cmd::Move(Direction::Right) => {
                self.states.incr_cursor();
                CmdResult::None
            }
            Cmd::GoTo(Position::Begin) => {
                self.states.cursor_at_begin();
                CmdResult::None
            }
            Cmd::GoTo(Position::End) => {
                self.states.cursor_at_end();
                CmdResult::None
            }
            Cmd::Type(ch) => {
                // Push char to input
                let prev_input = self.states.input.clone();
                self.states
                    .append(ch, &self.get_input_type().clone(), self.get_input_len());
                // Message on change
                if prev_input == self.states.input {
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

    use pretty_assertions::assert_eq;
    use tuirealm::props::HorizontalAlignment;

    use super::*;

    #[test]
    fn test_components_input_states() {
        let mut states: InputStates = InputStates::default();
        states.append('a', &InputType::Text, Some(3));
        assert_eq!(states.input, vec!['a']);
        states.append('b', &InputType::Text, Some(3));
        assert_eq!(states.input, vec!['a', 'b']);
        states.append('c', &InputType::Text, Some(3));
        assert_eq!(states.input, vec!['a', 'b', 'c']);
        // Reached length
        states.append('d', &InputType::Text, Some(3));
        assert_eq!(states.input, vec!['a', 'b', 'c']);
        // Push char to numbers
        states.append('d', &InputType::Number, None);
        assert_eq!(states.input, vec!['a', 'b', 'c']);
        // move cursor
        // decr cursor
        states.decr_cursor();
        assert_eq!(states.cursor, 2);
        states.cursor = 1;
        states.decr_cursor();
        assert_eq!(states.cursor, 0);
        states.decr_cursor();
        assert_eq!(states.cursor, 0);
        // Incr
        states.incr_cursor();
        assert_eq!(states.cursor, 1);
        states.incr_cursor();
        assert_eq!(states.cursor, 2);
        states.incr_cursor();
        assert_eq!(states.cursor, 3);
        // Render value
        assert_eq!(states.render_value(&InputType::Text).as_str(), "abc");
        assert_eq!(
            states.render_value(&InputType::Password('*')).as_str(),
            "***"
        );
    }

    #[test]
    fn test_components_input_text() {
        // Instantiate Input with value
        let mut component: Input = Input::default()
            .background(Color::Yellow)
            .borders(Borders::default())
            .foreground(Color::Cyan)
            .inactive(Style::default())
            .input_len(5)
            .input_type(InputType::Text)
            .title(Title::from("pippo").alignment(HorizontalAlignment::Center))
            .value("home");
        // Verify initial state
        assert_eq!(component.states.cursor, 4);
        assert_eq!(component.states.input.len(), 4);
        // Get value
        assert_eq!(
            component.state(),
            State::Single(StateValue::String(String::from("home")))
        );
        // Character
        assert_eq!(
            component.perform(Cmd::Type('/')),
            CmdResult::Changed(State::Single(StateValue::String(String::from("home/"))))
        );
        assert_eq!(
            component.state(),
            State::Single(StateValue::String(String::from("home/")))
        );
        assert_eq!(component.states.cursor, 5);
        // Verify max length (shouldn't push any character)
        assert_eq!(component.perform(Cmd::Type('a')), CmdResult::None);
        assert_eq!(
            component.state(),
            State::Single(StateValue::String(String::from("home/")))
        );
        assert_eq!(component.states.cursor, 5);
        // Submit
        assert_eq!(
            component.perform(Cmd::Submit),
            CmdResult::Submit(State::Single(StateValue::String(String::from("home/"))))
        );
        // Backspace
        assert_eq!(
            component.perform(Cmd::Delete),
            CmdResult::Changed(State::Single(StateValue::String(String::from("home"))))
        );
        assert_eq!(
            component.state(),
            State::Single(StateValue::String(String::from("home")))
        );
        assert_eq!(component.states.cursor, 4);
        // Check backspace at 0
        component.states.input = vec!['h'];
        component.states.cursor = 1;
        assert_eq!(
            component.perform(Cmd::Delete),
            CmdResult::Changed(State::Single(StateValue::String(String::new())))
        );
        assert_eq!(
            component.state(),
            State::Single(StateValue::String(String::new()))
        );
        assert_eq!(component.states.cursor, 0);
        // Another one...
        assert_eq!(component.perform(Cmd::Delete), CmdResult::None);
        assert_eq!(
            component.state(),
            State::Single(StateValue::String(String::new()))
        );
        assert_eq!(component.states.cursor, 0);
        // See del behaviour here
        assert_eq!(component.perform(Cmd::Cancel), CmdResult::None);
        assert_eq!(
            component.state(),
            State::Single(StateValue::String(String::new()))
        );
        assert_eq!(component.states.cursor, 0);
        // Check del behaviour
        component.states.input = vec!['h', 'e'];
        component.states.cursor = 1;
        assert_eq!(
            component.perform(Cmd::Cancel),
            CmdResult::Changed(State::Single(StateValue::String(String::from("h"))))
        );
        assert_eq!(
            component.state(),
            State::Single(StateValue::String(String::from("h")))
        );
        assert_eq!(component.states.cursor, 1);
        // Another one (should do nothing)
        assert_eq!(component.perform(Cmd::Cancel), CmdResult::None);
        assert_eq!(
            component.state(),
            State::Single(StateValue::String(String::from("h")))
        );
        assert_eq!(component.states.cursor, 1);
        // Move cursor right
        component.states.input = vec!['h', 'e', 'l', 'l', 'o'];
        // Update length to 16
        component.attr(Attribute::InputLength, AttrValue::Length(16));
        component.states.cursor = 1;
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)), // between 'e' and 'l'
            CmdResult::None
        );
        assert_eq!(component.states.cursor, 2);
        // Put a character here
        assert_eq!(
            component.perform(Cmd::Type('a')),
            CmdResult::Changed(State::Single(StateValue::String(String::from("heallo"))))
        );
        assert_eq!(
            component.state(),
            State::Single(StateValue::String(String::from("heallo")))
        );
        assert_eq!(component.states.cursor, 3);
        // Move left
        assert_eq!(
            component.perform(Cmd::Move(Direction::Left)),
            CmdResult::None
        );
        assert_eq!(component.states.cursor, 2);
        // Go at the end
        component.states.cursor = 6;
        // Move right
        assert_eq!(component.perform(Cmd::GoTo(Position::End)), CmdResult::None);
        assert_eq!(component.states.cursor, 6);
        // Move left
        assert_eq!(
            component.perform(Cmd::Move(Direction::Left)),
            CmdResult::None
        );
        assert_eq!(component.states.cursor, 5);
        // Go at the beginning
        component.states.cursor = 0;
        assert_eq!(
            component.perform(Cmd::Move(Direction::Left)),
            CmdResult::None
        );
        //assert_eq!(component.render().unwrap().cursor, 0); // Should stay
        assert_eq!(component.states.cursor, 0);
        // End - begin
        assert_eq!(component.perform(Cmd::GoTo(Position::End)), CmdResult::None);
        assert_eq!(component.states.cursor, 6);
        assert_eq!(
            component.perform(Cmd::GoTo(Position::Begin)),
            CmdResult::None
        );
        assert_eq!(component.states.cursor, 0);
        // Update value
        component.attr(Attribute::Value, AttrValue::String("new-value".to_string()));
        assert_eq!(
            component.state(),
            State::Single(StateValue::String(String::from("new-value")))
        );
        // Invalidate input type
        component.attr(
            Attribute::InputType,
            AttrValue::InputType(InputType::Number),
        );
        assert_eq!(component.state(), State::None);
    }

    #[test]
    fn should_keep_cursor_within_bounds() {
        let text = "The quick brown fox jumps over the lazy dog";
        assert!(text.len() > 15);

        let mut states = InputStates::default();

        for ch in text.chars() {
            states.append(ch, &InputType::Text, None);
        }

        // at first, without any "width" set, both functions should return the same
        assert_eq!(states.cursor, text.len());
        assert_eq!(
            states.render_value(&InputType::Text),
            states.render_value_offset(&InputType::Text)
        );

        states.update_width(10);

        assert_eq!(
            states.render_value_offset(&InputType::Text),
            text[text.len() - 10..]
        );

        // the displayed text should not change until being in PREVIEW_STEP
        for i in 1..8 {
            states.decr_cursor();
            assert_eq!(states.cursor, text.len() - i);
            let val = states.render_value_offset(&InputType::Text);
            assert_eq!(val, text[text.len() - 10..]);
        }

        // preview step space at the end
        states.decr_cursor();
        assert_eq!(states.cursor, text.len() - 8);
        assert_eq!(
            states.render_value_offset(&InputType::Text),
            text[text.len() - 10..]
        );

        states.decr_cursor();
        assert_eq!(states.cursor, text.len() - 9);
        assert_eq!(
            states.render_value_offset(&InputType::Text),
            text[text.len() - 11..]
        );

        states.decr_cursor();
        assert_eq!(states.cursor, text.len() - 10);
        assert_eq!(
            states.render_value_offset(&InputType::Text),
            text[text.len() - 12..]
        );

        states.cursor_at_begin();
        assert_eq!(states.cursor, 0);
        assert_eq!(states.render_value(&InputType::Text), text);

        // the displayed text should not change until being in PREVIEW_STEP
        for i in 1..9 {
            states.incr_cursor();
            assert_eq!(states.cursor, i);
            let val = states.render_value_offset(&InputType::Text);
            assert_eq!(val, text);
        }

        states.incr_cursor();
        assert_eq!(states.cursor, 9);
        assert_eq!(states.render_value_offset(&InputType::Text), text[1..]);

        states.incr_cursor();
        assert_eq!(states.cursor, 10);
        assert_eq!(states.render_value_offset(&InputType::Text), text[2..]);

        // increasing width should not change display_offset
        states.update_width(30);
        assert_eq!(states.cursor, 10);
        assert_eq!(states.render_value_offset(&InputType::Text), text[2..]);

        // reset to 10, should also not change
        states.update_width(10);
        assert_eq!(states.cursor, 10);
        assert_eq!(states.render_value_offset(&InputType::Text), text[2..]);

        // should change display_offset by 1
        states.update_width(9);
        assert_eq!(states.cursor, 10);
        assert_eq!(states.render_value_offset(&InputType::Text), text[3..]);

        // reset to end
        states.update_width(10);
        states.cursor_at_end();

        // the displayed text should not change until being in PREVIEW_STEP
        for i in 1..=4 {
            states.decr_cursor();
            assert_eq!(states.cursor, text.len() - i);
            let val = states.render_value_offset(&InputType::Text);
            assert_eq!(val, text[text.len() - 8..]);
        }

        assert_eq!(states.cursor, text.len() - 4);
        states.incr_cursor();
        assert_eq!(states.cursor, text.len() - 3);
        assert_eq!(
            states.render_value_offset(&InputType::Text),
            text[text.len() - 8..]
        );

        // note any width below PREVIEW_STEP * 2 + 1 is undefined behavior
    }
}
