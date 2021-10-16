//! ## props
//!
//! the props module expose the Properties supported by the components and all the values they can get.

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
use std::collections::HashMap;

// -- modules
mod borders;
mod dataset;
mod direction;
mod input_type;
mod shape;
mod texts;
mod value;

// -- exports
pub use crate::tui::layout::Alignment;
pub use crate::tui::style::{Color, Modifier as TextModifiers, Style};
pub use borders::{BorderSides, BorderType, Borders};
pub use dataset::Dataset;
pub use direction::Direction;
pub use input_type::InputType;
pub use shape::Shape;
pub use texts::{Table, TableBuilder, TextSpan};
pub use value::{PropPayload, PropValue};

/// ## Props
///
/// The props struct holds all the attributes associated to the component.
/// Properties have been designed to be versatile for all kind of components, but without introducing
/// too many attributes at the same time.
#[derive(Debug, PartialEq, Clone)]
pub struct Props {
    attrs: HashMap<Attribute, AttrValue>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            attrs: HashMap::default(),
        }
    }
}

impl Props {
    /// ### get
    ///
    /// Get, if any, the attribute associated to the selector
    pub fn get(&self, query: Attribute) -> Option<AttrValue> {
        self.attrs.get(&query).cloned()
    }

    /// ### get_or
    ///
    /// Get, if any, the attribute associated to the selector
    /// or return the fallback value `default`
    pub fn get_or(&self, query: Attribute, default: AttrValue) -> AttrValue {
        self.get(query).unwrap_or(default)
    }

    /// ### set
    ///
    /// Set a new attribute into Properties
    pub fn set(&mut self, query: Attribute, value: AttrValue) {
        self.attrs.insert(query, value);
    }
}

/// ## Attribute
///
/// Describes a "selector" to query an attribute on props.
/// The selector must identify uniquely an attribute in the properties.
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
pub enum Attribute {
    Alignment,
    Background,
    Borders,
    Color,
    Content,
    Direction,
    Display,
    Focus,
    FocusColor,
    Foreground,
    Height,
    HighlightedStr,
    InputLength,
    InputType,
    Rewind,
    Scroll,
    Step,
    Text,
    TextProps,
    TextWrap,
    Title,
    Width,
    /// User defined selector
    Custom(&'static str),
}

// -- AttrValues

/// ## AttrValue
///
/// Describes a single attribute in the component properties.
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum AttrValue {
    Alignment(Alignment),
    Borders(Borders),
    Color(Color),
    Dataset(Dataset),
    Direction(Direction),
    Flag(bool),
    InputType(InputType),
    Length(usize),
    Shape(Shape),
    Size(u16),
    String(String),
    Style(Style),
    Table(Table),
    Text(TextSpan),
    TextModifiers(TextModifiers),
    Title((String, Alignment)),
    /// User defined complex attribute value
    Payload(PropPayload),
}

impl AttrValue {
    // -- unwrappers

    pub fn unwrap_alignment(self) -> Alignment {
        match self {
            AttrValue::Alignment(x) => x,
            _ => panic!("AttrValue is not Alignment"),
        }
    }

    pub fn unwrap_borders(self) -> Borders {
        match self {
            AttrValue::Borders(b) => b,
            _ => panic!("AttrValue is not Borders"),
        }
    }

    pub fn unwrap_color(self) -> Color {
        match self {
            AttrValue::Color(x) => x,
            _ => panic!("AttrValue is not Color"),
        }
    }

    pub fn unwrap_dataset(self) -> Dataset {
        match self {
            AttrValue::Dataset(x) => x,
            _ => panic!("AttrValue is not Dataset"),
        }
    }

    pub fn unwrap_direction(self) -> Direction {
        match self {
            AttrValue::Direction(x) => x,
            _ => panic!("AttrValue is not Direction"),
        }
    }

    pub fn unwrap_flag(self) -> bool {
        match self {
            AttrValue::Flag(x) => x,
            _ => panic!("AttrValue is not Flag"),
        }
    }

    pub fn unwrap_input_type(self) -> InputType {
        match self {
            AttrValue::InputType(x) => x,
            _ => panic!("AttrValue is not InputType"),
        }
    }

    pub fn unwrap_length(self) -> usize {
        match self {
            AttrValue::Length(x) => x,
            _ => panic!("AttrValue is not Length"),
        }
    }

    pub fn unwrap_shape(self) -> Shape {
        match self {
            AttrValue::Shape(x) => x,
            _ => panic!("AttrValue is not Shape"),
        }
    }

    pub fn unwrap_size(self) -> u16 {
        match self {
            AttrValue::Size(x) => x,
            _ => panic!("AttrValue is not Size"),
        }
    }

    pub fn unwrap_string(self) -> String {
        match self {
            AttrValue::String(x) => x,
            _ => panic!("AttrValue is not String"),
        }
    }

    pub fn unwrap_style(self) -> Style {
        match self {
            AttrValue::Style(x) => x,
            _ => panic!("AttrValue is not Style"),
        }
    }

    pub fn unwrap_table(self) -> Table {
        match self {
            AttrValue::Table(x) => x,
            _ => panic!("AttrValue is not Table"),
        }
    }

    pub fn unwrap_text(self) -> TextSpan {
        match self {
            AttrValue::Text(x) => x,
            _ => panic!("AttrValue is not Text"),
        }
    }

