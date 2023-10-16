//! ## State
//!
//! This module exposes the state type and values

use std::collections::{HashMap, LinkedList};

use crate::props::Color;
use crate::utils::{Email, PhoneNumber};

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
    None,
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
    Isize(isize),
    F64(f64),
    String(String),
    // -- input types
    Color(Color),
    Email(Email),
    PhoneNumber(PhoneNumber),
}

impl State {
    pub fn unwrap_one(self) -> StateValue {
        match self {
            Self::One(val) => val,
            state => panic!("Could not unwrap {:?} as `One`", state),
        }
    }

    pub fn unwrap_tup2(self) -> (StateValue, StateValue) {
        match self {
            Self::Tup2(val) => val,
            state => panic!("Could not unwrap {:?} as `Tup2`", state),
        }
    }

    pub fn unwrap_tup3(self) -> (StateValue, StateValue, StateValue) {
        match self {
            Self::Tup3(val) => val,
            state => panic!("Could not unwrap {:?} as `Tup3`", state),
        }
    }

    pub fn unwrap_tup4(self) -> (StateValue, StateValue, StateValue, StateValue) {
        match self {
            Self::Tup4(val) => val,
            state => panic!("Could not unwrap {:?} as `Tup4`", state),
        }
    }

    pub fn unwrap_vec(self) -> Vec<StateValue> {
        match self {
            Self::Vec(val) => val,
            state => panic!("Could not unwrap {:?} as `Vec`", state),
        }
    }

    pub fn unwrap_map(self) -> HashMap<String, StateValue> {
        match self {
            Self::Map(val) => val,
            state => panic!("Could not unwrap {:?} as `Map`", state),
        }
    }

    pub fn unwrap_linked(self) -> LinkedList<State> {
        match self {
            Self::Linked(val) => val,
            state => panic!("Could not unwrap {:?} as `Linked`", state),
        }
    }

    /// Returns whether `State` is `State::None`
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl StateValue {
    /// Returns whether `StateValue` is `StateValue::None`
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn unwrap_bool(self) -> bool {
        match self {
            Self::Bool(val) => val,
            value => panic!("Could not unwrap {:?} as `Bool`", value),
        }
    }

    pub fn unwrap_u8(self) -> u8 {
        match self {
            Self::U8(val) => val,
            value => panic!("Could not unwrap {:?} as `U8`", value),
        }
    }

    pub fn unwrap_u16(self) -> u16 {
        match self {
            Self::U16(val) => val,
            value => panic!("Could not unwrap {:?} as `U16`", value),
        }
    }

    pub fn unwrap_u32(self) -> u32 {
        match self {
            Self::U32(val) => val,
            value => panic!("Could not unwrap {:?} as `U32`", value),
        }
    }

    pub fn unwrap_u64(self) -> u64 {
        match self {
            Self::U64(val) => val,
            value => panic!("Could not unwrap {:?} as `U64`", value),
        }
    }

    pub fn unwrap_u128(self) -> u128 {
        match self {
            Self::U128(val) => val,
            value => panic!("Could not unwrap {:?} as `U128`", value),
        }
    }

    pub fn unwrap_usize(self) -> usize {
        match self {
            Self::Usize(val) => val,
            value => panic!("Could not unwrap {:?} as `Usize`", value),
        }
    }

    pub fn unwrap_i8(self) -> i8 {
        match self {
            Self::I8(val) => val,
            value => panic!("Could not unwrap {:?} as `I8`", value),
        }
    }

    pub fn unwrap_i16(self) -> i16 {
        match self {
            Self::I16(val) => val,
            value => panic!("Could not unwrap {:?} as `I16`", value),
        }
    }

    pub fn unwrap_i32(self) -> i32 {
        match self {
            Self::I32(val) => val,
            value => panic!("Could not unwrap {:?} as `I32`", value),
        }
    }

    pub fn unwrap_i64(self) -> i64 {
        match self {
            Self::I64(val) => val,
            value => panic!("Could not unwrap {:?} as `I64`", value),
        }
    }

    pub fn unwrap_i128(self) -> i128 {
        match self {
            Self::I128(val) => val,
            value => panic!("Could not unwrap {:?} as `I128`", value),
        }
    }

    pub fn unwrap_isize(self) -> isize {
        match self {
            Self::Isize(val) => val,
            value => panic!("Could not unwrap {:?} as `Isize`", value),
        }
    }

    pub fn unwrap_f64(self) -> f64 {
        match self {
            Self::F64(val) => val,
            value => panic!("Could not unwrap {:?} as `F64`", value),
        }
    }

    pub fn unwrap_string(self) -> String {
        match self {
            Self::String(val) => val,
            value => panic!("Could not unwrap {:?} as `String`", value),
        }
    }

    pub fn unwrap_color(self) -> Color {
        match self {
            Self::Color(val) => val,
            value => panic!("Could not unwrap {:?} as `Color`", value),
        }
    }

    pub fn unwrap_email(self) -> Email {
        match self {
            Self::Email(val) => val,
            value => panic!("Could not unwrap {:?} as `Email`", value),
        }
    }

    pub fn unwrap_phone_number(self) -> PhoneNumber {
        match self {
            Self::PhoneNumber(val) => val,
            value => panic!("Could not unwrap {:?} as `PhoneNumber`", value),
        }
    }
}
