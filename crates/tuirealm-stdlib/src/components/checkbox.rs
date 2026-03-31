//! `Checkbox` component renders a checkbox group.

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
use tuirealm::component::Component;
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, QueryResult, Style,
    TextModifiers, Title,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::text::{Line, Span};
use tuirealm::ratatui::widgets::Tabs;
use tuirealm::state::{State, StateValue};

use crate::prop_ext::{CommonHighlight, CommonProps};

// -- states

/// The state that needs to be kept for [`Checkbox`].
#[derive(Default)]
pub struct CheckboxStates {
    /// Current hover option.
    pub choice: usize,
    /// Available choices.
    pub choices: Vec<String>,
    /// Enabled options.
    pub selection: Vec<usize>,
}

impl CheckboxStates {
    /// Move choice index to next choice.
    pub fn next_choice(&mut self, rewind: bool) {
        if rewind && self.choice + 1 >= self.choices.len() {
            self.choice = 0;
        } else if self.choice + 1 < self.choices.len() {
            self.choice += 1;
        }
    }

    /// Move choice index to previous choice.
    pub fn prev_choice(&mut self, rewind: bool) {
        if rewind && self.choice == 0 && !self.choices.is_empty() {
            self.choice = self.choices.len() - 1;
        } else if self.choice > 0 {
            self.choice -= 1;
        }
    }

    /// Check or uncheck the option.
    pub fn toggle(&mut self) {
        let option = self.choice;
        if self.selection.contains(&option) {
            let target_index = self.selection.iter().position(|x| *x == option).unwrap();
            self.selection.remove(target_index);
        } else {
            self.selection.push(option);
        }
    }

    /// Select a specific option.
    pub fn select(&mut self, i: usize) {
        if i < self.choices.len() && !self.selection.contains(&i) {
            self.selection.push(i);
        }
    }

    /// Determine if `options` is toggeled as selected.
    #[must_use]
    pub fn has(&self, option: usize) -> bool {
        self.selection.contains(&option)
    }

