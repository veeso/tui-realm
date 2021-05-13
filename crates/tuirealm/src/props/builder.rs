//! ## Builder
//!
//! `Builder` is the module which defines the prop builder trait.
//! In addition provides a Generic Props builder which exports all the possible properties in
//! the builder.

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
use super::borders::{BorderType, Borders};
use super::{InputType, PropPayload, Props, TextParts};

use tui::style::{Color, Modifier};

// -- Props builder

/// ## PropsBuilder
///
/// The PropsBuilder trait just defines the method build, which all the builders must implement.
/// This method must return the Props hold by the ProspBuilder.
/// If you're looking on how to implement a Props Builder, check out the `GenericPropsBuilder`.
pub trait PropsBuilder {
    /// ### build
    ///
    /// Build Props from builder
    /// You shouldn't allow this method to be called twice.
    /// Panic is ok.
    fn build(&mut self) -> Props;

    /// ### hidden
    ///
    /// Initialize props with visible set to False
    fn hidden(&mut self) -> &mut Self;

    /// ### visible
    ///
    /// Initialize props with visible set to True
    fn visible(&mut self) -> &mut Self;
}

/// ## GenericPropsBuilder
///
/// This props builder exports methods to set values for all the possible properties.
/// In a normal case you shouldn't use this builder, unless you can actually customize everything of your component.
/// For a Builder you should always implement only three traits: `Default`, `From<Props>` and `PropsBuilder`, then you should implement
/// the setter methods for it, for the only properties you need for the associated component.
pub struct GenericPropsBuilder {
    props: Option<Props>,
}

impl Default for GenericPropsBuilder {
    fn default() -> Self {
        GenericPropsBuilder {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for GenericPropsBuilder {
    fn build(&mut self) -> Props {
        self.props.take().unwrap()
    }

    fn hidden(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = false;
        }
        self
    }

    fn visible(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = true;
        }
        self
    }
}

impl From<Props> for GenericPropsBuilder {
    fn from(props: Props) -> Self {
        GenericPropsBuilder { props: Some(props) }
    }
}

impl GenericPropsBuilder {
    /// ### with_foreground
    ///
    /// Set foreground color for component
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    /// ### with_background
    ///
    /// Set background color for component
    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.background = color;
        }
        self
    }

    /// ### with_borders
    ///
    /// Set component borders style
    pub fn with_borders(
        &mut self,
        borders: Borders,
        variant: BorderType,
        color: Color,
    ) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.borders.borders = borders;
            props.borders.variant = variant;
            props.borders.color = color;
        }
        self
    }

    /// ### bold
    ///
    /// Set bold property for component
    pub fn bold(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::BOLD;
        }
        self
    }

    /// ### italic
    ///
    /// Set italic property for component
    pub fn italic(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::ITALIC;
        }
        self
    }

    /// ### underlined
    ///
    /// Set underlined property for component
    pub fn underlined(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::UNDERLINED;
        }
        self
    }

    /// ### slow_blink
    ///
    /// Set slow_blink property for component
    pub fn slow_blink(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::SLOW_BLINK;
        }
        self
    }

    /// ### rapid_blink
    ///
    /// Set rapid_blink property for component
    pub fn rapid_blink(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::RAPID_BLINK;
        }
        self
    }

    /// ### reversed
    ///
    /// Set reversed property for component
    pub fn reversed(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::REVERSED;
        }
        self
    }

    /// ### strikethrough
    ///
    /// Set strikethrough property for component
    pub fn strikethrough(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::CROSSED_OUT;
        }
        self
    }

    /// ### with_texts
    ///
    /// Set texts for component
    pub fn with_texts(&mut self, texts: TextParts) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.texts = texts;
        }
        self
    }

    /// ### with_input
    ///
    /// Set input type for component
    pub fn with_input(&mut self, input_type: InputType) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.input_type = input_type;
        }
        self
    }

    /// ### with_input_len
    ///
    /// Set max input len
    pub fn with_input_len(&mut self, len: usize) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.input_len = Some(len);
        }
        self
    }

    /// ### with_custom_color
    ///
    /// Set a custom color inside the color palette
    pub fn with_custom_color(&mut self, name: &'static str, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.palette.insert(name, color);
        }
        self
    }

    /// ### with_value
    ///
    /// Set initial value for component
    pub fn with_value(&mut self, value: PropPayload) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.value = value;
        }
        self
    }
}

#[cfg(test)]
mod test {

