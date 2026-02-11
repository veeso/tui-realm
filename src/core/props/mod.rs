//! The `props` module exposes the Properties supported by the components and all the values they can get.
use alloc::string::String;
use crate::core::compat::HashMap;

// -- modules
mod any;
mod borders;
mod direction;
mod input_type;
mod layout;
mod shape;
mod texts;
mod value;

// -- exports
pub use any::{AnyPropBox, PropBound};
pub use borders::{BorderSides, BorderType, Borders};
pub use direction::Direction;
pub use input_type::InputType;
pub use layout::Layout;
pub use shape::Shape;
pub use texts::{LineStatic, SpanStatic, Table, TableBuilder, TextStatic, Title};
pub use value::{PropPayload, PropValue};

pub use crate::ratatui::layout::{HorizontalAlignment, VerticalAlignment};
pub use crate::ratatui::style::{Color, Modifier as TextModifiers, Style};

/// The props struct holds all the attributes associated to the component.
/// Properties have been designed to be versatile for all kind of components, but without introducing
/// too many attributes at the same time.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Props {
    attrs: HashMap<Attribute, AttrValue>,
}

impl Props {
    /// Get, if any, the attribute associated to the selector.
    ///
    /// This function clones the returned value.
    pub fn get(&self, query: Attribute) -> Option<AttrValue> {
        self.attrs.get(&query).cloned()
    }

    /// Get, if any, the attribute associated to the selector
    /// or return the fallback value `default`
    pub fn get_or(&self, query: Attribute, default: AttrValue) -> AttrValue {
        self.get(query).unwrap_or(default)
    }

    /// Get, if any, the attribute associated to the selector by reference.
    pub fn get_ref(&self, query: Attribute) -> Option<&AttrValue> {
        self.attrs.get(&query)
    }

    /// Get, if any, the attribute associated to the selector by mutable reference.
    pub fn get_mut(&mut self, query: Attribute) -> Option<&mut AttrValue> {
        self.attrs.get_mut(&query)
    }

    /// Set a new attribute into Properties
    pub fn set(&mut self, query: Attribute, value: AttrValue) {
        self.attrs.insert(query, value);
    }
}

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

// -- AttrValues

/// Describes a single attribute in the component properties.
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum AttrValue {
    AlignmentHorizontal(HorizontalAlignment),
    AlignmentVertical(VerticalAlignment),
    Borders(Borders),
    Color(Color),
    Direction(Direction),
    Flag(bool),
    InputType(InputType),
    Layout(Layout),
    Length(usize),
    Number(isize),
    Shape(Shape),
    Size(u16),
    String(String),
    Style(Style),
    Table(Table),
    TextSpan(SpanStatic),
    TextLine(LineStatic),
    Text(TextStatic),
    TextModifiers(TextModifiers),
    Title(Title),
    /// User defined complex attribute value
    Payload(PropPayload),
}

impl AttrValue {
    // -- unwrappers

    /// Get the inner Horizontal Alignment value from AttrValue, or panic.
    pub fn unwrap_alignment_horizontal(self) -> HorizontalAlignment {
        match self {
            AttrValue::AlignmentHorizontal(v) => v,
            _ => panic!("AttrValue is not AlignmentHorizontal"),
        }
    }

    /// Get the inner Vertical Alignment value from AttrValue, or panic.
    pub fn unwrap_alignment_vertical(self) -> VerticalAlignment {
        match self {
            AttrValue::AlignmentVertical(v) => v,
            _ => panic!("AttrValue is not AlignmentVertical"),
        }
    }

    /// Get the inner Borders value from AttrValue, or panic.
    pub fn unwrap_borders(self) -> Borders {
        match self {
            AttrValue::Borders(b) => b,
            _ => panic!("AttrValue is not Borders"),
        }
    }

    /// Get the inner Color value from AttrValue, or panic.
    pub fn unwrap_color(self) -> Color {
        match self {
            AttrValue::Color(x) => x,
            _ => panic!("AttrValue is not Color"),
        }
    }

