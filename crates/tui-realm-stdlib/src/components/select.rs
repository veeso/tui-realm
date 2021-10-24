//! ## Select
//!
//! `Select` represents a select field, like in HTML. The size for the component must be 3 (border + selected) + the quantity of rows
//! you want to display other options when opened (at least 3)

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
use tuirealm::event::KeyCode;
use tuirealm::props::borders::Borders;
use tuirealm::props::{
    Alignment, BlockTitle, BordersProps, PropPayload, PropValue, Props, PropsBuilder,
};
use tuirealm::tui::{
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, BorderType, List, ListItem, ListState, Paragraph},
};
use tuirealm::{event::Event, CmdResult, Component, Frame, Payload, Value};

// -- props

const COLOR_HIGHLIGHTED: &str = "highlighted";
const PROP_HIGHLIGHTED_TXT: &str = "highlighted-txt";
const PROP_SELECTED: &str = "selected";
const PROP_CHOICES: &str = "choices";
const PROP_REWIND: &str = "rewind";

pub struct SelectPropsBuilder {
    props: Option<Props>,
}

impl Default for SelectPropsBuilder {
    fn default() -> Self {
        Self {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for SelectPropsBuilder {
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

impl From<Props> for SelectPropsBuilder {
    fn from(props: Props) -> Self {
        SelectPropsBuilder { props: Some(props) }
    }
}

impl SelectPropsBuilder {
    /// ### with_foreground
    ///
    /// Set foreground color
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    /// ### with_background
    ///
    /// Set inverted color
    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.background = color;
        }
        self
    }

    pub fn with_highlighted_color(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.palette.insert(COLOR_HIGHLIGHTED, color);
        }
        self
    }

    /// ### with_highlighted_str
    ///
    /// Display a symbol to highlighted line in scroll table
    pub fn with_highlighted_str(&mut self, s: Option<&str>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            match s {
                None => {
                    props.own.remove(PROP_HIGHLIGHTED_TXT);
                }
                Some(s) => {
                    props.own.insert(
                        PROP_HIGHLIGHTED_TXT,
                        PropPayload::One(PropValue::Str(s.to_string())),
                    );
                }
            }
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
    pub fn with_title<S: AsRef<str>>(&mut self, title: S, alignment: Alignment) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.title = Some(BlockTitle::new(title, alignment));
        }
        self
    }

    /// ### with_value
    ///
    /// Set initial value for choice
    pub fn with_value(&mut self, index: usize) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_SELECTED, PropPayload::One(PropValue::Usize(index)));
        }
        self
    }

    /// ### rewind
    ///
    /// If true, moving below or beyond limit will rewind the selected record
    pub fn rewind(&mut self, rewind: bool) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_REWIND, PropPayload::One(PropValue::Bool(rewind)));
        }
        self
    }
}

// -- states

/// ## OwnStates
///
/// Component states
struct OwnStates {
    choices: Vec<String>, // Available choices
    focus: bool,
    selected: usize,
    tab_open: bool,
}

impl Default for OwnStates {
    fn default() -> Self {
        Self {
            choices: Vec::new(),
            focus: false,
            selected: 0,
            tab_open: false,
        }
    }
}

impl OwnStates {
    /// ### next_choice
    ///
    /// Move choice index to next choice
    pub fn next_choice(&mut self, rewind: bool) {
        if self.tab_open {
            if rewind && self.selected + 1 >= self.choices.len() {
                self.selected = 0;
            } else if self.selected + 1 < self.choices.len() {
                self.selected += 1;
            }
        }
    }

    /// ### prev_choice
    ///
    /// Move choice index to previous choice
    pub fn prev_choice(&mut self, rewind: bool) {
        if self.tab_open {
            if rewind && self.selected == 0 && !self.choices.is_empty() {
                self.selected = self.choices.len() - 1;
            } else if self.selected > 0 {
                self.selected -= 1;
            }
        }
    }

    /// ### set_choices
    ///
    /// Set OwnStates choices from a vector of str
    /// In addition resets current selection and keep index if possible or set it to the first value
    /// available
    pub fn set_choices(&mut self, choices: &[&str]) {
        self.choices = choices.iter().map(|x| x.to_string()).collect();
        // Keep index if possible
        if self.selected >= self.choices.len() {
            self.selected = match self.choices.len() {
                0 => 0,
                l => l - 1,
            };
        }
    }

    /// ### close_tab
    ///
    /// Close tab
    pub fn close_tab(&mut self) {
        self.tab_open = false;
    }

