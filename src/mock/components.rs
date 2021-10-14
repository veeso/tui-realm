//! # Components
//!
//! mock components

use super::{MockEvent, MockMsg};
use crate::command::Direction;
use crate::event::{Event, Key, KeyEvent, KeyModifiers};
use crate::{
    command::{Cmd, CmdResult},
    AttrValue, Attribute, Component, MockComponent, Props, State, StateValue,
};

/// ## MockInput
///
/// Mocked component implementing `MockComponent`
pub struct MockInput {
    props: Props,
    states: MockInputStates,
}

impl Default for MockInput {
    fn default() -> Self {
        Self {
            props: Props::default(),
            states: MockInputStates::default(),
        }
    }
}

impl MockComponent for MockInput {
    fn view(&mut self, _: &mut crate::Frame, _: crate::tui::layout::Rect) {}

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, query: Attribute, attr: AttrValue) {
        self.props.set(query, attr);
    }

    fn state(&self) -> State {
        State::One(StateValue::String(self.states.text.clone()))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Left) => {
                self.states.left();
                CmdResult::Changed(State::One(StateValue::Usize(self.states.cursor)))
            }
            Cmd::Move(Direction::Right) => {
                self.states.right();
                CmdResult::Changed(State::One(StateValue::Usize(self.states.cursor)))
            }
            Cmd::Type(ch) => {
                self.states.input(ch);
                CmdResult::Changed(self.state())
            }
            Cmd::Press(key_event) => CmdResult::KeyBounced(key_event),
            _ => CmdResult::None,
        }
    }
}

// -- component states

struct MockInputStates {
    text: String,
    cursor: usize,
}

impl MockInputStates {
    fn default() -> Self {
        Self {
            text: String::default(),
            cursor: 0,
        }
    }
}

impl MockInputStates {
    fn input(&mut self, c: char) {
        self.text.push(c);
    }

    fn left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn right(&mut self) {
        self.cursor += 1;
    }
}

// -- component impl

// TODO: #[derive(MockComponent)]
pub struct MockFooInput {
    component: MockInput,
}

impl Default for MockFooInput {
    fn default() -> Self {
        Self {
            component: MockInput::default(),
        }
    }
}

impl MockComponent for MockFooInput {
    fn view(&mut self, frame: &mut crate::Frame, area: tuirs::layout::Rect) {
        self.component.view(frame, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, query: Attribute, attr: AttrValue) {
        self.component.attr(query, attr)
    }

    fn state(&self) -> State {
        self.component.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}

impl Component<MockMsg, MockEvent> for MockFooInput {
    fn on(&mut self, ev: Event<MockEvent>) -> Option<MockMsg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers: _,
            }) => Cmd::Move(Direction::Left),
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                modifiers: _,
            }) => Cmd::Move(Direction::Right),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => Cmd::Type(ch),
            Event::Keyboard(key) => Cmd::Press(key),
            _ => Cmd::None,
        };
        match self.component.perform(cmd) {
            CmdResult::Changed(State::One(StateValue::String(s))) => {
                Some(MockMsg::FooInputChanged(s))
            }
            _ => None,
        }
    }
}

pub struct MockBarInput {
    component: MockInput,
}

impl Default for MockBarInput {
    fn default() -> Self {
        Self {
            component: MockInput::default(),
        }
    }
}

impl MockComponent for MockBarInput {
    fn view(&mut self, frame: &mut crate::Frame, area: tuirs::layout::Rect) {
        self.component.view(frame, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, query: Attribute, attr: AttrValue) {
        self.component.attr(query, attr)
    }

    fn state(&self) -> State {
        self.component.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}

impl Component<MockMsg, MockEvent> for MockBarInput {
    fn on(&mut self, ev: Event<MockEvent>) -> Option<MockMsg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers: _,
            }) => Cmd::Move(Direction::Left),
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                modifiers: _,
            }) => Cmd::Move(Direction::Right),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => Cmd::Type(ch),
            Event::Keyboard(key) => Cmd::Press(key),
            _ => Cmd::None,
        };
        match self.component.perform(cmd) {
            CmdResult::Changed(State::One(StateValue::String(s))) => {
                Some(MockMsg::BarInputChanged(s))
            }
            _ => None,
        }
    }
}
