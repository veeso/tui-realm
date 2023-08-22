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
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, Style,
};
#[cfg(feature = "ratatui")]
use tuirealm::tui::text::Line as Spans;
#[cfg(feature = "tui")]
use tuirealm::tui::text::Spans;
use tuirealm::tui::{layout::Rect, text::Span, widgets::Tabs};
use tuirealm::{Frame, MockComponent, State, StateValue};

// -- states

/// ## CheckboxStates
///
/// CheckboxStates contains states for this component
#[derive(Default)]
pub struct CheckboxStates {
    pub choice: usize,         // Selected option
    pub choices: Vec<String>,  // Available choices
    pub selection: Vec<usize>, // Selected options
}

impl CheckboxStates {
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

    pub fn select(&mut self, i: usize) {
        if i < self.choices.len() && !self.selection.contains(&i) {
            self.selection.push(i);
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
    /// Set CheckboxStates choices from a vector of str
    /// In addition resets current selection and keep index if possible or set it to the first value
    /// available
    pub fn set_choices(&mut self, choices: &[String]) {
        self.choices = choices.to_vec();
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
#[derive(Default)]
pub struct Checkbox {
    props: Props,
    pub states: CheckboxStates,
}

impl Checkbox {
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

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
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

    pub fn choices<S: AsRef<str>>(mut self, choices: &[S]) -> Self {
        self.attr(
            Attribute::Content,
            AttrValue::Payload(PropPayload::Vec(
                choices
                    .iter()
                    .map(|x| PropValue::Str(x.as_ref().to_string()))
                    .collect(),
            )),
        );
        self
    }

    pub fn values(mut self, selected: &[usize]) -> Self {
        // Set state
        self.attr(
            Attribute::Value,
            AttrValue::Payload(PropPayload::Vec(
                selected.iter().map(|x| PropValue::Usize(*x)).collect(),
            )),
        );
        self
    }

    fn rewindable(&self) -> bool {
        self.props
            .get_or(Attribute::Rewind, AttrValue::Flag(false))
            .unwrap_flag()
    }
}

impl MockComponent for Checkbox {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
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
            let title = self.props.get(Attribute::Title).map(|x| x.unwrap_title());
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
            let (bg, fg, block_color): (Color, Color, Color) = match &focus {
                true => (foreground, background, foreground),
                false => (Color::Reset, foreground, Color::Reset),
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
                    let (fg, bg) = match focus {
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
            let checkbox: Tabs = Tabs::new(choices)
                .block(div)
                .select(self.states.choice)
                .style(Style::default().fg(block_color));
            render.render_widget(checkbox, area);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        match attr {
            Attribute::Content => {
                // Reset choices
                let current_selection = self.states.selection.clone();
                let choices: Vec<String> = value
                    .unwrap_payload()
                    .unwrap_vec()
                    .iter()
                    .cloned()
                    .map(|x| x.unwrap_str())
                    .collect();
                self.states.set_choices(&choices);
                // Preserve selection if possible
                for c in current_selection.into_iter() {
                    self.states.select(c);
                }
            }
            Attribute::Value => {
                // Clear section
                self.states.selection.clear();
                for c in value.unwrap_payload().unwrap_vec() {
                    self.states.select(c.unwrap_usize());
                }
            }
            attr => {
                self.props.set(attr, value);
            }
        }
    }

    /// ### get_state
    ///
    /// Get current state from component
    /// For this component returns the vec of selected items
    fn state(&self) -> State {
        State::Vec(
            self.states
                .selection
                .iter()
                .map(|x| StateValue::Usize(*x))
                .collect(),
        )
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Right) => {
                // Increment choice
                self.states.next_choice(self.rewindable());
                CmdResult::None
            }
            Cmd::Move(Direction::Left) => {
                // Decrement choice
                self.states.prev_choice(self.rewindable());
                CmdResult::None
            }
            Cmd::Toggle => {
                self.states.toggle();
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

    use pretty_assertions::{assert_eq, assert_ne};
    use tuirealm::props::{PropPayload, PropValue};

    #[test]
    fn test_components_checkbox_states() {
        let mut states: CheckboxStates = CheckboxStates::default();
        assert_eq!(states.choice, 0);
        assert_eq!(states.choices.len(), 0);
        assert_eq!(states.selection.len(), 0);
        let choices: &[String] = &[
            "lemon".to_string(),
            "strawberry".to_string(),
            "vanilla".to_string(),
            "chocolate".to_string(),
        ];
        states.set_choices(choices);
        assert_eq!(states.choice, 0);
        assert_eq!(states.choices.len(), 4);
        assert_eq!(states.selection.len(), 0);
        // Select
        states.toggle();
        assert_eq!(states.selection, vec![0]);
        // Move
        states.prev_choice(false);
        assert_eq!(states.choice, 0);
        states.next_choice(false);
        assert_eq!(states.choice, 1);
        states.next_choice(false);
        assert_eq!(states.choice, 2);
        states.toggle();
        assert_eq!(states.selection, vec![0, 2]);
        // Forward overflow
        states.next_choice(false);
        states.next_choice(false);
        assert_eq!(states.choice, 3);
        states.prev_choice(false);
        assert_eq!(states.choice, 2);
        states.toggle();
        assert_eq!(states.selection, vec![0]);
        // has
        assert_eq!(states.has(0), true);
        assert_ne!(states.has(2), true);
        // Update
        let choices: &[String] = &["lemon".to_string(), "strawberry".to_string()];
        states.set_choices(choices);
        assert_eq!(states.choice, 1); // Move to first index available
        assert_eq!(states.choices.len(), 2);
        assert_eq!(states.selection.len(), 0);
        let choices: &[String] = &[];
        states.set_choices(choices);
        assert_eq!(states.choice, 0); // Move to first index available
        assert_eq!(states.choices.len(), 0);
        assert_eq!(states.selection.len(), 0);
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
    fn test_components_checkbox() {
        // Make component
        let mut component = Checkbox::default()
            .background(Color::Blue)
            .foreground(Color::Red)
            .borders(Borders::default())
            .title("Which food do you prefer?", Alignment::Center)
            .choices(&["Pizza", "Hummus", "Ramen", "Gyoza", "Pasta"])
            .values(&[1, 4])
            .rewind(false);
        // Verify states
        assert_eq!(component.states.selection, vec![1, 4]);
        assert_eq!(component.states.choice, 0);
        assert_eq!(component.states.choices.len(), 5);
        component.attr(
            Attribute::Content,
            AttrValue::Payload(PropPayload::Vec(vec![
                PropValue::Str(String::from("Pizza")),
                PropValue::Str(String::from("Hummus")),
                PropValue::Str(String::from("Ramen")),
                PropValue::Str(String::from("Gyoza")),
                PropValue::Str(String::from("Pasta")),
                PropValue::Str(String::from("Falafel")),
            ])),
        );
        assert_eq!(component.states.selection, vec![1, 4]);
        assert_eq!(component.states.choices.len(), 6);
        // Get value
        component.attr(
            Attribute::Value,
            AttrValue::Payload(PropPayload::Vec(vec![PropValue::Usize(1)])),
        );
        assert_eq!(component.states.selection, vec![1]);
        assert_eq!(component.states.choices.len(), 6);
        assert_eq!(component.state(), State::Vec(vec![StateValue::Usize(1)]));
        // Handle events
        assert_eq!(
            component.perform(Cmd::Move(Direction::Left)),
            CmdResult::None,
        );
        assert_eq!(component.state(), State::Vec(vec![StateValue::Usize(1)]));
        // Toggle
        assert_eq!(
            component.perform(Cmd::Toggle),
            CmdResult::Changed(State::Vec(vec![StateValue::Usize(1), StateValue::Usize(0)]))
        );
        // Left again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Left)),
            CmdResult::None,
        );
        assert_eq!(component.states.choice, 0);
        // Right
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::None,
        );
        // Toggle
        assert_eq!(
            component.perform(Cmd::Toggle),
            CmdResult::Changed(State::Vec(vec![StateValue::Usize(0)]))
        );
        // Right again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::None,
        );
        assert_eq!(component.states.choice, 2);
        // Right again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::None,
        );
        assert_eq!(component.states.choice, 3);
        // Right again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::None,
        );
        assert_eq!(component.states.choice, 4);
        // Right again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::None,
        );
        assert_eq!(component.states.choice, 5);
        // Right again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::None,
        );
        assert_eq!(component.states.choice, 5);
        // Submit
        assert_eq!(
            component.perform(Cmd::Submit),
            CmdResult::Submit(State::Vec(vec![StateValue::Usize(0)])),
        );
    }
}
