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
use crate::event::KeyCode;
use crate::props::{BordersProps, PropPayload, PropValue, Props, PropsBuilder, TextSpan};
use crate::tui::{
    layout::{Corner, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
};
use crate::{Component, Event, Frame, Msg, Payload};

// -- Props

const PROP_HIGHLIGHTED_TXT: &str = "highlighted-txt";
const PROP_MAX_STEP: &str = "max-step";
const PROP_SPANS: &str = "spans";
const PROP_TITLE: &str = "title";

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

    /// ### with_highlighted_str
    ///
    /// Display a symbol to highlighted line in scroll table
    pub fn with_highlighted_str(&mut self, s: Option<&str>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            match s {
                None => {
                    props.own.remove(PROP_HIGHLIGHTED_TXT);
                }
                Some(s) => {
                    props.own.insert(
                        PROP_HIGHLIGHTED_TXT,
                        PropPayload::One(PropValue::Str(s.to_string())),
                    );
                }
            }
        }
        self
    }

    /// ### with_max_scroll_step
    ///
    /// Defines the max step for PAGEUP/PAGEDOWN keys
    pub fn with_max_scroll_step(&mut self, step: usize) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_MAX_STEP, PropPayload::One(PropValue::Usize(step)));
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

    /// ### list_index_at_first
    ///
    /// Set list index to the first item in the list
    pub fn list_index_at_first(&mut self) {
        self.list_index = 0;
    }

    /// ### list_index_at_last
    ///
    /// Set list index at the last item of the list
    pub fn list_index_at_last(&mut self) {
        if self.list_len > 0 {
            self.list_index = self.list_len - 1;
        } else {
            self.list_index = 0;
        }
    }

    /// ### calc_max_step_ahead
    ///
    /// Calculate the max step ahead to scroll list
    pub fn calc_max_step_ahead(&self, max: usize) -> usize {
        let remaining: usize = match self.list_len {
            0 => 0,
            len => len - 1 - self.list_index,
        };
        if remaining > max {
            max
        } else {
            remaining
        }
    }

    /// ### calc_max_step_ahead
    ///
    /// Calculate the max step ahead to scroll list
    pub fn calc_max_step_behind(&self, max: usize) -> usize {
        if self.list_index > max {
            max
        } else {
            self.list_index
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
        let len: usize = match props.own.get(PROP_SPANS).as_ref() {
            Some(PropPayload::Vec(s)) => s.len(),
            _ => 0,
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
    fn render(&self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.visible {
            // Make text items
            let lines: Vec<ListItem> = match self.props.own.get(PROP_SPANS).as_ref() {
                Some(PropPayload::Vec(spans)) => spans
                    .iter()
                    .map(|x| match x {
                        PropValue::TextSpan(x) => super::utils::wrap_spans(
                            vec![x.clone()].as_slice(),
                            area.width as usize,
                            &self.props,
                        ),
                        _ => panic!("Spans doesn't contain TextSpan"),
                    })
                    .map(ListItem::new)
                    .collect(),
                _ => Vec::new(),
            };
            let title: Option<&str> = match self.props.own.get(PROP_TITLE).as_ref() {
                Some(PropPayload::One(PropValue::Str(t))) => Some(t),
                _ => None,
            };
            let div: Block = super::utils::get_block(&self.props.borders, title, self.states.focus);
            let mut state: ListState = ListState::default();
            state.select(Some(self.states.list_index));
            // Make component
            let mut list = List::new(lines)
                .block(div)
                .start_corner(Corner::TopLeft)
                .style(
                    Style::default()
                        .fg(self.props.foreground)
                        .bg(self.props.background),
                );
            // Highlighted symbol
            if let Some(PropPayload::One(PropValue::Str(highlight))) =
                self.props.own.get(PROP_HIGHLIGHTED_TXT)
            {
                list = list.highlight_symbol(highlight);
            }
            render.render_stateful_widget(list, area, &mut state);
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
        self.states
            .set_list_len(match self.props.own.get(PROP_SPANS).as_ref() {
                Some(PropPayload::Vec(s)) => s.len(),
                _ => 0,
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
    fn on(&mut self, ev: Event) -> Msg {
        // Return key
        if let Event::Key(key) = ev {
            match key.code {
                KeyCode::Down => {
                    // Update states
                    self.states.incr_list_index();
                    Msg::OnKey(key)
                }
                KeyCode::Up => {
                    // Update states
                    self.states.decr_list_index();
                    Msg::OnKey(key)
                }
                KeyCode::PageDown => {
                    // Scroll by step
                    let step: usize =
                        self.states
                            .calc_max_step_ahead(match self.props.own.get(PROP_MAX_STEP) {
                                Some(PropPayload::One(PropValue::Usize(step))) => *step,
                                _ => 8,
                            });
                    (0..step).for_each(|_| self.states.incr_list_index());
                    Msg::OnKey(key)
                }
                KeyCode::PageUp => {
                    // Update states
                    let step: usize =
                        self.states
                            .calc_max_step_behind(match self.props.own.get(PROP_MAX_STEP) {
                                Some(PropPayload::One(PropValue::Usize(step))) => *step,
                                _ => 8,
                            });
                    (0..step).for_each(|_| self.states.decr_list_index());
                    Msg::OnKey(key)
                }
                KeyCode::End => {
                    self.states.list_index_at_last();
                    Msg::OnKey(key)
                }
                KeyCode::Home => {
                    self.states.list_index_at_first();
                    Msg::OnKey(key)
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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_textarea() {
        // Make component
        let mut component: Textarea = Textarea::new(
            TextareaPropsBuilder::default()
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
                .with_highlighted_str(Some("🚀"))
                .with_max_scroll_step(4)
                .with_title("textarea")
                .with_texts(vec![
                    TextSpan::from("welcome to "),
                    TextSpan::from("tui-realm"),
                ])
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
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
            &PropPayload::One(PropValue::Str("textarea".to_string()))
        );
        assert_eq!(
            component.props.own.get(PROP_HIGHLIGHTED_TXT).unwrap(),
            &PropPayload::One(PropValue::Str(String::from("🚀")))
        );
        assert_eq!(
            component.props.own.get(PROP_MAX_STEP).unwrap(),
            &PropPayload::One(PropValue::Usize(4))
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
                .hidden()
                .with_texts(vec![
                    TextSpan::from("welcome "),
                    TextSpan::from("to "),
                    TextSpan::from("tui-realm"),
                ])
                .build(),
        );
        assert_eq!(
            component.props.own.get(PROP_SPANS).unwrap(),
            &PropPayload::Vec(vec![
                PropValue::TextSpan(TextSpan::from("welcome ")),
                PropValue::TextSpan(TextSpan::from("to ")),
                PropValue::TextSpan(TextSpan::from("tui-realm")),
            ])
        );
        assert_eq!(component.props.visible, false);
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
            Msg::OnKey(KeyEvent::from(KeyCode::Down))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 2);
        // Index should be decremented
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Up))),
            Msg::OnKey(KeyEvent::from(KeyCode::Up))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 1);
        // Index should be 2
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::PageDown))),
            Msg::OnKey(KeyEvent::from(KeyCode::PageDown))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 2);
        // Index should be 0
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::PageUp))),
            Msg::OnKey(KeyEvent::from(KeyCode::PageUp))
        );
        // End
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::End))),
            Msg::OnKey(KeyEvent::from(KeyCode::End))
        );
        assert_eq!(component.states.list_index, 2);
        // Home
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Home))),
            Msg::OnKey(KeyEvent::from(KeyCode::Home))
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 0);
        // On key
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Backspace))),
            Msg::OnKey(KeyEvent::from(KeyCode::Backspace))
        );
        assert_eq!(component.on(Event::Resize(0, 0)), Msg::None);
    }
}
