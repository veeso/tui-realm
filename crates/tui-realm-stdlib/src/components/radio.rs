//! ## Radio
//!
//! `Radio` component renders a radio group

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
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, Style,
    TextModifiers,
};
use tuirealm::ratatui::text::Line as Spans;
use tuirealm::ratatui::{layout::Rect, widgets::Tabs};
use tuirealm::{Frame, MockComponent, State, StateValue};

// -- states

/// ## RadioStates
///
/// RadioStates contains states for this component
#[derive(Default)]
pub struct RadioStates {
    pub choice: usize,        // Selected option
    pub choices: Vec<String>, // Available choices
}

impl RadioStates {
    /// ### next_choice
    ///
    /// Move choice index to next choice
    pub fn next_choice(&mut self, rewind: bool) {
        if rewind && self.choice + 1 >= self.choices.len() {
            self.choice = 0;
        } else if self.choice + 1 < self.choices.len() {
            self.choice += 1;
        }
    }

    /// ### prev_choice
    ///
    /// Move choice index to previous choice
    pub fn prev_choice(&mut self, rewind: bool) {
        if rewind && self.choice == 0 && !self.choices.is_empty() {
            self.choice = self.choices.len() - 1;
        } else if self.choice > 0 {
            self.choice -= 1;
        }
    }

    /// ### set_choices
    ///
    /// Set RadioStates choices from a vector of text spans
    /// In addition resets current selection and keep index if possible or set it to the first value
    /// available
    pub fn set_choices(&mut self, choices: impl Into<Vec<String>>) {
        self.choices = choices.into();
        // Keep index if possible
        if self.choice >= self.choices.len() {
            self.choice = match self.choices.len() {
                0 => 0,
                l => l - 1,
            };
        }
    }

    pub fn select(&mut self, i: usize) {
        if i < self.choices.len() {
            self.choice = i;
        }
    }
}

// -- component

/// ## Radio
///
/// Radio component represents a group of tabs to select from
#[derive(Default)]
#[must_use]
pub struct Radio {
    props: Props,
    pub states: RadioStates,
}

impl Radio {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
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

    pub fn choices<S: Into<String>>(mut self, choices: impl IntoIterator<Item = S>) -> Self {
        self.attr(
            Attribute::Content,
            AttrValue::Payload(PropPayload::Vec(
                choices
                    .into_iter()
                    .map(|v| PropValue::Str(v.into()))
                    .collect(),
            )),
        );
        self
    }

    pub fn value(mut self, i: usize) -> Self {
        // Set state
        self.attr(
            Attribute::Value,
            AttrValue::Payload(PropPayload::One(PropValue::Usize(i))),
        );
        self
    }

    fn is_rewind(&self) -> bool {
        self.props
            .get_or(Attribute::Rewind, AttrValue::Flag(false))
            .unwrap_flag()
    }
}

