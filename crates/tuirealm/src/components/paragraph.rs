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
use crate::props::{Props, PropsBuilder, TextParts, TextSpan};
use crate::{Canvas, Component, Event, Msg, Payload};

use tui::{
    layout::{Corner, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, List, ListItem},
};

// -- Props

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

    /// ### with_spans
    ///
    /// Set spans
    /// You can define a title if you want. The title will be displayed on the upper border of the box
    pub fn with_texts(&mut self, title: Option<String>, spans: Vec<TextSpan>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.texts = TextParts::new(title, Some(spans));
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
    #[cfg(not(tarpaulin_include))]
    fn render(&self, render: &mut Canvas, area: Rect) {
        // Make a Span
        if self.props.visible {
            // Make text items
            let lines: Vec<ListItem> = match self.props.texts.spans.as_ref() {
                None => Vec::new(),
                Some(spans) => super::utils::wrap_spans(spans, area.width as usize, &self.props)
                    .into_iter()
                    .map(|x| ListItem::new(x))
                    .collect(),
            };
            // Make container div
            let div: Block =
                super::utils::get_block(&self.props.borders, &self.props.texts.title, true);
            render.render_widget(
                List::new(lines)
                    .block(div)
                    .start_corner(Corner::TopLeft)
                    .style(
                        Style::default()
                            .fg(self.props.foreground)
                            .bg(self.props.background),
                    ),
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
    /// Returns always None, since cannot have any focus
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

    #[test]
    fn test_components_paragraph() {
        // Make component
        let mut component: Paragraph = Paragraph::new(
            ParagraphPropsBuilder::default()
                .with_texts(
                    Some(String::from("paragraph")),
                    vec![TextSpan::from("welcome to"), TextSpan::from("tui-realm")],
                )
                .build(),
        );
        // Focus
        component.active();
        component.blur();
        // Update
        let props = ParagraphPropsBuilder::from(component.get_props())
            .with_foreground(Color::Red)
            .build();
        assert_eq!(component.update(props), Msg::None);
        assert_eq!(component.props.foreground, Color::Red);
        // Update
        component.update(
            ParagraphPropsBuilder::from(component.get_props())
                .with_texts(
                    Some(String::from("paragraph")),
                    vec![
                        TextSpan::from("welcome"),
                        TextSpan::from("to"),
                        TextSpan::from("tui-realm"),
                    ],
                )
                .build(),
        );
        // get value
        assert_eq!(component.get_state(), Payload::None);
        // On key
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Backspace))),
            Msg::OnKey(KeyEvent::from(KeyCode::Backspace))
        );
    }
}
