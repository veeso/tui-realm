//! `Select` represents a select field, like in HTML. The size for the component must be 3 (border + selected) + the quantity of rows
//! you want to display other options when opened (at least 3).

use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::component::Component;
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, LineStatic, PropPayload, PropValue, Props, Style,
    TextModifiers, Title,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::text::Line as Spans;
use tuirealm::ratatui::{
    layout::{Constraint, Direction as LayoutDirection, Layout, Rect},
    widgets::{List, ListItem, ListState, Paragraph},
};
use tuirealm::state::{State, StateValue};

use crate::prop_ext::CommonProps;
use crate::utils::borrow_clone_line;

// -- states

/// The states that need to be kept for the [`Select`] component.
#[derive(Default)]
pub struct SelectStates {
    /// Available choices
    pub choices: Vec<String>,
    /// Currently selected choice
    pub selected: usize,
    /// Choice selected before opening the tab
    pub previously_selected: usize,
    pub tab_open: bool,
}

impl SelectStates {
    /// Move choice index to next choice.
    pub fn next_choice(&mut self, rewind: bool) {
        if self.tab_open {
            if rewind && self.selected + 1 >= self.choices.len() {
                self.selected = 0;
            } else if self.selected + 1 < self.choices.len() {
                self.selected += 1;
            }
        }
    }

    /// Move choice index to previous choice.
    pub fn prev_choice(&mut self, rewind: bool) {
        if self.tab_open {
            if rewind && self.selected == 0 && !self.choices.is_empty() {
                self.selected = self.choices.len() - 1;
            } else if self.selected > 0 {
                self.selected -= 1;
            }
        }
    }

    /// Overwrite the choices available with new ones.
    ///
    /// In addition resets current selection and keep index if possible or set it to the first value
    /// available.
    pub fn set_choices(&mut self, choices: impl Into<Vec<String>>) {
        self.choices = choices.into();
        // Keep index if possible
        if self.selected >= self.choices.len() {
            self.selected = match self.choices.len() {
                0 => 0,
                l => l - 1,
            };
        }
    }

    pub fn select(&mut self, i: usize) {
        if i < self.choices.len() {
            self.selected = i;
        }
    }

    /// Close tab.
    pub fn close_tab(&mut self) {
        self.tab_open = false;
    }

    /// Open tab.
    pub fn open_tab(&mut self) {
        self.previously_selected = self.selected;
        self.tab_open = true;
    }

    /// Cancel tab open.
    pub fn cancel_tab(&mut self) {
        self.close_tab();
        self.selected = self.previously_selected;
    }

    /// Returns whether the tab is open.
    #[must_use]
    pub fn is_tab_open(&self) -> bool {
        self.tab_open
    }
}

// -- component

/// `Select` represents a select field, like in HTML. The size for the component must be 3 (border + selected) + the quantity of rows
/// you want to display other options when opened (at least 3).
///
/// Similar to [`Radio`](crate::Radio), [`Select`] is a single-choice selector, but the difference is that it does not show the selector
/// unless the "Tab" is open, and only shows the currently selected choice.
#[derive(Default)]
#[must_use]
pub struct Select {
    common: CommonProps,
    props: Props,
    pub states: SelectStates,
}

impl Select {
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

    /// Set whether wraparound should be possible (down on the last choice wraps around to 0, and the other way around).
    pub fn rewind(mut self, r: bool) -> Self {
        self.attr(Attribute::Rewind, AttrValue::Flag(r));
        self
    }

    /// Set the Symbol and Style for the indicator of the current line.
    pub fn highlighted_str<S: Into<LineStatic>>(mut self, s: S) -> Self {
        self.attr(Attribute::HighlightedStr, AttrValue::TextLine(s.into()));
        self
    }

    /// Set a custom foreground color for the currently highlighted item.
    pub fn highlighted_color(mut self, c: Color) -> Self {
        // TODO: shouldnt this be a highlight style instead?
        self.attr(Attribute::HighlightedColor, AttrValue::Color(c));
        self
    }

    /// Set the choices that should be possible.
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

    /// Set the initially selected choice.
    pub fn value(mut self, i: usize) -> Self {
        // Set state
        self.attr(
            Attribute::Value,
            AttrValue::Payload(PropPayload::Single(PropValue::Usize(i))),
        );
        self
    }