impl MockComponent for Radio {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Make choices
            let choices: Vec<Spans> = self
                .states
                .choices
                .iter()
                .map(|x| Spans::from(x.as_str()))
                .collect();
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let title = self
                .props
                .get_ref(Attribute::Title)
                .and_then(|x| x.as_title());
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            let inactive_style = self
                .props
                .get(Attribute::FocusStyle)
                .map(|x| x.unwrap_style());
            let div = crate::utils::get_block(borders, title, focus, inactive_style);
            // Make colors
            let (fg, block_color): (Color, Color) = if focus {
                (foreground, foreground)
            } else {
                (foreground, Color::Reset)
            };
            let modifiers = if focus {
                TextModifiers::REVERSED
            } else {
                TextModifiers::empty()
            };
            let radio: Tabs = Tabs::new(choices)
                .block(div)
                .select(self.states.choice)
                .style(Style::default().fg(block_color).bg(background))
                .highlight_style(Style::default().fg(fg).add_modifier(modifiers));
            render.render_widget(radio, area);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        match attr {
            Attribute::Content => {
                // Reset choices
                let choices: Vec<String> = value
                    .unwrap_payload()
                    .unwrap_vec()
                    .iter()
                    .map(|x| x.clone().unwrap_str())
                    .collect();
                self.states.set_choices(choices);
            }
            Attribute::Value => {
                self.states
                    .select(value.unwrap_payload().unwrap_one().unwrap_usize());
            }
            attr => {
                self.props.set(attr, value);
            }
        }
    }

    fn state(&self) -> State {
        State::One(StateValue::Usize(self.states.choice))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Right) => {
                // Increment choice
                self.states.next_choice(self.is_rewind());
                // Return CmdResult On Change
                CmdResult::Changed(self.state())
            }
            Cmd::Move(Direction::Left) => {
                // Decrement choice
                self.states.prev_choice(self.is_rewind());
                // Return CmdResult On Change
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                // Return Submit
                CmdResult::Submit(self.state())
            }
            _ => CmdResult::None,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use tuirealm::props::{PropPayload, PropValue};

    #[test]
    fn test_components_radio_states() {
        let mut states: RadioStates = RadioStates::default();
        assert_eq!(states.choice, 0);
        assert_eq!(states.choices.len(), 0);
        let choices: &[String] = &[
            "lemon".to_string(),
            "strawberry".to_string(),
            "vanilla".to_string(),
            "chocolate".to_string(),
        ];
        states.set_choices(choices);
        assert_eq!(states.choice, 0);
        assert_eq!(states.choices.len(), 4);
        // Move
        states.prev_choice(false);
        assert_eq!(states.choice, 0);
        states.next_choice(false);
        assert_eq!(states.choice, 1);
        states.next_choice(false);
        assert_eq!(states.choice, 2);
        // Forward overflow
        states.next_choice(false);
        states.next_choice(false);
        assert_eq!(states.choice, 3);
        states.prev_choice(false);
        assert_eq!(states.choice, 2);
        // Update
        let choices: &[String] = &["lemon".to_string(), "strawberry".to_string()];
        states.set_choices(choices);
        assert_eq!(states.choice, 1); // Move to first index available
        assert_eq!(states.choices.len(), 2);
        let choices: &[String] = &[];
        states.set_choices(choices);
        assert_eq!(states.choice, 0); // Move to first index available
        assert_eq!(states.choices.len(), 0);
        // Rewind
        let choices: &[String] = &[
            "lemon".to_string(),
            "strawberry".to_string(),
            "vanilla".to_string(),
            "chocolate".to_string(),
        ];
        states.set_choices(choices);
        assert_eq!(states.choice, 0);
        states.prev_choice(true);
        assert_eq!(states.choice, 3);
        states.next_choice(true);
        assert_eq!(states.choice, 0);
        states.next_choice(true);
        assert_eq!(states.choice, 1);
        states.prev_choice(true);
        assert_eq!(states.choice, 0);
    }

    #[test]
    fn test_components_radio() {
        // Make component
        let mut component = Radio::default()
            .background(Color::Blue)
            .foreground(Color::Red)
            .borders(Borders::default())
            .title("C'est oui ou bien c'est non?", Alignment::Center)
            .choices(["Oui!", "Non", "Peut-être"])
            .value(1)
            .rewind(false);
        // Verify states
        assert_eq!(component.states.choice, 1);
        assert_eq!(component.states.choices.len(), 3);
        component.attr(
            Attribute::Value,
            AttrValue::Payload(PropPayload::One(PropValue::Usize(2))),
        );
        assert_eq!(component.state(), State::One(StateValue::Usize(2)));
        // Get value
        component.states.choice = 1;
        assert_eq!(component.state(), State::One(StateValue::Usize(1)));
        // Handle events
        assert_eq!(
            component.perform(Cmd::Move(Direction::Left)),
            CmdResult::Changed(State::One(StateValue::Usize(0))),
        );
        assert_eq!(component.state(), State::One(StateValue::Usize(0)));
        // Left again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Left)),
            CmdResult::Changed(State::One(StateValue::Usize(0))),
        );
        assert_eq!(component.state(), State::One(StateValue::Usize(0)));
        // Right
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::Changed(State::One(StateValue::Usize(1))),
        );
        assert_eq!(component.state(), State::One(StateValue::Usize(1)));
        // Right again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::Changed(State::One(StateValue::Usize(2))),
        );
        assert_eq!(component.state(), State::One(StateValue::Usize(2)));
        // Right again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::Changed(State::One(StateValue::Usize(2))),
        );
        assert_eq!(component.state(), State::One(StateValue::Usize(2)));
        // Submit
        assert_eq!(
            component.perform(Cmd::Submit),
            CmdResult::Submit(State::One(StateValue::Usize(2))),
        );
    }

    #[test]
    fn various_set_choice_types() {
        // static array of strings
        RadioStates::default().set_choices(&["hello".to_string()]);
        // vector of strings
        RadioStates::default().set_choices(vec!["hello".to_string()]);
        // boxed array of strings
        RadioStates::default().set_choices(vec!["hello".to_string()].into_boxed_slice());
    }

    #[test]
    fn various_choice_types() {
        // static array of static strings
        let _ = Radio::default().choices(["hello"]);
        // static array of strings
        let _ = Radio::default().choices(["hello".to_string()]);
        // vec of static strings
        let _ = Radio::default().choices(vec!["hello"]);
        // vec of strings
        let _ = Radio::default().choices(vec!["hello".to_string()]);
        // boxed array of static strings
        let _ = Radio::default().choices(vec!["hello"].into_boxed_slice());
        // boxed array of strings
        let _ = Radio::default().choices(vec!["hello".to_string()].into_boxed_slice());
    }
}
