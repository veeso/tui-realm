//! # Mock
//!
//! This module contains data type for unit tests only

use crate::event::{Event, Key, KeyEvent};
use crate::listener::{ListenerResult, Poll};
use crate::Update;

use std::marker::PhantomData;

// -- modules
mod components;
pub use components::{MockBarInput, MockFooInput, MockInput};

// -- event

/// ## MockEvent
///
/// Mock UserEvent type
#[derive(Debug, Eq, PartialEq, Clone, PartialOrd)]
pub enum MockEvent {
    None,
    Foo,
    Bar,
    Hello(String),
}

/// ## MockComponentId
///
/// Mock component id type
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum MockComponentId {
    InputBar,
    InputFoo,
    InputOmar,
    Dyn(String),
}

// -- poll

/// ## MockPoll
///
/// Mock poll implementation
pub struct MockPoll<U: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send> {
    ghost: PhantomData<U>,
}

impl<U: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send> Default for MockPoll<U> {
    fn default() -> Self {
        Self {
            ghost: PhantomData::default(),
        }
    }
}

impl<U: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send + 'static> Poll<U>
    for MockPoll<U>
{
    fn poll(&mut self) -> ListenerResult<Option<Event<U>>> {
        Ok(Some(Event::Keyboard(KeyEvent::from(Key::Enter))))
    }
}

// -- msg

/// ## MockMsg
///
/// Mocked Msg for components and view
#[derive(Debug, PartialEq)]
pub enum MockMsg {
    FooInputChanged(String),
    FooSubmit(String),
    BarInputChanged(String),
    BarSubmit(String),
    BarTick,
}

#[derive(Debug)]
/// ## MockModel
///
/// Mock implementation of Update trait
pub struct MockModel {
    /// This function will call on update.
    /// Use it to call assertions for test
    validate: fn(Option<MockMsg>) -> Option<MockMsg>,
}

impl MockModel {
    pub fn new(validate: fn(Option<MockMsg>) -> Option<MockMsg>) -> Self {
        Self { validate }
    }
}

impl Update<MockComponentId, MockMsg, MockEvent> for MockModel {
    fn update(
        &mut self,
        _view: &mut crate::View<MockComponentId, MockMsg, MockEvent>,
        msg: Option<MockMsg>,
    ) -> Option<MockMsg> {
        (self.validate)(msg)
    }
}