    /// ### render_open_tab
    ///
    /// Render component when tab is open
    fn render_open_tab(&mut self, render: &mut Frame, mut area: Rect) {
        // Make choices
        let choices: Vec<ListItem> = self
            .states
            .choices
            .iter()
            .map(|x| ListItem::new(Spans::from(x.as_str())))
            .collect();

        let hg = self
            .props
            .get_ref(Attribute::HighlightedColor)
            .and_then(AttrValue::as_color);

        if let Some(block) = self.common.get_block() {
            let inner = block.inner(area);
            render.render_widget(block, area);
            area = inner;
        }

        // Prepare layout
        let [para_area, list_area] = Layout::default()
            .direction(LayoutDirection::Vertical)
            .margin(0)
            .constraints([Constraint::Length(2), Constraint::Min(1)])
            .areas(area);
        // Render like "closed" tab in chunk 0
        let selected_text: String = match self.states.choices.get(self.states.selected) {
            None => String::default(),
            Some(s) => s.clone(),
        };

        let para = Paragraph::new(selected_text).style(self.common.style);
        render.render_widget(para, para_area);

        let hg_style = if let Some(color) = hg {
            Style::new().fg(color)
        } else {
            Style::new()
        }
        .add_modifier(TextModifiers::REVERSED);

        // Render the list of elements in chunks [1]
        // Make list
        let mut list = List::new(choices)
            .direction(tuirealm::ratatui::widgets::ListDirection::TopToBottom)
            .style(self.common.style)
            .highlight_style(hg_style);
        // Highlighted symbol
        let hg_str = self
            .props
            .get_ref(Attribute::HighlightedStr)
            .and_then(|x| x.as_textline());
        if let Some(hg_str) = hg_str {
            list = list.highlight_symbol(borrow_clone_line(hg_str));
        }
        let mut state: ListState = ListState::default();
        state.select(Some(self.states.selected));

        render.render_stateful_widget(list, list_area, &mut state);
    }

    /// ### render_closed_tab
    ///
    /// Render component when tab is closed
    fn render_closed_tab(&self, render: &mut Frame, area: Rect) {
        let selected_text: String = match self.states.choices.get(self.states.selected) {
            None => String::default(),
            Some(s) => s.clone(),
        };
        let mut widget = Paragraph::new(selected_text).style(self.common.style);

        if let Some(block) = self.common.get_block() {
            widget = widget.block(block);
        }

        render.render_widget(widget, area);
    }

    fn rewindable(&self) -> bool {
        self.props
            .get_or(Attribute::Rewind, AttrValue::Flag(false))
            .unwrap_flag()
    }
}

impl Component for Select {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if !self.common.display {
            return;
        }

