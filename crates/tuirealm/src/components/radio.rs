//! ## Radio
//!
//! `Radio` component renders a radio group

use crate::event::KeyCode;
/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
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
use crate::props::{
    BordersProps, PropPayload, PropValue, Props, PropsBuilder, TextParts, TextSpan,
};
use crate::tui::{
    layout::Rect,
    style::{Color, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, Tabs},
};
use crate::{Canvas, Component, Event, Msg, Payload, Value};

// -- Props

pub struct RadioPropsBuilder {
    props: Option<Props>,
}

impl Default for RadioPropsBuilder {
    fn default() -> Self {
        let mut builder = RadioPropsBuilder {
            props: Some(Props::default()),
        };
        builder.with_inverted_color(Color::Black);
        builder
    }
}

impl PropsBuilder for RadioPropsBuilder {
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

impl From<Props> for RadioPropsBuilder {
    fn from(props: Props) -> Self {
        RadioPropsBuilder { props: Some(props) }
    }
}

impl RadioPropsBuilder {
    /// ### with_color
    ///
    /// Set radio group color
    pub fn with_color(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    /// ### with_inverted_color
    ///
    /// Set inverted color (black is default)
    pub fn with_inverted_color(&mut self, color: Color) -> &mut Self {
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

    /// ### with_options
    ///
    /// Set options and label
    /// If label is None, no block will be rendered
    pub fn with_options(&mut self, label: Option<String>, options: Vec<String>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.texts = TextParts::new(
                label,
                Some(options.into_iter().map(|x| TextSpan::from(x)).collect()), // Make textSpan from Strings);
            )
        }
        self
    }

    /// ### with_value
    ///
    /// Set initial value for choice
    pub fn with_value(&mut self, index: usize) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.value = PropPayload::One(PropValue::Usize(index));
        }
        self
    }
}

// -- states

/// ## OwnStates
///
/// OwnStates contains states for this component
#[derive(Clone)]
struct OwnStates {
    choice: usize,        // Selected option
    choices: Vec<String>, // Available choices
    focus: bool,          // has focus?
}

impl Default for OwnStates {
    fn default() -> Self {
        OwnStates {
            choice: 0,
            choices: Vec::new(),
            focus: false,
        }
    }
}

impl OwnStates {
    /// ### next_choice
    ///
    /// Move choice index to next choice
    pub fn next_choice(&mut self) {
        if self.choice + 1 < self.choices.len() {
            self.choice += 1;
        }
    }

    /// ### prev_choice
    ///
    /// Move choice index to previous choice
    pub fn prev_choice(&mut self) {
        if self.choice > 0 {
            self.choice -= 1;
        }
    }

    /// ### set_choices
    ///
    /// Set OwnStates choices from a vector of text spans
    /// In addition resets current selection and keep index if possible or set it to the first value
    /// available
    pub fn set_choices(&mut self, spans: &[TextSpan]) {
        self.choices = spans.iter().map(|x| x.content.clone()).collect();
        // Keep index if possible
        if self.choice >= self.choices.len() {
            self.choice = match self.choices.len() {
                0 => 0,
                l => l - 1,
            };
        }
    }
}

// -- component

/// ## Radio
///
/// Radio component represents a group of tabs to select from
pub struct Radio {
    props: Props,
    states: OwnStates,
}

impl Radio {
    /// ### new
    ///
    /// Instantiate a new Radio Group component
    pub fn new(props: Props) -> Self {
        // Make states
        let mut states: OwnStates = OwnStates::default();
        // Update choices (vec of TextSpan to String)
        states.set_choices(props.texts.spans.as_ref().unwrap_or(&Vec::new()));
        // Get value
        if let PropPayload::One(PropValue::Usize(choice)) = props.value {
            states.choice = choice;
        }
        Radio { props, states }
    }
}

impl Component for Radio {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    #[cfg(not(tarpaulin_include))]
    fn render(&self, render: &mut Canvas, area: Rect) {
        if self.props.visible {
            // Make choices
            let choices: Vec<Spans> = self
                .states
                .choices
                .iter()
                .map(|x| Spans::from(x.clone()))
                .collect();
            // Make colors
            let (bg, fg, block_color): (Color, Color, Color) = match &self.states.focus {
                true => (
                    self.props.foreground,
                    self.props.background,
                    self.props.foreground,
                ),
                false => (Color::Reset, self.props.foreground, Color::Reset),
            };
            let block: Block = super::utils::get_block(
                &self.props.borders,
                &self.props.texts.title,
                self.states.focus,
            );
            let radio: Tabs = Tabs::new(choices)
                .block(block)
                .select(self.states.choice)
                .style(Style::default().fg(block_color))
                .highlight_style(Style::default().fg(fg).bg(bg));
            render.render_widget(radio, area);
        }
    }

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update.
    /// Returns a Msg to the view
    fn update(&mut self, props: Props) -> Msg {
        let prev_index: usize = self.states.choice;
        // Reset choices
        self.states
            .set_choices(props.texts.spans.as_ref().unwrap_or(&Vec::new()));
        // Get value
        if let PropPayload::One(PropValue::Usize(choice)) = props.value {
            self.states.choice = choice;
        }
        self.props = props;
        // Msg none
        if prev_index != self.states.choice {
            Msg::OnChange(self.get_state())
        } else {
            Msg::None
        }
    }

