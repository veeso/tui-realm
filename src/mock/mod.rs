//! # Mock
//!
//! This module contains data type for unit tests only

use crate::event::{Event, Key, KeyEvent};
use crate::listener::{ListenerResult, Poll};

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

impl<U: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send> Poll<U> for MockPoll<U> {
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
    BarInputChanged(String),
}