    /// Overwrite the choices available with new ones.
    ///
    /// In addition resets current selection and keep index if possible or set it to the first value
    /// available.
    pub fn set_choices(&mut self, choices: impl Into<Vec<String>>) {
        self.choices = choices.into();
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

/// The checkbox component is a multi-choice selector.
///
/// Use [`Radio`](crate::Radio) if a single-choice selector is wanted.
#[derive(Default)]
#[must_use]
pub struct Checkbox {
    common: CommonProps,
    common_hg: CommonHighlight,
    props: Props,
    pub states: CheckboxStates,
}

impl Checkbox {
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

    /// Set a custom highlight style that is patched ontop of the normal style.
    ///
    /// By default the highlight style is just `Style::new().add_modifier(Modifier::REVERSED)`.
    pub fn highlight_style(mut self, s: Style) -> Self {
        self.attr(Attribute::HighlightStyle, AttrValue::Style(s));
        self
    }

    /// Set whether wraparound should be possible (down on the last choice wraps around to 0, and the other way around).
    pub fn rewind(mut self, r: bool) -> Self {
        self.attr(Attribute::Rewind, AttrValue::Flag(r));
        self
    }

    /// Set the choices that should be possible.
    pub fn choices<S: Into<String>>(mut self, choices: impl IntoIterator<Item = S>) -> Self {
        // TODO: we should consider using Spans or Lines
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

    /// Set the initially selected choices.
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

    /// Set the current component to be always active (show highligh even if unfocused)
    pub fn always_active(mut self) -> Self {
        self.attr(Attribute::AlwaysActive, AttrValue::Flag(true));
        self
    }

    fn rewindable(&self) -> bool {
        self.props
            .get(Attribute::Rewind)
            .and_then(AttrValue::as_flag)
            .unwrap_or_default()
    }
}

impl Component for Checkbox {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if !self.common.display {
            return;
        }

        // Make choices
        let choices: Vec<Line> = self
            .states
            .choices
            .iter()
            .enumerate()
            .map(|(idx, x)| {
                let checkbox: &str = if self.states.has(idx) { "☑ " } else { "☐ " };
                // Make Lines
                Line::from(vec![Span::raw(checkbox), Span::raw(x.to_string())])
            })
            .collect();
        let mut widget: Tabs = Tabs::new(choices)
            .select(self.states.choice)
            .style(self.common.style)
            .highlight_style(self.common_hg.get_style(self.common.style));

        if let Some(block) = self.common.get_block() {
            widget = widget.block(block);
        }

        render.render_widget(widget, area);
    }

    fn query<'a>(&'a self, attr: Attribute) -> Option<QueryResult<'a>> {
        if let Some(value) = self
            .common
            .get_for_query(attr)
            .or_else(|| self.common_hg.get_for_query(attr))
        {
            return Some(value);
        }

        self.props.get_for_query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if let Some(value) = self
            .common
            .set(attr, value)
            .and_then(|value| self.common_hg.set(attr, value))
        {
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
                    self.states.set_choices(choices);
                    // Preserve selection if possible
                    for c in current_selection {
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
                CmdResult::Visual
            }
            Cmd::Move(Direction::Left) => {
                // Decrement choice
                self.states.prev_choice(self.rewindable());
                CmdResult::Visual
            }
            Cmd::Toggle => {
                self.states.toggle();
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                // Return Submit
                CmdResult::Submit(self.state())
            }
            _ => CmdResult::Invalid(cmd),
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::{assert_eq, assert_ne};
    use tuirealm::props::{HorizontalAlignment, PropPayload, PropValue};

    use super::*;

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
            .title(Title::from("Which food do you prefer?").alignment(HorizontalAlignment::Center))
            .choices(["Pizza", "Hummus", "Ramen", "Gyoza", "Pasta"])
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
            CmdResult::Visual,
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
            CmdResult::Visual,
        );
        assert_eq!(component.states.choice, 0);
        // Right
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::Visual,
        );
        // Toggle
        assert_eq!(
            component.perform(Cmd::Toggle),
            CmdResult::Changed(State::Vec(vec![StateValue::Usize(0)]))
        );
        // Right again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::Visual,
        );
        assert_eq!(component.states.choice, 2);
        // Right again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::Visual,
        );
        assert_eq!(component.states.choice, 3);
        // Right again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::Visual,
        );
        assert_eq!(component.states.choice, 4);
        // Right again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::Visual,
        );
        assert_eq!(component.states.choice, 5);
        // Right again
        assert_eq!(
            component.perform(Cmd::Move(Direction::Right)),
            CmdResult::Visual,
        );
        assert_eq!(component.states.choice, 5);
        // Submit
        assert_eq!(
            component.perform(Cmd::Submit),
            CmdResult::Submit(State::Vec(vec![StateValue::Usize(0)])),
        );
    }

    #[test]
    fn various_set_choice_types() {
        // static array of strings
        CheckboxStates::default().set_choices(&["hello".to_string()]);
        // vector of strings
        CheckboxStates::default().set_choices(vec!["hello".to_string()]);
        // boxed array of strings
        CheckboxStates::default().set_choices(vec!["hello".to_string()].into_boxed_slice());
    }

    #[test]
    fn various_choice_types() {
        // static array of static strings
        let _ = Checkbox::default().choices(["hello"]);
        // static array of strings
        let _ = Checkbox::default().choices(["hello".to_string()]);
        // vec of static strings
        let _ = Checkbox::default().choices(vec!["hello"]);
        // vec of strings
        let _ = Checkbox::default().choices(vec!["hello".to_string()]);
        // boxed array of static strings
        let _ = Checkbox::default().choices(vec!["hello"].into_boxed_slice());
        // boxed array of strings
        let _ = Checkbox::default().choices(vec!["hello".to_string()].into_boxed_slice());
    }
}
