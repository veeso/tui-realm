//! ## Span
//!
//! `Span` represents a read-only text component without any container, but with the possibility to define multiple text parts.
//! The main difference with `Label` is that the Span allows different styles inside the same component for the texsts.

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
use tuirealm::props::{Alignment, PropPayload, PropValue, Props, PropsBuilder, TextSpan};
use tuirealm::tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span as TuiSpan, Spans, Text},
    widgets::Paragraph,
};
use tuirealm::{event::Event, CmdResult, Component, Frame, Payload};

const PROP_ALIGNMENT: &str = "text-alignment";
const PROP_SPANS: &str = "spans";

// -- Props

pub struct SpanPropsBuilder {
    props: Option<Props>,
}

impl Default for SpanPropsBuilder {
    fn default() -> Self {
        SpanPropsBuilder {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for SpanPropsBuilder {
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

impl From<Props> for SpanPropsBuilder {
    fn from(props: Props) -> Self {
        SpanPropsBuilder { props: Some(props) }
    }
}

impl SpanPropsBuilder {
    /// ### with_foreground
    ///
    /// Set foreground color for component
    /// Will be used as fallback, if not set for a span
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    /// ### with_background
    ///
    /// Set background color for component
    /// Will be used as fallback, if not set for a span
    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.background = color;
        }
        self
    }

    /// ### bold
    ///
    /// Set bold property for component
    /// Will be used as fallback, if not set for a span
    pub fn bold(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::BOLD;
        }
        self
    }

    /// ### italic
    ///
    /// Set italic property for component
    /// Will be used as fallback, if not set for a span
    pub fn italic(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::ITALIC;
        }
        self
    }

    /// ### underlined
    ///
    /// Set underlined property for component
    /// Will be used as fallback, if not set for a span
    pub fn underlined(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::UNDERLINED;
        }
        self
    }

    /// ### slow_blink
    ///
    /// Set slow_blink property for component
    /// Will be used as fallback, if not set for a span
    pub fn slow_blink(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::SLOW_BLINK;
        }
        self
    }

    /// ### rapid_blink
    ///
    /// Set rapid_blink property for component
    /// Will be used as fallback, if not set for a span
    pub fn rapid_blink(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::RAPID_BLINK;
        }
        self
    }

    /// ### reversed
    ///
    /// Set reversed property for component
    /// Will be used as fallback, if not set for a span
    pub fn reversed(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::REVERSED;
        }
        self
    }

    /// ### strikethrough
    ///
    /// Set strikethrough property for component
    /// Will be used as fallback, if not set for a span
    pub fn strikethrough(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.modifiers |= Modifier::CROSSED_OUT;
        }
        self
    }

    /// ### with_spans
    ///
    /// Set spans
    pub fn with_spans(&mut self, spans: Vec<TextSpan>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_SPANS,
                PropPayload::Vec(spans.into_iter().map(PropValue::TextSpan).collect()),
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
}

// -- Component

/// ## Span
///
/// represents a read-only text component without any container, but with multy-style text parts
pub struct Span {
    props: Props,
}

impl Span {
    /// ### new
    ///
    /// Instantiates a new `Span` component.
    pub fn new(props: Props) -> Self {
        Span { props }
    }
}