    /// Get the inner Direction value from AttrValue, or panic.
    pub fn unwrap_direction(self) -> Direction {
        match self {
            AttrValue::Direction(x) => x,
            _ => panic!("AttrValue is not Direction"),
        }
    }

    /// Get the inner Flag value from AttrValue, or panic.
    pub fn unwrap_flag(self) -> bool {
        match self {
            AttrValue::Flag(x) => x,
            _ => panic!("AttrValue is not Flag"),
        }
    }

    /// Get the inner InputType value from AttrValue, or panic.
    pub fn unwrap_input_type(self) -> InputType {
        match self {
            AttrValue::InputType(x) => x,
            _ => panic!("AttrValue is not InputType"),
        }
    }

    /// Get the inner Layout value from AttrValue, or panic.
    pub fn unwrap_layout(self) -> Layout {
        match self {
            AttrValue::Layout(l) => l,
            _ => panic!("AttrValue is not a Layout"),
        }
    }

    /// Get the inner Length value from AttrValue, or panic.
    pub fn unwrap_length(self) -> usize {
        match self {
            AttrValue::Length(x) => x,
            _ => panic!("AttrValue is not Length"),
        }
    }

    /// Get the inner Number value from AttrValue, or panic.
    pub fn unwrap_number(self) -> isize {
        match self {
            AttrValue::Number(x) => x,
            _ => panic!("AttrValue is not Number"),
        }
    }

    /// Get the inner Shape value from AttrValue, or panic.
    pub fn unwrap_shape(self) -> Shape {
        match self {
            AttrValue::Shape(x) => x,
            _ => panic!("AttrValue is not Shape"),
        }
    }

    /// Get the inner Size value from AttrValue, or panic.
    pub fn unwrap_size(self) -> u16 {
        match self {
            AttrValue::Size(x) => x,
            _ => panic!("AttrValue is not Size"),
        }
    }

    /// Get the inner String value from AttrValue, or panic.
    pub fn unwrap_string(self) -> String {
        match self {
            AttrValue::String(x) => x,
            _ => panic!("AttrValue is not String"),
        }
    }

    /// Get the inner Style value from AttrValue, or panic.
    pub fn unwrap_style(self) -> Style {
        match self {
            AttrValue::Style(x) => x,
            _ => panic!("AttrValue is not Style"),
        }
    }

    /// Get the inner Table value from AttrValue, or panic.
    pub fn unwrap_table(self) -> Table {
        match self {
            AttrValue::Table(x) => x,
            _ => panic!("AttrValue is not Table"),
        }
    }

    /// Get the inner [`SpanStatic`] value from AttrValue, or panic.
    pub fn unwrap_textspan(self) -> SpanStatic {
        match self {
            AttrValue::TextSpan(x) => x,
            _ => panic!("AttrValue is not TextSpan"),
        }
    }

    /// Get the inner [`LineStatic`] value from AttrValue, or panic.
    pub fn unwrap_textline(self) -> LineStatic {
        match self {
            AttrValue::TextLine(x) => x,
            _ => panic!("AttrValue is not TextLine"),
        }
    }

    /// Get the inner [`TextStatic`] value from AttrValue, or panic.
    pub fn unwrap_text(self) -> TextStatic {
        match self {
            AttrValue::Text(x) => x,
            _ => panic!("AttrValue is not Text"),
        }
    }

    /// Get the inner TextModifiers value from AttrValue, or panic.
    pub fn unwrap_text_modifiers(self) -> TextModifiers {
        match self {
            AttrValue::TextModifiers(x) => x,
            _ => panic!("AttrValue is not TextModifiers"),
        }
    }

    /// Get the inner [`Title`] value from AttrValue, or panic.
    pub fn unwrap_title(self) -> Title {
        match self {
            AttrValue::Title(x) => x,
            _ => panic!("AttrValue is not Title"),
        }
    }

