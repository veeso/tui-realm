//! This module exposes the prop values

use std::any::Any;
use std::collections::{HashMap, LinkedList};

use ratatui::text::{Line, Span, Text};

use super::{Color, HorizontalAlignment, InputType, Shape, Style, Table, VerticalAlignment};
use crate::props::{AnyPropBox, PropPayload, PropValue};
use crate::utils::{clone_line, clone_span, clone_text};

// -- Prop value

/// The payload contains the actual value for user defined properties
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PropPayloadRef<'a> {
    Single(PropValueRef<'a>),
    Pair((PropValueRef<'a>, PropValueRef<'a>)),
    Vec(&'a [PropValue]),
    Map(&'a HashMap<String, PropValue>),
    Linked(&'a LinkedList<PropPayload>),
    Any(&'a AnyPropBox),
    None,
}

/// Value describes the value contained in a `PropPayload`
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PropValueRef<'a> {
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
    Str(&'a str),
    // -- tui props
    AlignmentHorizontal(HorizontalAlignment),
    AlignmentVertical(VerticalAlignment),
    Color(Color),
    InputType(&'a InputType),
    Shape(&'a Shape),
    Style(Style),
    Table(&'a Table),
    TextSpan(&'a Span<'a>),
    TextLine(&'a Line<'a>),
    Text(&'a Text<'a>),
}

impl<'a> PropPayloadRef<'a> {
    // -- unwrappers

    /// Unwrap a Single value from PropPayload
    pub fn unwrap_single(self) -> PropValueRef<'a> {
        match self {
            PropPayloadRef::Single(v) => v,
            _ => panic!("Called `unwrap_single` on a bad value"),
        }
    }

    /// Unwrap a Pair value from PropPayload
    pub fn unwrap_pair(self) -> (PropValueRef<'a>, PropValueRef<'a>) {
        match self {
            PropPayloadRef::Pair(v) => v,
            _ => panic!("Called `unwrap_pair` on a bad value"),
        }
    }

    /// Unwrap a Vec value from PropPayload
    pub fn unwrap_vec(self) -> &'a [PropValue] {
        match self {
            PropPayloadRef::Vec(v) => v,
            _ => panic!("Called `unwrap_vec` on a bad value"),
        }
    }

    /// Unwrap a Map value from PropPayload
    pub fn unwrap_map(self) -> &'a HashMap<String, PropValue> {
        match self {
            PropPayloadRef::Map(v) => v,
            _ => panic!("Called `unwrap_map` on a bad value"),
        }
    }

    /// Unwrap a Linked list from PropPayload
    pub fn unwrap_linked(self) -> &'a LinkedList<PropPayload> {
        match self {
            PropPayloadRef::Linked(v) => v,
            _ => panic!("Called `unwrap_linked` on a bad value"),
        }
    }

    /// Unwrap a Any from PropPayload
    pub fn unwrap_any(self) -> &'a AnyPropBox {
        match self {
            PropPayloadRef::Any(v) => v,
            _ => panic!("Called `unwrap_any` on a bad value"),
        }
    }

    // -- as reference

    /// Get a Single value from PropPayload, or None
    pub fn as_single(&self) -> Option<PropValueRef<'a>> {
        match self {
            PropPayloadRef::Single(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Pair value from PropPayload, or None
    pub fn as_pair(&self) -> Option<(PropValueRef<'a>, PropValueRef<'a>)> {
        match self {
            PropPayloadRef::Pair(v) => Some((v.0, v.1)),
            _ => None,
        }
    }

    /// Get a Vec value from PropPayload, or None
    pub fn as_vec(&self) -> Option<&'a [PropValue]> {
        match self {
            PropPayloadRef::Vec(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Map value from PropPayload, or None
    pub fn as_map(&self) -> Option<&'a HashMap<String, PropValue>> {
        match self {
            PropPayloadRef::Map(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Linked value from PropPayload, or None
    pub fn as_linked(&self) -> Option<&'a LinkedList<PropPayload>> {
        match self {
            PropPayloadRef::Linked(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Any value from PropPayload, or None
    pub fn as_any(&self) -> Option<&dyn Any> {
        match self {
            PropPayloadRef::Any(v) => Some(v.as_ref()),
            _ => None,
        }
    }
}

impl<'a> PartialEq<PropPayload> for PropPayloadRef<'a> {
    fn eq(&self, other: &PropPayload) -> bool {
        match (other, self) {
            (PropPayload::Single(a), Self::Single(b)) => return *a == *b,
            (PropPayload::Pair((a1, a2)), Self::Pair((b1, b2))) => return a1 == b1 && a2 == b2,
            _ => (),
        }

        match other {
            PropPayload::Single(_) | PropPayload::Pair(_) => false,
            // PropPayload::Single(prop_value) => Some(prop_value) == self.as_single(),
            // PropPayload::Pair((a, b)) => Some((a, b)) == self.as_pair(),
            PropPayload::Vec(prop_values) => Some(prop_values.as_slice()) == self.as_vec(),
            PropPayload::Map(hash_map) => Some(hash_map) == self.as_map(),
            PropPayload::Linked(prop_payloads) => Some(prop_payloads) == self.as_linked(),
            PropPayload::Any(prop_bound) => {
                if let Self::Any(any) = self {
                    &prop_bound == any
                } else {
                    false
                }
            }
            PropPayload::None => *self == Self::None,
        }
    }
}

// reverse impl to not have position-dependent implementations
// ex. allow `PropPayload == PropPayloadRef` AND `PropPayloadRef == PropPayload`, without this, it would only allow one of them
impl<'a> PartialEq<PropPayloadRef<'a>> for PropPayload {
    fn eq(&self, other: &PropPayloadRef<'a>) -> bool {
        *other == *self
    }
}

impl<'a> From<&'a PropPayload> for PropPayloadRef<'a> {
    fn from(value: &'a PropPayload) -> Self {
        match value {
            PropPayload::Single(prop_value) => Self::Single(prop_value.into()),
            PropPayload::Pair((a, b)) => Self::Pair((a.into(), b.into())),
            PropPayload::Vec(prop_values) => Self::Vec(prop_values.as_slice()),
            PropPayload::Map(hash_map) => Self::Map(hash_map),
            PropPayload::Linked(prop_payloads) => Self::Linked(prop_payloads),
            PropPayload::Any(prop_bound) => Self::Any(prop_bound),
            PropPayload::None => Self::None,
        }
    }
}

impl<'a> From<PropPayloadRef<'a>> for PropPayload {
    fn from(value: PropPayloadRef) -> Self {
        match value {
            PropPayloadRef::Single(prop_value) => Self::Single(prop_value.into()),
            PropPayloadRef::Pair((a, b)) => Self::Pair((a.into(), b.into())),
            PropPayloadRef::Vec(prop_values) => Self::Vec(prop_values.to_owned()),
            PropPayloadRef::Map(hash_map) => Self::Map(hash_map.to_owned()),
            PropPayloadRef::Linked(prop_payloads) => Self::Linked(prop_payloads.to_owned()),
            PropPayloadRef::Any(prop_bound) => Self::Any((*prop_bound).clone()),
            PropPayloadRef::None => Self::None,
        }
    }
}

impl<'a> PropValueRef<'a> {
    // -- unwrappers

    /// Unwrap PropValue as Bool.
    /// Panics otherwise
    pub fn unwrap_bool(self) -> bool {
        match self {
            PropValueRef::Bool(v) => v,
            _ => panic!("Called `unwrap_bool` on a bad value"),
        }
    }

    /// Unwrap PropValue as u8.
    /// Panics otherwise
    pub fn unwrap_u8(self) -> u8 {
        match self {
            PropValueRef::U8(v) => v,
            _ => panic!("Called `unwrap_u8` on a bad value"),
        }
    }

    /// Unwrap PropValue as u16.
    /// Panics otherwise
    pub fn unwrap_u16(self) -> u16 {
        match self {
            PropValueRef::U16(v) => v,
            _ => panic!("Called `unwrap_u16` on a bad value"),
        }
    }

    /// Unwrap PropValue as Bool.
    /// Panics otherwise
    pub fn unwrap_u32(self) -> u32 {
        match self {
            PropValueRef::U32(v) => v,
            _ => panic!("Called `unwrap_u32` on a bad value"),
        }
    }

    /// Unwrap PropValue as u64.
    /// Panics otherwise
    pub fn unwrap_u64(self) -> u64 {
        match self {
            PropValueRef::U64(v) => v,
            _ => panic!("Called `unwrap_u64` on a bad value"),
        }
    }

    /// Unwrap PropValue as u128.
    /// Panics otherwise
    pub fn unwrap_u128(self) -> u128 {
        match self {
            PropValueRef::U128(v) => v,
            _ => panic!("Called `unwrap_u128` on a bad value"),
        }
    }

    /// Unwrap PropValue as usize.
    /// Panics otherwise
    pub fn unwrap_usize(self) -> usize {
        match self {
            PropValueRef::Usize(v) => v,
            _ => panic!("Called `unwrap_usize` on a bad value"),
        }
    }

    /// Unwrap PropValue as i8.
    /// Panics otherwise
    pub fn unwrap_i8(self) -> i8 {
        match self {
            PropValueRef::I8(v) => v,
            _ => panic!("Called `unwrap_i8` on a bad value"),
        }
    }

    /// Unwrap PropValue as i16.
    /// Panics otherwise
    pub fn unwrap_i16(self) -> i16 {
        match self {
            PropValueRef::I16(v) => v,
            _ => panic!("Called `unwrap_i16` on a bad value"),
        }
    }

    /// Unwrap PropValue as i32.
    /// Panics otherwise
    pub fn unwrap_i32(self) -> i32 {
        match self {
            PropValueRef::I32(v) => v,
            _ => panic!("Called `unwrap_i32` on a bad value"),
        }
    }

    /// Unwrap PropValue as i64.
    /// Panics otherwise
    pub fn unwrap_i64(self) -> i64 {
        match self {
            PropValueRef::I64(v) => v,
            _ => panic!("Called `unwrap_i64` on a bad value"),
        }
    }

    /// Unwrap PropValue as i128.
    /// Panics otherwise
    pub fn unwrap_i128(self) -> i128 {
        match self {
            PropValueRef::I128(v) => v,
            _ => panic!("Called `unwrap_i128` on a bad value"),
        }
    }

    /// Unwrap PropValue as isize.
    /// Panics otherwise
    pub fn unwrap_isize(self) -> isize {
        match self {
            PropValueRef::Isize(v) => v,
            _ => panic!("Called `unwrap_isize` on a bad value"),
        }
    }

    /// Unwrap PropValue as f32.
    /// Panics otherwise
    pub fn unwrap_f32(self) -> f32 {
        match self {
            PropValueRef::F32(v) => v,
            _ => panic!("Called `unwrap_f32` on a bad value"),
        }
    }

    /// Unwrap PropValue as f64.
    /// Panics otherwise
    pub fn unwrap_f64(self) -> f64 {
        match self {
            PropValueRef::F64(v) => v,
            _ => panic!("Called `unwrap_f64` on a bad value"),
        }
    }

    /// Unwrap PropValue as String.
    /// Panics otherwise
    pub fn unwrap_str(self) -> &'a str {
        match self {
            PropValueRef::Str(v) => v,
            _ => panic!("Called `unwrap_str` on a bad value"),
        }
    }

    /// Unwrap PropValue as Horizontal Alignment.
    /// Panics otherwise
    pub fn unwrap_alignment_horizontal(self) -> HorizontalAlignment {
        match self {
            PropValueRef::AlignmentHorizontal(v) => v,
            _ => panic!("Called `unwrap_alignment_horizontal` on a bad value"),
        }
    }

    /// Unwrap PropValue as Vertical Alignment.
    /// Panics otherwise
    pub fn unwrap_alignment_vertical(self) -> VerticalAlignment {
        match self {
            PropValueRef::AlignmentVertical(v) => v,
            _ => panic!("Called `unwrap_alignment_vertical` on a bad value"),
        }
    }

    /// Unwrap PropValue as Color.
    /// Panics otherwise
    pub fn unwrap_color(self) -> Color {
        match self {
            PropValueRef::Color(v) => v,
            _ => panic!("Called `unwrap_color` on a bad value"),
        }
    }

    /// Unwrap PropValue as InputType.
    /// Panics otherwise
    pub fn unwrap_input_type(self) -> &'a InputType {
        match self {
            PropValueRef::InputType(v) => v,
            _ => panic!("Called `unwrap_input_type` on a bad value"),
        }
    }

    /// Unwrap PropValue as Shape.
    /// Panics otherwise
    pub fn unwrap_shape(self) -> &'a Shape {
        match self {
            PropValueRef::Shape(v) => v,
            _ => panic!("Called `unwrap_shape` on a bad value"),
        }
    }

    /// Unwrap PropValue as Style.
    /// Panics otherwise
    pub fn unwrap_style(self) -> Style {
        match self {
            PropValueRef::Style(v) => v,
            _ => panic!("Called `unwrap_style` on a bad value"),
        }
    }

    /// Unwrap PropValue as Table.
    /// Panics otherwise
    pub fn unwrap_table(self) -> &'a Table {
        match self {
            PropValueRef::Table(v) => v,
            _ => panic!("Called `unwrap_table` on a bad value"),
        }
    }

    /// Unwrap PropValue as [`SpanStatic`].
    /// Panics otherwise
    pub fn unwrap_textspan(self) -> &'a Span<'a> {
        match self {
            PropValueRef::TextSpan(v) => v,
            _ => panic!("Called `unwrap_textspan` on a bad value"),
        }
    }

    /// Unwrap PropValue as [`LineStatic`].
    /// Panics otherwise
    pub fn unwrap_textline(self) -> &'a Line<'a> {
        match self {
            PropValueRef::TextLine(b) => b,
            _ => panic!("Called `unwrap_textline` on a bad value"),
        }
    }

    /// Unwrap PropValue as [`TextStatic`].
    /// Panics otherwise
    pub fn unwrap_text(self) -> &'a Text<'a> {
        match self {
            PropValueRef::Text(v) => v,
            _ => panic!("Called `unwrap_text` on a bad value"),
        }
    }

    // -- as reference

    /// Get a Bool value from PropValue, or None
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            // cheap copy, so no reference
            PropValueRef::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a u8 value from PropValue, or None
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            // cheap copy, so no reference
            PropValueRef::U8(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a u16 value from PropValue, or None
    pub fn as_u16(&self) -> Option<u16> {
        match self {
            // cheap copy, so no reference
            PropValueRef::U16(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a u32 value from PropValue, or None
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            // cheap copy, so no reference
            PropValueRef::U32(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a u64 value from PropValue, or None
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            // cheap copy, so no reference
            PropValueRef::U64(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a u128 value from PropValue, or None
    pub fn as_u128(&self) -> Option<u128> {
        match self {
            // cheap copy, so no reference
            PropValueRef::U128(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a usize value from PropValue, or None
    pub fn as_usize(&self) -> Option<usize> {
        match self {
            // cheap copy, so no reference
            PropValueRef::Usize(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a i8 value from PropValue, or None
    pub fn as_i8(&self) -> Option<i8> {
        match self {
            // cheap copy, so no reference
            PropValueRef::I8(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a i16 value from PropValue, or None
    pub fn as_i16(&self) -> Option<i16> {
        match self {
            // cheap copy, so no reference
            PropValueRef::I16(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a i32 value from PropValue, or None
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            // cheap copy, so no reference
            PropValueRef::I32(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a i64 value from PropValue, or None
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            // cheap copy, so no reference
            PropValueRef::I64(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a i128 value from PropValue, or None
    pub fn as_i128(&self) -> Option<i128> {
        match self {
            // cheap copy, so no reference
            PropValueRef::I128(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a isize value from PropValue, or None
    pub fn as_isize(&self) -> Option<isize> {
        match self {
            // cheap copy, so no reference
            PropValueRef::Isize(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a f32 value from PropValue, or None
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            // cheap copy, so no reference
            PropValueRef::F32(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a f64 value from PropValue, or None
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            // cheap copy, so no reference
            PropValueRef::F64(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a String value from PropValue, or None
    pub fn as_str(&self) -> Option<&'a str> {
        match self {
            PropValueRef::Str(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Horizontal Alignment value from PropValue, or None
    pub fn as_alignment_horizontal(&self) -> Option<HorizontalAlignment> {
        match self {
            // cheap copy, so no reference
            PropValueRef::AlignmentHorizontal(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Vertical Alignment value from PropValue, or None
    pub fn as_alignment_vertical(&self) -> Option<VerticalAlignment> {
        match self {
            // cheap copy, so no reference
            PropValueRef::AlignmentVertical(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Color value from PropValue, or None
    pub fn as_color(&self) -> Option<Color> {
        match self {
            // cheap copy, so no reference
            PropValueRef::Color(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a InputType value from PropValue, or None
    pub fn as_input_type(&self) -> Option<&'a InputType> {
        match self {
            PropValueRef::InputType(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Shape value from PropValue, or None
    pub fn as_shape(&self) -> Option<&'a Shape> {
        match self {
            PropValueRef::Shape(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Style value from PropValue, or None
    pub fn as_style(&self) -> Option<Style> {
        match self {
            PropValueRef::Style(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Table value from PropValue, or None
    pub fn as_table(&self) -> Option<&Table> {
        match self {
            PropValueRef::Table(v) => Some(v),
            _ => None,
        }
    }

    /// Get a [`SpanStatic`] value from PropValue, or None
    pub fn as_textspan(&self) -> Option<&'a Span<'a>> {
        match self {
            PropValueRef::TextSpan(v) => Some(v),
            _ => None,
        }
    }

    /// Get a [`LineStatic`] value from PropValue, or None
    pub fn as_textline(&self) -> Option<&'a Line<'a>> {
        match self {
            PropValueRef::TextLine(v) => Some(v),
            _ => None,
        }
    }

    /// Get a [`TextStatic`] value from PropValue, or None
    pub fn as_text(&self) -> Option<&'a Text<'a>> {
        match self {
            PropValueRef::Text(v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> PartialEq<PropValue> for PropValueRef<'a> {
    fn eq(&self, other: &PropValue) -> bool {
        match other {
            PropValue::Bool(bool) => Some(*bool) == self.as_bool(),
            PropValue::U8(num) => Some(*num) == self.as_u8(),
            PropValue::U16(num) => Some(*num) == self.as_u16(),
            PropValue::U32(num) => Some(*num) == self.as_u32(),
            PropValue::U64(num) => Some(*num) == self.as_u64(),
            PropValue::U128(num) => Some(*num) == self.as_u128(),
            PropValue::Usize(num) => Some(*num) == self.as_usize(),
            PropValue::I8(num) => Some(*num) == self.as_i8(),
            PropValue::I16(num) => Some(*num) == self.as_i16(),
            PropValue::I32(num) => Some(*num) == self.as_i32(),
            PropValue::I64(num) => Some(*num) == self.as_i64(),
            PropValue::I128(num) => Some(*num) == self.as_i128(),
            PropValue::Isize(num) => Some(*num) == self.as_isize(),
            PropValue::F64(num) => Some(*num) == self.as_f64(),
            PropValue::F32(num) => Some(*num) == self.as_f32(),
            PropValue::Str(string) => Some(string.as_str()) == self.as_str(),
            PropValue::AlignmentHorizontal(horizontal_alignment) => {
                Some(*horizontal_alignment) == self.as_alignment_horizontal()
            }
            PropValue::AlignmentVertical(vertical_alignment) => {
                Some(*vertical_alignment) == self.as_alignment_vertical()
            }
            PropValue::Color(color) => Some(*color) == self.as_color(),
            PropValue::InputType(input_type) => Some(input_type) == self.as_input_type(),
            PropValue::Shape(shape) => Some(shape) == self.as_shape(),
            PropValue::Style(style) => Some(*style) == self.as_style(),
            PropValue::Table(items) => Some(items) == self.as_table(),
            PropValue::TextSpan(span) => Some(span) == self.as_textspan(),
            PropValue::TextLine(line) => Some(line) == self.as_textline(),
            PropValue::Text(text) => Some(text) == self.as_text(),
        }
    }
}

// reverse impl to not have position-dependent implementations
// ex. allow `PropValue == PropValueRef` AND `PropValueRef == PropValue`, without this, it would only allow one of them
impl<'a> PartialEq<PropValueRef<'a>> for PropValue {
    fn eq(&self, other: &PropValueRef<'a>) -> bool {
        *other == *self
    }
}

impl<'a> From<&'a PropValue> for PropValueRef<'a> {
    fn from(value: &'a PropValue) -> Self {
        match value {
            PropValue::Bool(flag) => Self::Bool(*flag),
            PropValue::U8(num) => Self::U8(*num),
            PropValue::U16(num) => Self::U16(*num),
            PropValue::U32(num) => Self::U32(*num),
            PropValue::U64(num) => Self::U64(*num),
            PropValue::U128(num) => Self::U128(*num),
            PropValue::Usize(num) => Self::Usize(*num),
            PropValue::I8(num) => Self::I8(*num),
            PropValue::I16(num) => Self::I16(*num),
            PropValue::I32(num) => Self::I32(*num),
            PropValue::I64(num) => Self::I64(*num),
            PropValue::I128(num) => Self::I128(*num),
            PropValue::Isize(num) => Self::Isize(*num),
            PropValue::F64(num) => Self::F64(*num),
            PropValue::F32(num) => Self::F32(*num),
            PropValue::Str(str) => Self::Str(str),
            PropValue::AlignmentHorizontal(horizontal_alignment) => {
                Self::AlignmentHorizontal(*horizontal_alignment)
            }
            PropValue::AlignmentVertical(vertical_alignment) => {
                Self::AlignmentVertical(*vertical_alignment)
            }
            PropValue::Color(color) => Self::Color(*color),
            PropValue::InputType(input_type) => Self::InputType(input_type),
            PropValue::Shape(shape) => Self::Shape(shape),
            PropValue::Style(style) => Self::Style(*style),
            PropValue::Table(items) => Self::Table(items),
            PropValue::TextSpan(span) => Self::TextSpan(span),
            PropValue::TextLine(line) => Self::TextLine(line),
            PropValue::Text(text) => Self::Text(text),
        }
    }
}

impl<'a> From<PropValueRef<'a>> for PropValue {
    fn from(value: PropValueRef<'a>) -> Self {
        match value {
            PropValueRef::Bool(flag) => Self::Bool(flag),
            PropValueRef::U8(num) => Self::U8(num),
            PropValueRef::U16(num) => Self::U16(num),
            PropValueRef::U32(num) => Self::U32(num),
            PropValueRef::U64(num) => Self::U64(num),
            PropValueRef::U128(num) => Self::U128(num),
            PropValueRef::Usize(num) => Self::Usize(num),
            PropValueRef::I8(num) => Self::I8(num),
            PropValueRef::I16(num) => Self::I16(num),
            PropValueRef::I32(num) => Self::I32(num),
            PropValueRef::I64(num) => Self::I64(num),
            PropValueRef::I128(num) => Self::I128(num),
            PropValueRef::Isize(num) => Self::Isize(num),
            PropValueRef::F64(num) => Self::F64(num),
            PropValueRef::F32(num) => Self::F32(num),
            PropValueRef::Str(str) => Self::Str(str.to_owned()),
            PropValueRef::AlignmentHorizontal(horizontal_alignment) => {
                Self::AlignmentHorizontal(horizontal_alignment)
            }
            PropValueRef::AlignmentVertical(vertical_alignment) => {
                Self::AlignmentVertical(vertical_alignment)
            }
            PropValueRef::Color(color) => Self::Color(color),
            PropValueRef::InputType(input_type) => Self::InputType(input_type.to_owned()),
            PropValueRef::Shape(shape) => Self::Shape(shape.to_owned()),
            PropValueRef::Style(style) => Self::Style(style),
            PropValueRef::Table(items) => Self::Table(items.to_owned()),
            PropValueRef::TextSpan(span) => Self::TextSpan(clone_span(span)),
            PropValueRef::TextLine(line) => Self::TextLine(clone_line(line)),
            PropValueRef::Text(text) => Self::Text(clone_text(text)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;
    use crate::props::{LineStatic, PropBound, SpanStatic, TextStatic};
    use crate::ratatui::widgets::canvas::Map;

    #[test]
    fn prop_values() {
        // test that values can be created without compile errors
        let _ = PropPayloadRef::Single(PropValueRef::Usize(2));
        let _ = PropPayloadRef::Pair((PropValueRef::Bool(true), PropValueRef::Usize(128)));
        let _ = PropPayloadRef::Vec(&[
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
        map.insert(
            String::from("u"),
            PropValue::AlignmentHorizontal(HorizontalAlignment::Center),
        );
        assert_eq!(
            *map.get("u").unwrap(),
            PropValue::AlignmentHorizontal(HorizontalAlignment::Center)
        );

        let _ = PropPayloadRef::Map(&map);
        let mut link: LinkedList<PropPayload> = LinkedList::new();
        link.push_back(PropPayload::Single(PropValue::Usize(1)));
        link.push_back(PropPayload::Pair((
            PropValue::Usize(2),
            PropValue::Usize(4),
        )));
        let _ = PropPayloadRef::Linked(&link);
    }

    #[test]
    fn unwrap_prop_values() {
        assert_eq!(
            PropValueRef::AlignmentHorizontal(HorizontalAlignment::Center)
                .unwrap_alignment_horizontal(),
            HorizontalAlignment::Center
        );
        assert_eq!(
            PropValueRef::AlignmentVertical(VerticalAlignment::Top).unwrap_alignment_vertical(),
            VerticalAlignment::Top
        );
        assert_eq!(PropValue::Color(Color::Black).unwrap_color(), Color::Black);
        assert!(PropValueRef::Bool(true).unwrap_bool());
        assert_eq!(PropValueRef::F32(0.32).unwrap_f32(), 0.32);
        assert_eq!(PropValueRef::F64(0.32).unwrap_f64(), 0.32);
        assert_eq!(PropValueRef::I128(5).unwrap_i128(), 5);
        assert_eq!(PropValueRef::I64(5).unwrap_i64(), 5);
        assert_eq!(PropValueRef::I32(5).unwrap_i32(), 5);
        assert_eq!(PropValueRef::I16(5).unwrap_i16(), 5);
        assert_eq!(PropValueRef::I8(5).unwrap_i8(), 5);
        assert_eq!(PropValueRef::Isize(5).unwrap_isize(), 5);
        assert_eq!(PropValueRef::U128(5).unwrap_u128(), 5);
        assert_eq!(PropValueRef::U64(5).unwrap_u64(), 5);
        assert_eq!(PropValueRef::U32(5).unwrap_u32(), 5);
        assert_eq!(PropValueRef::U16(5).unwrap_u16(), 5);
        assert_eq!(PropValueRef::U8(5).unwrap_u8(), 5);
        assert_eq!(PropValueRef::Usize(5).unwrap_usize(), 5);
        assert_eq!(
            PropValueRef::InputType(&InputType::Number).unwrap_input_type(),
            &InputType::Number
        );
        assert_eq!(
            PropValueRef::Shape(&Shape::Layer).unwrap_shape(),
            &Shape::Layer
        );
        assert_eq!(
            PropValueRef::Str(&String::from("ciao")).unwrap_str(),
            "ciao".to_string()
        );
        assert_eq!(
            PropValueRef::Style(Style::default()).unwrap_style(),
            Style::default()
        );
        assert_eq!(
            PropValue::Table(vec![vec![LineStatic::from("test")]]).unwrap_table(),
            vec![vec![LineStatic::from("test")]]
        );
        assert_eq!(
            PropValueRef::TextSpan(&SpanStatic::from("ciao")).unwrap_textspan(),
            &SpanStatic::from("ciao")
        );
        assert_eq!(
            PropValueRef::TextLine(&LineStatic::from("ciao")).unwrap_textline(),
            &LineStatic::from("ciao")
        );
        assert_eq!(
            PropValueRef::Text(&TextStatic::from("ciao")).unwrap_text(),
            &TextStatic::from("ciao")
        );
    }

    #[test]
    fn as_prop_value() {
        assert_eq!(PropValueRef::Bool(true).as_bool(), Some(true));
        assert_eq!(PropValueRef::U8(0).as_bool(), None);

        assert_eq!(PropValueRef::U8(1).as_u8(), Some(1));
        assert_eq!(PropValueRef::Bool(true).as_u8(), None);

        assert_eq!(PropValueRef::U16(1).as_u16(), Some(1));
        assert_eq!(PropValueRef::Bool(true).as_u16(), None);

        assert_eq!(PropValueRef::U32(1).as_u32(), Some(1));
        assert_eq!(PropValueRef::Bool(true).as_u32(), None);

        assert_eq!(PropValueRef::U64(1).as_u64(), Some(1));
        assert_eq!(PropValueRef::Bool(true).as_u64(), None);

        assert_eq!(PropValueRef::U128(1).as_u128(), Some(1));
        assert_eq!(PropValueRef::Bool(true).as_u128(), None);

        assert_eq!(PropValueRef::Usize(1).as_usize(), Some(1));
        assert_eq!(PropValueRef::Bool(true).as_usize(), None);

        assert_eq!(PropValueRef::I8(-1).as_i8(), Some(-1));
        assert_eq!(PropValueRef::Bool(true).as_i8(), None);

        assert_eq!(PropValueRef::I16(-1).as_i16(), Some(-1));
        assert_eq!(PropValueRef::Bool(true).as_i16(), None);

        assert_eq!(PropValueRef::I32(-1).as_i32(), Some(-1));
        assert_eq!(PropValueRef::Bool(true).as_i32(), None);

        assert_eq!(PropValueRef::I64(-1).as_i64(), Some(-1));
        assert_eq!(PropValueRef::Bool(true).as_i64(), None);

        assert_eq!(PropValueRef::I128(-1).as_i128(), Some(-1));
        assert_eq!(PropValueRef::Bool(true).as_i128(), None);

        assert_eq!(PropValueRef::Isize(-1).as_isize(), Some(-1));
        assert_eq!(PropValueRef::Bool(true).as_isize(), None);

        assert_eq!(PropValueRef::F32(1.1).as_f32(), Some(1.1));
        assert_eq!(PropValueRef::Bool(true).as_f32(), None);

        assert_eq!(PropValueRef::F64(1.1).as_f64(), Some(1.1));
        assert_eq!(PropValueRef::Bool(true).as_f64(), None);

        assert_eq!(PropValueRef::Str("hello").as_str(), Some("hello"));
        assert_eq!(PropValueRef::Bool(true).as_str(), None);

        assert_eq!(
            PropValueRef::AlignmentHorizontal(HorizontalAlignment::Center)
                .as_alignment_horizontal(),
            Some(HorizontalAlignment::Center)
        );
        assert_eq!(PropValueRef::Bool(true).as_alignment_horizontal(), None);

        assert_eq!(
            PropValueRef::AlignmentVertical(VerticalAlignment::Top).as_alignment_vertical(),
            Some(VerticalAlignment::Top)
        );
        assert_eq!(PropValueRef::Bool(true).as_alignment_vertical(), None);

        assert_eq!(
            PropValue::Color(Color::Black).as_color(),
            Some(Color::Black)
        );
        assert_eq!(PropValue::Bool(true).as_color(), None);

        assert_eq!(
            PropValueRef::InputType(&InputType::Color).as_input_type(),
            Some(&InputType::Color)
        );
        assert_eq!(PropValueRef::Bool(true).as_input_type(), None);

        assert_eq!(
            PropValueRef::Shape(&Shape::Layer).as_shape(),
            Some(&Shape::Layer)
        );
        assert_eq!(PropValueRef::Bool(true).as_shape(), None);

        assert_eq!(
            PropValueRef::Style(Style::new()).as_style(),
            Some(Style::new())
        );
        assert_eq!(PropValueRef::Bool(true).as_style(), None);

        assert_eq!(
            PropValue::Table(Table::new()).as_table(),
            Some(&Table::new())
        );
        assert_eq!(PropValue::Bool(true).as_table(), None);

        assert_eq!(
            PropValueRef::TextSpan(&SpanStatic::from("hello")).as_textspan(),
            Some(&SpanStatic::from("hello"))
        );
        assert_eq!(PropValueRef::Bool(true).as_textspan(), None);

        assert_eq!(
            PropValueRef::TextLine(&LineStatic::from("hello")).as_textline(),
            Some(&LineStatic::from("hello"))
        );
        assert_eq!(PropValueRef::Bool(true).as_textline(), None);

        assert_eq!(
            PropValueRef::Text(&TextStatic::from("hello")).as_text(),
            Some(&TextStatic::from("hello"))
        );
        assert_eq!(PropValueRef::Bool(true).as_text(), None);
    }

    #[test]
    fn unwrap_prop_payloads() {
        assert!(
            !PropPayloadRef::Single(PropValueRef::Bool(false))
                .unwrap_single()
                .unwrap_bool(),
        );
        assert_eq!(
            PropPayloadRef::Pair((PropValueRef::Bool(false), PropValueRef::Bool(false)))
                .unwrap_pair(),
            (PropValueRef::Bool(false), PropValueRef::Bool(false))
        );
        assert_eq!(
            PropPayloadRef::Vec(&[PropValue::Bool(false), PropValue::Bool(false)]).unwrap_vec(),
            &[PropValue::Bool(false), PropValue::Bool(false)]
        );
    }

    #[test]
    fn as_prop_payloads() {
        assert_eq!(
            PropPayloadRef::Single(PropValueRef::Bool(true)).as_single(),
            Some(PropValueRef::Bool(true))
        );
        assert_eq!(PropPayloadRef::None.as_single(), None);

        assert_eq!(
            PropPayloadRef::Pair((PropValueRef::Bool(true), PropValueRef::Bool(true))).as_pair(),
            Some((PropValueRef::Bool(true), PropValueRef::Bool(true)))
        );
        assert_eq!(PropPayloadRef::None.as_pair(), None);

        assert_eq!(
            PropPayloadRef::Vec(&[PropValue::Bool(true)]).as_vec(),
            Some([PropValue::Bool(true)].as_slice())
        );
        assert_eq!(PropPayloadRef::None.as_vec(), None);

        assert_eq!(
            PropPayloadRef::Map(&HashMap::from([(
                "hello".to_string(),
                PropValue::Bool(true)
            )]))
            .as_map(),
            Some(&HashMap::from([(
                "hello".to_string(),
                PropValue::Bool(true)
            )]))
        );
        assert_eq!(PropPayloadRef::None.as_map(), None);

        assert_eq!(
            PropPayloadRef::Linked(&LinkedList::new()).as_linked(),
            Some(&LinkedList::new())
        );
        assert_eq!(PropPayloadRef::None.as_linked(), None);
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
        let any_data = input.to_any_prop();
        let single_value = PropPayloadRef::Any(&any_data);

        assert_eq!(
            single_value,
            PropPayloadRef::Any(
                &SomeCustomType {
                    field1: true,
                    field2: false
                }
                .to_any_prop()
            )
        );
        assert_ne!(
            single_value,
            PropPayloadRef::Any(
                &SomeCustomType {
                    field1: false,
                    field2: true
                }
                .to_any_prop()
            )
        );

        assert_ne!(
            single_value,
            PropPayloadRef::Any(&SomeDifferentCustomType { field1: true }.to_any_prop())
        );

        #[derive(Debug, Clone, PartialEq)]
        struct CloneableType {
            field1: String,
        }

        let any_data = CloneableType {
            field1: "Hello".to_string(),
        }
        .to_any_prop();
        let input = PropPayloadRef::Any(&any_data);

        let input_downcasted = input
            .as_any()
            .unwrap()
            .downcast_ref::<CloneableType>()
            .expect("Erased type should be CloneableType");
        let copied_downcasted = input
            .as_any()
            .unwrap()
            .downcast_ref::<CloneableType>()
            .expect("Erased type should be CloneableType");
        // should be copied and so have the same memory pointer
        assert_eq!(
            input_downcasted.field1.as_ptr(),
            copied_downcasted.field1.as_ptr()
        );
    }

    #[test]
    fn eq_nonref_proppayload() {
        assert!(
            PropPayloadRef::Single(PropValueRef::Bool(true))
                == PropPayload::Single(PropValue::Bool(true))
        );
        assert!(!(PropPayloadRef::Single(PropValueRef::Bool(true)) == PropPayload::None));

        assert!(
            PropPayloadRef::Pair((PropValueRef::Bool(true), PropValueRef::Bool(true)))
                == PropPayload::Pair((PropValue::Bool(true), PropValue::Bool(true)))
        );
        assert!(
            !(PropPayloadRef::Pair((PropValueRef::Bool(true), PropValueRef::Bool(true)))
                == PropPayload::None)
        );

        assert!(
            PropPayloadRef::Vec(&[PropValue::Bool(true)])
                == PropPayload::Vec(vec![PropValue::Bool(true)])
        );
        assert!(!(PropPayloadRef::Vec(&[PropValue::Bool(true)]) == PropPayload::None));

        assert!(
            PropPayloadRef::Map(&HashMap::from([("key".to_string(), PropValue::Bool(true))]))
                == PropPayload::Map(HashMap::from([("key".to_string(), PropValue::Bool(true))]))
        );
        assert!(
            !(PropPayloadRef::Map(&HashMap::from([("key".to_string(), PropValue::Bool(true))]))
                == PropPayload::None)
        );

        assert!(
            PropPayloadRef::Linked(&LinkedList::new()) == PropPayload::Linked(LinkedList::new())
        );
        assert!(!(PropPayloadRef::Linked(&LinkedList::new()) == PropPayload::None));

        #[derive(Debug, Clone, PartialEq)]
        struct CloneableType {
            field1: String,
        }

        assert!(
            PropPayloadRef::Any(
                &CloneableType {
                    field1: "Hello".to_string(),
                }
                .to_any_prop()
            ) == PropPayload::Any(
                CloneableType {
                    field1: "Hello".to_string(),
                }
                .to_any_prop()
            )
        );
        assert!(
            !(PropPayloadRef::Any(
                &CloneableType {
                    field1: "Hello".to_string(),
                }
                .to_any_prop()
            ) == PropPayload::None)
        );
    }

    #[test]
    fn eq_nonref_propvalue() {
        assert!(PropValueRef::Bool(true) == PropValue::Bool(true));
        assert!(PropValueRef::Bool(false) == PropValue::Bool(false));
        assert!(!(PropValueRef::Bool(true) == PropValue::U8(0)));

        assert!(PropValueRef::U8(1) == PropValue::U8(1));
        assert!(!(PropValueRef::U8(1) == PropValue::Bool(false)));

        assert!(PropValueRef::U16(1) == PropValue::U16(1));
        assert!(!(PropValueRef::U16(1) == PropValue::Bool(false)));

        assert!(PropValueRef::U32(1) == PropValue::U32(1));
        assert!(!(PropValueRef::U32(1) == PropValue::Bool(false)));

        assert!(PropValueRef::U64(1) == PropValue::U64(1));
        assert!(!(PropValueRef::U64(1) == PropValue::Bool(false)));

        assert!(PropValueRef::U128(1) == PropValue::U128(1));
        assert!(!(PropValueRef::U128(1) == PropValue::Bool(false)));

        assert!(PropValueRef::Usize(1) == PropValue::Usize(1));
        assert!(!(PropValueRef::Usize(1) == PropValue::Bool(false)));

        assert!(PropValueRef::I8(1) == PropValue::I8(1));
        assert!(!(PropValueRef::I8(1) == PropValue::Bool(false)));

        assert!(PropValueRef::I16(1) == PropValue::I16(1));
        assert!(!(PropValueRef::I16(1) == PropValue::Bool(false)));

        assert!(PropValueRef::I32(1) == PropValue::I32(1));
        assert!(!(PropValueRef::I32(1) == PropValue::Bool(false)));

        assert!(PropValueRef::I64(1) == PropValue::I64(1));
        assert!(!(PropValueRef::I64(1) == PropValue::Bool(false)));

        assert!(PropValueRef::I128(1) == PropValue::I128(1));
        assert!(!(PropValueRef::I128(1) == PropValue::Bool(false)));

        assert!(PropValueRef::Isize(1) == PropValue::Isize(1));
        assert!(!(PropValueRef::Isize(1) == PropValue::Bool(false)));

        assert!(PropValueRef::F32(1.0) == PropValue::F32(1.0));
        assert!(!(PropValueRef::F32(1.0) == PropValue::Bool(false)));

        assert!(PropValueRef::F64(1.0) == PropValue::F64(1.0));
        assert!(!(PropValueRef::F64(1.0) == PropValue::Bool(false)));

        assert!(PropValueRef::Str("hello") == PropValue::Str("hello".to_string()));
        assert!(!(PropValueRef::Str("hello") == PropValue::Bool(false)));

        assert!(
            PropValueRef::AlignmentHorizontal(HorizontalAlignment::Center)
                == PropValue::AlignmentHorizontal(HorizontalAlignment::Center)
        );
        assert!(
            !(PropValueRef::AlignmentHorizontal(HorizontalAlignment::Center)
                == PropValue::Bool(false))
        );

        assert!(
            PropValueRef::AlignmentVertical(VerticalAlignment::Bottom)
                == PropValue::AlignmentVertical(VerticalAlignment::Bottom)
        );
        assert!(
            !(PropValueRef::AlignmentVertical(VerticalAlignment::Bottom) == PropValue::Bool(false))
        );

        assert!(PropValueRef::Color(Color::Black) == PropValue::Color(Color::Black));
        assert!(!(PropValueRef::Color(Color::Black) == PropValue::Bool(false)));

        assert!(
            PropValueRef::InputType(&InputType::Color) == PropValue::InputType(InputType::Color)
        );
        assert!(!(PropValueRef::InputType(&InputType::Color) == PropValue::Bool(false)));

        assert!(PropValueRef::Shape(&Shape::Layer) == PropValue::Shape(Shape::Layer));
        assert!(!(PropValueRef::Shape(&Shape::Layer) == PropValue::Bool(false)));

        assert!(PropValueRef::Style(Style::default()) == PropValue::Style(Style::default()));
        assert!(!(PropValueRef::Style(Style::default()) == PropValue::Bool(false)));

        assert!(PropValueRef::Table(&Table::new()) == PropValue::Table(Table::new()));
        assert!(!(PropValueRef::Table(&Table::new()) == PropValue::Bool(false)));

        assert!(
            PropValueRef::TextSpan(&SpanStatic::from("hello"))
                == PropValue::TextSpan(SpanStatic::from("hello"))
        );
        assert!(!(PropValueRef::TextSpan(&SpanStatic::from("hello")) == PropValue::Bool(false)));

        assert!(
            PropValueRef::TextLine(&LineStatic::from("hello"))
                == PropValue::TextLine(LineStatic::from("hello"))
        );
        assert!(!(PropValueRef::TextLine(&LineStatic::from("hello")) == PropValue::Bool(false)));

        assert!(
            PropValueRef::Text(&TextStatic::from("hello"))
                == PropValue::Text(TextStatic::from("hello"))
        );
        assert!(!(PropValueRef::Text(&TextStatic::from("hello")) == PropValue::Bool(false)));
    }

    #[test]
    fn from_nonref_proppayload() {
        assert_eq!(
            PropPayloadRef::from(&PropPayload::Single(PropValue::Bool(true))),
            PropPayloadRef::Single(PropValueRef::Bool(true))
        );
        assert_eq!(
            PropPayloadRef::from(&PropPayload::Pair((
                PropValue::Bool(true),
                PropValue::Bool(true)
            ))),
            PropPayloadRef::Pair((PropValueRef::Bool(true), PropValueRef::Bool(true)))
        );
        assert_eq!(
            PropPayloadRef::from(&PropPayload::Vec(vec![PropValue::Bool(true)])),
            PropPayloadRef::Vec([PropValue::Bool(true)].as_slice())
        );
        assert_eq!(
            PropPayloadRef::from(&PropPayload::Map(HashMap::from([(
                "key".to_string(),
                PropValue::Bool(true)
            )]))),
            PropPayloadRef::Map(&HashMap::from([("key".to_string(), PropValue::Bool(true))]))
        );
        assert_eq!(
            PropPayloadRef::from(&PropPayload::Linked(LinkedList::new())),
            PropPayloadRef::Linked(&LinkedList::new())
        );
        assert_eq!(
            PropPayloadRef::from(&PropPayload::None),
            PropPayloadRef::None
        );

        #[derive(Debug, Clone, PartialEq)]
        struct CloneableType {
            field1: String,
        }

        assert_eq!(
            PropPayloadRef::from(&PropPayload::Any(
                CloneableType {
                    field1: "Hello".to_string(),
                }
                .to_any_prop()
            )),
            PropPayloadRef::Any(
                &CloneableType {
                    field1: "Hello".to_string(),
                }
                .to_any_prop()
            )
        );
    }

    #[test]
    fn into_nonref_proppayload() {
        assert_eq!(
            PropPayload::from(PropPayloadRef::Single(PropValueRef::Bool(true))),
            PropPayload::Single(PropValue::Bool(true))
        );
        assert_eq!(
            PropPayload::from(PropPayloadRef::Pair((
                PropValueRef::Bool(true),
                PropValueRef::Bool(true)
            ))),
            PropPayload::Pair((PropValue::Bool(true), PropValue::Bool(true)))
        );
        assert_eq!(
            PropPayload::from(PropPayloadRef::Vec([PropValue::Bool(true)].as_slice())),
            PropPayload::Vec(vec![PropValue::Bool(true)])
        );
        assert_eq!(
            PropPayload::from(PropPayloadRef::Map(&HashMap::from([(
                "key".to_string(),
                PropValue::Bool(true)
            )]))),
            PropPayload::Map(HashMap::from([("key".to_string(), PropValue::Bool(true))]))
        );
        assert_eq!(
            PropPayload::from(PropPayloadRef::Linked(&LinkedList::new())),
            PropPayload::Linked(LinkedList::new())
        );
        assert_eq!(PropPayload::from(PropPayloadRef::None), PropPayload::None);

        #[derive(Debug, Clone, PartialEq)]
        struct CloneableType {
            field1: String,
        }

        assert_eq!(
            PropPayload::from(PropPayloadRef::Any(
                &CloneableType {
                    field1: "Hello".to_string(),
                }
                .to_any_prop()
            )),
            PropPayload::Any(
                CloneableType {
                    field1: "Hello".to_string(),
                }
                .to_any_prop()
            )
        );
    }

    #[test]
    fn from_nonref_propvalue() {
        assert_eq!(
            PropValueRef::from(&PropValue::Bool(true)),
            PropValue::Bool(true)
        );

        assert_eq!(PropValueRef::from(&PropValue::U8(1)), PropValueRef::U8(1));
        assert_eq!(PropValueRef::from(&PropValue::U16(1)), PropValueRef::U16(1));
        assert_eq!(PropValueRef::from(&PropValue::U32(1)), PropValueRef::U32(1));
        assert_eq!(PropValueRef::from(&PropValue::U64(1)), PropValueRef::U64(1));
        assert_eq!(
            PropValueRef::from(&PropValue::U128(1)),
            PropValueRef::U128(1)
        );
        assert_eq!(
            PropValueRef::from(&PropValue::Usize(1)),
            PropValueRef::Usize(1)
        );
        assert_eq!(PropValueRef::from(&PropValue::I8(1)), PropValueRef::I8(1));
        assert_eq!(PropValueRef::from(&PropValue::I16(1)), PropValueRef::I16(1));
        assert_eq!(PropValueRef::from(&PropValue::I32(1)), PropValueRef::I32(1));
        assert_eq!(PropValueRef::from(&PropValue::I64(1)), PropValueRef::I64(1));
        assert_eq!(
            PropValueRef::from(&PropValue::I128(1)),
            PropValueRef::I128(1)
        );
        assert_eq!(
            PropValueRef::from(&PropValue::Isize(1)),
            PropValueRef::Isize(1)
        );
        assert_eq!(
            PropValueRef::from(&PropValue::F32(1.0)),
            PropValueRef::F32(1.0)
        );
        assert_eq!(
            PropValueRef::from(&PropValue::F64(1.0)),
            PropValueRef::F64(1.0)
        );

        assert_eq!(
            PropValueRef::from(&PropValue::Str("hello".to_string())),
            PropValueRef::Str("hello")
        );

        assert_eq!(
            PropValueRef::from(&PropValue::AlignmentHorizontal(HorizontalAlignment::Center)),
            PropValueRef::AlignmentHorizontal(HorizontalAlignment::Center)
        );
        assert_eq!(
            PropValueRef::from(&PropValue::AlignmentVertical(VerticalAlignment::Bottom)),
            PropValueRef::AlignmentVertical(VerticalAlignment::Bottom)
        );

        assert_eq!(
            PropValueRef::from(&PropValue::Color(Color::Black)),
            PropValueRef::Color(Color::Black)
        );

        assert_eq!(
            PropValueRef::from(&PropValue::InputType(InputType::Color)),
            PropValueRef::InputType(&InputType::Color)
        );

        assert_eq!(
            PropValueRef::from(&PropValue::Shape(Shape::Layer)),
            PropValueRef::Shape(&Shape::Layer)
        );

        assert_eq!(
            PropValueRef::from(&PropValue::Style(Style::default())),
            PropValueRef::Style(Style::default())
        );

        assert_eq!(
            PropValueRef::from(&PropValue::Table(Table::new())),
            PropValueRef::Table(&Table::new())
        );

        assert_eq!(
            PropValueRef::from(&PropValue::TextSpan(SpanStatic::from("hello"))),
            PropValueRef::TextSpan(&SpanStatic::from("hello"))
        );
        assert_eq!(
            PropValueRef::from(&PropValue::TextLine(LineStatic::from("hello"))),
            PropValueRef::TextLine(&LineStatic::from("hello"))
        );
        assert_eq!(
            PropValueRef::from(&PropValue::Text(TextStatic::from("hello"))),
            PropValueRef::Text(&TextStatic::from("hello"))
        );
    }

    #[test]
    fn into_nonref_propvalue() {
        assert_eq!(
            PropValue::from(PropValueRef::Bool(true)),
            PropValue::Bool(true)
        );

        assert_eq!(PropValue::from(PropValueRef::U8(1)), PropValueRef::U8(1));
        assert_eq!(PropValue::from(PropValueRef::U16(1)), PropValue::U16(1));
        assert_eq!(PropValue::from(PropValueRef::U32(1)), PropValue::U32(1));
        assert_eq!(PropValue::from(PropValueRef::U64(1)), PropValue::U64(1));
        assert_eq!(PropValue::from(PropValueRef::U128(1)), PropValue::U128(1));
        assert_eq!(PropValue::from(PropValueRef::Usize(1)), PropValue::Usize(1));
        assert_eq!(PropValue::from(PropValueRef::I8(1)), PropValueRef::I8(1));
        assert_eq!(PropValue::from(PropValueRef::I16(1)), PropValue::I16(1));
        assert_eq!(PropValue::from(PropValueRef::I32(1)), PropValue::I32(1));
        assert_eq!(PropValue::from(PropValueRef::I64(1)), PropValue::I64(1));
        assert_eq!(PropValue::from(PropValueRef::I128(1)), PropValue::I128(1));
        assert_eq!(PropValue::from(PropValueRef::Isize(1)), PropValue::Isize(1));
        assert_eq!(PropValue::from(PropValueRef::F32(1.0)), PropValue::F32(1.0));
        assert_eq!(PropValue::from(PropValueRef::F64(1.0)), PropValue::F64(1.0));

        assert_eq!(
            PropValue::from(PropValueRef::Str("hello")),
            PropValue::Str("hello".to_string())
        );

        assert_eq!(
            PropValue::from(PropValueRef::AlignmentHorizontal(
                HorizontalAlignment::Center
            )),
            PropValue::AlignmentHorizontal(HorizontalAlignment::Center)
        );
        assert_eq!(
            PropValue::from(PropValueRef::AlignmentVertical(VerticalAlignment::Bottom)),
            PropValue::AlignmentVertical(VerticalAlignment::Bottom)
        );

        assert_eq!(
            PropValue::from(PropValueRef::Color(Color::Black)),
            PropValue::Color(Color::Black)
        );

        assert_eq!(
            PropValue::from(PropValueRef::InputType(&InputType::Color)),
            PropValue::InputType(InputType::Color)
        );

        assert_eq!(
            PropValue::from(PropValueRef::Shape(&Shape::Layer)),
            PropValue::Shape(Shape::Layer)
        );

        assert_eq!(
            PropValue::from(PropValueRef::Style(Style::default())),
            PropValue::Style(Style::default())
        );

        assert_eq!(
            PropValue::from(PropValueRef::Table(&Table::new())),
            PropValue::Table(Table::new())
        );

        assert_eq!(
            PropValue::from(PropValueRef::TextSpan(&SpanStatic::from("hello"))),
            PropValue::TextSpan(SpanStatic::from("hello"))
        );
        assert_eq!(
            PropValue::from(PropValueRef::TextLine(&LineStatic::from("hello"))),
            PropValue::TextLine(LineStatic::from("hello"))
        );
        assert_eq!(
            PropValue::from(PropValueRef::Text(&TextStatic::from("hello"))),
            PropValue::Text(TextStatic::from("hello"))
        );
    }
}
