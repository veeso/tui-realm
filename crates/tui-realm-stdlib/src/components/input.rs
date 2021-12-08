//! ## Input
//!
//! `Input` represents a read-write input field. This component supports different input types, input length
//! and handles input events related to cursor position, backspace, canc, ...

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
use super::props::{INPUT_INVALID_STYLE, INPUT_PLACEHOLDER, INPUT_PLACEHOLDER_STYLE};
use crate::utils::calc_utf8_cursor_position;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, InputType, Props, Style, TextModifiers,
};
use tuirealm::tui::{layout::Rect, widgets::Paragraph};
use tuirealm::{Frame, MockComponent, State, StateValue};

// -- states

#[derive(Default)]
pub struct InputStates {
    pub input: Vec<char>, // Current input
    pub cursor: usize,    // Input position
}

impl InputStates {
    /// ### append
    ///
    /// Append, if possible according to input type, the character to the input vec
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

    /// ### backspace
    ///
    /// Delete element at cursor -1; then decrement cursor by 1
    pub fn backspace(&mut self) {
        if self.cursor > 0 && !self.input.is_empty() {
            self.input.remove(self.cursor - 1);
            // Decrement cursor
            self.cursor -= 1;
        }
    }

    /// ### delete
    ///
    /// Delete element at cursor
    pub fn delete(&mut self) {
        if self.cursor < self.input.len() {
            self.input.remove(self.cursor);
        }
    }

    /// ### incr_cursor
    ///
    /// Increment cursor value by one if possible
    pub fn incr_cursor(&mut self) {
        if self.cursor < self.input.len() {
            self.cursor += 1;
        }
    }

    /// ### cursoro_at_begin
    ///
    /// Place cursor at the begin of the input
    pub fn cursor_at_begin(&mut self) {
        self.cursor = 0;
    }

    /// ### cursor_at_end
    ///
    /// Place cursor at the end of the input
    pub fn cursor_at_end(&mut self) {
        self.cursor = self.input.len();
    }

    /// ### decr_cursor
    ///
    /// Decrement cursor value by one if possible
    pub fn decr_cursor(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// ### render_value
    ///
    /// Get value as string to render
    pub fn render_value(&self, itype: InputType) -> String {
        self.render_value_chars(itype).iter().collect::<String>()
    }

    /// ### render_value_chars
    ///
    /// Render value as a vec of chars
    pub fn render_value_chars(&self, itype: InputType) -> Vec<char> {
        match itype {
            InputType::Password(ch) | InputType::CustomPassword(ch, _, _) => {
                (0..self.input.len()).map(|_| ch).collect()
            }
            _ => self.input.clone(),
        }
    }

    /// ### get_value
    ///
    /// Get value as string
    pub fn get_value(&self) -> String {
        self.input.iter().collect()
    }
}

// -- Component

/// ## Input
///
/// Input list component
#[derive(Default)]
pub struct Input {
    props: Props,
    pub states: InputStates,
}

impl Input {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn inactive(mut self, s: Style) -> Self {
        self.attr(Attribute::FocusStyle, AttrValue::Style(s));
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

    pub fn input_type(mut self, itype: InputType) -> Self {
        self.attr(Attribute::InputType, AttrValue::InputType(itype));
        self
    }

    pub fn input_len(mut self, ilen: usize) -> Self {
        self.attr(Attribute::InputLength, AttrValue::Length(ilen));
        self
    }

    pub fn value<S: AsRef<str>>(mut self, s: S) -> Self {
        self.attr(Attribute::Value, AttrValue::String(s.as_ref().to_string()));
        self
    }

    pub fn invalid_style(mut self, s: Style) -> Self {
        self.attr(Attribute::Custom(INPUT_INVALID_STYLE), AttrValue::Style(s));
        self
    }

    pub fn placeholder<S: AsRef<str>>(mut self, placeholder: S, style: Style) -> Self {
        self.attr(
            Attribute::Custom(INPUT_PLACEHOLDER),
            AttrValue::String(placeholder.as_ref().to_string()),
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
            .map(|x| x.unwrap_length())
    }

    fn get_input_type(&self) -> InputType {
        self.props
            .get_or(Attribute::InputType, AttrValue::InputType(InputType::Text))
            .unwrap_input_type()
    }

    /// ### is_valid
    ///
    /// Checks whether current input is valid
    fn is_valid(&self) -> bool {
        let value = self.states.get_value();
        self.get_input_type().validate(value.as_str())
    }
}

impl MockComponent for Input {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let mut foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let mut background = self
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
            let itype = self.get_input_type();
            let mut block = crate::utils::get_block(borders, Some(title), focus, inactive_style);
            // Apply invalid style
            if focus && !self.is_valid() {
                if let Some(style) = self
                    .props
                    .get(Attribute::Custom(INPUT_INVALID_STYLE))
                    .map(|x| x.unwrap_style())
                {
                    let borders = self
                        .props
                        .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                        .unwrap_borders()
                        .color(style.fg.unwrap_or(Color::Reset));
                    let title = self
                        .props
                        .get_or(
                            Attribute::Title,
                            AttrValue::Title((String::default(), Alignment::Center)),
                        )
                        .unwrap_title();
                    block = crate::utils::get_block(borders, Some(title), focus, None);
                    foreground = style.fg.unwrap_or(Color::Reset);
                    background = style.bg.unwrap_or(Color::Reset);
                }
            }
            let text_to_display = self.states.render_value(self.get_input_type());
            let show_placeholder = text_to_display.is_empty();
            // Choose whether to show placeholder; if placeholder is unset, show nothing
            let text_to_display = match show_placeholder {
                true => self
                    .props
                    .get_or(
                        Attribute::Custom(INPUT_PLACEHOLDER),
                        AttrValue::String(String::new()),
                    )
                    .unwrap_string(),
                false => text_to_display,
            };
            // Choose paragraph style based on whether is valid or not and if has focus and if should show placeholder
            let paragraph_style = match focus {
                true => Style::default()
                    .fg(foreground)
                    .bg(background)
                    .add_modifier(modifiers),
                false => inactive_style.unwrap_or_default(),
            };
            let paragraph_style = match show_placeholder {
                true => self
                    .props
                    .get_or(
                        Attribute::Custom(INPUT_PLACEHOLDER_STYLE),
                        AttrValue::Style(paragraph_style),
                    )
                    .unwrap_style(),
                false => paragraph_style,
            };
            // Create widget
            let p: Paragraph = Paragraph::new(text_to_display)
                .style(paragraph_style)
                .block(block);
            render.render_widget(p, area);
            // Set cursor, if focus
            if focus {
                let x: u16 = area.x
                    + calc_utf8_cursor_position(
                        &self.states.render_value_chars(itype)[0..self.states.cursor],
                    )
                    + 1;
                render.set_cursor(x, area.y + 1);
            }
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
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
            let itype = self.get_input_type();
            let max_len = self.get_input_len();
            for ch in input.into_iter() {
                self.states.append(ch, &itype, max_len);
            }
        }
    }