    /// Get the inner Payload value from AttrValue, or panic.
    pub fn unwrap_payload(self) -> PropPayload {
        match self {
            AttrValue::Payload(x) => x,
            _ => panic!("AttrValue is not Payload"),
        }
    }

    // -- as reference

    /// Get a Horizontal Alignment value from AttrValue, or None
    pub fn as_alignment_horizontal(&self) -> Option<HorizontalAlignment> {
        match self {
            // cheap copy, so no reference
            AttrValue::AlignmentHorizontal(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Vertical Alignment value from AttrValue, or None
    pub fn as_alignment_vertical(&self) -> Option<VerticalAlignment> {
        match self {
            // cheap copy, so no reference
            AttrValue::AlignmentVertical(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Borders value from AttrValue, or None
    pub fn as_borders(&self) -> Option<&Borders> {
        match self {
            AttrValue::Borders(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Color value from AttrValue, or None
    pub fn as_color(&self) -> Option<Color> {
        match self {
            // cheap copy, so no reference
            AttrValue::Color(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Direction value from AttrValue, or None
    pub fn as_direction(&self) -> Option<Direction> {
        match self {
            // cheap copy, so no reference
            AttrValue::Direction(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Flag value from AttrValue, or None
    pub fn as_flag(&self) -> Option<bool> {
        match self {
            // cheap copy, so no reference
            AttrValue::Flag(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a InputType value from AttrValue, or None
    pub fn as_input_type(&self) -> Option<&InputType> {
        match self {
            AttrValue::InputType(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Layout value from AttrValue, or None
    pub fn as_layout(&self) -> Option<&Layout> {
        match self {
            AttrValue::Layout(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Length value from AttrValue, or None
    pub fn as_length(&self) -> Option<usize> {
        match self {
            // cheap copy, so no reference
            AttrValue::Length(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Number value from AttrValue, or None
    pub fn as_number(&self) -> Option<isize> {
        match self {
            // cheap copy, so no reference
            AttrValue::Number(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Shape value from AttrValue, or None
    pub fn as_shape(&self) -> Option<&Shape> {
        match self {
            AttrValue::Shape(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Size value from AttrValue, or None
    pub fn as_size(&self) -> Option<u16> {
        match self {
            // cheap copy, so no reference
            AttrValue::Size(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a String value from AttrValue, or None
    pub fn as_string(&self) -> Option<&String> {
        match self {
            AttrValue::String(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Style value from AttrValue, or None
    pub fn as_style(&self) -> Option<Style> {
        match self {
            // cheap copy, so no reference
            AttrValue::Style(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Table value from AttrValue, or None
    pub fn as_table(&self) -> Option<&Table> {
        match self {
            AttrValue::Table(v) => Some(v),
            _ => None,
        }
    }

    /// Get a [`SpanStatic`] value from AttrValue, or None
    pub fn as_textspan(&self) -> Option<&SpanStatic> {
        match self {
            AttrValue::TextSpan(v) => Some(v),
            _ => None,
        }
    }

    /// Get a [`LineStatic`] value from AttrValue, or None
    pub fn as_textline(&self) -> Option<&LineStatic> {
        match self {
            AttrValue::TextLine(v) => Some(v),
            _ => None,
        }
    }

    /// Get a [`TextStatic`] value from AttrValue, or None
    pub fn as_text(&self) -> Option<&TextStatic> {
        match self {
            AttrValue::Text(v) => Some(v),
            _ => None,
        }
    }

    /// Get a TextModifiers value from AttrValue, or None
    pub fn as_text_modifiers(&self) -> Option<TextModifiers> {
        match self {
            // cheap copy, so no reference
            AttrValue::TextModifiers(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a [`Title`] value from AttrValue, or None
    pub fn as_title(&self) -> Option<&Title> {
        match self {
            AttrValue::Title(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Payload value from AttrValue, or None
    pub fn as_payload(&self) -> Option<&PropPayload> {
        match self {
            AttrValue::Payload(v) => Some(v),
            _ => None,
        }
    }

    // -- as mutable references

    /// Get a Horizontal Alignment value from AttrValue, or None
    pub fn as_alignment_horizontal_mut(&mut self) -> Option<&mut HorizontalAlignment> {
        match self {
            AttrValue::AlignmentHorizontal(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Vertical Alignment value from AttrValue, or None
    pub fn as_alignment_vertical_mut(&mut self) -> Option<&mut VerticalAlignment> {
        match self {
            AttrValue::AlignmentVertical(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Borders value from AttrValue, or None
    pub fn as_borders_mut(&mut self) -> Option<&mut Borders> {
        match self {
            AttrValue::Borders(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Color value from AttrValue, or None
    pub fn as_color_mut(&mut self) -> Option<&mut Color> {
        match self {
            AttrValue::Color(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Direction value from AttrValue, or None
    pub fn as_direction_mut(&mut self) -> Option<&mut Direction> {
        match self {
            AttrValue::Direction(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Flag value from AttrValue, or None
    pub fn as_flag_mut(&mut self) -> Option<&mut bool> {
        match self {
            AttrValue::Flag(v) => Some(v),
            _ => None,
        }
    }

    /// Get a InputType value from AttrValue, or None
    pub fn as_input_type_mut(&mut self) -> Option<&mut InputType> {
        match self {
            AttrValue::InputType(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Layout value from AttrValue, or None
    pub fn as_layout_mut(&mut self) -> Option<&mut Layout> {
        match self {
            AttrValue::Layout(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Length value from AttrValue, or None
    pub fn as_length_mut(&mut self) -> Option<&mut usize> {
        match self {
            AttrValue::Length(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Number value from AttrValue, or None
    pub fn as_number_mut(&mut self) -> Option<&mut isize> {
        match self {
            AttrValue::Number(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Shape value from AttrValue, or None
    pub fn as_shape_mut(&mut self) -> Option<&mut Shape> {
        match self {
            AttrValue::Shape(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Size value from AttrValue, or None
    pub fn as_size_mut(&mut self) -> Option<&mut u16> {
        match self {
            AttrValue::Size(v) => Some(v),
            _ => None,
        }
    }

    /// Get a String value from AttrValue, or None
    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        match self {
            AttrValue::String(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Style value from AttrValue, or None
    pub fn as_style_mut(&mut self) -> Option<&mut Style> {
        match self {
            AttrValue::Style(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Table value from AttrValue, or None
    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        match self {
            AttrValue::Table(v) => Some(v),
            _ => None,
        }
    }

    /// Get a [`SpanStatic`] value from AttrValue, or None
    pub fn as_textspan_mut(&mut self) -> Option<&mut SpanStatic> {
        match self {
            AttrValue::TextSpan(v) => Some(v),
            _ => None,
        }
    }

    /// Get a [`LineStatic`] value from AttrValue, or None
    pub fn as_textline_mut(&mut self) -> Option<&mut LineStatic> {
        match self {
            AttrValue::TextLine(v) => Some(v),
            _ => None,
        }
    }

    /// Get a [`TextStatic`] value from AttrValue, or None
    pub fn as_text_mut(&mut self) -> Option<&mut TextStatic> {
        match self {
            AttrValue::Text(v) => Some(v),
            _ => None,
        }
    }

    /// Get a TextModifiers value from AttrValue, or None
    pub fn as_text_modifiers_mut(&mut self) -> Option<&mut TextModifiers> {
        match self {
            AttrValue::TextModifiers(v) => Some(v),
            _ => None,
        }
    }

    /// Get a [`Title`] value from AttrValue, or None
    pub fn as_title_mut(&mut self) -> Option<&mut Title> {
        match self {
            AttrValue::Title(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Payload value from AttrValue, or None
    pub fn as_payload_mut(&mut self) -> Option<&mut PropPayload> {
        match self {
            AttrValue::Payload(v) => Some(v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_set_get_props() {
        let mut props = Props::default();
        assert_eq!(props.get(Attribute::AlignmentHorizontal), None);
        assert_eq!(
            props.get_or(
                Attribute::AlignmentHorizontal,
                AttrValue::AlignmentHorizontal(HorizontalAlignment::Center)
            ),
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center)
        );
        assert_eq!(props.get_ref(Attribute::AlignmentHorizontal), None);

        props.set(
            Attribute::AlignmentHorizontal,
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Left),
        );
        assert_eq!(
            props.get(Attribute::AlignmentHorizontal),
            Some(AttrValue::AlignmentHorizontal(HorizontalAlignment::Left))
        );
        assert_eq!(
            props.get_or(
                Attribute::AlignmentHorizontal,
                AttrValue::AlignmentHorizontal(HorizontalAlignment::Center)
            ),
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Left)
        );
        assert_eq!(
            props.get_ref(Attribute::AlignmentHorizontal),
            Some(&AttrValue::AlignmentHorizontal(HorizontalAlignment::Left))
        );

        let val = props.get_mut(Attribute::AlignmentHorizontal).unwrap();
        assert_eq!(
            val,
            &AttrValue::AlignmentHorizontal(HorizontalAlignment::Left)
        );
        let v = val.as_alignment_horizontal_mut().unwrap();
        *v = HorizontalAlignment::Center;

        assert_eq!(
            props.get_ref(Attribute::AlignmentHorizontal).unwrap(),
            &AttrValue::AlignmentHorizontal(HorizontalAlignment::Center)
        );
    }

    #[test]
    fn unwrapping_should_unwrap() {
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center)
                .unwrap_alignment_horizontal(),
            HorizontalAlignment::Center
        );
        assert_eq!(
            AttrValue::AlignmentVertical(VerticalAlignment::Top).unwrap_alignment_vertical(),
            VerticalAlignment::Top
        );
        assert_eq!(
            AttrValue::Borders(Borders::default()).unwrap_borders(),
            Borders::default()
        );
        assert_eq!(AttrValue::Color(Color::Red).unwrap_color(), Color::Red);
        assert_eq!(
            AttrValue::Direction(Direction::Left).unwrap_direction(),
            Direction::Left
        );
        assert_eq!(AttrValue::Flag(true).unwrap_flag(), true);
        assert_eq!(
            AttrValue::InputType(InputType::Number).unwrap_input_type(),
            InputType::Number
        );
        assert_eq!(
            AttrValue::Layout(Layout::default()).unwrap_layout(),
            Layout::default()
        );
        assert_eq!(AttrValue::Length(12).unwrap_length(), 12);
        assert_eq!(AttrValue::Number(-24).unwrap_number(), -24);
        assert_eq!(AttrValue::Shape(Shape::Layer).unwrap_shape(), Shape::Layer);
        assert_eq!(AttrValue::Size(12).unwrap_size(), 12);
        assert_eq!(
            AttrValue::String(String::from("pippo")).unwrap_string(),
            String::from("pippo")
        );
        assert_eq!(
            AttrValue::Style(Style::default()).unwrap_style(),
            Style::default()
        );
        assert_eq!(
            AttrValue::Table(Table::default()).unwrap_table(),
            Table::default()
        );
        assert_eq!(
            AttrValue::TextSpan(SpanStatic::default()).unwrap_textspan(),
            SpanStatic::default()
        );
        assert_eq!(
            AttrValue::TextLine(LineStatic::default()).unwrap_textline(),
            LineStatic::default()
        );
        assert_eq!(
            AttrValue::Text(TextStatic::default()).unwrap_text(),
            TextStatic::default()
        );
        assert_eq!(
            AttrValue::TextModifiers(TextModifiers::BOLD).unwrap_text_modifiers(),
            TextModifiers::BOLD
        );
        assert_eq!(
            AttrValue::Title(
                Title::from(String::from("pippo")).alignment(HorizontalAlignment::Left)
            )
            .unwrap_title(),
            (Title::from(String::from("pippo")).alignment(HorizontalAlignment::Left))
        );
        assert_eq!(
            AttrValue::Payload(PropPayload::None).unwrap_payload(),
            PropPayload::None
        );
    }

    #[test]
    fn as_attrvalue() {
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_alignment_horizontal(),
            Some(HorizontalAlignment::Center)
        );
        assert_eq!(
            AttrValue::Color(Color::Black).as_alignment_horizontal(),
            None
        );
        assert_eq!(
            AttrValue::AlignmentVertical(VerticalAlignment::Top).as_alignment_vertical(),
            Some(VerticalAlignment::Top)
        );
        assert_eq!(AttrValue::Color(Color::Black).as_alignment_vertical(), None);

        assert_eq!(
            AttrValue::Borders(Borders::default()).as_borders(),
            Some(&Borders::default())
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_borders(),
            None
        );

        assert_eq!(
            AttrValue::Color(Color::Black).as_color(),
            Some(Color::Black)
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_color(),
            None
        );

        assert_eq!(
            AttrValue::Direction(Direction::Down).as_direction(),
            Some(Direction::Down)
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_direction(),
            None
        );

        assert_eq!(AttrValue::Flag(true).as_flag(), Some(true));
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_flag(),
            None
        );

        assert_eq!(
            AttrValue::InputType(InputType::Color).as_input_type(),
            Some(&InputType::Color)
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_input_type(),
            None
        );

        assert_eq!(
            AttrValue::Layout(Layout::default()).as_layout(),
            Some(&Layout::default())
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_layout(),
            None
        );

        assert_eq!(AttrValue::Length(1).as_length(), Some(1));
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_length(),
            None
        );

        assert_eq!(AttrValue::Number(-1).as_number(), Some(-1));
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_number(),
            None
        );

        assert_eq!(
            AttrValue::Shape(Shape::Layer).as_shape(),
            Some(&Shape::Layer)
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_shape(),
            None
        );

        assert_eq!(AttrValue::Size(1).as_size(), Some(1));
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_size(),
            None
        );

        assert_eq!(
            AttrValue::String("hello".into()).as_string(),
            Some(&"hello".to_string())
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_string(),
            None
        );

        assert_eq!(
            AttrValue::Style(Style::default()).as_style(),
            Some(Style::default())
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_style(),
            None
        );

        assert_eq!(AttrValue::Table(Vec::new()).as_table(), Some(&Vec::new()));
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_table(),
            None
        );

        assert_eq!(
            AttrValue::TextSpan(SpanStatic::default()).as_textspan(),
            Some(&SpanStatic::default())
        );
        assert_eq!(
            AttrValue::TextLine(LineStatic::default()).as_textline(),
            Some(&LineStatic::default())
        );
        assert_eq!(
            AttrValue::Text(TextStatic::default()).as_text(),
            Some(&TextStatic::default())
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_text(),
            None
        );

        assert_eq!(
            AttrValue::TextModifiers(TextModifiers::all()).as_text_modifiers(),
            Some(TextModifiers::all())
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_text_modifiers(),
            None
        );

        assert_eq!(
            AttrValue::Title(Title::from("hello").alignment(HorizontalAlignment::Center))
                .as_title(),
            Some(&Title::from("hello").alignment(HorizontalAlignment::Center))
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_title(),
            None
        );

        assert_eq!(
            AttrValue::Payload(PropPayload::None).as_payload(),
            Some(&PropPayload::None)
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_payload(),
            None
        );
    }

    #[test]
    fn as_attrvalue_mut() {
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center)
                .as_alignment_horizontal_mut(),
            Some(&mut HorizontalAlignment::Center)
        );
        assert_eq!(
            AttrValue::Color(Color::Black).as_alignment_horizontal_mut(),
            None
        );
        assert_eq!(
            AttrValue::AlignmentVertical(VerticalAlignment::Top).as_alignment_vertical_mut(),
            Some(&mut VerticalAlignment::Top)
        );
        assert_eq!(
            AttrValue::Color(Color::Black).as_alignment_vertical_mut(),
            None
        );

        assert_eq!(
            AttrValue::Borders(Borders::default()).as_borders_mut(),
            Some(&mut Borders::default())
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_borders_mut(),
            None
        );

        assert_eq!(
            AttrValue::Color(Color::Black).as_color_mut(),
            Some(&mut Color::Black)
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_color_mut(),
            None
        );

        assert_eq!(
            AttrValue::Direction(Direction::Down).as_direction_mut(),
            Some(&mut Direction::Down)
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_direction_mut(),
            None
        );

        assert_eq!(AttrValue::Flag(true).as_flag_mut(), Some(&mut true));
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_flag_mut(),
            None
        );

        assert_eq!(
            AttrValue::InputType(InputType::Color).as_input_type_mut(),
            Some(&mut InputType::Color)
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_input_type_mut(),
            None
        );

        assert_eq!(
            AttrValue::Layout(Layout::default()).as_layout_mut(),
            Some(&mut Layout::default())
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_layout_mut(),
            None
        );

        assert_eq!(AttrValue::Length(1).as_length_mut(), Some(&mut 1));
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_length_mut(),
            None
        );

        assert_eq!(AttrValue::Number(-1).as_number_mut(), Some(&mut -1));
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_number_mut(),
            None
        );

        assert_eq!(
            AttrValue::Shape(Shape::Layer).as_shape_mut(),
            Some(&mut Shape::Layer)
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_shape_mut(),
            None
        );

        assert_eq!(AttrValue::Size(1).as_size_mut(), Some(&mut 1));
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_size_mut(),
            None
        );

        assert_eq!(
            AttrValue::String("hello".into()).as_string_mut(),
            Some(&mut "hello".to_string())
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_string_mut(),
            None
        );

        assert_eq!(
            AttrValue::Style(Style::default()).as_style_mut(),
            Some(&mut Style::default())
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_style_mut(),
            None
        );

        assert_eq!(
            AttrValue::Table(Vec::new()).as_table_mut(),
            Some(&mut Vec::new())
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_table_mut(),
            None
        );

        assert_eq!(
            AttrValue::TextSpan(SpanStatic::default()).as_textspan_mut(),
            Some(&mut SpanStatic::default())
        );
        assert_eq!(
            AttrValue::TextLine(LineStatic::default()).as_textline_mut(),
            Some(&mut LineStatic::default())
        );
        assert_eq!(
            AttrValue::Text(TextStatic::default()).as_text_mut(),
            Some(&mut TextStatic::default())
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_text_mut(),
            None
        );

        assert_eq!(
            AttrValue::TextModifiers(TextModifiers::all()).as_text_modifiers_mut(),
            Some(&mut TextModifiers::all())
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_text_modifiers_mut(),
            None
        );

        assert_eq!(
            AttrValue::Title(Title::from("hello").alignment(HorizontalAlignment::Right))
                .as_title_mut(),
            Some(&mut Title::from("hello").alignment(HorizontalAlignment::Right))
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_title_mut(),
            None
        );

        assert_eq!(
            AttrValue::Payload(PropPayload::None).as_payload_mut(),
            Some(&mut PropPayload::None)
        );
        assert_eq!(
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Center).as_payload_mut(),
            None
        );
    }

    #[test]
    #[should_panic]
    fn unwrapping_alignment_horizontal_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_alignment_horizontal();
    }

    #[test]
    #[should_panic]
    fn unwrapping_alignment_vertical_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_alignment_vertical();
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
    fn unwrapping_layout_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_layout();
    }

    #[test]
    #[should_panic]
    fn unwrapping_length_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_length();
    }

    #[test]
    #[should_panic]
    fn unwrapping_number_should_panic_if_not_identity() {
        AttrValue::Flag(true).unwrap_number();
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
