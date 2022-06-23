//! ## Value
//!
//! This module exposes the prop values

use super::{Alignment, Color, Dataset, InputType, Shape, Style, Table, TextSpan};
use std::collections::{HashMap, LinkedList};

// -- Prop value

/// The payload contains the actual value for user defined properties
#[derive(Debug, PartialEq, Clone)]
pub enum PropPayload {
    One(PropValue),
    Tup2((PropValue, PropValue)),
    Tup3((PropValue, PropValue, PropValue)),
    Tup4((PropValue, PropValue, PropValue, PropValue)),
    Vec(Vec<PropValue>),
    Map(HashMap<String, PropValue>),
    Linked(LinkedList<PropPayload>),
    None,
}

/// Value describes the value contained in a `PropPayload`
#[derive(Debug, PartialEq, Clone)]
pub enum PropValue {
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
    F32(f32),
    Str(String),
    // -- tui props
    Alignment(Alignment),
    Color(Color),
    Dataset(Dataset),
    InputType(InputType),
    Shape(Shape),
    Style(Style),
    Table(Table),
    TextSpan(TextSpan),
}

impl PropPayload {
    /// Unwrap a One value from PropPayload
    pub fn unwrap_one(self) -> PropValue {
        match self {
            PropPayload::One(one) => one,
            _ => panic!("Called `unwrap_one` on a bad value"),
        }
    }

    /// Unwrap a Tup2 value from PropPayload
    pub fn unwrap_tup2(self) -> (PropValue, PropValue) {
        match self {
            PropPayload::Tup2(t) => t,
            _ => panic!("Called `unwrap_tup2` on a bad value"),
        }
    }

    /// Unwrap a Tup3 value from PropPayload
    pub fn unwrap_tup3(self) -> (PropValue, PropValue, PropValue) {
        match self {
            PropPayload::Tup3(t) => t,
            _ => panic!("Called `unwrap_tup3` on a bad value"),
        }
    }

    /// Unwrap a Tup4 value from PropPayload
    pub fn unwrap_tup4(self) -> (PropValue, PropValue, PropValue, PropValue) {
        match self {
            PropPayload::Tup4(t) => t,
            _ => panic!("Called `unwrap_tup4` on a bad value"),
        }
    }

    /// Unwrap a Vec value from PropPayload
    pub fn unwrap_vec(self) -> Vec<PropValue> {
        match self {
            PropPayload::Vec(v) => v,
            _ => panic!("Called `unwrap_vec` on a bad value"),
        }
    }

    /// Unwrap a Map value from PropPayload
    pub fn unwrap_map(self) -> HashMap<String, PropValue> {
        match self {
            PropPayload::Map(m) => m,
            _ => panic!("Called `unwrap_map` on a bad value"),
        }
    }

    /// Unwrap a Linked list from PropPayload
    pub fn unwrap_linked(self) -> LinkedList<PropPayload> {
        match self {
            PropPayload::Linked(l) => l,
            _ => panic!("Called `unwrap_linked` on a bad value"),
        }
    }
}

impl PropValue {
    /// Unwrap PropValue as Bool.
    /// Panics otherwise
    pub fn unwrap_bool(self) -> bool {
        match self {
            PropValue::Bool(b) => b,
            _ => panic!("Called `unwrap_bool` on a bad value"),
        }
    }

    /// Unwrap PropValue as U8.
    /// Panics otherwise
    pub fn unwrap_u8(self) -> u8 {
        match self {
            PropValue::U8(v) => v,
            _ => panic!("Called `unwrap_u8` on a bad value"),
        }
    }

    /// Unwrap PropValue as u16.
    /// Panics otherwise
    pub fn unwrap_u16(self) -> u16 {
        match self {
            PropValue::U16(b) => b,
            _ => panic!("Called `unwrap_bool` on a bad value"),
        }
    }

    /// Unwrap PropValue as Bool.
    /// Panics otherwise
    pub fn unwrap_u32(self) -> u32 {
        match self {
            PropValue::U32(b) => b,
            _ => panic!("Called `unwrap_u32` on a bad value"),
        }
    }

