//! ## State
//!
//! This module exposes the state type and values

use std::collections::{HashMap, LinkedList};

use crate::props::{AnyPropBox, Color};
use crate::utils::{Email, PhoneNumber};

/// State describes a component state
#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Single(StateValue),
    Pair((StateValue, StateValue)),
    Vec(Vec<StateValue>),
    Map(HashMap<String, StateValue>),
    Linked(LinkedList<State>),
    Any(AnyPropBox),
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
    pub fn unwrap_single(self) -> StateValue {
        match self {
            Self::Single(val) => val,
            state => panic!("Could not unwrap {state:?} as `Single`"),
        }
    }

    pub fn unwrap_pair(self) -> (StateValue, StateValue) {
        match self {
            Self::Pair(val) => val,
            state => panic!("Could not unwrap {state:?} as `Pair`"),
        }
    }

    pub fn unwrap_vec(self) -> Vec<StateValue> {
        match self {
            Self::Vec(val) => val,
            state => panic!("Could not unwrap {state:?} as `Vec`"),
        }
    }

    pub fn unwrap_map(self) -> HashMap<String, StateValue> {
        match self {
            Self::Map(val) => val,
            state => panic!("Could not unwrap {state:?} as `Map`"),
        }
    }

    pub fn unwrap_linked(self) -> LinkedList<State> {
        match self {
            Self::Linked(val) => val,
            state => panic!("Could not unwrap {state:?} as `Linked`"),
        }
    }

    pub fn unwrap_any(self) -> AnyPropBox {
        match self {
            Self::Any(val) => val,
            state => panic!("Could not unwrap {state:?} as `Any`"),
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
            value => panic!("Could not unwrap {value:?} as `Bool`"),
        }
    }

    pub fn unwrap_u8(self) -> u8 {
        match self {
            Self::U8(val) => val,
            value => panic!("Could not unwrap {value:?} as `U8`"),
        }
    }

    pub fn unwrap_u16(self) -> u16 {
        match self {
            Self::U16(val) => val,
            value => panic!("Could not unwrap {value:?} as `U16`"),
        }
    }

    pub fn unwrap_u32(self) -> u32 {
        match self {
            Self::U32(val) => val,
            value => panic!("Could not unwrap {value:?} as `U32`"),
        }
    }

    pub fn unwrap_u64(self) -> u64 {
        match self {
            Self::U64(val) => val,
            value => panic!("Could not unwrap {value:?} as `U64`"),
        }
    }

    pub fn unwrap_u128(self) -> u128 {
        match self {
            Self::U128(val) => val,
            value => panic!("Could not unwrap {value:?} as `U128`"),
        }
    }

    pub fn unwrap_usize(self) -> usize {
        match self {
            Self::Usize(val) => val,
            value => panic!("Could not unwrap {value:?} as `Usize`"),
        }
    }

    pub fn unwrap_i8(self) -> i8 {
        match self {
            Self::I8(val) => val,
            value => panic!("Could not unwrap {value:?} as `I8`"),
        }
    }

    pub fn unwrap_i16(self) -> i16 {
        match self {
            Self::I16(val) => val,
            value => panic!("Could not unwrap {value:?} as `I16`"),
        }
    }

    pub fn unwrap_i32(self) -> i32 {
        match self {
            Self::I32(val) => val,
            value => panic!("Could not unwrap {value:?} as `I32`"),
        }
    }

    pub fn unwrap_i64(self) -> i64 {
        match self {
            Self::I64(val) => val,
            value => panic!("Could not unwrap {value:?} as `I64`"),
        }
    }

    pub fn unwrap_i128(self) -> i128 {
        match self {
            Self::I128(val) => val,
            value => panic!("Could not unwrap {value:?} as `I128`"),
        }
    }

    pub fn unwrap_isize(self) -> isize {
        match self {
            Self::Isize(val) => val,
            value => panic!("Could not unwrap {value:?} as `Isize`"),
        }
    }

    pub fn unwrap_f64(self) -> f64 {
        match self {
            Self::F64(val) => val,
            value => panic!("Could not unwrap {value:?} as `F64`"),
        }
    }

    pub fn unwrap_string(self) -> String {
        match self {
            Self::String(val) => val,
            value => panic!("Could not unwrap {value:?} as `String`"),
        }
    }

    pub fn unwrap_color(self) -> Color {
        match self {
            Self::Color(val) => val,
            value => panic!("Could not unwrap {value:?} as `Color`"),
        }
    }

    pub fn unwrap_email(self) -> Email {
        match self {
            Self::Email(val) => val,
            value => panic!("Could not unwrap {value:?} as `Email`"),
        }
    }

    pub fn unwrap_phone_number(self) -> PhoneNumber {
        match self {
            Self::PhoneNumber(val) => val,
            value => panic!("Could not unwrap {value:?} as `PhoneNumber`"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::State;
    use crate::props::PropBound;

    #[test]
    fn any() {
        #[derive(Debug, Clone, Copy, PartialEq)]
        struct SomeCustomType {
            field1: bool,
            field2: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        struct SomeDifferentCustomType {
            field1: bool,
        }

        let input = SomeCustomType {
            field1: true,
            field2: false,
        };
        let single_value = State::Any(input.to_any_prop());

        assert_eq!(
            single_value,
            State::Any(
                SomeCustomType {
                    field1: true,
                    field2: false
                }
                .to_any_prop()
            )
        );
        assert_ne!(
            single_value,
            State::Any(
                SomeCustomType {
                    field1: false,
                    field2: true
                }
                .to_any_prop()
            )
        );

        assert_ne!(
            single_value,
            State::Any(SomeDifferentCustomType { field1: true }.to_any_prop())
        );

        #[derive(Debug, Clone, PartialEq)]
        struct CloneableType {
            field1: String,
        }

        let input = State::Any(
            CloneableType {
                field1: "Hello".to_string(),
            }
            .to_any_prop(),
        );
        let cloned = input.clone();

        assert_eq!(input, cloned);
        let input_downcasted = match &input {
            State::Any(v) => v,
            _ => unimplemented!(),
        }
        .as_any()
        .downcast_ref::<CloneableType>()
        .expect("Erased type should be CloneableType");
        let cloned_downcasted = match &cloned {
            State::Any(v) => v,
            _ => unimplemented!(),
        }
        .as_any()
        .downcast_ref::<CloneableType>()
        .expect("Erased type should be CloneableType");
        // should be cloned and so not have the same memory pointer
        assert_ne!(
            input_downcasted.field1.as_ptr(),
            cloned_downcasted.field1.as_ptr()
        );

        let mut changed_data = cloned;

        let downcasted = match &mut changed_data {
            State::Any(v) => v,
            _ => unimplemented!(),
        }
        .as_any_mut()
        .downcast_mut::<CloneableType>()
        .expect("Erased type should be CloneableType");

        downcasted.field1 = "Changed later".to_string();

        assert_ne!(input_downcasted, downcasted);
    }
}
