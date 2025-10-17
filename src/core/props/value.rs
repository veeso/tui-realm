//! ## Value
//!
//! This module exposes the prop values

use std::collections::{HashMap, LinkedList};

use super::{Alignment, Color, Dataset, InputType, Shape, Style, Table, TextSpan};
use crate::props::AnyProp;

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
    Any(AnyProp),
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
    // -- unwrappers

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

    /// Unwrap a Any from PropPayload
    pub fn unwrap_any(self) -> AnyProp {
        match self {
            PropPayload::Any(l) => l,
            _ => panic!("Called `unwrap_any` on a bad value"),
        }
    }

    // -- as reference

    /// Get a One value from PropPayload, or None
    pub fn as_one(&self) -> Option<&PropValue> {
        match self {
            PropPayload::One(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Tup2 value from PropPayload, or None
    pub fn as_tup2(&self) -> Option<&(PropValue, PropValue)> {
        match self {
            PropPayload::Tup2(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Tup3 value from PropPayload, or None
    pub fn as_tup3(&self) -> Option<&(PropValue, PropValue, PropValue)> {
        match self {
            PropPayload::Tup3(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Tup4 value from PropPayload, or None
    pub fn as_tup4(&self) -> Option<&(PropValue, PropValue, PropValue, PropValue)> {
        match self {
            PropPayload::Tup4(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Vec value from PropPayload, or None
    pub fn as_vec(&self) -> Option<&Vec<PropValue>> {
        match self {
            PropPayload::Vec(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Map value from PropPayload, or None
    pub fn as_map(&self) -> Option<&HashMap<String, PropValue>> {
        match self {
            PropPayload::Map(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Linked value from PropPayload, or None
    pub fn as_linked(&self) -> Option<&LinkedList<PropPayload>> {
        match self {
            PropPayload::Linked(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Any value from PropPayload, or None
    pub fn as_any(&self) -> Option<&AnyProp> {
        match self {
            PropPayload::Any(v) => Some(v),
            _ => None,
        }
    }

    // -- as mutable references

    /// Get a One value from PropPayload, or None
    pub fn as_one_mut(&mut self) -> Option<&mut PropValue> {
        match self {
            PropPayload::One(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Tup2 value from PropPayload, or None
    pub fn as_tup2_mut(&mut self) -> Option<&mut (PropValue, PropValue)> {
        match self {
            PropPayload::Tup2(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Tup3 value from PropPayload, or None
    pub fn as_tup3_mut(&mut self) -> Option<&mut (PropValue, PropValue, PropValue)> {
        match self {
            PropPayload::Tup3(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Tup4 value from PropPayload, or None
    pub fn as_tup4_mut(&mut self) -> Option<&mut (PropValue, PropValue, PropValue, PropValue)> {
        match self {
            PropPayload::Tup4(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Vec value from PropPayload, or None
    pub fn as_vec_mut(&mut self) -> Option<&mut Vec<PropValue>> {
        match self {
            PropPayload::Vec(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Map value from PropPayload, or None
    pub fn as_map_mut(&mut self) -> Option<&mut HashMap<String, PropValue>> {
        match self {
            PropPayload::Map(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Linked value from PropPayload, or None
    pub fn as_linked_mut(&mut self) -> Option<&mut LinkedList<PropPayload>> {
        match self {
            PropPayload::Linked(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Any value from PropPayload, or None
    pub fn as_any_mut(&mut self) -> Option<&mut AnyProp> {
        match self {
            PropPayload::Any(v) => Some(v),
            _ => None,
        }
    }
}

impl PropValue {
    // -- unwrappers

    /// Unwrap PropValue as Bool.
    /// Panics otherwise
    pub fn unwrap_bool(self) -> bool {
        match self {
            PropValue::Bool(b) => b,
            _ => panic!("Called `unwrap_bool` on a bad value"),
        }
    }

    /// Unwrap PropValue as u8.
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

    /// Unwrap PropValue as i8.
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

    /// Unwrap PropValue as i64.
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

    /// Unwrap PropValue as f32.
    /// Panics otherwise
    pub fn unwrap_f32(self) -> f32 {
        match self {
            PropValue::F32(b) => b,
            _ => panic!("Called `unwrap_f32` on a bad value"),
        }
    }

    /// Unwrap PropValue as f64.
    /// Panics otherwise
    pub fn unwrap_f64(self) -> f64 {
        match self {
            PropValue::F64(b) => b,
            _ => panic!("Called `unwrap_f64` on a bad value"),
        }
    }

    /// Unwrap PropValue as String.
    /// Panics otherwise
    pub fn unwrap_str(self) -> String {
        match self {
            PropValue::Str(s) => s,
            _ => panic!("Called `unwrap_str` on a bad value"),
        }
    }

    /// Unwrap PropValue as Alignment.
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

    /// Unwrap PropValue as Shape.
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

    /// Unwrap PropValue as TextSpan.
    /// Panics otherwise
    pub fn unwrap_text_span(self) -> TextSpan {
        match self {
            PropValue::TextSpan(b) => b,
            _ => panic!("Called `unwrap_text_span` on a bad value"),
        }
    }

    // -- as reference

    /// Get a Bool value from PropValue, or None
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            // cheap copy, so no reference
            PropValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a u8 value from PropValue, or None
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            // cheap copy, so no reference
            PropValue::U8(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a u16 value from PropValue, or None
    pub fn as_u16(&self) -> Option<u16> {
        match self {
            // cheap copy, so no reference
            PropValue::U16(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a u32 value from PropValue, or None
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            // cheap copy, so no reference
            PropValue::U32(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a u64 value from PropValue, or None
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            // cheap copy, so no reference
            PropValue::U64(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a u128 value from PropValue, or None
    pub fn as_u128(&self) -> Option<u128> {
        match self {
            // cheap copy, so no reference
            PropValue::U128(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a usize value from PropValue, or None
    pub fn as_usize(&self) -> Option<usize> {
        match self {
            // cheap copy, so no reference
            PropValue::Usize(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a i8 value from PropValue, or None
    pub fn as_i8(&self) -> Option<i8> {
        match self {
            // cheap copy, so no reference
            PropValue::I8(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a i16 value from PropValue, or None
    pub fn as_i16(&self) -> Option<i16> {
        match self {
            // cheap copy, so no reference
            PropValue::I16(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a i32 value from PropValue, or None
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            // cheap copy, so no reference
            PropValue::I32(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a i64 value from PropValue, or None
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            // cheap copy, so no reference
            PropValue::I64(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a i128 value from PropValue, or None
    pub fn as_i128(&self) -> Option<i128> {
        match self {
            // cheap copy, so no reference
            PropValue::I128(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a isize value from PropValue, or None
    pub fn as_isize(&self) -> Option<isize> {
        match self {
            // cheap copy, so no reference
            PropValue::Isize(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a f32 value from PropValue, or None
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            // cheap copy, so no reference
            PropValue::F32(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a f64 value from PropValue, or None
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            // cheap copy, so no reference
            PropValue::F64(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a String value from PropValue, or None
    pub fn as_str(&self) -> Option<&String> {
        match self {
            PropValue::Str(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Alignment value from PropValue, or None
    pub fn as_alignment(&self) -> Option<Alignment> {
        match self {
            // cheap copy, so no reference
            PropValue::Alignment(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Dataset value from PropValue, or None
    pub fn as_dataset(&self) -> Option<&Dataset> {
        match self {
            PropValue::Dataset(v) => Some(v),
            _ => None,
        }
    }

    /// Get a InputType value from PropValue, or None
    pub fn as_input_type(&self) -> Option<&InputType> {
        match self {
            PropValue::InputType(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Shape value from PropValue, or None
    pub fn as_shape(&self) -> Option<&Shape> {
        match self {
            PropValue::Shape(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Style value from PropValue, or None
    pub fn as_style(&self) -> Option<&Style> {
        match self {
            PropValue::Style(v) => Some(v),
            _ => None,
        }
    }

    /// Get a TextSpan value from PropValue, or None
    pub fn as_text_span(&self) -> Option<&TextSpan> {
        match self {
            PropValue::TextSpan(v) => Some(v),
            _ => None,
        }
    }

    // -- as mutable references

    /// Get a Bool value from PropValue, or None
    pub fn as_bool_mut(&mut self) -> Option<&mut bool> {
        match self {
            PropValue::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// Get a u8 value from PropValue, or None
    pub fn as_u8_mut(&mut self) -> Option<&mut u8> {
        match self {
            PropValue::U8(v) => Some(v),
            _ => None,
        }
    }

    /// Get a u16 value from PropValue, or None
    pub fn as_u16_mut(&mut self) -> Option<&mut u16> {
        match self {
            PropValue::U16(v) => Some(v),
            _ => None,
        }
    }

    /// Get a u32 value from PropValue, or None
    pub fn as_u32_mut(&mut self) -> Option<&mut u32> {
        match self {
            PropValue::U32(v) => Some(v),
            _ => None,
        }
    }

    /// Get a u64 value from PropValue, or None
    pub fn as_u64_mut(&mut self) -> Option<&mut u64> {
        match self {
            PropValue::U64(v) => Some(v),
            _ => None,
        }
    }

    /// Get a u128 value from PropValue, or None
    pub fn as_u128_mut(&mut self) -> Option<&mut u128> {
        match self {
            PropValue::U128(v) => Some(v),
            _ => None,
        }
    }

    /// Get a usize value from PropValue, or None
    pub fn as_usize_mut(&mut self) -> Option<&mut usize> {
        match self {
            PropValue::Usize(v) => Some(v),
            _ => None,
        }
    }

    /// Get a i8 value from PropValue, or None
    pub fn as_i8_mut(&mut self) -> Option<&mut i8> {
        match self {
            PropValue::I8(v) => Some(v),
            _ => None,
        }
    }

    /// Get a i16 value from PropValue, or None
    pub fn as_i16_mut(&mut self) -> Option<&mut i16> {
        match self {
            PropValue::I16(v) => Some(v),
            _ => None,
        }
    }

    /// Get a i32 value from PropValue, or None
    pub fn as_i32_mut(&mut self) -> Option<&mut i32> {
        match self {
            PropValue::I32(v) => Some(v),
            _ => None,
        }
    }

    /// Get a i64 value from PropValue, or None
    pub fn as_i64_mut(&mut self) -> Option<&mut i64> {
        match self {
            PropValue::I64(v) => Some(v),
            _ => None,
        }
    }

    /// Get a i128 value from PropValue, or None
    pub fn as_i128_mut(&mut self) -> Option<&mut i128> {
        match self {
            PropValue::I128(v) => Some(v),
            _ => None,
        }
    }

    /// Get a isize value from PropValue, or None
    pub fn as_isize_mut(&mut self) -> Option<&mut isize> {
        match self {
            PropValue::Isize(v) => Some(v),
            _ => None,
        }
    }

    /// Get a f32 value from PropValue, or None
    pub fn as_f32_mut(&mut self) -> Option<&mut f32> {
        match self {
            PropValue::F32(v) => Some(v),
            _ => None,
        }
    }

    /// Get a f64 value from PropValue, or None
    pub fn as_f64_mut(&mut self) -> Option<&mut f64> {
        match self {
            PropValue::F64(v) => Some(v),
            _ => None,
        }
    }

    /// Get a String value from PropValue, or None
    pub fn as_str_mut(&mut self) -> Option<&mut String> {
        match self {
            PropValue::Str(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Alignment value from PropValue, or None
    pub fn as_alignment_mut(&mut self) -> Option<&mut Alignment> {
        match self {
            PropValue::Alignment(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Dataset value from PropValue, or None
    pub fn as_dataset_mut(&mut self) -> Option<&mut Dataset> {
        match self {
            PropValue::Dataset(v) => Some(v),
            _ => None,
        }
    }

    /// Get a InputType value from PropValue, or None
    pub fn as_input_type_mut(&mut self) -> Option<&mut InputType> {
        match self {
            PropValue::InputType(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Shape value from PropValue, or None
    pub fn as_shape_mut(&mut self) -> Option<&mut Shape> {
        match self {
            PropValue::Shape(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Style value from PropValue, or None
    pub fn as_style_mut(&mut self) -> Option<&mut Style> {
        match self {
            PropValue::Style(v) => Some(v),
            _ => None,
        }
    }

    /// Get a TextSpan value from PropValue, or None
    pub fn as_text_span_mut(&mut self) -> Option<&mut TextSpan> {
        match self {
            PropValue::TextSpan(v) => Some(v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;
    use crate::props::{PropBound, PropBoundExt};
    use crate::ratatui::widgets::canvas::Map;

    #[test]
    fn prop_values() {
        // test that values can be created without compile errors
        let _ = PropPayload::One(PropValue::Usize(2));
        let _ = PropPayload::Tup2((PropValue::Bool(true), PropValue::Usize(128)));
        let _ = PropPayload::Tup3((
            PropValue::Bool(true),
            PropValue::Usize(128),
            PropValue::Str(String::from("omar")),
        ));
        let _ = PropPayload::Tup4((
            PropValue::Bool(true),
            PropValue::U8(128),
            PropValue::Str(String::from("pippo")),
            PropValue::Isize(-2),
        ));
        let _ = PropPayload::Vec(vec![
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

        let value = map.get_mut("u").unwrap();
        *value.as_alignment_mut().unwrap() = Alignment::Left;
        assert_eq!(
            *map.get("u").unwrap(),
            PropValue::Alignment(Alignment::Left)
        );

        let _ = PropPayload::Map(map);
        let mut link: LinkedList<PropPayload> = LinkedList::new();
        link.push_back(PropPayload::One(PropValue::Usize(1)));
        link.push_back(PropPayload::Tup2((
            PropValue::Usize(2),
            PropValue::Usize(4),
        )));
        let _ = PropPayload::Linked(link);
    }

    #[test]
    fn unwrap_prop_values() {
        assert_eq!(
            PropValue::Alignment(Alignment::Center).unwrap_alignment(),
            Alignment::Center
        );
        assert!(PropValue::Bool(true).unwrap_bool());
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
    fn as_prop_value() {
        assert_eq!(PropValue::Bool(true).as_bool(), Some(true));
        assert_eq!(PropValue::U8(0).as_bool(), None);

        assert_eq!(PropValue::U8(1).as_u8(), Some(1));
        assert_eq!(PropValue::Bool(true).as_u8(), None);

        assert_eq!(PropValue::U16(1).as_u16(), Some(1));
        assert_eq!(PropValue::Bool(true).as_u16(), None);

        assert_eq!(PropValue::U32(1).as_u32(), Some(1));
        assert_eq!(PropValue::Bool(true).as_u32(), None);

        assert_eq!(PropValue::U64(1).as_u64(), Some(1));
        assert_eq!(PropValue::Bool(true).as_u64(), None);

        assert_eq!(PropValue::U128(1).as_u128(), Some(1));
        assert_eq!(PropValue::Bool(true).as_u128(), None);

        assert_eq!(PropValue::Usize(1).as_usize(), Some(1));
        assert_eq!(PropValue::Bool(true).as_usize(), None);

        assert_eq!(PropValue::I8(-1).as_i8(), Some(-1));
        assert_eq!(PropValue::Bool(true).as_i8(), None);

        assert_eq!(PropValue::I16(-1).as_i16(), Some(-1));
        assert_eq!(PropValue::Bool(true).as_i16(), None);

        assert_eq!(PropValue::I32(-1).as_i32(), Some(-1));
        assert_eq!(PropValue::Bool(true).as_i32(), None);

        assert_eq!(PropValue::I64(-1).as_i64(), Some(-1));
        assert_eq!(PropValue::Bool(true).as_i64(), None);

        assert_eq!(PropValue::I128(-1).as_i128(), Some(-1));
        assert_eq!(PropValue::Bool(true).as_i128(), None);

        assert_eq!(PropValue::Isize(-1).as_isize(), Some(-1));
        assert_eq!(PropValue::Bool(true).as_isize(), None);

        assert_eq!(PropValue::F32(1.1).as_f32(), Some(1.1));
        assert_eq!(PropValue::Bool(true).as_f32(), None);

        assert_eq!(PropValue::F64(1.1).as_f64(), Some(1.1));
        assert_eq!(PropValue::Bool(true).as_f64(), None);

        assert_eq!(
            PropValue::Str("hello".to_string()).as_str(),
            Some(&"hello".to_string())
        );
        assert_eq!(PropValue::Bool(true).as_str(), None);

        assert_eq!(
            PropValue::Alignment(Alignment::Center).as_alignment(),
            Some(Alignment::Center)
        );
        assert_eq!(PropValue::Bool(true).as_alignment(), None);

        assert_eq!(
            PropValue::Dataset(Dataset::default()).as_dataset(),
            Some(&Dataset::default())
        );
        assert_eq!(PropValue::Bool(true).as_dataset(), None);

        assert_eq!(
            PropValue::InputType(InputType::Color).as_input_type(),
            Some(&InputType::Color)
        );
        assert_eq!(PropValue::Bool(true).as_input_type(), None);

        assert_eq!(
            PropValue::Shape(Shape::Layer).as_shape(),
            Some(&Shape::Layer)
        );
        assert_eq!(PropValue::Bool(true).as_shape(), None);

        assert_eq!(
            PropValue::Style(Style::new()).as_style(),
            Some(&Style::new())
        );
        assert_eq!(PropValue::Bool(true).as_style(), None);

        assert_eq!(
            PropValue::TextSpan(TextSpan::new("hello")).as_text_span(),
            Some(&TextSpan::new("hello"))
        );
        assert_eq!(PropValue::Bool(true).as_text_span(), None);
    }

    #[test]
    fn as_prop_value_mut() {
        assert_eq!(PropValue::Bool(true).as_bool_mut(), Some(&mut true));
        assert_eq!(PropValue::U8(0).as_bool_mut(), None);

        assert_eq!(PropValue::U8(1).as_u8_mut(), Some(&mut 1));
        assert_eq!(PropValue::Bool(true).as_u8_mut(), None);

        assert_eq!(PropValue::U16(1).as_u16_mut(), Some(&mut 1));
        assert_eq!(PropValue::Bool(true).as_u16_mut(), None);

        assert_eq!(PropValue::U32(1).as_u32_mut(), Some(&mut 1));
        assert_eq!(PropValue::Bool(true).as_u32_mut(), None);

        assert_eq!(PropValue::U64(1).as_u64_mut(), Some(&mut 1));
        assert_eq!(PropValue::Bool(true).as_u64_mut(), None);

        assert_eq!(PropValue::U128(1).as_u128_mut(), Some(&mut 1));
        assert_eq!(PropValue::Bool(true).as_u128_mut(), None);

        assert_eq!(PropValue::Usize(1).as_usize_mut(), Some(&mut 1));
        assert_eq!(PropValue::Bool(true).as_usize_mut(), None);

        assert_eq!(PropValue::I8(-1).as_i8_mut(), Some(&mut -1));
        assert_eq!(PropValue::Bool(true).as_i8_mut(), None);

        assert_eq!(PropValue::I16(-1).as_i16_mut(), Some(&mut -1));
        assert_eq!(PropValue::Bool(true).as_i16_mut(), None);

        assert_eq!(PropValue::I32(-1).as_i32_mut(), Some(&mut -1));
        assert_eq!(PropValue::Bool(true).as_i32_mut(), None);

        assert_eq!(PropValue::I64(-1).as_i64_mut(), Some(&mut -1));
        assert_eq!(PropValue::Bool(true).as_i64_mut(), None);

        assert_eq!(PropValue::I128(-1).as_i128_mut(), Some(&mut -1));
        assert_eq!(PropValue::Bool(true).as_i128_mut(), None);

        assert_eq!(PropValue::Isize(-1).as_isize_mut(), Some(&mut -1));
        assert_eq!(PropValue::Bool(true).as_isize_mut(), None);

        assert_eq!(PropValue::F32(1.1).as_f32_mut(), Some(&mut 1.1));
        assert_eq!(PropValue::Bool(true).as_f32_mut(), None);

        assert_eq!(PropValue::F64(1.1).as_f64_mut(), Some(&mut 1.1));
        assert_eq!(PropValue::Bool(true).as_f64_mut(), None);

        assert_eq!(
            PropValue::Str("hello".to_string()).as_str_mut(),
            Some(&mut "hello".to_string())
        );
        assert_eq!(PropValue::Bool(true).as_str_mut(), None);

        assert_eq!(
            PropValue::Alignment(Alignment::Center).as_alignment_mut(),
            Some(&mut Alignment::Center)
        );
        assert_eq!(PropValue::Bool(true).as_alignment_mut(), None);

        assert_eq!(
            PropValue::Dataset(Dataset::default()).as_dataset_mut(),
            Some(&mut Dataset::default())
        );
        assert_eq!(PropValue::Bool(true).as_dataset_mut(), None);

        assert_eq!(
            PropValue::InputType(InputType::Color).as_input_type_mut(),
            Some(&mut InputType::Color)
        );
        assert_eq!(PropValue::Bool(true).as_input_type_mut(), None);

        assert_eq!(
            PropValue::Shape(Shape::Layer).as_shape_mut(),
            Some(&mut Shape::Layer)
        );
        assert_eq!(PropValue::Bool(true).as_shape_mut(), None);

        assert_eq!(
            PropValue::Style(Style::new()).as_style_mut(),
            Some(&mut Style::new())
        );
        assert_eq!(PropValue::Bool(true).as_style_mut(), None);

        assert_eq!(
            PropValue::TextSpan(TextSpan::new("hello")).as_text_span_mut(),
            Some(&mut TextSpan::new("hello"))
        );
        assert_eq!(PropValue::Bool(true).as_text_span_mut(), None);
    }

    #[test]
    fn unwrap_prop_payloads() {
        assert!(
            !PropPayload::One(PropValue::Bool(false))
                .unwrap_one()
                .unwrap_bool(),
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

    #[test]
    fn as_prop_payloads() {
        assert_eq!(
            PropPayload::One(PropValue::Bool(true)).as_one(),
            Some(&PropValue::Bool(true))
        );
        assert_eq!(PropPayload::None.as_one(), None);

        assert_eq!(
            PropPayload::Tup2((PropValue::Bool(true), PropValue::Bool(true))).as_tup2(),
            Some(&(PropValue::Bool(true), PropValue::Bool(true)))
        );
        assert_eq!(PropPayload::None.as_tup2(), None);

        assert_eq!(
            PropPayload::Tup3((
                PropValue::Bool(true),
                PropValue::Bool(true),
                PropValue::Bool(true)
            ))
            .as_tup3(),
            Some(&(
                PropValue::Bool(true),
                PropValue::Bool(true),
                PropValue::Bool(true)
            ))
        );
        assert_eq!(PropPayload::None.as_tup3(), None);

        assert_eq!(
            PropPayload::Tup4((
                PropValue::Bool(true),
                PropValue::Bool(true),
                PropValue::Bool(true),
                PropValue::Bool(true)
            ))
            .as_tup4(),
            Some(&(
                PropValue::Bool(true),
                PropValue::Bool(true),
                PropValue::Bool(true),
                PropValue::Bool(true)
            ))
        );
        assert_eq!(PropPayload::None.as_tup4(), None);

        assert_eq!(
            PropPayload::Vec(vec![PropValue::Bool(true)]).as_vec(),
            Some(&vec![PropValue::Bool(true)])
        );
        assert_eq!(PropPayload::None.as_vec(), None);

        assert_eq!(
            PropPayload::Map(HashMap::from([(
                "hello".to_string(),
                PropValue::Bool(true)
            )]))
            .as_map(),
            Some(&HashMap::from([(
                "hello".to_string(),
                PropValue::Bool(true)
            )]))
        );
        assert_eq!(PropPayload::None.as_map(), None);

        assert_eq!(
            PropPayload::Linked(LinkedList::new()).as_linked(),
            Some(&LinkedList::new())
        );
        assert_eq!(PropPayload::None.as_linked(), None);
    }

    #[test]
    fn as_prop_payloads_mut() {
        assert_eq!(
            PropPayload::One(PropValue::Bool(true)).as_one_mut(),
            Some(&mut PropValue::Bool(true))
        );
        assert_eq!(PropPayload::None.as_one_mut(), None);

        assert_eq!(
            PropPayload::Tup2((PropValue::Bool(true), PropValue::Bool(true))).as_tup2_mut(),
            Some(&mut (PropValue::Bool(true), PropValue::Bool(true)))
        );
        assert_eq!(PropPayload::None.as_tup2_mut(), None);

        assert_eq!(
            PropPayload::Tup3((
                PropValue::Bool(true),
                PropValue::Bool(true),
                PropValue::Bool(true)
            ))
            .as_tup3_mut(),
            Some(&mut (
                PropValue::Bool(true),
                PropValue::Bool(true),
                PropValue::Bool(true)
            ))
        );
        assert_eq!(PropPayload::None.as_tup3_mut(), None);

        assert_eq!(
            PropPayload::Tup4((
                PropValue::Bool(true),
                PropValue::Bool(true),
                PropValue::Bool(true),
                PropValue::Bool(true)
            ))
            .as_tup4_mut(),
            Some(&mut (
                PropValue::Bool(true),
                PropValue::Bool(true),
                PropValue::Bool(true),
                PropValue::Bool(true)
            ))
        );
        assert_eq!(PropPayload::None.as_tup4_mut(), None);

        assert_eq!(
            PropPayload::Vec(vec![PropValue::Bool(true)]).as_vec_mut(),
            Some(&mut vec![PropValue::Bool(true)])
        );
        assert_eq!(PropPayload::None.as_vec_mut(), None);

        assert_eq!(
            PropPayload::Map(HashMap::from([(
                "hello".to_string(),
                PropValue::Bool(true)
            )]))
            .as_map_mut(),
            Some(&mut HashMap::from([(
                "hello".to_string(),
                PropValue::Bool(true)
            )]))
        );
        assert_eq!(PropPayload::None.as_map_mut(), None);

        assert_eq!(
            PropPayload::Linked(LinkedList::new()).as_linked_mut(),
            Some(&mut LinkedList::new())
        );
        assert_eq!(PropPayload::None.as_linked_mut(), None);
    }

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
        let single_value = PropPayload::Any(input.to_any_prop());

        assert_eq!(
            single_value,
            PropPayload::Any(
                SomeCustomType {
                    field1: true,
                    field2: false
                }
                .to_any_prop()
            )
        );
        assert_ne!(
            single_value,
            PropPayload::Any(
                SomeCustomType {
                    field1: false,
                    field2: true
                }
                .to_any_prop()
            )
        );

        assert_ne!(
            single_value,
            PropPayload::Any(SomeDifferentCustomType { field1: true }.to_any_prop())
        );

        #[derive(Debug, Clone, PartialEq)]
        struct CloneableType {
            field1: String,
        }

        let input = PropPayload::Any(
            CloneableType {
                field1: "Hello".to_string(),
            }
            .to_any_prop(),
        );
        let cloned = input.clone();

        assert_eq!(input, cloned);
        let input_downcasted = input
            .as_any()
            .unwrap()
            .as_any()
            .downcast_ref::<CloneableType>()
            .expect("Erased type should be CloneableType");
        let cloned_downcasted = cloned
            .as_any()
            .unwrap()
            .as_any()
            .downcast_ref::<CloneableType>()
            .expect("Erased type should be CloneableType");
        // should be cloned and so not have the same memory pointer
        assert_ne!(
            input_downcasted.field1.as_ptr(),
            cloned_downcasted.field1.as_ptr()
        );

        let mut changed_data = cloned;

        let downcasted = changed_data
            .as_any_mut()
            .unwrap()
            .as_any_mut()
            .downcast_mut::<CloneableType>()
            .expect("Erased type should be CloneableType");

        downcasted.field1 = "Changed later".to_string();

        assert_ne!(input_downcasted, downcasted);
    }
}