    /// ### open_tab
    ///
    /// Open tab
    pub fn open_tab(&mut self) {
        self.tab_open = true;
    }

    /// ### is_tab_open
    ///
    /// Returns whether the tab is open
    pub fn is_tab_open(&self) -> bool {
        self.tab_open
    }
}

// -- component

pub struct Select {
    props: Props,
    states: OwnStates,
}

impl Select {
    /// ### new
    ///
    /// Instantiate a new Select component
    pub fn new(props: Props) -> Self {
        // Make states
        let mut states: OwnStates = OwnStates::default();
        // Update choices (vec of TextSpan to String)
        let choices: Vec<&str> = match props.own.get(PROP_CHOICES).as_ref() {
            Some(PropPayload::Vec(choices)) => {
                choices.iter().map(|x| x.unwrap_str().as_str()).collect()
            }
            _ => Vec::new(),
        };
        states.set_choices(&choices);
        // Get value
        if let Some(PropPayload::One(PropValue::Usize(choice))) = props.own.get(PROP_SELECTED) {
            states.selected = *choice;
        }
        Self { props, states }
    }

    /// ### render_open_tab
    ///
    /// Render component when tab is open
    fn render_open_tab(&self, render: &mut Frame, area: Rect) {
        // Make choices
        let choices: Vec<ListItem> = self
            .states
            .choices
            .iter()
            .map(|x| ListItem::new(Spans::from(x.clone())))
            .collect();
        let hg: Color = self
            .props
            .palette
            .get(COLOR_HIGHLIGHTED)
            .cloned()
            .unwrap_or(foreground);
        // Make colors
        let (bg, hg): (Color, Color) = (background, hg);
        // Prepare layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Length(2), Constraint::Min(1)].as_ref())
            .split(area);
        // Render like "closed" tab in chunk 0
        let selected_text: String = match self.states.choices.get(self.states.selected) {
            None => String::default(),
            Some(s) => s.clone(),
        };
        let block: Block = Block::default()
            .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
            .border_style(self.props.borders.style())
            .style(Style::default().bg(background));
        let block: Block = match self.props.title.as_ref() {
            Some(t) => block
                .title(t.text().to_string())
                .title_alignment(t.alignment()),
            None => block,
        };
        let p: Paragraph = Paragraph::new(selected_text)
            .style(match focus {
                true => self.props.borders.style(),
                false => Style::default(),
            })
            .block(block);
        render.render_widget(p, chunks[0]);
        // Render the list of elements in chunks [1]
        // Make list
        let mut list = List::new(choices)
            .block(
                Block::default()
                    .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
                    .border_style(match focus {
                        true => self.props.borders.style(),
                        false => Style::default(),
                    })
                    .style(Style::default().bg(background)),
            )
            .start_corner(Corner::TopLeft)
            .style(
                Style::default()
                    .fg(foreground)
                    .bg(background)
                    .add_modifier(self.props.modifiers),
            )
            .highlight_style(
                Style::default()
                    .fg(bg)
                    .bg(hg)
                    .add_modifier(self.props.modifiers),
            );
        // Highlighted symbol
        if let Some(PropPayload::One(PropValue::Str(highlight))) =
            self.props.own.get(PROP_HIGHLIGHTED_TXT)
        {
            list = list.highlight_symbol(highlight);
        }
        let mut state: ListState = ListState::default();
        state.select(Some(self.states.selected));
        render.render_stateful_widget(list, chunks[1], &mut state);
    }

    /// ### render_closed_tab
    ///
    /// Render component when tab is closed
    fn render_closed_tab(&self, render: &mut Frame, area: Rect) {
        let div: Block =
            crate::utils::get_block(&self.props.borders, self.props.title.as_ref(), focus);
        let selected_text: String = match self.states.choices.get(self.states.selected) {
            None => String::default(),
            Some(s) => s.clone(),
        };
        let p: Paragraph = Paragraph::new(selected_text)
            .style(match focus {
                true => Style::default().fg(foreground).bg(background),
                false => Style::default(),
            })
            .block(div);
        render.render_widget(p, area);
    }

    fn rewind(&self) -> bool {
        match self.props.own.get(PROP_REWIND) {
            Some(PropPayload::One(PropValue::Bool(b))) => *b,
            _ => false,
        }
    }
}