impl MockComponent for Span {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    fn render(&self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Make text
            let spans: Vec<TuiSpan> = match self.props.own.get(PROP_SPANS).as_ref() {
                Some(PropPayload::Vec(spans)) => spans
                    .iter()
                    .map(|x| x.unwrap_text_span())
                    .map(|x| {
                        // Keep colors and modifiers, or use default
                        let (fg, bg, modifiers) =
                            crate::utils::use_or_default_styles(&self.props, x);
                        TuiSpan::styled(
                            x.content.clone(),
                            Style::default().add_modifier(modifiers).fg(fg).bg(bg),
                        )
                    })
                    .collect(),
                _ => Vec::new(),
            };
            let text: Text = Text::from(Spans::from(spans));
            // Text properties
            let alignment: Alignment = match self.props.own.get(PROP_ALIGNMENT) {
                Some(PropPayload::One(PropValue::Alignment(alignment))) => *alignment,
                _ => Alignment::Left,
            };
            render.render_widget(Paragraph::new(text).alignment(alignment), area);
        }
    }

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update.
    /// Returns a CmdResult to the view
    fn update(&mut self, props: Props) -> CmdResult {
        self.props = props;
        // Return None
        CmdResult::None
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
    /// Returns a CmdResult to the view.
    fn on(&mut self, ev: Event) -> CmdResult {
        // Return key
        if let Cmd::Key(key) = ev {
            Cmd::None(key)
        } else {
            CmdResult::None
        }
    }

    /// ### get_state
    ///
    /// Get current state from component
    /// For this component returns always None
    fn get_state(&self) -> Payload {
        State::None
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
    use tuirealm::tui::style::Color;

    #[test]
    fn test_components_span() {
        let mut component: Span = Span::new(
            SpanPropsBuilder::default()
                .with_background(Color::Blue)
                .with_foreground(Color::Red)
                .hidden()
                .visible()
                .bold()
                .italic()
                .rapid_blink()
                .reversed()
                .slow_blink()
                .strikethrough()
                .underlined()
                .with_text_alignment(Alignment::Center)
                .with_spans(vec![
                    TextSpan::from("Press "),
                    TextSpan::from("<ESC>").fg(Color::Cyan).bold(),
                    TextSpan::from(" to quit"),
                ])
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
        assert_eq!(
            component.props.own.get(PROP_SPANS).unwrap(),
            &PropPayload::Vec(vec![
                PropValue::TextSpan(TextSpan::from("Press ")),
                PropValue::TextSpan(TextSpan::from("<ESC>").fg(Color::Cyan).bold()),
                PropValue::TextSpan(TextSpan::from(" to quit")),
            ])
        );
        assert_eq!(
            *component.props.own.get(PROP_ALIGNMENT).unwrap(),
            PropPayload::One(PropValue::Alignment(Alignment::Center))
        );
        component.active();
        component.blur();
        // Update
        let props = SpanPropsBuilder::from(component.get_props())
            .with_foreground(Color::Red)
            .hidden()
            .build();
        assert_eq!(component.update(props), CmdResult::None);
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.visible, false);
        // Get value
        assert_eq!(component.get_state(), State::None);
        // Event
        assert_eq!(
            component.on(Cmd::Key(KeyCmd::from(KeyCode::Delete))),
            Cmd::None(KeyCmd::from(KeyCode::Delete))
        );
        assert_eq!(component.on(Cmd::Resize(0, 0)), CmdResult::None);
    }

    #[test]
    fn test_components_span_propsbuilder() {
        let props: Props = SpanPropsBuilder::default()
            .hidden()
            .with_background(Color::Blue)
            .with_foreground(Color::Green)
            .bold()
            .italic()
            .underlined()
            .strikethrough()
            .reversed()
            .rapid_blink()
            .slow_blink()
            .with_spans(vec![TextSpan::from("test")])
            .build();
        assert_eq!(props.background, Color::Blue);
        assert!(props.modifiers.intersects(Modifier::BOLD));
        assert!(props.modifiers.intersects(Modifier::ITALIC));
        assert!(props.modifiers.intersects(Modifier::UNDERLINED));
        assert!(props.modifiers.intersects(Modifier::SLOW_BLINK));
        assert!(props.modifiers.intersects(Modifier::RAPID_BLINK));
        assert!(props.modifiers.intersects(Modifier::REVERSED));
        assert!(props.modifiers.intersects(Modifier::CROSSED_OUT));
        assert_eq!(props.foreground, Color::Green);
        assert_eq!(
            props.own.get(PROP_SPANS).unwrap(),
            &PropPayload::Vec(vec![PropValue::TextSpan(TextSpan::from("test")),])
        );
    }
}