    /// Unwrap PropValue as u64.
    /// Panics otherwise
    pub fn unwrap_u64(self) -> u64 {
        match self {
            PropValue::U64(b) => b,
            _ => panic!("Called `unwrap_u64` on a bad value"),
        }
    }

    /// Unwrap PropValue as u128.
    /// Panics otherwise
    pub fn unwrap_u128(self) -> u128 {
        match self {
            PropValue::U128(b) => b,
            _ => panic!("Called `unwrap_u128` on a bad value"),
        }
    }

    /// Unwrap PropValue as usize.
    /// Panics otherwise
    pub fn unwrap_usize(self) -> usize {
        match self {
            PropValue::Usize(b) => b,
            _ => panic!("Called `unwrap_usize` on a bad value"),
        }
    }

    /// Unwrap PropValue as I8.
    /// Panics otherwise
    pub fn unwrap_i8(self) -> i8 {
        match self {
            PropValue::I8(v) => v,
            _ => panic!("Called `unwrap_i8` on a bad value"),
        }
    }

    /// Unwrap PropValue as i16.
    /// Panics otherwise
    pub fn unwrap_i16(self) -> i16 {
        match self {
            PropValue::I16(b) => b,
            _ => panic!("Called `unwrap_i16` on a bad value"),
        }
    }

    /// Unwrap PropValue as i32.
    /// Panics otherwise
    pub fn unwrap_i32(self) -> i32 {
        match self {
            PropValue::I32(b) => b,
            _ => panic!("Called `unwrap_i32` on a bad value"),
        }
    }

    /// Unwrap PropValue as u64.
    /// Panics otherwise
    pub fn unwrap_i64(self) -> i64 {
        match self {
            PropValue::I64(b) => b,
            _ => panic!("Called `unwrap_i64` on a bad value"),
        }
    }

    /// Unwrap PropValue as i128.
    /// Panics otherwise
    pub fn unwrap_i128(self) -> i128 {
        match self {
            PropValue::I128(b) => b,
            _ => panic!("Called `unwrap_i128` on a bad value"),
        }
    }

    /// Unwrap PropValue as isize.
    /// Panics otherwise
    pub fn unwrap_isize(self) -> isize {
        match self {
            PropValue::Isize(b) => b,
            _ => panic!("Called `unwrap_isize` on a bad value"),
        }
    }

    /// Unwrap PropValue as f3.
    /// Panics otherwise
    pub fn unwrap_f32(self) -> f32 {
        match self {
            PropValue::F32(b) => b,
            _ => panic!("Called `unwrap_f32` on a bad value"),
        }
    }

    /// Unwrap PropValue as F64.
    /// Panics otherwise
    pub fn unwrap_f64(self) -> f64 {
        match self {
            PropValue::F64(b) => b,
            _ => panic!("Called `unwrap_f64` on a bad value"),
        }
    }

    /// Unwrap PropValue as str.
    /// Panics otherwise
    pub fn unwrap_str(self) -> String {
        match self {
            PropValue::Str(s) => s,
            _ => panic!("Called `unwrap_str` on a bad value"),
        }
    }

    /// Unwrap PropValue as alignment.
    /// Panics otherwise
    pub fn unwrap_alignment(self) -> Alignment {
        match self {
            PropValue::Alignment(b) => b,
            _ => panic!("Called `unwrap_alignment` on a bad value"),
        }
    }

    /// Unwrap PropValue as Dataset.
    /// Panics otherwise
    pub fn unwrap_dataset(self) -> Dataset {
        match self {
            PropValue::Dataset(b) => b,
            _ => panic!("Called `unwrap_dataset` on a bad value"),
        }
    }

    /// Unwrap PropValue as InputType.
    /// Panics otherwise
    pub fn unwrap_input_type(self) -> InputType {
        match self {
            PropValue::InputType(b) => b,
            _ => panic!("Called `unwrap_input_type` on a bad value"),
        }
    }

    /// Unwrap PropValue as shape.
    /// Panics otherwise
    pub fn unwrap_shape(self) -> Shape {
        match self {
            PropValue::Shape(b) => b,
            _ => panic!("Called `unwrap_shape` on a bad value"),
        }
    }