impl MockComponent for Select {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    fn render(&self, render: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            match self.states.is_tab_open() {
                true => self.render_open_tab(render, area),
                false => self.render_closed_tab(render, area),
            }
        }
    }

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update.
    /// Returns a CmdResult to the view
    fn update(&mut self, props: Props) -> CmdResult {
        let prev_index: usize = self.states.selected;
        // Reset choices
        let choices: Vec<&str> = match props.own.get(PROP_CHOICES).as_ref() {
            Some(PropPayload::Vec(choices)) => {
                choices.iter().map(|x| x.unwrap_str().as_str()).collect()
            }
            _ => Vec::new(),
        };
        self.states.set_choices(&choices);
        // Get value
        if let Some(PropPayload::One(PropValue::Usize(choice))) = props.own.get(PROP_SELECTED) {
            self.states.selected = *choice;
        }
        self.props = props;
        // CmdResult none
        if prev_index != self.states.selected {
            CmdResult::Changed(self.get_state())
        } else {
            CmdResult::None
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
    /// Returns a CmdResult to the view.
    fn on(&mut self, ev: Event) -> CmdResult {
        // Match event
        if let Cmd::Key(key) = ev {
            match key.code {
                KeyCode::Down => {
                    // Increment choice
                    self.states.next_choice(self.rewind());
                    // Return CmdResult On Change or None if tab is closed
                    match self.states.is_tab_open() {
                        false => CmdResult::None,
                        true => {
                            CmdResult::Changed(Payload::One(Value::Usize(self.states.selected)))
                        }
                    }
                }
                KeyCode::Up => {
                    // Decrement choice
                    self.states.prev_choice(self.rewind());
                    // Return CmdResult On Change or None if tab is closed
                    match self.states.is_tab_open() {
                        false => CmdResult::None,
                        true => {
                            CmdResult::Changed(Payload::One(Value::Usize(self.states.selected)))
                        }
                    }
                }
                KeyCode::Enter => {
                    // Open or close tab
                    if self.states.is_tab_open() {
                        self.states.close_tab();
                        CmdResult::Submit(Payload::One(Value::Usize(self.states.selected)))
                    } else {
                        self.states.open_tab();
                        CmdResult::None
                    }
                }
                _ => {
                    // Return key event to activity
                    Cmd::None(key)
                }
            }
        } else {
            // Ignore event
            CmdResult::None
        }
    }

    /// ### get_state
    ///
    /// Get current state from component
    /// For this component returns the index of the selected choice, but only when the tab is closed
    /// Returns None otherwise
    fn get_state(&self) -> Payload {
        match self.states.is_tab_open() {
            false => State::One(Value::Usize(self.states.selected)),
            true => State::None,
        }
    }

    // -- events

    /// ### blur
    ///
    /// Blur component
    fn blur(&mut self) {
        focus = false;
        // Tab gets closed
        self.states.close_tab();
    }

    /// ### active
    ///
    /// Active component
    fn active(&mut self) {
        focus = true;
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use crossterm::event::{KeyCode, KeyEvent};

    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_select_states() {
        let mut states: OwnStates = OwnStates::default();
        assert_eq!(states.selected, 0);
        assert_eq!(states.choices.len(), 0);
        assert_eq!(states.tab_open, false);
        let choices = vec!["lemon", "strawberry", "vanilla", "chocolate"];
        states.set_choices(&choices);
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
        let choices = vec!["lemon", "strawberry"];
        states.set_choices(&choices);
        assert_eq!(states.selected, 1); // Move to first index available
        assert_eq!(states.choices.len(), 2);
        let choices = vec![];
        states.set_choices(&choices);
        assert_eq!(states.selected, 0); // Move to first index available
        assert_eq!(states.choices.len(), 0);
        // Rewind
        let choices: &[&str] = &["lemon", "strawberry", "vanilla", "chocolate"];
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
    }

    #[test]
    fn test_components_select() {
        // Make component
        let mut component: Select = Select::new(
            SelectPropsBuilder::default()
                .hidden()
                .visible()
                .with_foreground(Color::Red)
                .with_background(Color::Blue)
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .with_highlighted_color(Color::Red)
                .with_highlighted_str(Some(">>"))
                .with_title("C'est oui ou bien c'est non?", Alignment::Center)
                .with_options(&["Oui!", "Non", "Peut-être"])
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .with_value(1)
                .rewind(false)
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.background, Color::Blue);
        assert_eq!(component.props.visible, true);
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Double);
        assert_eq!(component.props.borders.color, Color::Red);
        assert_eq!(
            *component
                .props
                .own
                .get(PROP_REWIND)
                .unwrap()
                .unwrap_one()
                .unwrap_bool(),
            false
        );
        assert_eq!(
            *component.props.palette.get(COLOR_HIGHLIGHTED).unwrap(),
            Color::Red
        );
        assert_eq!(
            *component.props.own.get(PROP_HIGHLIGHTED_TXT).unwrap(),
            PropPayload::One(PropValue::Str(String::from(">>")))
        );
        assert_eq!(
            *component.props.own.get(PROP_SELECTED).unwrap(),
            PropPayload::One(PropValue::Usize(1))
        );
        assert_eq!(
            component.props.title.as_ref().unwrap().text(),
            "C'est oui ou bien c'est non?"
        );
        assert_eq!(
            component.props.title.as_ref().unwrap().alignment(),
            Alignment::Center
        );
        assert_eq!(
            component.props.own.get(PROP_CHOICES).unwrap(),
            &PropPayload::Vec(vec![
                PropValue::Str(String::from("Oui!")),
                PropValue::Str(String::from("Non")),
                PropValue::Str(String::from("Peut-être")),
            ])
        );
        // Focus
        component.active();
        assert_eq!(component.states.focus, true);
        assert_eq!(component.states.is_tab_open(), false);
        component.states.open_tab();
        assert_eq!(component.states.is_tab_open(), true);
        component.blur();
        assert_eq!(component.states.focus, false);
        assert_eq!(component.states.is_tab_open(), false);
        // Update
        let props = SelectPropsBuilder::from(component.get_props())
            .with_foreground(Color::Red)
            .hidden()
            .build();
        assert_eq!(component.update(props), CmdResult::None);
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.visible, false);
        let props = SelectPropsBuilder::from(component.get_props())
            .with_value(2)
            .hidden()
            .build();
        assert_eq!(
            component.update(props),
            CmdResult::Changed(Payload::One(Value::Usize(2)))
        );
        // Get value
        assert_eq!(component.get_state(), State::One(Value::Usize(2)));
        // Open tab
        component.states.open_tab();
        assert_eq!(component.get_state(), State::None);
        // Events
        // Move cursor
        assert_eq!(
            component.on(Cmd::Move(Direction::Up)),
            CmdResult::Changed(Payload::One(Value::Usize(1))),
        );
        assert_eq!(
            component.on(Cmd::Move(Direction::Up)),
            CmdResult::Changed(Payload::One(Value::Usize(0))),
        );
        // Upper boundary
        assert_eq!(
            component.on(Cmd::Move(Direction::Up)),
            CmdResult::Changed(Payload::One(Value::Usize(0))),
        );
        // Move down
        assert_eq!(
            component.on(Cmd::Move(Direction::Down)),
            CmdResult::Changed(Payload::One(Value::Usize(1))),
        );
        assert_eq!(
            component.on(Cmd::Move(Direction::Down)),
            CmdResult::Changed(Payload::One(Value::Usize(2))),
        );
        // Lower boundary
        assert_eq!(
            component.on(Cmd::Move(Direction::Down)),
            CmdResult::Changed(Payload::One(Value::Usize(2))),
        );
        // Press enter
        assert_eq!(
            component.on(Cmd::Key(KeyCmd::from(KeyCode::Enter))),
            CmdResult::Submit(Payload::One(Value::Usize(2))),
        );
        // Tab should be closed
        assert_eq!(component.states.is_tab_open(), false);
        // Re open
        assert_eq!(
            component.on(Cmd::Key(KeyCmd::from(KeyCode::Enter))),
            CmdResult::None,
        );
        assert_eq!(component.states.is_tab_open(), true);
        // Move arrows
        assert_eq!(
            component.on(Cmd::Key(KeyCmd::from(KeyCode::Enter))),
            CmdResult::Submit(Payload::One(Value::Usize(2))),
        );
        assert_eq!(component.states.is_tab_open(), false);
        assert_eq!(component.on(Cmd::Move(Direction::Down)), CmdResult::None,);
        assert_eq!(component.on(Cmd::Move(Direction::Up)), CmdResult::None,);
        // Char
        assert_eq!(
            component.on(Cmd::Key(KeyCmd::from(KeyCode::Char('a')))),
            Cmd::None(KeyCmd::from(KeyCode::Char('a'))),
        );
        assert_eq!(component.on(Cmd::Resize(0, 0)), CmdResult::None);
    }
}