        if self.states.is_tab_open() {
            self.render_open_tab(render, area);
        } else {
            self.render_closed_tab(render, area);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        if let Some(value) = self.common.get(attr) {
            return Some(value);
        }

        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if let Some(value) = self.common.set(attr, value) {
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
                        .select(value.unwrap_payload().unwrap_single().unwrap_usize());
                }
                Attribute::Focus if self.states.is_tab_open() => {
                    if let AttrValue::Flag(false) = value {
                        self.states.cancel_tab();
                    }
                    self.props.set(attr, value);
                }
                attr => {
                    self.props.set(attr, value);
                }
            }
        }
    }

    fn state(&self) -> State {
        if self.states.is_tab_open() {
            State::None
        } else {
            State::Single(StateValue::Usize(self.states.selected))
        }
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Down) => {
                // Increment choice
                self.states.next_choice(self.rewindable());
                // Return CmdResult On Change or None if tab is closed
                if self.states.is_tab_open() {
                    CmdResult::Changed(State::Single(StateValue::Usize(self.states.selected)))
                } else {
                    CmdResult::None
                }
            }
            Cmd::Move(Direction::Up) => {
                // Increment choice
                self.states.prev_choice(self.rewindable());
                // Return CmdResult On Change or None if tab is closed
                if self.states.is_tab_open() {
                    CmdResult::Changed(State::Single(StateValue::Usize(self.states.selected)))
                } else {
                    CmdResult::None
                }
            }
            Cmd::Cancel => {
                self.states.cancel_tab();
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                // Open or close tab
                if self.states.is_tab_open() {
                    self.states.close_tab();
                    CmdResult::Submit(self.state())
                } else {
                    self.states.open_tab();
                    CmdResult::None
                }
            }
            _ => CmdResult::None,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    use tuirealm::props::{HorizontalAlignment, PropPayload, PropValue};

    #[test]
    fn test_components_select_states() {
        let mut states: SelectStates = SelectStates::default();
        assert_eq!(states.selected, 0);
        assert_eq!(states.choices.len(), 0);
        assert_eq!(states.tab_open, false);
        let choices: &[String] = &[
            "lemon".to_string(),
            "strawberry".to_string(),
            "vanilla".to_string(),
            "chocolate".to_string(),
        ];
        states.set_choices(choices);
        assert_eq!(states.selected, 0);
        assert_eq!(states.choices.len(), 4);
        // Move
        states.prev_choice(false);
        assert_eq!(states.selected, 0);
        states.next_choice(false);
        // Tab is closed!!!
        assert_eq!(states.selected, 0);
        states.open_tab();
        assert_eq!(states.is_tab_open(), true);
        // Now we can move
        states.next_choice(false);
        assert_eq!(states.selected, 1);
        states.next_choice(false);
        assert_eq!(states.selected, 2);
        // Forward overflow
        states.next_choice(false);
        states.next_choice(false);
        assert_eq!(states.selected, 3);
        states.prev_choice(false);
        assert_eq!(states.selected, 2);
        // Close tab
        states.close_tab();
        assert_eq!(states.is_tab_open(), false);
        states.prev_choice(false);
        assert_eq!(states.selected, 2);
        // Update
        let choices: &[String] = &["lemon".to_string(), "strawberry".to_string()];
        states.set_choices(choices);
        assert_eq!(states.selected, 1); // Move to first index available
        assert_eq!(states.choices.len(), 2);
        let choices = vec![];
        states.set_choices(choices);
        assert_eq!(states.selected, 0); // Move to first index available
        assert_eq!(states.choices.len(), 0);
        // Rewind
        let choices: &[String] = &[
            "lemon".to_string(),
            "strawberry".to_string(),
            "vanilla".to_string(),
            "chocolate".to_string(),
        ];
        states.set_choices(choices);
        states.open_tab();
        assert_eq!(states.selected, 0);
        states.prev_choice(true);
        assert_eq!(states.selected, 3);
        states.next_choice(true);
        assert_eq!(states.selected, 0);
        states.next_choice(true);
        assert_eq!(states.selected, 1);
        states.prev_choice(true);
        assert_eq!(states.selected, 0);
        // Cancel tab
        states.close_tab();
        states.select(2);
        states.open_tab();
        states.prev_choice(true);
        states.prev_choice(true);
        assert_eq!(states.selected, 0);
        states.cancel_tab();
        assert_eq!(states.selected, 2);
        assert_eq!(states.is_tab_open(), false);
    }

    #[test]
    fn test_components_select() {
        // Make component
        let mut component = Select::default()
            .foreground(Color::Red)
            .background(Color::Black)
            .borders(Borders::default())
            .highlighted_color(Color::Red)
            .highlighted_str(">>")
            .title(
                Title::from("C'est oui ou bien c'est non?").alignment(HorizontalAlignment::Center),
            )
            .choices(["Oui!", "Non", "Peut-être"])
            .value(1)
            .rewind(false);
        assert_eq!(component.states.is_tab_open(), false);
        component.states.open_tab();
        assert_eq!(component.states.is_tab_open(), true);
        component.states.close_tab();
        assert_eq!(component.states.is_tab_open(), false);
        // Update
        component.attr(
            Attribute::Value,
            AttrValue::Payload(PropPayload::Single(PropValue::Usize(2))),
        );
        // Get value
        assert_eq!(component.state(), State::Single(StateValue::Usize(2)));
        // Open tab
        component.states.open_tab();
        // Events
        // Move cursor
        assert_eq!(
            component.perform(Cmd::Move(Direction::Up)),
            CmdResult::Changed(State::Single(StateValue::Usize(1))),
        );
        assert_eq!(
            component.perform(Cmd::Move(Direction::Up)),
            CmdResult::Changed(State::Single(StateValue::Usize(0))),
        );
        // Upper boundary
        assert_eq!(
            component.perform(Cmd::Move(Direction::Up)),
            CmdResult::Changed(State::Single(StateValue::Usize(0))),
        );
        // Move down
        assert_eq!(
            component.perform(Cmd::Move(Direction::Down)),
            CmdResult::Changed(State::Single(StateValue::Usize(1))),
        );
        assert_eq!(
            component.perform(Cmd::Move(Direction::Down)),
            CmdResult::Changed(State::Single(StateValue::Usize(2))),
        );
        // Lower boundary
        assert_eq!(
            component.perform(Cmd::Move(Direction::Down)),
            CmdResult::Changed(State::Single(StateValue::Usize(2))),
        );
        // Press enter
        assert_eq!(
            component.perform(Cmd::Submit),
            CmdResult::Submit(State::Single(StateValue::Usize(2))),
        );
        // Tab should be closed
        assert_eq!(component.states.is_tab_open(), false);
        // Re open
        assert_eq!(component.perform(Cmd::Submit), CmdResult::None);
        assert_eq!(component.states.is_tab_open(), true);
        // Move arrows
        assert_eq!(
            component.perform(Cmd::Submit),
            CmdResult::Submit(State::Single(StateValue::Usize(2))),
        );
        assert_eq!(component.states.is_tab_open(), false);
        assert_eq!(
            component.perform(Cmd::Move(Direction::Down)),
            CmdResult::None
        );
        assert_eq!(component.perform(Cmd::Move(Direction::Up)), CmdResult::None);
    }

    #[test]
    fn various_set_choice_types() {
        // static array of strings
        SelectStates::default().set_choices(&["hello".to_string()]);
        // vector of strings
        SelectStates::default().set_choices(vec!["hello".to_string()]);
        // boxed array of strings
        SelectStates::default().set_choices(vec!["hello".to_string()].into_boxed_slice());
    }

    #[test]
    fn various_choice_types() {
        // static array of static strings
        let _ = Select::default().choices(["hello"]);
        // static array of strings
        let _ = Select::default().choices(["hello".to_string()]);
        // vec of static strings
        let _ = Select::default().choices(vec!["hello"]);
        // vec of strings
        let _ = Select::default().choices(vec!["hello".to_string()]);
        // boxed array of static strings
        let _ = Select::default().choices(vec!["hello"].into_boxed_slice());
        // boxed array of strings
        let _ = Select::default().choices(vec!["hello".to_string()].into_boxed_slice());
    }
}