    /// Unwrap PropValue as Style.
    /// Panics otherwise
    pub fn unwrap_style(self) -> Style {
        match self {
            PropValue::Style(b) => b,
            _ => panic!("Called `unwrap_style` on a bad value"),
        }
    }

    /// Unwrap PropValue as Bool.
    /// Panics otherwise
    pub fn unwrap_text_span(self) -> TextSpan {
        match self {
            PropValue::TextSpan(b) => b,
            _ => panic!("Called `unwrap_text_span` on a bad value"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::tui::widgets::canvas::Map;
    use std::collections::HashMap;

    #[test]
    fn prop_values() {
        PropPayload::One(PropValue::Usize(2));
        PropPayload::Tup2((PropValue::Bool(true), PropValue::Usize(128)));
        PropPayload::Tup3((
            PropValue::Bool(true),
            PropValue::Usize(128),
            PropValue::Str(String::from("omar")),
        ));
        PropPayload::Tup4((
            PropValue::Bool(true),
            PropValue::U8(128),
            PropValue::Str(String::from("pippo")),
            PropValue::Isize(-2),
        ));
        PropPayload::Vec(vec![
            PropValue::U16(1),
            PropValue::U32(2),
            PropValue::U64(3),
            PropValue::U128(4),
        ]);
        let mut map: HashMap<String, PropValue> = HashMap::new();
        map.insert(String::from("a"), PropValue::I8(4));
        assert_eq!(*map.get("a").unwrap(), PropValue::I8(4));
        map.insert(String::from("b"), PropValue::I16(-8));
        assert_eq!(*map.get("b").unwrap(), PropValue::I16(-8));
        map.insert(String::from("c"), PropValue::I32(16));
        assert_eq!(*map.get("c").unwrap(), PropValue::I32(16));
        map.insert(String::from("d"), PropValue::I64(-32));
        assert_eq!(*map.get("d").unwrap(), PropValue::I64(-32));
        map.insert(String::from("e"), PropValue::I128(64));
        assert_eq!(*map.get("e").unwrap(), PropValue::I128(64));
        map.insert(String::from("g"), PropValue::InputType(InputType::Number));
        assert_eq!(
            *map.get("g").unwrap(),
            PropValue::InputType(InputType::Number)
        );
        map.insert(String::from("h"), PropValue::U8(0));
        assert_eq!(*map.get("h").unwrap(), PropValue::U8(0));
        map.insert(String::from("i"), PropValue::Bool(true));
        assert_eq!(*map.get("i").unwrap(), PropValue::Bool(true));
        map.insert(String::from("j"), PropValue::U16(256));
        assert_eq!(*map.get("j").unwrap(), PropValue::U16(256));
        map.insert(String::from("k"), PropValue::U32(65536));
        assert_eq!(*map.get("k").unwrap(), PropValue::U32(65536));
        map.insert(String::from("l"), PropValue::U64(10000000000));
        assert_eq!(*map.get("l").unwrap(), PropValue::U64(10000000000));
        map.insert(String::from("m"), PropValue::Isize(200));
        assert_eq!(*map.get("m").unwrap(), PropValue::Isize(200));
        map.insert(String::from("n"), PropValue::F32(0.23));
        assert_eq!(*map.get("n").unwrap(), PropValue::F32(0.23));
        map.insert(
            String::from("p"),
            PropValue::Style(Style::default().fg(Color::Red)),
        );
        assert_eq!(
            *map.get("p").unwrap(),
            PropValue::Style(Style::default().fg(Color::Red))
        );
        map.insert(
            String::from("q"),
            PropValue::Dataset(Dataset::default().name("omar")),
        );
        assert_eq!(
            *map.get("q").unwrap(),
            PropValue::Dataset(Dataset::default().name("omar"))
        );
        map.insert(String::from("s"), PropValue::InputType(InputType::Number));
        assert_eq!(
            *map.get("s").unwrap(),
            PropValue::InputType(InputType::Number)
        );
        map.insert(
            String::from("t"),
            PropValue::Shape(Shape::Map(Map::default())),
        );
        assert_eq!(
            *map.get("t").unwrap(),
            PropValue::Shape(Shape::Map(Map::default()))
        );
        map.insert(String::from("u"), PropValue::Alignment(Alignment::Center));
        assert_eq!(
            *map.get("u").unwrap(),
            PropValue::Alignment(Alignment::Center)
        );
        PropPayload::Map(map);
        let mut link: LinkedList<PropPayload> = LinkedList::new();
        link.push_back(PropPayload::One(PropValue::Usize(1)));
        link.push_back(PropPayload::Tup2((
            PropValue::Usize(2),
            PropValue::Usize(4),
        )));
        PropPayload::Linked(link);
    }

    #[test]
    fn unwrap_prop_values() {
        assert_eq!(
            PropValue::Alignment(Alignment::Center).unwrap_alignment(),
            Alignment::Center
        );
        assert_eq!(PropValue::Bool(true).unwrap_bool(), true);
        assert_eq!(
            PropValue::Dataset(Dataset::default()).unwrap_dataset(),
            Dataset::default()
        );
        assert_eq!(PropValue::F32(0.32).unwrap_f32(), 0.32);
        assert_eq!(PropValue::F64(0.32).unwrap_f64(), 0.32);
        assert_eq!(PropValue::I128(5).unwrap_i128(), 5);
        assert_eq!(PropValue::I64(5).unwrap_i64(), 5);
        assert_eq!(PropValue::I32(5).unwrap_i32(), 5);
        assert_eq!(PropValue::I16(5).unwrap_i16(), 5);
        assert_eq!(PropValue::I8(5).unwrap_i8(), 5);
        assert_eq!(PropValue::Isize(5).unwrap_isize(), 5);
        assert_eq!(PropValue::U128(5).unwrap_u128(), 5);
        assert_eq!(PropValue::U64(5).unwrap_u64(), 5);
        assert_eq!(PropValue::U32(5).unwrap_u32(), 5);
        assert_eq!(PropValue::U16(5).unwrap_u16(), 5);
        assert_eq!(PropValue::U8(5).unwrap_u8(), 5);
        assert_eq!(PropValue::Usize(5).unwrap_usize(), 5);
        assert_eq!(
            PropValue::InputType(InputType::Number).unwrap_input_type(),
            InputType::Number
        );
        assert_eq!(PropValue::Shape(Shape::Layer).unwrap_shape(), Shape::Layer);
        assert_eq!(
            PropValue::Str(String::from("ciao")).unwrap_str(),
            "ciao".to_string()
        );
        assert_eq!(
            PropValue::Style(Style::default()).unwrap_style(),
            Style::default()
        );
        assert_eq!(
            PropValue::TextSpan(TextSpan::from("ciao")).unwrap_text_span(),
            TextSpan::from("ciao")
        );
    }

    #[test]
    fn unwrap_prop_payloads() {
        assert_eq!(
            PropPayload::One(PropValue::Bool(false))
                .unwrap_one()
                .unwrap_bool(),
            false
        );
        assert_eq!(
            PropPayload::Tup2((PropValue::Bool(false), PropValue::Bool(false))).unwrap_tup2(),
            (PropValue::Bool(false), PropValue::Bool(false))
        );
        assert_eq!(
            PropPayload::Tup3((
                PropValue::Bool(false),
                PropValue::Bool(false),
                PropValue::Bool(false)
            ))
            .unwrap_tup3(),
            (
                PropValue::Bool(false),
                PropValue::Bool(false),
                PropValue::Bool(false)
            )
        );
        assert_eq!(
            PropPayload::Tup4((
                PropValue::Bool(false),
                PropValue::Bool(false),
                PropValue::Bool(false),
                PropValue::Bool(false)
            ))
            .unwrap_tup4(),
            (
                PropValue::Bool(false),
                PropValue::Bool(false),
                PropValue::Bool(false),
                PropValue::Bool(false)
            )
        );
        assert_eq!(
            PropPayload::Vec(vec![PropValue::Bool(false), PropValue::Bool(false)]).unwrap_vec(),
            &[PropValue::Bool(false), PropValue::Bool(false)]
        );
    }
}
