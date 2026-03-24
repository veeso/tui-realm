use ratatui::layout::{HorizontalAlignment, VerticalAlignment};
use ratatui::style::{Color, Modifier as TextModifiers, Style};
use ratatui::text::{Line, Span, Text};

use crate::props::prop_value_ref::PropPayloadRef;
use crate::props::{AttrValue, Borders, Direction, InputType, Layout, Shape, Table, Title};

/// Describes a single attribute in the component properties as a reference.
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum AttrValueRef<'a> {
    AlignmentHorizontal(HorizontalAlignment),
    AlignmentVertical(VerticalAlignment),
    Borders(Borders),
    Color(Color),
    Direction(Direction),
    Flag(bool),
    InputType(&'a InputType),
    Layout(&'a Layout),
    Length(usize),
    Number(isize),
    Shape(&'a Shape),
    Size(u16),
    String(&'a str),
    Style(Style),
    Table(&'a Table),
    TextSpan(&'a Span<'a>),
    TextLine(&'a Line<'a>),
    Text(&'a Text<'a>),
    TextModifiers(TextModifiers),
    Title(&'a Title),
    /// User defined complex attribute value
    Payload(PropPayloadRef<'a>),
}

impl<'a> AttrValueRef<'a> {
    // -- unwrappers

    /// Get the inner Horizontal Alignment value from AttrValue, or panic.
    pub fn unwrap_alignment_horizontal(self) -> HorizontalAlignment {
        match self {
            AttrValueRef::AlignmentHorizontal(v) => v,
            _ => panic!("AttrValue is not AlignmentHorizontal"),
        }
    }

    /// Get the inner Vertical Alignment value from AttrValue, or panic.
    pub fn unwrap_alignment_vertical(self) -> VerticalAlignment {
        match self {
            AttrValueRef::AlignmentVertical(v) => v,
            _ => panic!("AttrValue is not AlignmentVertical"),
        }
    }

    /// Get the inner Borders value from AttrValue, or panic.
    pub fn unwrap_borders(self) -> Borders {
        match self {
            AttrValueRef::Borders(b) => b,
            _ => panic!("AttrValue is not Borders"),
        }
    }

    /// Get the inner Color value from AttrValue, or panic.
    pub fn unwrap_color(self) -> Color {
        match self {
            AttrValueRef::Color(x) => x,
            _ => panic!("AttrValue is not Color"),
        }
    }

    /// Get the inner Direction value from AttrValue, or panic.
    pub fn unwrap_direction(self) -> Direction {
        match self {
            AttrValueRef::Direction(x) => x,
            _ => panic!("AttrValue is not Direction"),
        }
    }

    /// Get the inner Flag value from AttrValue, or panic.
    pub fn unwrap_flag(self) -> bool {
        match self {
            AttrValueRef::Flag(x) => x,
            _ => panic!("AttrValue is not Flag"),
        }
    }

    /// Get the inner InputType value from AttrValue, or panic.
    pub fn unwrap_input_type(self) -> &'a InputType {
        match self {
            AttrValueRef::InputType(x) => x,
            _ => panic!("AttrValue is not InputType"),
        }
    }

    /// Get the inner Layout value from AttrValue, or panic.
    pub fn unwrap_layout(self) -> &'a Layout {
        match self {
            AttrValueRef::Layout(l) => l,
            _ => panic!("AttrValue is not a Layout"),
        }
    }

    /// Get the inner Length value from AttrValue, or panic.
    pub fn unwrap_length(self) -> usize {
        match self {
            AttrValueRef::Length(x) => x,
            _ => panic!("AttrValue is not Length"),
        }
    }

    /// Get the inner Number value from AttrValue, or panic.
    pub fn unwrap_number(self) -> isize {
        match self {
            AttrValueRef::Number(x) => x,
            _ => panic!("AttrValue is not Number"),
        }
    }

    /// Get the inner Shape value from AttrValue, or panic.
    pub fn unwrap_shape(self) -> &'a Shape {
        match self {
            AttrValueRef::Shape(x) => x,
            _ => panic!("AttrValue is not Shape"),
        }
    }

    /// Get the inner Size value from AttrValue, or panic.
    pub fn unwrap_size(self) -> u16 {
        match self {
            AttrValueRef::Size(x) => x,
            _ => panic!("AttrValue is not Size"),
        }
    }

    /// Get the inner String value from AttrValue, or panic.
    pub fn unwrap_string(self) -> &'a str {
        match self {
            AttrValueRef::String(x) => x,
            _ => panic!("AttrValue is not String"),
        }
    }

    /// Get the inner Style value from AttrValue, or panic.
    pub fn unwrap_style(self) -> Style {
        match self {
            AttrValueRef::Style(x) => x,
            _ => panic!("AttrValue is not Style"),
        }
    }

    /// Get the inner Table value from AttrValue, or panic.
    pub fn unwrap_table(self) -> &'a Table {
        match self {
            AttrValueRef::Table(x) => x,
            _ => panic!("AttrValue is not Table"),
        }
    }

    /// Get the inner [`SpanStatic`] value from AttrValue, or panic.
    pub fn unwrap_textspan(self) -> &'a Span<'a> {
        match self {
            AttrValueRef::TextSpan(x) => x,
            _ => panic!("AttrValue is not TextSpan"),
        }
    }

    /// Get the inner [`LineStatic`] value from AttrValue, or panic.
    pub fn unwrap_textline(self) -> &'a Line<'a> {
        match self {
            AttrValueRef::TextLine(x) => x,
            _ => panic!("AttrValue is not TextLine"),
        }
    }

    /// Get the inner [`TextStatic`] value from AttrValue, or panic.
    pub fn unwrap_text(self) -> &'a Text<'a> {
        match self {
            AttrValueRef::Text(x) => x,
            _ => panic!("AttrValue is not Text"),
        }
    }

    /// Get the inner TextModifiers value from AttrValue, or panic.
    pub fn unwrap_text_modifiers(self) -> TextModifiers {
        match self {
            AttrValueRef::TextModifiers(x) => x,
            _ => panic!("AttrValue is not TextModifiers"),
        }
    }

    /// Get the inner [`Title`] value from AttrValue, or panic.
    pub fn unwrap_title(self) -> &'a Title {
        match self {
            AttrValueRef::Title(x) => x,
            _ => panic!("AttrValue is not Title"),
        }
    }

    /// Get the inner Payload value from AttrValue, or panic.
    pub fn unwrap_payload(self) -> PropPayloadRef<'a> {
        match self {
            AttrValueRef::Payload(x) => x,
            _ => panic!("AttrValue is not Payload"),
        }
    }

    // -- as reference

    /// Get a Horizontal Alignment value from AttrValue, or None
    pub fn as_alignment_horizontal(&self) -> Option<HorizontalAlignment> {
        match self {
            // cheap copy, so no reference
            AttrValueRef::AlignmentHorizontal(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Vertical Alignment value from AttrValue, or None
    pub fn as_alignment_vertical(&self) -> Option<VerticalAlignment> {
        match self {
            // cheap copy, so no reference
            AttrValueRef::AlignmentVertical(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Borders value from AttrValue, or None
    pub fn as_borders(&self) -> Option<Borders> {
        match self {
            // cheap copy, so no reference
            AttrValueRef::Borders(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Color value from AttrValue, or None
    pub fn as_color(&self) -> Option<Color> {
        match self {
            // cheap copy, so no reference
            AttrValueRef::Color(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Direction value from AttrValue, or None
    pub fn as_direction(&self) -> Option<Direction> {
        match self {
            // cheap copy, so no reference
            AttrValueRef::Direction(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Flag value from AttrValue, or None
    pub fn as_flag(&self) -> Option<bool> {
        match self {
            // cheap copy, so no reference
            AttrValueRef::Flag(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a InputType value from AttrValue, or None
    pub fn as_input_type(&self) -> Option<&'a InputType> {
        match self {
            AttrValueRef::InputType(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Layout value from AttrValue, or None
    pub fn as_layout(&self) -> Option<&'a Layout> {
        match self {
            AttrValueRef::Layout(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Length value from AttrValue, or None
    pub fn as_length(&self) -> Option<usize> {
        match self {
            // cheap copy, so no reference
            AttrValueRef::Length(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Number value from AttrValue, or None
    pub fn as_number(&self) -> Option<isize> {
        match self {
            // cheap copy, so no reference
            AttrValueRef::Number(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Shape value from AttrValue, or None
    pub fn as_shape(&self) -> Option<&'a Shape> {
        match self {
            AttrValueRef::Shape(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Size value from AttrValue, or None
    pub fn as_size(&self) -> Option<u16> {
        match self {
            // cheap copy, so no reference
            AttrValueRef::Size(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a String value from AttrValue, or None
    pub fn as_string(&self) -> Option<&'a str> {
        match self {
            AttrValueRef::String(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Style value from AttrValue, or None
    pub fn as_style(&self) -> Option<Style> {
        match self {
            // cheap copy, so no reference
            AttrValueRef::Style(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a Table value from AttrValue, or None
    pub fn as_table(&self) -> Option<&'a Table> {
        match self {
            AttrValueRef::Table(v) => Some(v),
            _ => None,
        }
    }

    /// Get a [`SpanStatic`] value from AttrValue, or None
    pub fn as_textspan(&self) -> Option<&'a Span<'a>> {
        match self {
            AttrValueRef::TextSpan(v) => Some(v),
            _ => None,
        }
    }

    /// Get a [`LineStatic`] value from AttrValue, or None
    pub fn as_textline(&self) -> Option<&'a Line<'a>> {
        match self {
            AttrValueRef::TextLine(v) => Some(v),
            _ => None,
        }
    }

    /// Get a [`TextStatic`] value from AttrValue, or None
    pub fn as_text(&self) -> Option<&'a Text<'a>> {
        match self {
            AttrValueRef::Text(v) => Some(v),
            _ => None,
        }
    }

    /// Get a TextModifiers value from AttrValue, or None
    pub fn as_text_modifiers(&self) -> Option<TextModifiers> {
        match self {
            // cheap copy, so no reference
            AttrValueRef::TextModifiers(v) => Some(*v),
            _ => None,
        }
    }

    /// Get a [`Title`] value from AttrValue, or None
    pub fn as_title(&self) -> Option<&Title> {
        match self {
            AttrValueRef::Title(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Payload value from AttrValue, or None
    pub fn as_payload(&self) -> Option<PropPayloadRef<'a>> {
        match self {
            AttrValueRef::Payload(v) => Some(*v),
            _ => None,
        }
    }

    // -- as mutable references

    /// Get a Horizontal Alignment value from AttrValue, or None
    pub fn as_alignment_horizontal_mut(&mut self) -> Option<&mut HorizontalAlignment> {
        match self {
            AttrValueRef::AlignmentHorizontal(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Vertical Alignment value from AttrValue, or None
    pub fn as_alignment_vertical_mut(&mut self) -> Option<&mut VerticalAlignment> {
        match self {
            AttrValueRef::AlignmentVertical(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Borders value from AttrValue, or None
    pub fn as_borders_mut(&mut self) -> Option<&mut Borders> {
        match self {
            AttrValueRef::Borders(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Color value from AttrValue, or None
    pub fn as_color_mut(&mut self) -> Option<&mut Color> {
        match self {
            AttrValueRef::Color(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Direction value from AttrValue, or None
    pub fn as_direction_mut(&mut self) -> Option<&mut Direction> {
        match self {
            AttrValueRef::Direction(v) => Some(v),
            _ => None,
        }
    }

    /// Get a Flag value from AttrValue, or None
    pub fn as_flag_mut(&mut self) -> Option<&mut bool> {
        match self {
            AttrValueRef::Flag(v) => Some(v),
            _ => None,
        }
    }
}

impl<'a> PartialEq<AttrValue> for AttrValueRef<'a> {
    fn eq(&self, other: &AttrValue) -> bool {
        match other {
            AttrValue::AlignmentHorizontal(horizontal_alignment) => {
                Some(*horizontal_alignment) == self.as_alignment_horizontal()
            }
            AttrValue::AlignmentVertical(vertical_alignment) => {
                Some(*vertical_alignment) == self.as_alignment_vertical()
            }
            AttrValue::Borders(borders) => Some(*borders) == self.as_borders(),
            AttrValue::Color(color) => Some(*color) == self.as_color(),
            AttrValue::Direction(direction) => Some(*direction) == self.as_direction(),
            AttrValue::Flag(flag) => Some(*flag) == self.as_flag(),
            AttrValue::InputType(input_type) => Some(input_type) == self.as_input_type(),
            AttrValue::Layout(layout) => Some(layout) == self.as_layout(),
            AttrValue::Length(length) => Some(*length) == self.as_length(),
            AttrValue::Number(number) => Some(*number) == self.as_number(),
            AttrValue::Shape(shape) => Some(shape) == self.as_shape(),
            AttrValue::Size(size) => Some(*size) == self.as_size(),
            AttrValue::String(string) => Some(string.as_str()) == self.as_string(),
            AttrValue::Style(style) => Some(*style) == self.as_style(),
            AttrValue::Table(items) => Some(items) == self.as_table(),
            AttrValue::TextSpan(span) => Some(span) == self.as_textspan(),
            AttrValue::TextLine(line) => Some(line) == self.as_textline(),
            AttrValue::Text(text) => Some(text) == self.as_text(),
            AttrValue::TextModifiers(modifier) => Some(*modifier) == self.as_text_modifiers(),
            AttrValue::Title(title) => Some(title) == self.as_title(),
            AttrValue::Payload(prop_payload) => self
                .as_payload()
                .map(|p| p == *prop_payload)
                .unwrap_or_default(),
        }
    }
}

// reverse impl to not have position-dependent implementations
// ex. allow `AttrValue == AttrValueRef` AND `AttrValueRef == AttrValue`, without this, it would only allow one of them
impl<'a> PartialEq<AttrValueRef<'a>> for AttrValue {
    fn eq(&self, other: &AttrValueRef<'a>) -> bool {
        *other == *self
    }
}

impl<'a> From<&'a AttrValue> for AttrValueRef<'a> {
    fn from(value: &'a AttrValue) -> Self {
        match value {
            AttrValue::AlignmentHorizontal(horizontal_alignment) => {
                Self::AlignmentHorizontal(*horizontal_alignment)
            }
            AttrValue::AlignmentVertical(vertical_alignment) => {
                Self::AlignmentVertical(*vertical_alignment)
            }
            AttrValue::Borders(borders) => Self::Borders(*borders),
            AttrValue::Color(color) => Self::Color(*color),
            AttrValue::Direction(direction) => Self::Direction(*direction),
            AttrValue::Flag(flag) => Self::Flag(*flag),
            AttrValue::InputType(input_type) => Self::InputType(input_type),
            AttrValue::Layout(layout) => Self::Layout(layout),
            AttrValue::Length(length) => Self::Length(*length),
            AttrValue::Number(number) => Self::Number(*number),
            AttrValue::Shape(shape) => Self::Shape(shape),
            AttrValue::Size(size) => Self::Size(*size),
            AttrValue::String(string) => Self::String(string),
            AttrValue::Style(style) => Self::Style(*style),
            AttrValue::Table(items) => Self::Table(items),
            AttrValue::TextSpan(span) => Self::TextSpan(span),
            AttrValue::TextLine(line) => Self::TextLine(line),
            AttrValue::Text(text) => Self::Text(text),
            AttrValue::TextModifiers(modifier) => Self::TextModifiers(*modifier),
            AttrValue::Title(title) => Self::Title(title),
            AttrValue::Payload(prop_payload) => Self::Payload(prop_payload.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::props::{LineStatic, PropPayload, SpanStatic, TextStatic};

    #[test]
    fn unwrapping_should_unwrap() {
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center)
                .unwrap_alignment_horizontal(),
            HorizontalAlignment::Center
        );
        assert_eq!(
            AttrValueRef::AlignmentVertical(VerticalAlignment::Top).unwrap_alignment_vertical(),
            VerticalAlignment::Top
        );
        assert_eq!(
            AttrValueRef::Borders(Borders::default()).unwrap_borders(),
            Borders::default()
        );
        assert_eq!(AttrValueRef::Color(Color::Red).unwrap_color(), Color::Red);
        assert_eq!(
            AttrValueRef::Direction(Direction::Left).unwrap_direction(),
            Direction::Left
        );
        assert_eq!(AttrValueRef::Flag(true).unwrap_flag(), true);
        assert_eq!(
            AttrValueRef::InputType(&InputType::Number).unwrap_input_type(),
            &InputType::Number
        );
        assert_eq!(
            AttrValueRef::Layout(&Layout::default()).unwrap_layout(),
            &Layout::default()
        );
        assert_eq!(AttrValueRef::Length(12).unwrap_length(), 12);
        assert_eq!(AttrValueRef::Number(-24).unwrap_number(), -24);
        assert_eq!(
            AttrValueRef::Shape(&Shape::Layer).unwrap_shape(),
            &Shape::Layer
        );
        assert_eq!(AttrValueRef::Size(12).unwrap_size(), 12);
        assert_eq!(
            AttrValueRef::String(&String::from("pippo")).unwrap_string(),
            String::from("pippo")
        );
        assert_eq!(
            AttrValueRef::Style(Style::default()).unwrap_style(),
            Style::default()
        );
        assert_eq!(
            AttrValueRef::Table(&Table::default()).unwrap_table(),
            &Table::default()
        );
        assert_eq!(
            AttrValueRef::TextSpan(&SpanStatic::default()).unwrap_textspan(),
            &SpanStatic::default()
        );
        assert_eq!(
            AttrValueRef::TextLine(&LineStatic::default()).unwrap_textline(),
            &LineStatic::default()
        );
        assert_eq!(
            AttrValueRef::Text(&TextStatic::default()).unwrap_text(),
            &TextStatic::default()
        );
        assert_eq!(
            AttrValueRef::TextModifiers(TextModifiers::BOLD).unwrap_text_modifiers(),
            TextModifiers::BOLD
        );
        assert_eq!(
            AttrValueRef::Title(
                &Title::from(String::from("pippo")).alignment(HorizontalAlignment::Left)
            )
            .unwrap_title(),
            (&Title::from(String::from("pippo")).alignment(HorizontalAlignment::Left))
        );
        assert_eq!(
            AttrValueRef::Payload(PropPayloadRef::None).unwrap_payload(),
            PropPayloadRef::None
        );
    }

    #[test]
    fn as_attrvalue() {
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center)
                .as_alignment_horizontal(),
            Some(HorizontalAlignment::Center)
        );
        assert_eq!(
            AttrValueRef::Color(Color::Black).as_alignment_horizontal(),
            None
        );
        assert_eq!(
            AttrValueRef::AlignmentVertical(VerticalAlignment::Top).as_alignment_vertical(),
            Some(VerticalAlignment::Top)
        );
        assert_eq!(
            AttrValueRef::Color(Color::Black).as_alignment_vertical(),
            None
        );

        assert_eq!(
            AttrValueRef::Borders(Borders::default()).as_borders(),
            Some(Borders::default())
        );
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_borders(),
            None
        );

        assert_eq!(
            AttrValueRef::Color(Color::Black).as_color(),
            Some(Color::Black)
        );
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_color(),
            None
        );

        assert_eq!(
            AttrValueRef::Direction(Direction::Down).as_direction(),
            Some(Direction::Down)
        );
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_direction(),
            None
        );

        assert_eq!(AttrValueRef::Flag(true).as_flag(), Some(true));
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_flag(),
            None
        );

        assert_eq!(
            AttrValueRef::InputType(&InputType::Color).as_input_type(),
            Some(&InputType::Color)
        );
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_input_type(),
            None
        );

        assert_eq!(
            AttrValueRef::Layout(&Layout::default()).as_layout(),
            Some(&Layout::default())
        );
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_layout(),
            None
        );

        assert_eq!(AttrValueRef::Length(1).as_length(), Some(1));
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_length(),
            None
        );

        assert_eq!(AttrValueRef::Number(-1).as_number(), Some(-1));
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_number(),
            None
        );

        assert_eq!(
            AttrValueRef::Shape(&Shape::Layer).as_shape(),
            Some(&Shape::Layer)
        );
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_shape(),
            None
        );

        assert_eq!(AttrValueRef::Size(1).as_size(), Some(1));
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_size(),
            None
        );

        assert_eq!(AttrValueRef::String("hello").as_string(), Some("hello"));
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_string(),
            None
        );

        assert_eq!(
            AttrValueRef::Style(Style::default()).as_style(),
            Some(Style::default())
        );
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_style(),
            None
        );

        assert_eq!(
            AttrValueRef::Table(&Vec::new()).as_table(),
            Some(&Vec::new())
        );
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_table(),
            None
        );

        assert_eq!(
            AttrValueRef::TextSpan(&SpanStatic::default()).as_textspan(),
            Some(&SpanStatic::default())
        );
        assert_eq!(
            AttrValueRef::TextLine(&LineStatic::default()).as_textline(),
            Some(&LineStatic::default())
        );
        assert_eq!(
            AttrValueRef::Text(&TextStatic::default()).as_text(),
            Some(&TextStatic::default())
        );
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_text(),
            None
        );

        assert_eq!(
            AttrValueRef::TextModifiers(TextModifiers::all()).as_text_modifiers(),
            Some(TextModifiers::all())
        );
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_text_modifiers(),
            None
        );

        assert_eq!(
            AttrValueRef::Title(&Title::from("hello").alignment(HorizontalAlignment::Center))
                .as_title(),
            Some(&Title::from("hello").alignment(HorizontalAlignment::Center))
        );
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_title(),
            None
        );

        assert_eq!(
            AttrValueRef::Payload(PropPayloadRef::None).as_payload(),
            Some(PropPayloadRef::None)
        );
        assert_eq!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center).as_payload(),
            None
        );
    }

    #[test]
    fn eq_nonref_attrvalue() {
        assert!(AttrValueRef::Flag(true) == AttrValue::Flag(true));
        assert!(!(AttrValueRef::Flag(true) == AttrValue::Size(1)));

        assert!(AttrValueRef::Size(1) == AttrValue::Size(1));
        assert!(!(AttrValueRef::Size(1) == AttrValue::Flag(false)));

        assert!(
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center)
                == AttrValue::AlignmentHorizontal(HorizontalAlignment::Center)
        );
        assert!(
            !(AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center)
                == AttrValue::Flag(false))
        );

        assert!(
            AttrValueRef::AlignmentVertical(VerticalAlignment::Bottom)
                == AttrValue::AlignmentVertical(VerticalAlignment::Bottom)
        );
        assert!(
            !(AttrValueRef::AlignmentVertical(VerticalAlignment::Bottom) == AttrValue::Flag(false))
        );

        assert!(
            AttrValueRef::Borders(Borders::default()) == AttrValue::Borders(Borders::default())
        );
        assert!(!(AttrValueRef::Borders(Borders::default()) == AttrValue::Flag(false)));

        assert!(AttrValueRef::Color(Color::Black) == AttrValue::Color(Color::Black));
        assert!(!(AttrValueRef::Color(Color::Black) == AttrValue::Flag(false)));

        assert!(AttrValueRef::Direction(Direction::Down) == AttrValue::Direction(Direction::Down));
        assert!(!(AttrValueRef::Direction(Direction::Down) == AttrValue::Flag(false)));

        assert!(
            AttrValueRef::InputType(&InputType::Color) == AttrValue::InputType(InputType::Color)
        );
        assert!(!(AttrValueRef::InputType(&InputType::Color) == AttrValue::Flag(false)));

        assert!(AttrValueRef::Layout(&Layout::default()) == AttrValue::Layout(Layout::default()));
        assert!(!(AttrValueRef::Layout(&Layout::default()) == AttrValue::Flag(false)));

        assert!(AttrValueRef::Length(1) == AttrValue::Length(1));
        assert!(!(AttrValueRef::Length(1) == AttrValue::Flag(false)));

        assert!(AttrValueRef::Number(1) == AttrValue::Number(1));
        assert!(!(AttrValueRef::Number(1) == AttrValue::Flag(false)));

        assert!(AttrValueRef::Shape(&Shape::Layer) == AttrValue::Shape(Shape::Layer));
        assert!(!(AttrValueRef::Shape(&Shape::Layer) == AttrValue::Flag(false)));

        assert!(AttrValueRef::String("hello") == AttrValue::String("hello".to_string()));
        assert!(!(AttrValueRef::String("hello") == AttrValue::Flag(false)));

        assert!(AttrValueRef::Style(Style::default()) == AttrValue::Style(Style::default()));
        assert!(!(AttrValueRef::Style(Style::default()) == AttrValue::Flag(false)));

        assert!(AttrValueRef::Table(&Table::new()) == AttrValue::Table(Table::new()));
        assert!(!(AttrValueRef::Table(&Table::new()) == AttrValue::Flag(false)));

        assert!(
            AttrValueRef::TextSpan(&SpanStatic::from("hello"))
                == AttrValue::TextSpan(SpanStatic::from("hello"))
        );
        assert!(!(AttrValueRef::TextSpan(&SpanStatic::from("hello")) == AttrValue::Flag(false)));

        assert!(
            AttrValueRef::TextLine(&LineStatic::from("hello"))
                == AttrValue::TextLine(LineStatic::from("hello"))
        );
        assert!(!(AttrValueRef::TextLine(&LineStatic::from("hello")) == AttrValue::Flag(false)));

        assert!(
            AttrValueRef::Text(&TextStatic::from("hello"))
                == AttrValue::Text(TextStatic::from("hello"))
        );
        assert!(!(AttrValueRef::Text(&TextStatic::from("hello")) == AttrValue::Flag(false)));

        assert!(
            AttrValueRef::TextModifiers(TextModifiers::default())
                == AttrValue::TextModifiers(TextModifiers::default())
        );
        assert!(!(AttrValueRef::TextModifiers(TextModifiers::default()) == AttrValue::Flag(false)));

        assert!(AttrValueRef::Title(&Title::default()) == AttrValue::Title(Title::default()));
        assert!(!(AttrValueRef::Title(&Title::default()) == AttrValue::Flag(false)));

        assert!(
            AttrValueRef::Payload(PropPayloadRef::None) == AttrValue::Payload(PropPayload::None)
        );
        assert!(!(AttrValueRef::Payload(PropPayloadRef::None) == AttrValue::Flag(false)));
    }

    #[test]
    fn from_nonref_attrvalue() {
        assert_eq!(
            AttrValueRef::from(&AttrValue::Flag(true)),
            AttrValueRef::Flag(true)
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::Size(1)),
            AttrValueRef::Size(1)
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::AlignmentHorizontal(HorizontalAlignment::Center)),
            AttrValueRef::AlignmentHorizontal(HorizontalAlignment::Center)
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::AlignmentVertical(VerticalAlignment::Bottom)),
            AttrValueRef::AlignmentVertical(VerticalAlignment::Bottom)
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::Borders(Borders::default())),
            AttrValueRef::Borders(Borders::default())
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::Color(Color::Black)),
            AttrValueRef::Color(Color::Black)
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::Direction(Direction::Down)),
            AttrValueRef::Direction(Direction::Down)
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::InputType(InputType::Color)),
            AttrValueRef::InputType(&InputType::Color)
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::Layout(Layout::default())),
            AttrValueRef::Layout(&Layout::default())
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::Length(1)),
            AttrValueRef::Length(1)
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::Number(1)),
            AttrValueRef::Number(1)
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::Shape(Shape::Layer)),
            AttrValueRef::Shape(&Shape::Layer)
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::String("hello".to_string())),
            AttrValueRef::String("hello")
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::Style(Style::default())),
            AttrValueRef::Style(Style::default())
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::Table(Table::new())),
            AttrValueRef::Table(&Table::new())
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::TextSpan(SpanStatic::from("hello"))),
            AttrValueRef::TextSpan(&SpanStatic::from("hello"))
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::TextLine(LineStatic::from("hello"))),
            AttrValueRef::TextLine(&LineStatic::from("hello"))
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::Text(TextStatic::from("hello"))),
            AttrValueRef::Text(&TextStatic::from("hello"))
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::TextModifiers(TextModifiers::default())),
            AttrValueRef::TextModifiers(TextModifiers::default())
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::Title(Title::default())),
            AttrValueRef::Title(&Title::default())
        );
        assert_eq!(
            AttrValueRef::from(&AttrValue::Payload(PropPayload::None)),
            AttrValueRef::Payload(PropPayloadRef::None)
        );
    }

    #[test]
    #[should_panic]
    fn unwrapping_alignment_horizontal_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_alignment_horizontal();
    }

    #[test]
    #[should_panic]
    fn unwrapping_alignment_vertical_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_alignment_vertical();
    }

    #[test]
    #[should_panic]
    fn unwrapping_borders_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_borders();
    }

    #[test]
    #[should_panic]
    fn unwrapping_color_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_color();
    }

    #[test]
    #[should_panic]
    fn unwrapping_direction_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_direction();
    }

    #[test]
    #[should_panic]
    fn unwrapping_flag_should_panic_if_not_identity() {
        AttrValueRef::Borders(Borders::default()).unwrap_flag();
    }

    #[test]
    #[should_panic]
    fn unwrapping_inputtype_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_input_type();
    }

    #[test]
    #[should_panic]
    fn unwrapping_layout_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_layout();
    }

    #[test]
    #[should_panic]
    fn unwrapping_length_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_length();
    }

    #[test]
    #[should_panic]
    fn unwrapping_number_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_number();
    }

    #[test]
    #[should_panic]
    fn unwrapping_shape_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_shape();
    }

    #[test]
    #[should_panic]
    fn unwrapping_size_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_size();
    }

    #[test]
    #[should_panic]
    fn unwrapping_string_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_string();
    }

    #[test]
    #[should_panic]
    fn unwrapping_style_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_style();
    }

    #[test]
    #[should_panic]
    fn unwrapping_table_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_table();
    }

    #[test]
    #[should_panic]
    fn unwrapping_text_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_text();
    }

    #[test]
    #[should_panic]
    fn unwrapping_textmodifiers_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_text_modifiers();
    }

    #[test]
    #[should_panic]
    fn unwrapping_title_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_title();
    }

    #[test]
    #[should_panic]
    fn unwrapping_payload_should_panic_if_not_identity() {
        AttrValueRef::Flag(true).unwrap_payload();
    }
}
