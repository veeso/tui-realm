//! ## State
//!
//! This module exposes the state type and values

use crate::props::Color;
use crate::utils::{Email, PhoneNumber};
use std::collections::{HashMap, LinkedList};

/// State describes a component state
#[derive(Debug, PartialEq, Clone)]
pub enum State {
    One(StateValue),
    Tup2((StateValue, StateValue)),
    Tup3((StateValue, StateValue, StateValue)),
    Tup4((StateValue, StateValue, StateValue, StateValue)),
    Vec(Vec<StateValue>),
    Map(HashMap<String, StateValue>),
    Linked(LinkedList<State>),
    None,
}

/// StateValue describes the value contained in a State
#[derive(Debug, PartialEq, Clone)]
pub enum StateValue {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F64(f64),
    Isize(isize),
    String(String),
    // -- input types
    Color(Color),
    Email(Email),
    PhoneNumber(PhoneNumber),
}