    /// ### get_props
    ///
    /// Returns a copy of the component properties.
    fn get_props(&self) -> Props {
        self.props.clone()
    }

    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view.
    fn on(&mut self, ev: Event) -> Msg {
        // Match event
        if let Event::Key(key) = ev {
            match key.code {
                KeyCode::Right => {
                    // Increment choice
                    self.states.next_choice();
                    // Return Msg On Change
                    Msg::OnChange(self.get_state())
                }
                KeyCode::Left => {
                    // Decrement choice
                    self.states.prev_choice();
                    // Return Msg On Change
                    Msg::OnChange(self.get_state())
                }
                KeyCode::Enter => {
                    // Return Submit
                    Msg::OnSubmit(self.get_state())
                }
                _ => {
                    // Return key event to activity
                    Msg::OnKey(key)
                }
            }
        } else {
            // Ignore event
            Msg::None
        }
    }

    /// ### get_state
    ///
    /// Get current state from component
    /// For this component returns the index of the selected choice
    fn get_state(&self) -> Payload {
        Payload::One(Value::Usize(self.states.choice))
    }

    // -- events

    /// ### blur
    ///
    /// Blur component
    fn blur(&mut self) {
        self.states.focus = false;
    }

    /// ### active
    ///
    /// Active component
    fn active(&mut self) {
        self.states.focus = true;
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crossterm::event::{KeyCode, KeyEvent};

    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_radio_states() {
        let mut states: OwnStates = OwnStates::default();
        assert_eq!(states.choice, 0);
        assert_eq!(states.choices.len(), 0);
        let choices: Vec<TextSpan> = vec![
            TextSpan::from("lemon"),
            TextSpan::from("strawberry"),
            TextSpan::from("vanilla"),
            TextSpan::from("chocolate"),
        ];
        states.set_choices(&choices);
        assert_eq!(states.choice, 0);
        assert_eq!(states.choices.len(), 4);
        // Move
        states.prev_choice();
        assert_eq!(states.choice, 0);
        states.next_choice();
        assert_eq!(states.choice, 1);
        states.next_choice();
        assert_eq!(states.choice, 2);
        // Forward overflow
        states.next_choice();
        states.next_choice();
        assert_eq!(states.choice, 3);
        states.prev_choice();
        assert_eq!(states.choice, 2);
        // Update
        let choices: Vec<TextSpan> = vec![TextSpan::from("lemon"), TextSpan::from("strawberry")];
        states.set_choices(&choices);
        assert_eq!(states.choice, 1); // Move to first index available
        assert_eq!(states.choices.len(), 2);
        let choices: Vec<TextSpan> = vec![];
        states.set_choices(&choices);
        assert_eq!(states.choice, 0); // Move to first index available
        assert_eq!(states.choices.len(), 0);
    }

    #[test]
    fn test_components_radio() {
        // Make component
        let mut component: Radio = Radio::new(
            RadioPropsBuilder::default()
                .hidden()
                .visible()
                .with_color(Color::Red)
                .with_inverted_color(Color::Blue)
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .with_options(
                    Some(String::from("C'est oui ou bien c'est non?")),
                    vec![
                        String::from("Oui!"),
                        String::from("Non"),
                        String::from("Peut-être"),
                    ],
                )
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .with_value(1)
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.background, Color::Blue);
        assert_eq!(component.props.visible, true);
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Double);
        assert_eq!(component.props.borders.color, Color::Red);
        assert_eq!(
            component.props.texts.title.as_ref().unwrap().as_str(),
            "C'est oui ou bien c'est non?"
        );
        assert_eq!(component.props.texts.spans.as_ref().unwrap().len(), 3);
        assert_eq!(component.props.value, PropPayload::One(PropValue::Usize(1)));
        // Verify states
        assert_eq!(component.states.choice, 1);
        assert_eq!(component.states.choices.len(), 3);
        // Focus
        assert_eq!(component.states.focus, false);
        component.active();
        assert_eq!(component.states.focus, true);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Update
        let props = RadioPropsBuilder::from(component.get_props())
            .with_color(Color::Red)
            .hidden()
            .build();
        assert_eq!(component.update(props), Msg::None);
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.visible, false);
        let props = RadioPropsBuilder::from(component.get_props())
            .with_value(2)
            .hidden()
            .build();
        assert_eq!(
            component.update(props),
            Msg::OnChange(Payload::One(Value::Usize(2)))
        );
        // Get value
        component.states.choice = 1;
        assert_eq!(component.get_state(), Payload::One(Value::Usize(1)));
        // Handle events
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Left))),
            Msg::OnChange(Payload::One(Value::Usize(0))),
        );
        assert_eq!(component.get_state(), Payload::One(Value::Usize(0)));
        // Left again
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Left))),
            Msg::OnChange(Payload::One(Value::Usize(0))),
        );
        assert_eq!(component.get_state(), Payload::One(Value::Usize(0)));
        // Right
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::OnChange(Payload::One(Value::Usize(1))),
        );
        assert_eq!(component.get_state(), Payload::One(Value::Usize(1)));
        // Right again
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::OnChange(Payload::One(Value::Usize(2))),
        );
        assert_eq!(component.get_state(), Payload::One(Value::Usize(2)));
        // Right again
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::OnChange(Payload::One(Value::Usize(2))),
        );
        assert_eq!(component.get_state(), Payload::One(Value::Usize(2)));
        // Submit
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Enter))),
            Msg::OnSubmit(Payload::One(Value::Usize(2))),
        );
        // Any key
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::OnKey(KeyEvent::from(KeyCode::Char('a'))),
        );
    }
}
