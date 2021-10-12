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
mod builder;
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
pub use builder::PropsBuilder;
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
    attrs: HashMap<AttrSelector, Attribute>,
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
    pub fn get(&self, query: AttrSelector) -> Option<Attribute> {
        self.attrs.get(&query).cloned()
    }

    /// ### get_or
    ///
    /// Get, if any, the attribute associated to the selector
    /// or return the fallback value `default`
    pub fn get_or(&self, query: AttrSelector, default: Attribute) -> Attribute {
        self.get(query).unwrap_or(default)
    }

    /// ### set
    ///
    /// Set a new attribute into Properties
    pub fn set(&mut self, query: AttrSelector, value: Attribute) {
        self.attrs.insert(query, value);
    }
}

// -- Attributes

/// ## Attribute
///
/// Describes a single attribute in the component properties.
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Attribute {
    Alignment(Alignment),
    Borders(Borders),
    Color(Color),
    Dataset(Dataset),
    Direction(Direction),
    Flag(bool),
    InputType(InputType),
    Shape(Shape),
    Size(u16),
    Style(Style),
    Table(Table),
    Text(TextSpan),
    TextModifiers(TextModifiers),
    Title((String, Alignment)),
    /// User defined complex attribute value
    Payload(PropPayload),
}

/// ## AttrSelector
///
/// Describes a "selector" to query an attribute on props.
/// The selector must identify uniquely an attribute in the properties.
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
pub enum AttrSelector {
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
