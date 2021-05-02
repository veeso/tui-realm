//! ## Checkbox
//!
//! `Checkbox` component renders a checkbox group

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
use crate::props::{BordersProps, PropValue, Props, PropsBuilder, TextParts, TextSpan};
use crate::tui::{
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Tabs},
};
use crate::{Canvas, Component, Event, Msg, Payload, Value};

// -- Props

pub struct CheckboxPropsBuilder {
    props: Option<Props>,
}

impl Default for CheckboxPropsBuilder {
    fn default() -> Self {
        let mut builder = CheckboxPropsBuilder {
            props: Some(Props::default()),
        };
        builder.with_inverted_color(Color::Black);
        builder
    }
}

impl PropsBuilder for CheckboxPropsBuilder {
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

impl From<Props> for CheckboxPropsBuilder {
    fn from(props: Props) -> Self {
        CheckboxPropsBuilder { props: Some(props) }
    }
}

impl CheckboxPropsBuilder {
    /// ### with_color
    ///
    /// Set Checkbox group color
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
                Some(options.into_iter().map(|x| TextSpan::from(x)).collect()), // Make textSpan from Strings
            );
        }
        self
    }

    /// ### with_value
    ///
    /// Set initial value for choice
    pub fn with_value(&mut self, choices: Vec<usize>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.value = PropValue::VecOfUsize(choices);
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
    choice: usize,         // Selected option
    choices: Vec<String>,  // Available choices
    selection: Vec<usize>, // Selected options
    focus: bool,           // has focus?
}