    use super::super::PropValue;
    use super::super::TextSpan;
    use super::*;

    #[test]
    fn test_props_builder() {
        let props: Props = GenericPropsBuilder::default()
            .hidden()
            .with_background(Color::Blue)
            .with_foreground(Color::Green)
            .with_borders(Borders::BOTTOM, BorderType::Plain, Color::White)
            .bold()
            .italic()
            .underlined()
            .strikethrough()
            .reversed()
            .rapid_blink()
            .slow_blink()
            .with_custom_color("arrows", Color::Red)
            .with_texts(TextParts::new(
                Some(String::from("hello")),
                Some(vec![TextSpan::from("hey")]),
            ))
            .with_input(InputType::Password)
            .with_input_len(16)
            .with_value(PropPayload::One(PropValue::Str(String::from("Hello"))))
            .build();
        assert_eq!(props.background, Color::Blue);
        assert_eq!(props.borders.borders, Borders::BOTTOM);
        assert_eq!(props.borders.color, Color::White);
        assert_eq!(props.borders.variant, BorderType::Plain);
        assert!(props.modifiers.intersects(Modifier::BOLD));
        assert!(props.modifiers.intersects(Modifier::ITALIC));
        assert!(props.modifiers.intersects(Modifier::UNDERLINED));
        assert!(props.modifiers.intersects(Modifier::SLOW_BLINK));
        assert!(props.modifiers.intersects(Modifier::RAPID_BLINK));
        assert!(props.modifiers.intersects(Modifier::REVERSED));
        assert!(props.modifiers.intersects(Modifier::CROSSED_OUT));
        assert_eq!(props.foreground, Color::Green);
        assert_eq!(*props.palette.get("arrows").unwrap(), Color::Red);
        assert!(props.palette.get("omar").is_none());
        assert_eq!(props.texts.title.as_ref().unwrap().as_str(), "hello");
        assert_eq!(props.input_type, InputType::Password);
        assert_eq!(*props.input_len.as_ref().unwrap(), 16);
        if let PropPayload::One(PropValue::Str(s)) = props.value {
            assert_eq!(s.as_str(), "Hello");
        } else {
            panic!("Expected value to be a string");
        }
        assert_eq!(
            props
                .texts
                .spans
                .as_ref()
                .unwrap()
                .get(0)
                .unwrap()
                .content
                .as_str(),
            "hey"
        );
        assert_eq!(props.visible, false);
        let props: Props = GenericPropsBuilder::default()
            .visible()
            .with_background(Color::Blue)
            .with_foreground(Color::Green)
            .bold()
            .italic()
            .underlined()
            .with_texts(TextParts::new(
                Some(String::from("hello")),
                Some(vec![TextSpan::from("hey")]),
            ))
            .build();
        assert_eq!(props.background, Color::Blue);
        assert!(props.modifiers.intersects(Modifier::BOLD));
        assert_eq!(props.foreground, Color::Green);
        assert!(props.modifiers.intersects(Modifier::ITALIC));
        assert_eq!(props.texts.title.as_ref().unwrap().as_str(), "hello");
        assert_eq!(
            props
                .texts
                .spans
                .as_ref()
                .unwrap()
                .get(0)
                .unwrap()
                .content
                .as_str(),
            "hey"
        );
        assert!(props.modifiers.intersects(Modifier::UNDERLINED));
        assert_eq!(props.visible, true);
    }

    #[test]
    #[should_panic]
    fn test_props_build_twice() {
        let mut builder: GenericPropsBuilder = GenericPropsBuilder::default();
        let _ = builder.build();
        builder
            .hidden()
            .with_background(Color::Blue)
            .with_foreground(Color::Green)
            .bold()
            .italic()
            .underlined()
            .with_texts(TextParts::new(
                Some(String::from("hello")),
                Some(vec![TextSpan::from("hey")]),
            ));
        // Rebuild
        let _ = builder.build();
    }

    #[test]
    fn test_props_builder_from_props() {
        let props: Props = GenericPropsBuilder::default()
            .hidden()
            .with_background(Color::Blue)
            .with_foreground(Color::Green)
            .bold()
            .italic()
            .underlined()
            .with_texts(TextParts::new(
                Some(String::from("hello")),
                Some(vec![TextSpan::from("hey")]),
            ))
            .build();
        // Ok, now make a builder from properties
        let builder: GenericPropsBuilder = GenericPropsBuilder::from(props);
        assert!(builder.props.is_some());
    }
}
