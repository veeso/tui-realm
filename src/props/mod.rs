//! ## Props
//!
//! `Props` is the module which defines properties for layout components

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// ext
use tui::style::{Color, Modifier};

// modules
pub mod borders;
pub mod builder;
pub mod texts;

// Exports
pub use borders::{Borders, BordersProps};
pub use builder::{GenericPropsBuilder, PropsBuilder};
pub use texts::{Table, TableBuilder, TextParts, TextSpan, TextSpanBuilder};

use std::collections::HashMap;

// -- Props

/// ## Props
///
/// Props holds all the possible properties for a layout component
#[derive(Clone)]
pub struct Props {
    // Values
    pub visible: bool,         // Is the element visible ON CREATE?
    pub foreground: Color,     // Foreground color
    pub background: Color,     // Background color
    pub borders: BordersProps, // Borders
    pub modifiers: Modifier,
    pub input_type: InputType,                 // Input type
    pub input_len: Option<usize>,              // max input len
    pub palette: HashMap<&'static str, Color>, // Use palette to store extra colors
    pub texts: TextParts,                      // text parts
    pub value: PropPayload,                    // Initial value
}

impl Default for Props {
    fn default() -> Self {
        Self {
            // Values
            visible: true,
            foreground: Color::Reset,
            background: Color::Reset,
            borders: BordersProps::default(),
            modifiers: Modifier::empty(),
            input_type: InputType::Text,
            input_len: None,
            palette: HashMap::new(),
            texts: TextParts::default(),
            value: PropPayload::None,
        }
    }
}

// -- Prop value

/// ## PropPayload
///
/// Payload describes a property initial value payload, which contains the actual value in different kind of storage
#[derive(Debug, PartialEq, Clone)]
pub enum PropPayload {
    One(PropValue),
    Tup2((PropValue, PropValue)),
    Tup3((PropValue, PropValue, PropValue)),
    Tup4((PropValue, PropValue, PropValue, PropValue)),
    Vec(Vec<PropValue>),
    Map(HashMap<String, PropValue>),
    None,
}

/// ## PropValue
///
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
}

// -- Input Type

/// ## InputType
///
/// Input type for text inputs
#[derive(Clone, Copy, PartialEq, std::fmt::Debug)]
pub enum InputType {
    Text,
    Number,
    Password,
}

#[cfg(test)]
mod tests {

    use super::borders::BorderType;
    use super::*;

    #[test]
    fn test_props_default() {
        let props: Props = Props::default();
        assert_eq!(props.visible, true);
        assert_eq!(props.background, Color::Reset);
        assert_eq!(props.foreground, Color::Reset);
        assert_eq!(props.borders.borders, Borders::ALL);
        assert_eq!(props.borders.color, Color::Reset);
        assert_eq!(props.borders.variant, BorderType::Plain);
        assert_eq!(props.modifiers, Modifier::empty());
        assert_eq!(props.palette.len(), 0);
        assert!(props.texts.title.is_none());
        assert_eq!(props.input_type, InputType::Text);
        assert!(props.input_len.is_none());
        assert_eq!(props.value, PropPayload::None);
        assert!(props.texts.spans.is_none());
    }
}
