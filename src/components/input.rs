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
use crate::components::utils::calc_utf8_cursor_position;
use crate::event::{KeyCode, KeyModifiers};
use crate::props::{BordersProps, PropPayload, PropValue, Props, PropsBuilder, TextParts};
use crate::tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use crate::{Canvas, Component, Event, InputType, Msg, Payload, Value};

// -- Props
const PROP_VALUE: &str = "value";
const PROP_INPUT_TYPE: &str = "input_type";
const PROP_INPUT_LENGHT: &str = "input_length";

pub struct InputPropsBuilder {
    props: Option<Props>,
}

impl Default for InputPropsBuilder {
    fn default() -> Self {
        InputPropsBuilder {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for InputPropsBuilder {
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

impl From<Props> for InputPropsBuilder {
    fn from(props: Props) -> Self {
        InputPropsBuilder { props: Some(props) }
    }
}

impl InputPropsBuilder {
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

    /// ### with_input
    ///
    /// Set input type for component
    pub fn with_input(&mut self, input_type: InputType) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_INPUT_TYPE,
                PropPayload::One(PropValue::InputType(input_type)),
            );
        }
        self
    }

    /// ### with_input_len
    ///
    /// Set max input len
    pub fn with_input_len(&mut self, len: usize) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_INPUT_LENGHT, PropPayload::One(PropValue::Usize(len)));
        }
        self
    }

    /// ### with_value
    ///
    /// Set initial value for component
    pub fn with_value(&mut self, value: String) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_VALUE, PropPayload::One(PropValue::Str(value)));
        }
        self
    }
}

// -- states

#[derive(Clone)]
struct OwnStates {
    input: Vec<char>, // Current input
    cursor: usize,    // Input position
    focus: bool,      // Focus
}

impl Default for OwnStates {
    fn default() -> Self {
        OwnStates {
            input: Vec::new(),
            cursor: 0,
            focus: false,
        }
    }
}

