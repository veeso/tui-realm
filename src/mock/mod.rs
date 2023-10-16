//! # Mock
//!
//! This module contains data type for unit tests only

use std::marker::PhantomData;

use crate::event::{Event, Key, KeyEvent};
use crate::listener::{ListenerResult, Poll};
use crate::{AttrValue, Attribute, Injector};

// -- modules
mod components;
pub use components::{MockBarInput, MockFooInput, MockInput};

// -- event

/// Mock UserEvent type
#[derive(Debug, Eq, PartialEq, Clone, PartialOrd)]
pub enum MockEvent {
    None,
    Foo,
    Bar,
    Hello(String),
}

/// Mock component id type
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum MockComponentId {
    InputBar,
    InputFoo,
    InputOmar,
    Dyn(String),
}

// -- poll

/// Mock poll implementation
pub struct MockPoll<U: Eq + PartialEq + Clone + PartialOrd + Send> {
    ghost: PhantomData<U>,
}

impl<U: Eq + PartialEq + Clone + PartialOrd + Send> Default for MockPoll<U> {
    fn default() -> Self {
        Self { ghost: PhantomData }
    }
}

impl<U: Eq + PartialEq + Clone + PartialOrd + Send + 'static> Poll<U> for MockPoll<U> {
    fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
        Ok(Some(Event::Keyboard(KeyEvent::from(Key::Enter))))
    }
}

// -- msg

/// Mocked Msg for components and view
#[derive(Debug, PartialEq)]
pub enum MockMsg {
    FooInputChanged(String),
    FooSubmit(String),
    BarInputChanged(String),
    BarSubmit(String),
    BarTick,
}

// -- injector

#[derive(Default)]
pub struct MockInjector;

impl Injector<MockComponentId> for MockInjector {
    fn inject(&self, id: &MockComponentId) -> Vec<(Attribute, AttrValue)> {
        match id {
            &MockComponentId::InputBar => vec![(
                Attribute::Text,
                AttrValue::String(String::from("hello, world!")),
            )],
            _ => vec![],
        }
    }
}
