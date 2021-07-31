//! ## Checkbox
//!
//! `Checkbox` component renders a checkbox group

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
use crate::event::KeyCode;
use crate::props::{BordersProps, PropPayload, PropValue, Props, PropsBuilder};
use crate::tui::{
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Tabs},
};
use crate::{Component, Event, Frame, Msg, Payload, Value};

const PROP_CHOICES: &str = "choices";
const PROP_SELECTED: &str = "selected";
const PROP_TITLE: &str = "title";

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
    /// Set options for radio group
    pub fn with_options<S: AsRef<str>>(&mut self, options: &[S]) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_CHOICES,
                PropPayload::Vec(
                    options
                        .iter()
                        .map(|x| PropValue::Str(x.as_ref().to_string()))
                        .collect(),
                ),
            );
        }
        self
    }

    /// ### with_title
    ///
    /// Set title
    pub fn with_title<S: AsRef<str>>(&mut self, title: S) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_TITLE,
                PropPayload::One(PropValue::Str(title.as_ref().to_string())),
            );
        }
        self
    }

    /// ### with_value
    ///
    /// Set selected choices
    pub fn with_value(&mut self, choices: Vec<usize>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_SELECTED,
                PropPayload::Vec(choices.into_iter().map(PropValue::Usize).collect()),
            );
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

    /// ### set_choices
    ///
    /// Set OwnStates choices from a vector of str
    /// In addition resets current selection and keep index if possible or set it to the first value
    /// available
    pub fn set_choices(&mut self, choices: &[&str]) {
        self.choices = choices.iter().map(|x| x.to_string()).collect();
        // Clear selection
        self.selection.clear();
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
        let choices: Vec<&str> = match props.own.get(PROP_CHOICES).as_ref() {
            Some(PropPayload::Vec(choices)) => choices
                .iter()
                .map(|x| match x {
                    PropValue::Str(s) => s.as_str(),
                    _ => panic!("Choices is not a vec of Str"),
                })
                .collect(),
            _ => Vec::new(),
        };
        states.set_choices(&choices);
        // Get value
        if let Some(PropPayload::Vec(choices)) = &props.own.get(PROP_SELECTED) {
            states.selection = choices
                .clone()
                .iter()
                .map(|x| match x {
                    PropValue::Usize(u) => *u,
                    _ => 0,
                })
                .collect();
        }
        Checkbox { props, states }
    }
}

impl Component for Checkbox {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    fn render(&self, render: &mut Frame, area: Rect) {
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
            let title: Option<&str> = match self.props.own.get(PROP_TITLE).as_ref() {
                Some(PropPayload::One(PropValue::Str(t))) => Some(t),
                _ => None,
            };
            let block: Block =
                super::utils::get_block(&self.props.borders, title, self.states.focus);
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
        let choices: Vec<&str> = match props.own.get(PROP_CHOICES).as_ref() {
            Some(PropPayload::Vec(choices)) => choices
                .iter()
                .map(|x| match x {
                    PropValue::Str(s) => s.as_str(),
                    _ => panic!("Choices is not a vec of Str"),
                })
                .collect(),
            _ => Vec::new(),
        };
        self.states.set_choices(&choices);
        // Get value
        if let Some(PropPayload::Vec(choices)) = &props.own.get(PROP_SELECTED) {
            self.states.selection = choices
                .clone()
                .iter()
                .map(|x| match x {
                    PropValue::Usize(u) => *u,
                    _ => 0,
                })
                .collect();
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
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_components_checkbox_states() {
        let mut states: OwnStates = OwnStates::default();
        assert_eq!(states.choice, 0);
        assert_eq!(states.choices.len(), 0);
        assert_eq!(states.selection.len(), 0);
        let choices: &[&str] = &["lemon", "strawberry", "vanilla", "chocolate"];
        states.set_choices(choices);
        assert_eq!(states.choice, 0);
        assert_eq!(states.choices.len(), 4);
        assert_eq!(states.selection.len(), 0);
        // Select
        states.toggle();
        assert_eq!(states.selection, vec![0]);
        // Move
        states.prev_choice();
        assert_eq!(states.choice, 0);
        states.next_choice();
        assert_eq!(states.choice, 1);
        states.next_choice();
        assert_eq!(states.choice, 2);
        states.toggle();
        assert_eq!(states.selection, vec![0, 2]);
        // Forward overflow
        states.next_choice();
        states.next_choice();
        assert_eq!(states.choice, 3);
        states.prev_choice();
        assert_eq!(states.choice, 2);
        states.toggle();
        assert_eq!(states.selection, vec![0]);
        // has
        assert_eq!(states.has(0), true);
        assert_ne!(states.has(2), true);
        // Update
        let choices: &[&str] = &["lemon", "strawberry"];
        states.set_choices(choices);
        assert_eq!(states.choice, 1); // Move to first index available
        assert_eq!(states.choices.len(), 2);
        assert_eq!(states.selection.len(), 0);
        let choices: &[&str] = &[];
        states.set_choices(choices);
        assert_eq!(states.choice, 0); // Move to first index available
        assert_eq!(states.choices.len(), 0);
        assert_eq!(states.selection.len(), 0);
    }

    #[test]
    fn test_components_checkbox() {
        // Make component
        let mut component: Checkbox = Checkbox::new(
            CheckboxPropsBuilder::default()
                .with_title("Which food do you prefer?")
                .with_options(&["Pizza", "Hummus", "Ramen", "Gyoza", "Pasta", "Falafel"])
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
        assert_eq!(
            *component.props.own.get(PROP_SELECTED).unwrap(),
            PropPayload::Vec(vec![PropValue::Usize(1), PropValue::Usize(5)])
        );
        assert_eq!(
            component.props.own.get(PROP_TITLE).unwrap(),
            &PropPayload::One(PropValue::Str("Which food do you prefer?".to_string()))
        );
        assert_eq!(
            component.props.own.get(PROP_CHOICES).unwrap(),
            &PropPayload::Vec(vec![
                PropValue::Str(String::from("Pizza")),
                PropValue::Str(String::from("Hummus")),
                PropValue::Str(String::from("Ramen")),
                PropValue::Str(String::from("Gyoza")),
                PropValue::Str(String::from("Pasta")),
                PropValue::Str(String::from("Falafel"))
            ])
        );
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
        assert_eq!(component.on(Event::Resize(0, 0)), Msg::None);
    }
}
