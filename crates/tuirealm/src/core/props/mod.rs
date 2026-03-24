//! The `props` module exposes the Properties supported by the components and all the values they can get.

// -- modules
mod any;
mod attr_value;
mod borders;
mod direction;
mod input_type;
mod layout;
mod prop_value;
mod props_store;
mod shape;
mod texts;

// -- exports
pub use any::{AnyPropBox, PropBound};
pub use attr_value::AttrValue;
pub use borders::{BorderSides, BorderType, Borders};
pub use direction::Direction;
pub use input_type::InputType;
pub use layout::Layout;
pub use prop_value::{PropPayload, PropValue};
pub use props_store::Props;
pub use shape::Shape;
pub use texts::{LineStatic, SpanStatic, Table, TableBuilder, TextStatic, Title};

pub use crate::ratatui::layout::{HorizontalAlignment, VerticalAlignment};
pub use crate::ratatui::style::{Color, Modifier as TextModifiers, Style};

/// Describes a "selector" to query a attribute on props.
///
/// The selector must uniquely identify a attribute in the properties.
/// Check each attribute documentation to see how they're supposed to be used, but remember that
/// when implementing a component, you're free to use each attribute as you prefer!
/// (But consider documenting if they have different meaning)
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
pub enum Attribute {
    /// Horizontal Layout Alignment
    AlignmentHorizontal,
    /// Vertical Layout Alignment
    AlignmentVertical,
    /// Background color or style
    Background,
    /// Borders styles
    Borders,
    /// Main color
    Color,
    /// Component content. Generic purpose
    Content,
    /// Dataset of component; should be associated to `AttrValue::Dataset`
    Dataset,
    /// Component layout direction
    Direction,
    /// Describes whether the component is disabled (e.g. an Input)
    Disabled,
    /// Whether to display or not the component. This should be reserved to hide components.
    /// As shown in stdlib and in example, its value should be `AttrValue::Flag` and should be checked on top of the
    /// `view()` method to choose whether to or not to render the component.
    Display,
    /// Reserved for tracking focus on component.
    /// You should not implement focus by yourself, since it's already read/written by the `active()` and `blur()` methods on
    /// view/application. When implementing a component, its value should be read-only.
    /// The value is always `AttrValue::Flag`
    Focus,
    /// Should be used to use a different style from default when component is not enabled.
    FocusStyle,
    /// Foreground color or style
    Foreground,
    /// Height size. Useful when building layouts or containers
    Height,
    /// String to prepend to highlighted items in list or other
    HighlightedStr,
    /// Color to apply to highlighted items
    HighlightedColor,
    /// Maximum input length for input fields
    InputLength,
    /// Input type for input fields
    InputType,
    /// Defines a layout
    Layout,
    /// A map of colors for complex components
    Palette,
    /// Intended to decide whether to rewind when reaching boundaries on list/tables
    Rewind,
    /// Intended to store a `AttrValue::Shape`
    Shape,
    /// Should be used to choose whether to make list interactive (scrollable) or not
    Scroll,
    /// Intended as scroll step for fast scroll, for example when using `PageUp`
    ScrollStep,
    /// Component style
    Style,
    /// Component text content
    Text,
    /// Text align
    TextAlign,
    /// Text properties
    TextProps,
    /// Whether to wrap text (or how)
    TextWrap,
    /// Component box title
    Title,
    /// A generic component value
    Value,
    /// Component width; useful when using containers or layouts
    Width,
    /// A user defined property
    Custom(&'static str),
}