    fn state(&self) -> State {
        // Validate input
        if self.is_valid() {
            State::One(StateValue::String(self.states.get_value()))
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
                if prev_input != self.states.input {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::Cancel => {
                // Delete and None
                let prev_input = self.states.input.clone();
                self.states.delete();
                if prev_input != self.states.input {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
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
                    .append(ch, &self.get_input_type(), self.get_input_len());
                // Message on change
                if prev_input != self.states.input {
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

    use super::*;

    use pretty_assertions::assert_eq;

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
        assert_eq!(states.render_value(InputType::Text).as_str(), "abc");
        assert_eq!(
            states.render_value(InputType::Password('*')).as_str(),
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
            .title("pippo", Alignment::Center)
            .value("home");
        // Verify initial state
        assert_eq!(component.states.cursor, 4);
        assert_eq!(component.states.input.len(), 4);
        // Get value
        assert_eq!(
            component.state(),
            State::One(StateValue::String(String::from("home")))
        );
        // Character
        assert_eq!(
            component.perform(Cmd::Type('/')),
            CmdResult::Changed(State::One(StateValue::String(String::from("home/"))))
        );
        assert_eq!(
            component.state(),
            State::One(StateValue::String(String::from("home/")))
        );
        assert_eq!(component.states.cursor, 5);
        // Verify max length (shouldn't push any character)
        assert_eq!(component.perform(Cmd::Type('a')), CmdResult::None);
        assert_eq!(
            component.state(),
            State::One(StateValue::String(String::from("home/")))
        );
        assert_eq!(component.states.cursor, 5);
        // Submit
        assert_eq!(
            component.perform(Cmd::Submit),
            CmdResult::Submit(State::One(StateValue::String(String::from("home/"))))
        );
        // Backspace
        assert_eq!(
            component.perform(Cmd::Delete),
            CmdResult::Changed(State::One(StateValue::String(String::from("home"))))
        );
        assert_eq!(
            component.state(),
            State::One(StateValue::String(String::from("home")))
        );
        assert_eq!(component.states.cursor, 4);
        // Check backspace at 0
        component.states.input = vec!['h'];
        component.states.cursor = 1;
        assert_eq!(
            component.perform(Cmd::Delete),
            CmdResult::Changed(State::One(StateValue::String(String::from(""))))
        );
        assert_eq!(
            component.state(),
            State::One(StateValue::String(String::from("")))
        );
        assert_eq!(component.states.cursor, 0);
        // Another one...
        assert_eq!(component.perform(Cmd::Delete), CmdResult::None);
        assert_eq!(
            component.state(),
            State::One(StateValue::String(String::from("")))
        );
        assert_eq!(component.states.cursor, 0);
        // See del behaviour here
        assert_eq!(component.perform(Cmd::Cancel), CmdResult::None);
        assert_eq!(
            component.state(),
            State::One(StateValue::String(String::from("")))
        );
        assert_eq!(component.states.cursor, 0);
        // Check del behaviour
        component.states.input = vec!['h', 'e'];
        component.states.cursor = 1;
        assert_eq!(
            component.perform(Cmd::Cancel),
            CmdResult::Changed(State::One(StateValue::String(String::from("h"))))
        );
        assert_eq!(
            component.state(),
            State::One(StateValue::String(String::from("h")))
        );
        assert_eq!(component.states.cursor, 1);
        // Another one (should do nothing)
        assert_eq!(component.perform(Cmd::Cancel), CmdResult::None);
        assert_eq!(
            component.state(),
            State::One(StateValue::String(String::from("h")))
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
            CmdResult::Changed(State::One(StateValue::String(String::from("heallo"))))
        );
        assert_eq!(
            component.state(),
            State::One(StateValue::String(String::from("heallo")))
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
            State::One(StateValue::String(String::from("new-value")))
        );
        // Invalidate input type
        component.attr(
            Attribute::InputType,
            AttrValue::InputType(InputType::Number),
        );
        assert_eq!(component.state(), State::None);
    }
}
