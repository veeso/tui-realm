//! ## Textarea
//!
//! `Textarea` represents a read-only text component inside a container, the text is wrapped inside the container automatically
//! using the [textwrap](https://docs.rs/textwrap/0.13.4/textwrap/) crate.
//! The textarea supports multi-style spans and it is scrollable with arrows.

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

use crossterm::event::KeyCode;
use tui::{
    layout::{Corner, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, List, ListItem, ListState},
};

// -- Props

pub struct TextareaPropsBuilder {
    props: Option<Props>,
}

impl Default for TextareaPropsBuilder {
    fn default() -> Self {
        TextareaPropsBuilder {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for TextareaPropsBuilder {
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

impl From<Props> for TextareaPropsBuilder {
    fn from(props: Props) -> Self {
        TextareaPropsBuilder { props: Some(props) }
    }
}

impl TextareaPropsBuilder {
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

// -- States

struct OwnStates {
    list_index: usize, // Index of selected item in textarea
    list_len: usize,   // Lines in text area
    focus: bool,       // Has focus?
}

impl OwnStates {
    /// ### set_list_len
    ///
    /// Set list length
    pub fn set_list_len(&mut self, len: usize) {
        self.list_len = len;
    }

    /// ### incr_list_index
    ///
    /// Incremenet list index
    pub fn incr_list_index(&mut self) {
        // Check if index is at last element
        if self.list_index + 1 < self.list_len {
            self.list_index += 1;
        }
    }

    /// ### decr_list_index
    ///
    /// Decrement list index
    pub fn decr_list_index(&mut self) {
        // Check if index is bigger than 0
        if self.list_index > 0 {
            self.list_index -= 1;
        }
    }

    /// ### fix_list_index
    ///
    /// Keep index if possible, otherwise set to lenght - 1
    pub fn fix_list_index(&mut self) {
        if self.list_index >= self.list_len && self.list_len > 0 {
            self.list_index = self.list_len - 1;
        } else if self.list_len == 0 {
            self.list_index = 0;
        }
    }
}

// -- Component

/// ## Textarea
///
/// represents a read-only text component without any container.
pub struct Textarea {
    props: Props,
    states: OwnStates,
}

impl Textarea {
    /// ### new
    ///
    /// Instantiates a new `Textarea` component.
    pub fn new(props: Props) -> Self {
        let len: usize = match props.texts.spans.as_ref() {
            Some(s) => s.len(),
            None => 0,
        };
        Textarea {
            props,
            states: OwnStates {
                list_index: 0,
                list_len: len,
                focus: false,
            },
        }
    }
}

impl Component for Textarea {
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
            let div: Block = Block::default()
                .borders(self.props.borders.borders)
                .border_style(match self.states.focus {
                    true => self.props.borders.style(),
                    false => Style::default(),
                })
                .border_type(self.props.borders.variant);
            // Set title
            let div: Block = match self.props.texts.title.as_ref() {
                Some(t) => div.title(t.to_string()),
                None => div,
            };
            let mut state: ListState = ListState::default();
            state.select(Some(self.states.list_index));
            render.render_stateful_widget(
                List::new(lines)
                    .block(div)
                    .start_corner(Corner::TopLeft)
                    .style(
                        Style::default()
                            .fg(self.props.foreground)
                            .bg(self.props.background),
                    ),
                area,
                &mut state,
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
        // re-Set list length
        self.states.set_list_len(match &self.props.texts.spans {
            Some(tokens) => tokens.len(),
            None => 0,
        });
        // Fix list index
        self.states.fix_list_index();
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
            match key.code {
                KeyCode::Down => {
                    // Update states
                    self.states.incr_list_index();
                    Msg::None
                }
                KeyCode::Up => {
                    // Update states
                    self.states.decr_list_index();
                    Msg::None
                }
                KeyCode::PageDown => {
                    // Update states
                    for _ in 0..8 {
                        self.states.incr_list_index();
                    }
                    Msg::None
                }
                KeyCode::PageUp => {
                    // Update states
                    for _ in 0..8 {
                        self.states.decr_list_index();
                    }
                    Msg::None
                }
                _ => Msg::OnKey(key),
            }
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
    fn blur(&mut self) {
        self.states.focus = false;
    }

    /// ### active
    ///
    /// Active component
    fn active(&mut self) {
        self.states.focus = true;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crossterm::event::KeyEvent;

    #[test]
    fn test_components_textarea() {
        // Make component
        let mut component: Textarea = Textarea::new(
            TextareaPropsBuilder::default()
                .with_texts(
                    Some(String::from("textarea")),
                    vec![TextSpan::from("welcome to"), TextSpan::from("tui-realm")],
                )
                .build(),
        );
        // Verify states
        assert_eq!(component.states.list_index, 0);
        assert_eq!(component.states.list_len, 2);
        assert_eq!(component.states.focus, false);
        // Focus
        component.active();
        assert_eq!(component.states.focus, true);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Update
        let props = TextareaPropsBuilder::from(component.get_props())
            .with_foreground(Color::Red)
            .build();
        assert_eq!(component.update(props), Msg::None);
        assert_eq!(component.props.foreground, Color::Red);
        // Increment list index
        component.states.list_index += 1;
        assert_eq!(component.states.list_index, 1);
        // Update
        component.update(
            TextareaPropsBuilder::from(component.get_props())
                .with_texts(
                    Some(String::from("textarea")),
                    vec![
                        TextSpan::from("welcome"),
                        TextSpan::from("to"),
                        TextSpan::from("tui-realm"),
                    ],
                )
                .build(),
        );
        // Verify states
        assert_eq!(component.states.list_index, 1); // Kept
        assert_eq!(component.states.list_len, 3);
        // get value
        assert_eq!(component.get_state(), Payload::None);
        // Render
        assert_eq!(component.states.list_index, 1);
        // Handle inputs
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Down))),
            Msg::None
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 2);
        // Index should be decremented
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Up))),
            Msg::None
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 1);
        // Index should be 2
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::PageDown))),
            Msg::None
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 2);
        // Index should be 0
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::PageUp))),
            Msg::None
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 0);
        // On key
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Backspace))),
            Msg::OnKey(KeyEvent::from(KeyCode::Backspace))
        );
    }
}
