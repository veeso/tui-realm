//! ## Label
//!
//! label component

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{
    AttrValue, AttrValueRef, Attribute, Borders, Color, HorizontalAlignment, LineStatic, Props,
    QueryResult, Style, TextModifiers, Title,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::{BorderType, Paragraph};
use tuirealm::state::{State, StateValue};

use super::{Msg, get_block};

/// Counter which increments its value on Submit
#[derive(Default)]
struct Counter {
    props: Props,
    states: OwnStates,
}

impl Counter {
    pub fn label<S>(mut self, label: S) -> Self
    where
        S: Into<LineStatic>,
    {
        self.attr(
            Attribute::Title,
            AttrValue::Title(Title::from(label).alignment(HorizontalAlignment::Center)),
        );
        self
    }

    pub fn value(mut self, n: isize) -> Self {
        self.attr(Attribute::Value, AttrValue::Number(n));
        self
    }

    pub fn alignment(mut self, a: HorizontalAlignment) -> Self {
        self.attr(Attribute::TextAlign, AttrValue::AlignmentHorizontal(a));
        self
    }

    pub fn foreground(mut self, c: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(c));
        self
    }

    pub fn background(mut self, c: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(c));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }
}

impl Component for Counter {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if matches!(
            self.props.get(Attribute::Display),
            Some(AttrValue::Flag(false))
        ) {
            return;
        }

        // Get properties
        let text = self.states.counter.to_string();
        let alignment = self
            .props
            .get(Attribute::TextAlign)
            .and_then(AttrValue::as_alignment_horizontal)
            .unwrap_or(HorizontalAlignment::Left);
        let foreground = self
            .props
            .get(Attribute::Foreground)
            .and_then(AttrValue::as_color)
            .unwrap_or(Color::Reset);
        let background = self
            .props
            .get(Attribute::Background)
            .and_then(AttrValue::as_color)
            .unwrap_or(Color::Reset);
        let modifiers = self
            .props
            .get(Attribute::TextProps)
            .and_then(AttrValue::as_text_modifiers)
            .unwrap_or(TextModifiers::empty());
        let title = self
            .props
            .get(Attribute::Title)
            .and_then(AttrValue::as_title)
            .cloned()
            .unwrap_or(Title::default().alignment(HorizontalAlignment::Center));
        let borders = self
            .props
            .get(Attribute::Borders)
            .and_then(AttrValue::as_borders)
            .unwrap_or_default();
        let focus = self
            .props
            .get(Attribute::Focus)
            .and_then(AttrValue::as_flag)
            .unwrap_or(false);
        frame.render_widget(
            Paragraph::new(text)
                .block(get_block(borders, title.clone(), focus))
                .style(
                    Style::default()
                        .fg(foreground)
                        .bg(background)
                        .add_modifier(modifiers),
                )
                .alignment(alignment),
            area,
        );
    }

    fn query<'a>(&'a self, attr: Attribute) -> Option<QueryResult<'a>> {
        if attr == Attribute::Value {
            return Some(AttrValueRef::Number(self.states.counter).into());
        }

        self.props.get_for_query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if attr == Attribute::Value
            && let Some(value) = value.as_number()
        {
            self.states.counter = value;
        }

        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::Single(StateValue::Isize(self.states.counter))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Submit => {
                self.states.incr();
                CmdResult::Changed(self.state())
            }
            _ => CmdResult::None,
        }
    }
}

#[derive(Default)]
struct OwnStates {
    counter: isize,
}

impl OwnStates {
    fn incr(&mut self) {
        self.counter += 1;
    }
}

// -- Counter components

#[derive(Component)]
pub struct LetterCounter {
    component: Counter,
}

impl LetterCounter {
    pub fn new(initial_value: isize) -> Self {
        Self {
            component: Counter::default()
                .alignment(HorizontalAlignment::Center)
                .background(Color::Reset)
                .borders(
                    Borders::default()
                        .color(Color::LightGreen)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::LightGreen)
                .modifiers(TextModifiers::BOLD)
                .value(initial_value)
                .label("Letter counter"),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for LetterCounter {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        // Get command
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) if ch.is_alphabetic() => Cmd::Submit,
            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::LetterCounterBlur), // Return focus lost
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };
        // perform
        match self.perform(cmd) {
            CmdResult::Changed(State::Single(StateValue::Isize(c))) => {
                Some(Msg::LetterCounterChanged(c))
            }
            _ => None,
        }
    }
}

#[derive(Component)]
pub struct DigitCounter {
    component: Counter,
}

impl DigitCounter {
    pub fn new(initial_value: isize) -> Self {
        Self {
            component: Counter::default()
                .alignment(HorizontalAlignment::Center)
                .background(Color::Reset)
                .borders(
                    Borders::default()
                        .color(Color::Yellow)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::Yellow)
                .modifiers(TextModifiers::BOLD)
                .value(initial_value)
                .label("Digit counter"),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for DigitCounter {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        // Get command
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) if ch.is_ascii_digit() => Cmd::Submit,
            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::DigitCounterBlur), // Return focus lost
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };
        // perform
        match self.perform(cmd) {
            CmdResult::Changed(State::Single(StateValue::Isize(c))) => {
                Some(Msg::DigitCounterChanged(c))
            }
            _ => None,
        }
    }
}
