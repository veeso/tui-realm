//! ## Paragraph
//!
//! `Paragraph` represents a read-only text component inside a container, the text is wrapped inside the container automatically
//! using the [textwrap](https://docs.rs/textwrap/0.13.4/textwrap/) crate.
//! The textarea supports multi-style spans.
//! The component is not scrollable and doesn't handle any input. The text must then fit into the area.
//! If you want scroll support, use a `Textarea` instead.

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
use crate::props::{
    Alignment, BordersProps, PropPayload, PropValue, Props, PropsBuilder, TextSpan,
};
use crate::tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph as TuiParagraph, Wrap},
};
use crate::{Component, Event, Frame, Msg, Payload};

// -- Props

const PROP_SPANS: &str = "spans";
const PROP_ALIGNMENT: &str = "text-align";
const PROP_TITLE: &str = "title";
const PROP_TRIM: &str = "wrap";

pub struct ParagraphPropsBuilder {
    props: Option<Props>,
}

impl Default for ParagraphPropsBuilder {
    fn default() -> Self {
        ParagraphPropsBuilder {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for ParagraphPropsBuilder {
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

impl From<Props> for ParagraphPropsBuilder {
    fn from(props: Props) -> Self {
        ParagraphPropsBuilder { props: Some(props) }
    }
}

impl ParagraphPropsBuilder {
    /// ### with_foreground
    ///
    /// Set foreground color for area
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    /// ### with_background
    ///
    /// Set background color for area
    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.background = color;
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
            props.borders = BordersProps {
                borders,
                variant,
                color,
            }
        }
        self
    }

    /// ### with_title
    ///
    /// Set title
    pub fn with_title<S: AsRef<str>>(&mut self, title: S) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_TITLE,
                PropPayload::One(PropValue::Str(title.as_ref().to_string())),
            );
        }
        self
    }

    /// ### with_texts
    ///
    /// Set spans
    pub fn with_texts(&mut self, spans: Vec<TextSpan>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_SPANS,
                PropPayload::Vec(spans.into_iter().map(|x| PropValue::TextSpan(x)).collect()),
            );
        }
        self
    }

    /// ### with_text_alignment
    ///
    /// Set text alignment for paragraph
    pub fn with_text_alignment(&mut self, alignment: Alignment) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_ALIGNMENT,
                PropPayload::One(PropValue::Alignment(alignment)),
            );
        }
        self
    }

    /// ### with_trim
    ///
    /// Set whether wrapped text should be trimmed or not on newlines
    pub fn with_trim(&mut self, trim: bool) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_TRIM, PropPayload::One(PropValue::Bool(trim)));
        }
        self
    }
}

// -- Component

/// ## Paragraph
///
/// represents a read-only text component without any container.
pub struct Paragraph {
    props: Props,
}

impl Paragraph {
    /// ### new
    ///
    /// Instantiates a new `Paragraph` component.
    pub fn new(props: Props) -> Self {
        Paragraph { props }
    }
}

impl Component for Paragraph {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    fn render(&self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.visible {
            // Make text items
            let text: Vec<Spans> = match self.props.own.get(PROP_SPANS).as_ref() {
                Some(PropPayload::Vec(spans)) => spans
                    .iter()
                    .map(|x| match x {
                        PropValue::TextSpan(x) => {
                            let (fg, bg, modifiers) =
                                super::utils::use_or_default_styles(&self.props, x);
                            Spans::from(vec![Span::styled(
                                x.content.clone(),
                                Style::default().add_modifier(modifiers).fg(fg).bg(bg),
                            )])
                        }
                        _ => panic!("Spans doesn't contain TextSpan"),
                    })
                    .collect(),
                _ => Vec::new(),
            };
            // Make container div
            let title: Option<&str> = match self.props.own.get(PROP_TITLE).as_ref() {
                Some(PropPayload::One(PropValue::Str(t))) => Some(t),
                _ => None,
            };
            let div: Block = super::utils::get_block(&self.props.borders, title, true);
            // Text properties
            let alignment: Alignment = match self.props.own.get(PROP_ALIGNMENT) {
                Some(PropPayload::One(PropValue::Alignment(alignment))) => *alignment,
                _ => Alignment::Left,
            };
            // Wrap
            let trim: bool = match self.props.own.get(PROP_TRIM) {
                Some(PropPayload::One(PropValue::Bool(trim))) => *trim,
                _ => false,
            };
            render.render_widget(
                TuiParagraph::new(text)
                    .block(div)
                    .style(
                        Style::default()
                            .fg(self.props.foreground)
                            .bg(self.props.background),
                    )
                    .alignment(alignment)
                    .wrap(Wrap { trim }),
                area,
            );
        }
    }

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update.
    /// Returns a Msg to the view
    fn update(&mut self, props: Props) -> Msg {
        self.props = props;
        // Return None
        Msg::None
    }