impl OwnStates {
    /// ### append
    ///
    /// Append, if possible according to input type, the character to the input vec
    pub fn append(&mut self, ch: char, itype: InputType, max_len: Option<usize>) {
        // Check if max length has been reached
        if self.input.len() < max_len.unwrap_or(usize::MAX) {
            match itype {
                InputType::Number => {
                    if ch.is_digit(10) {
                        // Must be digit
                        self.input.insert(self.cursor, ch);
                        // Increment cursor
                        self.cursor += 1;
                    }
                }
                _ => {
                    // No rule
                    self.input.insert(self.cursor, ch);
                    // Increment cursor
                    self.cursor += 1;
                }
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
            InputType::Password => (0..self.input.len()).map(|_| '*').collect(),
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
pub struct Input {
    props: Props,
    states: OwnStates,
}

impl Input {
    /// ### new
    ///
    /// Instantiates a new Input starting from Props
    /// The method also initializes the component states.
    pub fn new(props: Props) -> Self {
        // Initialize states
        let mut states: OwnStates = OwnStates::default();
        // Input type
        // Set state value from props
        if let Some(PropPayload::One(PropValue::Str(val))) = props.own.get(PROP_VALUE) {
            for ch in val.chars() {
                states.append(
                    ch,
                    Self::get_input_type(&props),
                    Self::get_input_len(&props),
                );
            }
        }
        Input { props, states }
    }

    fn get_input_type(props: &Props) -> InputType {
        match props.own.get(PROP_INPUT_TYPE) {
            Some(PropPayload::One(PropValue::InputType(itype))) => *itype,
            _ => InputType::Text, // Default
        }
    }

    fn get_input_len(props: &Props) -> Option<usize> {
        match props.own.get(PROP_INPUT_LENGHT) {
            Some(PropPayload::One(PropValue::Usize(ilen))) => Some(*ilen),
            _ => None, // Default
        }
    }
}

impl Component for Input {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    #[cfg(not(tarpaulin_include))]
    fn render(&self, render: &mut Canvas, area: Rect) {
        if self.props.visible {
            let div: Block = super::utils::get_block(
                &self.props.borders,
                &self.props.texts.title,
                self.states.focus,
            );
            let p: Paragraph =
                Paragraph::new(self.states.render_value(Self::get_input_type(&self.props)))
                    .style(match self.states.focus {
                        true => Style::default().fg(self.props.foreground),
                        false => Style::default(),
                    })
                    .block(div);
            render.render_widget(p, area);
            // Set cursor, if focus
            if self.states.focus {
                let x: u16 = area.x
                    + calc_utf8_cursor_position(
                        &self
                            .states
                            .render_value_chars(Self::get_input_type(&self.props))
                            [0..self.states.cursor],
                    )
                    + 1;
                render.set_cursor(x, area.y + 1);
            }
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
        // Set value from props
        if let Some(PropPayload::One(PropValue::Str(val))) = self.props.own.get(PROP_VALUE) {
            let prev_input = self.states.input.clone();
            self.states.input = Vec::new();
            self.states.cursor = 0;
            for ch in val.chars() {
                self.states.append(
                    ch,
                    Self::get_input_type(&self.props),
                    Self::get_input_len(&self.props),
                );
            }
            if prev_input != self.states.input {
                Msg::OnChange(self.get_state())
            } else {
                Msg::None
            }
        } else {
            Msg::None
        }
    }

    /// ### get_props
    ///
    /// Returns a props builder starting from component properties.
    /// This returns a prop builder in order to make easier to create
    /// new properties for the element.
    fn get_props(&self) -> Props {
        // Make properties with value from states
        let mut props: Props = self.props.clone();
        props.own.insert(
            PROP_VALUE,
            PropPayload::One(PropValue::Str(self.states.get_value())),
        );
        props
    }

    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view
    fn on(&mut self, ev: Event) -> Msg {
        if let Event::Key(key) = ev {
            match key.code {
                KeyCode::Backspace => {
                    // Backspace and None
                    let prev_input = self.states.input.clone();
                    self.states.backspace();
                    if prev_input != self.states.input {
                        Msg::OnChange(self.get_state())
                    } else {
                        Msg::None
                    }
                }
                KeyCode::Delete => {
                    // Delete and None
                    let prev_input = self.states.input.clone();
                    self.states.delete();
                    if prev_input != self.states.input {
                        Msg::OnChange(self.get_state())
                    } else {
                        Msg::None
                    }
                }
                KeyCode::Enter => Msg::OnSubmit(self.get_state()),
                KeyCode::Left => {
                    // Move cursor left; msg None
                    self.states.decr_cursor();
                    Msg::None
                }
                KeyCode::Right => {
                    // Move cursor right; Msg None
                    self.states.incr_cursor();
                    Msg::None
                }
                KeyCode::End => {
                    // Cursor at last position
                    self.states.cursor_at_end();
                    Msg::None
                }
                KeyCode::Home => {
                    // Cursor at first positon
                    self.states.cursor_at_begin();
                    Msg::None
                }
                KeyCode::Char(ch) => {
                    // Check if modifiers is NOT CTRL OR ALT
                    if !key.modifiers.intersects(KeyModifiers::CONTROL)
                        && !key.modifiers.intersects(KeyModifiers::ALT)
                    {
                        // Push char to input
                        let prev_input = self.states.input.clone();
                        self.states.append(
                            ch,
                            Self::get_input_type(&self.props),
                            Self::get_input_len(&self.props),
                        );
                        // Message on change
                        if prev_input != self.states.input {
                            Msg::OnChange(self.get_state())
                        } else {
                            Msg::None
                        }
                    } else {
                        // Return key
                        Msg::OnKey(key)
                    }
                }
                _ => Msg::OnKey(key),
            }
        } else {
            Msg::None
        }
    }

    /// ### get_state
    ///
    /// Get current state from component
    /// For this component returns Unsigned if the input type is a number, otherwise a text
    /// The value is always the current input.
    fn get_state(&self) -> Payload {
        match Self::get_input_type(&self.props) {
            InputType::Number => Payload::One(Value::Usize(
                self.states.get_value().parse::<usize>().ok().unwrap_or(0),
            )),
            _ => Payload::One(Value::Str(self.states.get_value())),
        }
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
    fn active(&mut self) {
        self.states.focus = true;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::tui::style::Color;
    use crossterm::event::KeyEvent;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_input_states() {
        let mut states: OwnStates = OwnStates::default();
        states.append('a', InputType::Text, Some(3));
        assert_eq!(states.input, vec!['a']);
        states.append('b', InputType::Text, Some(3));
        assert_eq!(states.input, vec!['a', 'b']);
        states.append('c', InputType::Text, Some(3));
        assert_eq!(states.input, vec!['a', 'b', 'c']);
        // Reached length
        states.append('d', InputType::Text, Some(3));
        assert_eq!(states.input, vec!['a', 'b', 'c']);
        // Push char to numbers
        states.append('d', InputType::Number, None);
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
        assert_eq!(states.render_value(InputType::Password).as_str(), "***");
    }

    #[test]
    fn test_components_input_text() {
        // Instantiate Input with value
        let mut component: Input = Input::new(
            InputPropsBuilder::default()
                .with_input(InputType::Text)
                .visible()
                .with_input_len(5)
                .with_value(String::from("home"))
                .with_foreground(Color::Red)
                .with_background(Color::White)
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.background, Color::White);
        assert_eq!(component.props.visible, true);
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Double);
        assert_eq!(component.props.borders.color, Color::Red);
        assert_eq!(
            *component.props.own.get(PROP_VALUE).unwrap(),
            PropPayload::One(PropValue::Str(String::from("home")))
        );
        // Verify initial state
        assert_eq!(component.states.cursor, 4);
        assert_eq!(component.states.input.len(), 4);
        // Focus
        assert_eq!(component.states.focus, false);
        component.active();
        assert_eq!(component.states.focus, true);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Update
        let props = InputPropsBuilder::from(component.get_props())
            .with_foreground(Color::Red)
            .hidden()
            .build();
        assert_eq!(component.update(props), Msg::None);
        assert_eq!(component.props.visible, false);
        assert_eq!(component.props.foreground, Color::Red);
        // Get value
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("home")))
        );
        // RenderData
        //assert_eq!(component.render().unwrap().cursor, 4);
        assert_eq!(component.states.cursor, 4);
        // Handle events
        // Try key with ctrl
        assert_eq!(
            component.on(Event::Key(KeyEvent::new(
                KeyCode::Char('a'),
                KeyModifiers::CONTROL
            ))),
            Msg::OnKey(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL)),
        );
        // String shouldn't have changed
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("home")))
        );
        //assert_eq!(component.render().unwrap().cursor, 4);
        assert_eq!(component.states.cursor, 4);
        // Character
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char('/')))),
            Msg::OnChange(Payload::One(Value::Str(String::from("home/"))))
        );
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("home/")))
        );
        //assert_eq!(component.render().unwrap().cursor, 5);
        assert_eq!(component.states.cursor, 5);
        // Verify max length (shouldn't push any character)
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::None
        );
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("home/")))
        );
        //assert_eq!(component.render().unwrap().cursor, 5);
        assert_eq!(component.states.cursor, 5);
        // Enter
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Enter))),
            Msg::OnSubmit(Payload::One(Value::Str(String::from("home/"))))
        );
        // Backspace
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Backspace))),
            Msg::OnChange(Payload::One(Value::Str(String::from("home"))))
        );
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("home")))
        );
        //assert_eq!(component.render().unwrap().cursor, 4);
        assert_eq!(component.states.cursor, 4);
        // Check backspace at 0
        component.states.input = vec!['h'];
        component.states.cursor = 1;
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Backspace))),
            Msg::OnChange(Payload::One(Value::Str(String::from(""))))
        );
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("")))
        );
        //assert_eq!(component.render().unwrap().cursor, 0);
        assert_eq!(component.states.cursor, 0);
        // Another one...
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Backspace))),
            Msg::None
        );
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("")))
        );
        //assert_eq!(component.render().unwrap().cursor, 0);
        assert_eq!(component.states.cursor, 0);
        // See del behaviour here
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Delete))),
            Msg::None
        );
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("")))
        );
        //assert_eq!(component.render().unwrap().cursor, 0);
        assert_eq!(component.states.cursor, 0);
        // Check del behaviour
        component.states.input = vec!['h', 'e'];
        component.states.cursor = 1;
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Delete))),
            Msg::OnChange(Payload::One(Value::Str(String::from("h"))))
        );
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("h")))
        );
        //assert_eq!(component.render().unwrap().cursor, 1); // Shouldn't move
        assert_eq!(component.states.cursor, 1);
        // Another one (should do nothing)
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Delete))),
            Msg::None
        );
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("h")))
        );
        //assert_eq!(component.render().unwrap().cursor, 1); // Shouldn't move
        assert_eq!(component.states.cursor, 1);
        // Move cursor right
        component.states.input = vec!['h', 'e', 'l', 'l', 'o'];
        let props = InputPropsBuilder::from(component.get_props())
            .with_input_len(16)
            .hidden()
            .build();
        assert_eq!(component.update(props), Msg::None);
        component.states.cursor = 1;
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))), // between 'e' and 'l'
            Msg::None
        );
        //assert_eq!(component.render().unwrap().cursor, 2); // Should increment
        assert_eq!(component.states.cursor, 2);
        // Put a character here
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::OnChange(Payload::One(Value::Str(String::from("heallo"))))
        );
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("heallo")))
        );
        //assert_eq!(component.render().unwrap().cursor, 3);
        assert_eq!(component.states.cursor, 3);
        // Move left
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Left))),
            Msg::None
        );
        //assert_eq!(component.render().unwrap().cursor, 2); // Should decrement
        assert_eq!(component.states.cursor, 2);
        // Go at the end
        component.states.cursor = 6;
        // Move right
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::None
        );
        //assert_eq!(component.render().unwrap().cursor, 6); // Should stay
        assert_eq!(component.states.cursor, 6);
        // Move left
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Left))),
            Msg::None
        );
        //assert_eq!(component.render().unwrap().cursor, 5); // Should decrement
        assert_eq!(component.states.cursor, 5);
        // Go at the beginning
        component.states.cursor = 0;
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Left))),
            Msg::None
        );
        //assert_eq!(component.render().unwrap().cursor, 0); // Should stay
        assert_eq!(component.states.cursor, 0);
        // End - begin
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::End))),
            Msg::None
        );
        assert_eq!(component.states.cursor, 6);
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Home))),
            Msg::None
        );
        assert_eq!(component.states.cursor, 0);
        // Update value
        assert_eq!(
            component.update(
                InputPropsBuilder::from(component.get_props())
                    .with_value("new-value".to_string())
                    .build(),
            ),
            Msg::OnChange(Payload::One(Value::Str(String::from("new-value"))))
        );
        assert_eq!(
            component.update(
                InputPropsBuilder::from(component.get_props())
                    .with_value("new-value".to_string())
                    .build(),
            ),
            Msg::None // Didn't change at all
        );
        assert_eq!(
            component.get_state(),
            Payload::One(Value::Str(String::from("new-value")))
        );
        assert_eq!(component.on(Event::Resize(0, 0)), Msg::None);
    }

    #[test]
    fn test_components_input_number() {
        // Instantiate Input with value
        let mut component: Input = Input::new(
            InputPropsBuilder::default()
                .with_input(InputType::Number)
                .with_input_len(5)
                .with_value(String::from("3000"))
                .build(),
        );
        // Verify initial state
        assert_eq!(component.states.cursor, 4);
        assert_eq!(component.states.input.len(), 4);
        // Push a non numeric value
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::None
        );
        assert_eq!(component.get_state(), Payload::One(Value::Usize(3000)));
        //assert_eq!(component.render().unwrap().cursor, 4);
        assert_eq!(component.states.cursor, 4);
        // Push a number
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char('1')))),
            Msg::OnChange(Payload::One(Value::Usize(30001)))
        );
        assert_eq!(component.get_state(), Payload::One(Value::Usize(30001)));
        //assert_eq!(component.render().unwrap().cursor, 5);
        assert_eq!(component.states.cursor, 5);
    }
}