impl Default for OwnStates {
    fn default() -> Self {
        OwnStates {
            choice: 0,
            choices: Vec::new(),
            selection: Vec::new(),
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

    /// ### toggle
    ///
    /// Check or uncheck the option
    pub fn toggle(&mut self) {
        let option = self.choice;
        if self.selection.contains(&option) {
            let target_index = self.selection.iter().position(|x| *x == option).unwrap();
            self.selection.remove(target_index);
        } else {
            self.selection.push(option);
        }
    }

    /// ### has
    ///
    /// Returns whether selection contains option
    pub fn has(&self, option: usize) -> bool {
        self.selection.contains(&option)
    }

    /// ### make_choices
    ///
    /// Set OwnStates choices from a vector of text spans
    pub fn make_choices(&mut self, spans: &[TextSpan]) {
        self.choices = spans.iter().map(|x| x.content.clone()).collect();
    }
}

// -- component

/// ## Checkbox
///
/// Checkbox component represents a group of tabs to select from
pub struct Checkbox {
    props: Props,
    states: OwnStates,
}

impl Checkbox {
    /// ### new
    ///
    /// Instantiate a new Checkbox Group component
    pub fn new(props: Props) -> Self {
        // Make states
        let mut states: OwnStates = OwnStates::default();
        // Update choices (vec of TextSpan to String)
        states.make_choices(props.texts.spans.as_ref().unwrap_or(&Vec::new()));
        // Get value
        if let PropValue::VecOfUsize(choices) = &props.value {
            states.selection = choices.clone();
        }
        Checkbox { props, states }
    }
}

impl Component for Checkbox {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    #[cfg(not(tarpaulin_include))]
    fn render(&self, render: &mut Canvas, area: Rect) {
        if self.props.visible {
            // Make colors
            let (bg, fg, block_color): (Color, Color, Color) = match &self.states.focus {
                true => (
                    self.props.foreground,
                    self.props.background,
                    self.props.foreground,
                ),
                false => (Color::Reset, self.props.foreground, Color::Reset),
            };
            // Make choices
            let choices: Vec<Spans> = self
                .states
                .choices
                .iter()
                .enumerate()
                .map(|(idx, x)| {
                    let checkbox: &str = match self.states.has(idx) {
                        true => "☑ ",
                        false => "☐ ",
                    };
                    let (fg, bg) = match self.states.focus {
                        true => match self.states.choice == idx {
                            true => (fg, bg),
                            false => (bg, fg),
                        },
                        false => (fg, bg),
                    };
                    // Make spans
                    Spans::from(vec![
                        Span::styled(checkbox, Style::default().fg(fg).bg(bg)),
                        Span::styled(x.to_string(), Style::default().fg(fg).bg(bg)),
                    ])
                })
                .collect();
            let block: Block = super::utils::get_block(
                &self.props.borders,
                &self.props.texts.title,
                self.states.focus,
            );
            let checkbox: Tabs = Tabs::new(choices)
                .block(block)
                .select(self.states.choice)
                .style(Style::default().fg(block_color));
            render.render_widget(checkbox, area);
        }
    }

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update.
    /// Returns a Msg to the view
    fn update(&mut self, props: Props) -> Msg {
        let prev_selection = self.states.selection.clone();
        // Reset choices
        self.states
            .make_choices(props.texts.spans.as_ref().unwrap_or(&Vec::new()));
        // Get value
        self.states.selection.clear();
        if let PropValue::VecOfUsize(choices) = &props.value {
            self.states.selection = choices.clone();
        }
        self.props = props;
        // Msg none
        if prev_selection != self.states.selection {
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
                    Msg::None
                }
                KeyCode::Left => {
                    // Decrement choice
                    self.states.prev_choice();
                    // Return Msg On Change
                    Msg::None
                }
                KeyCode::Char(' ') => {
                    // Select index
                    self.states.toggle();
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
    /// For this component returns the vec of selected items
    fn get_state(&self) -> Payload {
        Payload::Vec(
            self.states
                .selection
                .iter()
                .map(|x| Value::Usize(*x))
                .collect(),
        )
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

    #[test]
    fn test_components_checkbox() {
        // Make component
        let mut component: Checkbox = Checkbox::new(
            CheckboxPropsBuilder::default()
                .with_options(
                    Some(String::from("Which food do you like?")),
                    vec![
                        String::from("Pizza"),
                        String::from("Hummus"),
                        String::from("Ramen"),
                        String::from("Gyoza"),
                        String::from("Pasta"),
                        String::from("Falafel"),
                    ],
                )
                .visible()
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .with_color(Color::Red)
                .with_inverted_color(Color::White)
                .with_value(vec![1, 5])
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.background, Color::White);
        assert_eq!(component.props.visible, true);
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Double);
        assert_eq!(component.props.borders.color, Color::Red);
        assert_eq!(component.props.value, PropValue::VecOfUsize(vec![1, 5]));
        // Verify states
        assert_eq!(component.states.choice, 0);
        assert_eq!(component.states.selection, vec![1, 5]);
        assert_eq!(component.states.choices.len(), 6);
        // Focus
        assert_eq!(component.states.focus, false);
        component.active();
        assert_eq!(component.states.focus, true);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Update
        let props = CheckboxPropsBuilder::from(component.get_props())
            .with_color(Color::Yellow)
            .hidden()
            .build();
        assert_eq!(component.update(props), Msg::None);
        assert_eq!(component.props.visible, false);
        assert_eq!(component.props.foreground, Color::Yellow);
        let props = CheckboxPropsBuilder::from(component.get_props())
            .with_value(vec![1])
            .hidden()
            .build();
        assert_eq!(
            component.update(props),
            Msg::OnChange(Payload::Vec(vec![Value::Usize(1)]))
        );
        // Get value
        assert_eq!(component.get_state(), Payload::Vec(vec![Value::Usize(1)]));
        // Handle events
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Left))),
            Msg::None,
        );
        assert_eq!(component.get_state(), Payload::Vec(vec![Value::Usize(1)]));
        // Toggle
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char(' ')))),
            Msg::OnChange(Payload::Vec(vec![Value::Usize(1), Value::Usize(0)]))
        );
        // Left again
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Left))),
            Msg::None,
        );
        assert_eq!(component.states.choice, 0);
        // Right
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::None,
        );
        // Toggle
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char(' ')))),
            Msg::OnChange(Payload::Vec(vec![Value::Usize(0)]))
        );
        // Right again
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::None,
        );
        assert_eq!(component.states.choice, 2);
        // Right again
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::None,
        );
        assert_eq!(component.states.choice, 3);
        // Right again
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::None,
        );
        assert_eq!(component.states.choice, 4);
        // Right again
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::None,
        );
        assert_eq!(component.states.choice, 5);
        // Right again
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Right))),
            Msg::None,
        );
        assert_eq!(component.states.choice, 5);
        // Submit
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Enter))),
            Msg::OnSubmit(Payload::Vec(vec![Value::Usize(0)])),
        );
        // Any key
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::OnKey(KeyEvent::from(KeyCode::Char('a'))),
        );
    }
}