    pub fn unwrap_text_modifiers(self) -> TextModifiers {
        match self {
            AttrValue::TextModifiers(x) => x,
            _ => panic!("AttrValue is not TextModifiers"),
        }
    }

    pub fn unwrap_title(self) -> (String, Alignment) {
        match self {
            AttrValue::Title(x) => x,
            _ => panic!("AttrValue is not Title"),
        }
    }

    pub fn unwrap_payload(self) -> PropPayload {
        match self {
            AttrValue::Payload(x) => x,
            _ => panic!("AttrValue is not Payload"),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn unwrapping_alignment_should_unwrap() {
        assert_eq!(
            AttrValue::Alignment(Alignment::Center).unwrap_alignment(),
            Alignment::Center
        );
    }

    #[test]
    fn unwrapping_borders_should_unwrap() {
        assert_eq!(
            AttrValue::Borders(Borders::default()).unwrap_borders(),
            Borders::default()
        );
    }

    #[test]
    fn unwrapping_color_should_unwrap() {
        assert_eq!(AttrValue::Color(Color::Red).unwrap_color(), Color::Red);
    }

    #[test]
    fn unwrapping_dataset_should_unwrap() {
        assert_eq!(
            AttrValue::Dataset(Dataset::default()).unwrap_dataset(),
            Dataset::default()
        );
    }

    #[test]
    fn unwrapping_direction_should_unwrap() {
        assert_eq!(
            AttrValue::Direction(Direction::Left).unwrap_direction(),
            Direction::Left
        );
    }

    #[test]
    fn unwrapping_flag_should_unwrap() {
        assert_eq!(AttrValue::Flag(true).unwrap_flag(), true);
    }

    #[test]

    fn unwrapping_inputtype_should_unwrap() {
        assert_eq!(
            AttrValue::InputType(InputType::Number).unwrap_input_type(),
            InputType::Number
        );
    }

    #[test]
    fn unwrapping_length_should_unwrap() {
        assert_eq!(AttrValue::Length(12).unwrap_length(), 12);
    }

    #[test]
    fn unwrapping_shape_should_unwrap() {
        assert_eq!(AttrValue::Shape(Shape::Layer).unwrap_shape(), Shape::Layer);
    }

    #[test]
    fn unwrapping_size_should_unwrap() {
        assert_eq!(AttrValue::Size(12).unwrap_size(), 12);
    }

    #[test]

    fn unwrapping_string_should_unwrap() {
        assert_eq!(
            AttrValue::String(String::from("pippo")).unwrap_string(),
            String::from("pippo")
        );
    }

    #[test]
    fn unwrapping_style_should_unwrap() {
        assert_eq!(
            AttrValue::Style(Style::default()).unwrap_style(),
            Style::default()
        );
    }

    #[test]

    fn unwrapping_table_should_unwrap() {
        assert_eq!(
            AttrValue::Table(Table::default()).unwrap_table(),
            Table::default()
        );
    }

    #[test]
    fn unwrapping_text_should_unwrap() {
        assert_eq!(
            AttrValue::Text(TextSpan::default()).unwrap_text(),
            TextSpan::default()
        );
    }

    #[test]
    fn unwrapping_textmodifiers_should_unwrap() {
        assert_eq!(
            AttrValue::TextModifiers(TextModifiers::BOLD).unwrap_text_modifiers(),
            TextModifiers::BOLD
        );
    }

    #[test]
    fn unwrapping_title_should_unwrap() {
        assert_eq!(
            AttrValue::Title((String::from("pippo"), Alignment::Left)).unwrap_title(),
            (String::from("pippo"), Alignment::Left)
        );
    }

    #[test]
    fn unwrapping_payload_should_unwrap() {
        assert_eq!(
            AttrValue::Payload(PropPayload::None).unwrap_payload(),
            PropPayload::None
        );
    }

    #[test]
    #[should_panic]
    fn unwrapping_alignment_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_alignment();
    }

    #[test]
    #[should_panic]
    fn unwrapping_borders_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_borders();
    }

    #[test]
    #[should_panic]
    fn unwrapping_color_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_color();
    }

    #[test]
    #[should_panic]
    fn unwrapping_dataset_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_dataset();
    }

    #[test]
    #[should_panic]
    fn unwrapping_direction_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_direction();
    }

    #[test]
    #[should_panic]
    fn unwrapping_flag_should_panic_if_not_identity() {
        AttrValue::Borders(Borders::default()).unwrap_flag();
    }

    #[test]
    #[should_panic]
    fn unwrapping_inputtype_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_input_type();
    }

    #[test]
    #[should_panic]
    fn unwrapping_length_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_length();
    }

    #[test]
    #[should_panic]
    fn unwrapping_shape_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_shape();
    }

    #[test]
    #[should_panic]
    fn unwrapping_size_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_size();
    }

    #[test]
    #[should_panic]
    fn unwrapping_string_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_string();
    }

    #[test]
    #[should_panic]
    fn unwrapping_style_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_style();
    }

    #[test]
    #[should_panic]
    fn unwrapping_table_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_table();
    }

    #[test]
    #[should_panic]
    fn unwrapping_text_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_text();
    }

    #[test]
    #[should_panic]
    fn unwrapping_textmodifiers_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_text_modifiers();
    }

    #[test]
    #[should_panic]
    fn unwrapping_title_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_title();
    }

    #[test]
    #[should_panic]
    fn unwrapping_payload_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_payload();
    }
}
