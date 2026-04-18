//! Mock Components for testing

use ratatui::Frame;

use super::{MockEvent, MockMsg};
use crate::command::{Cmd, CmdResult, Direction};
use crate::component::{AppComponent, Component};
use crate::event::{Event, Key, KeyEvent, KeyModifiers};
use crate::props::{AttrValue, Attribute, Props, QueryResult};
use crate::state::{State, StateValue};

/// Mocked component implementing `Component`
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

impl Component for MockInput {
    fn view(&mut self, _: &mut Frame, _: crate::ratatui::layout::Rect) {}

    fn query<'a>(&'a self, attr: Attribute) -> Option<QueryResult<'a>> {
        self.props.get_for_query(attr)
    }

    fn attr(&mut self, query: Attribute, attr: AttrValue) {
        self.props.set(query, attr);
    }

    fn state(&self) -> State {
        State::Single(StateValue::String(self.states.text.clone()))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Left) => {
                self.states.left();
                CmdResult::Changed(State::Single(StateValue::Usize(self.states.cursor)))
            }
            Cmd::Move(Direction::Right) => {
                self.states.right();
                CmdResult::Changed(State::Single(StateValue::Usize(self.states.cursor)))
            }
            Cmd::Type(ch) => {
                self.states.input(ch);
                CmdResult::Changed(self.state())
            }
            _ => CmdResult::Invalid(cmd),
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

#[derive(Component, Default)]
pub struct MockFooInput {
    component: MockInput,
}

impl AppComponent<MockMsg, MockEvent> for MockFooInput {
    fn on(&mut self, ev: &Event<MockEvent>) -> Option<MockMsg> {
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
            }) => Cmd::Type(*ch),
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
            }) => return Some(MockMsg::FooSubmit(self.component.states.text.clone())),
            _ => Cmd::None,
        };
        match self.component.perform(cmd) {
            CmdResult::Changed(State::Single(StateValue::String(s))) => {
                Some(MockMsg::FooInputChanged(s))
            }
            _ => None,
        }
    }
}

#[derive(Component, Default)]
pub struct MockBarInput {
    component: MockInput,
}

impl AppComponent<MockMsg, MockEvent> for MockBarInput {
    fn on(&mut self, ev: &Event<MockEvent>) -> Option<MockMsg> {
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
            }) => Cmd::Type(*ch),
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
            }) => return Some(MockMsg::BarSubmit(self.component.states.text.clone())),
            Event::Tick => return Some(MockMsg::BarTick),
            _ => Cmd::None,
        };
        match self.component.perform(cmd) {
            CmdResult::Changed(State::Single(StateValue::String(s))) => {
                Some(MockMsg::BarInputChanged(s))
            }
            _ => None,
        }
    }
}