    /// ### get_props
    ///
    /// Returns a copy of the component properties.
    fn get_props(&self) -> Props {
        self.props.clone()
    }

    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view.
    fn on(&mut self, ev: Event) -> Msg {
        // Return key
        if let Event::Key(key) = ev {
            Msg::OnKey(key)
        } else {
            Msg::None
        }
    }

    /// ### get_state
    ///
    /// Get current state from component
    /// For this component returns always None
    fn get_state(&self) -> Payload {
        Payload::None
    }

    // -- events

    /// ### blur
    ///
    /// Blur component
    fn blur(&mut self) {}

    /// ### active
    ///
    /// Active component
    fn active(&mut self) {}
}

#[cfg(test)]
mod tests {

    use super::*;

    use crossterm::event::{KeyCode, KeyEvent};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_paragraph() {
        // Make component
        let mut component: Paragraph = Paragraph::new(
            ParagraphPropsBuilder::default()
                .with_foreground(Color::Red)
                .with_background(Color::Blue)
                .hidden()
                .visible()
                .bold()
                .italic()
                .rapid_blink()
                .reversed()
                .slow_blink()
                .strikethrough()
                .underlined()
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .with_texts(vec![
                    TextSpan::from("welcome to "),
                    TextSpan::from("tui-realm"),
                ])
                .with_title("paragraph")
                .with_trim(true)
                .with_text_alignment(Alignment::Center)
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.background, Color::Blue);
        assert_eq!(component.props.visible, true);
        assert!(component.props.modifiers.intersects(Modifier::BOLD));
        assert!(component.props.modifiers.intersects(Modifier::ITALIC));
        assert!(component.props.modifiers.intersects(Modifier::UNDERLINED));
        assert!(component.props.modifiers.intersects(Modifier::SLOW_BLINK));
        assert!(component.props.modifiers.intersects(Modifier::RAPID_BLINK));
        assert!(component.props.modifiers.intersects(Modifier::REVERSED));
        assert!(component.props.modifiers.intersects(Modifier::CROSSED_OUT));
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Double);
        assert_eq!(component.props.borders.color, Color::Red);
        assert_eq!(
            component.props.own.get(PROP_SPANS).unwrap(),
            &PropPayload::Vec(vec![
                PropValue::TextSpan(TextSpan::from("welcome to ")),
                PropValue::TextSpan(TextSpan::from("tui-realm")),
            ])
        );
        assert_eq!(
            component.props.own.get(PROP_TITLE).unwrap(),
            &PropPayload::One(PropValue::Str("paragraph".to_string()))
        );
        assert_eq!(
            *component.props.own.get(PROP_ALIGNMENT).unwrap(),
            PropPayload::One(PropValue::Alignment(Alignment::Center))
        );
        assert_eq!(
            *component.props.own.get(PROP_TRIM).unwrap(),
            PropPayload::One(PropValue::Bool(true))
        );
        // Focus
        component.active();
        component.blur();
        // Update
        let props = ParagraphPropsBuilder::from(component.get_props())
            .with_foreground(Color::Green)
            .hidden()
            .build();
        assert_eq!(component.update(props), Msg::None);
        assert_eq!(component.props.visible, false);
        assert_eq!(component.props.foreground, Color::Green);
        // Update
        component.update(
            ParagraphPropsBuilder::from(component.get_props())
                .with_texts(vec![
                    TextSpan::from("welcome"),
                    TextSpan::from("to"),
                    TextSpan::from("tui-realm"),
                ])
                .build(),
        );
        // get value
        assert_eq!(component.get_state(), Payload::None);
        // On key
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Backspace))),
            Msg::OnKey(KeyEvent::from(KeyCode::Backspace))
        );
        assert_eq!(component.on(Event::Resize(0, 0)), Msg::None);
    }
}
