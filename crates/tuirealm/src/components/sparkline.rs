//! ## Sparkline
//!
//! A sparkline over more lines

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
use crate::props::{BordersProps, PropPayload, PropValue, Props, PropsBuilder, TextParts};
use crate::tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Sparkline as TuiSparkline},
};
use crate::{Canvas, Component, Event, Msg, Payload};
use std::collections::LinkedList;

// -- Props
const PROP_DATA: &str = "data";
const PROP_MAX_ENTRIES: &str = "max-bars";

pub struct SparklinePropsBuilder {
    props: Option<Props>,
}

impl Default for SparklinePropsBuilder {
    fn default() -> Self {
        Self {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for SparklinePropsBuilder {
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

impl From<Props> for SparklinePropsBuilder {
    fn from(props: Props) -> Self {
        Self { props: Some(props) }
    }
}

impl SparklinePropsBuilder {
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
    /// Set chart title
    pub fn with_title(&mut self, label: String) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.texts = TextParts::new(Some(label), None);
        }
        self
    }

    /// ### with_max_entries
    ///
    /// Define maximum amount of entries to be displayed
    pub fn with_max_entries(&mut self, max: u64) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_MAX_ENTRIES, PropPayload::One(PropValue::U64(max)));
        }
        self
    }

    /// ### with_data
    ///
    /// Define chart data
    pub fn with_data(&mut self, data: &[u64]) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            // Create linked list
            let mut list: LinkedList<PropPayload> = LinkedList::new();
            data.iter()
                .for_each(|value| list.push_back(PropPayload::One(PropValue::U64(*value))));
            props.own.insert(PROP_DATA, PropPayload::Linked(list));
        }
        self
    }

    /// ### push_record_back
    ///
    /// Just pushes a record to the back of the data
    pub fn push_record_back(&mut self, value: u64) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            match props.own.get_mut(PROP_DATA) {
                Some(PropPayload::Linked(list)) => {
                    list.push_back(PropPayload::One(PropValue::U64(value)));
                }
                _ => {
                    // Create list
                    let mut l: LinkedList<PropPayload> = LinkedList::new();
                    l.push_back(PropPayload::One(PropValue::U64(value)));
                    props.own.insert(PROP_DATA, PropPayload::Linked(l));
                }
            }
        }
        self
    }

    /// ### push_record_front
    ///
    /// Just pushes a record to the front of the data
    pub fn push_record_front(&mut self, value: u64) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            match props.own.get_mut(PROP_DATA) {
                Some(PropPayload::Linked(list)) => {
                    list.push_front(PropPayload::One(PropValue::U64(value)));
                }
                _ => {
                    // Create list
                    let mut l: LinkedList<PropPayload> = LinkedList::new();
                    l.push_front(PropPayload::One(PropValue::U64(value)));
                    props.own.insert(PROP_DATA, PropPayload::Linked(l));
                }
            }
        }
        self
    }

    /// ### pop_record_back
    ///
    /// Pop first record on the back of the data list
    pub fn pop_record_back(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            if let Some(PropPayload::Linked(list)) = props.own.get_mut(PROP_DATA) {
                list.pop_back();
            }
        }
        self
    }

    /// ### pop_record_front
    ///
    /// Pop first record on the front of the data list
    pub fn pop_record_front(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            if let Some(PropPayload::Linked(list)) = props.own.get_mut(PROP_DATA) {
                list.pop_front();
            }
        }
        self
    }
}

// -- component

/// ## Sparkline
///
/// A sparkline over more lines
pub struct Sparkline {
    props: Props,
}

impl Sparkline {
    /// ### new
    ///
    /// Instantiates a new `Sparkline`
    pub fn new(props: Props) -> Self {
        Self { props }
    }

    /// ### data_len
    ///
    /// Retrieve current data len from properties
    fn data_len(&self) -> usize {
        self.props
            .own
            .get(PROP_DATA)
            .map(|x| match x {
                PropPayload::Linked(l) => l.len(),
                _ => 0,
            })
            .unwrap_or(0)
    }

    /// ### data
    ///
    /// Get data to be displayed, starting from provided index at `start` with a max length of `len`
    fn data(&self, max: usize) -> Vec<u64> {
        match self.props.own.get(PROP_DATA) {
            Some(PropPayload::Linked(list)) => {
                let mut data: Vec<u64> = Vec::with_capacity(max);
                for item in list.iter() {
                    if let PropPayload::One(PropValue::U64(item)) = item {
                        data.push(*item);
                    }
                    if data.len() >= max {
                        break;
                    }
                }
                data
            }
            _ => Vec::new(),
        }
    }
}

impl Component for Sparkline {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    fn render(&self, render: &mut Canvas, area: Rect) {
        if self.props.visible {
            let block: Block =
                super::utils::get_block(&self.props.borders, &self.props.texts.title, true);
            // Get max elements
            let data_max_len: u64 = self
                .props
                .own
                .get(PROP_MAX_ENTRIES)
                .map(|x| match x {
                    PropPayload::One(PropValue::U64(l)) => *l,
                    _ => panic!("Max-entries is not u64"),
                })
                .unwrap_or(self.data_len() as u64);
            // Get data
            let data: Vec<u64> = self.data(data_max_len as usize);
            // Create widget
            let widget: TuiSparkline = TuiSparkline::default()
                .block(block)
                .data(data.as_slice())
                .max(data_max_len)
                .style(
                    Style::default()
                        .fg(self.props.foreground)
                        .bg(self.props.background),
                );
            // Render
            render.render_widget(widget, area);
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
        Msg::None
    }

    /// ### get_props
    ///
    /// Returns a props builder starting from component properties.
    /// This returns a prop builder in order to make easier to create
    /// new properties for the element.
    fn get_props(&self) -> Props {
        self.props.clone()
    }

    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view
    fn on(&mut self, ev: Event) -> Msg {
        if let Event::Key(key) = ev {
            Msg::OnKey(key)
        } else {
            Msg::None
        }
    }

    /// ### get_state
    ///
    /// Get current state from component
    /// This component always returns `None`
    fn get_state(&self) -> Payload {
        Payload::None
    }

    // -- events

    /// ### blur
    ///
    /// Remove focus
    /// Sparkline doesn't support focus
    fn blur(&mut self) {}

    /// ### active
    ///
    /// Active component; basically give focus
    /// Sparkline doesn't support focus
    fn active(&mut self) {}
}

#[cfg(test)]
mod test {

    use super::*;

    use crossterm::event::{KeyCode, KeyEvent};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_sparkline() {
        let mut component: Sparkline = Sparkline::new(
            SparklinePropsBuilder::default()
                .hidden()
                .visible()
                .with_background(Color::White)
                .with_foreground(Color::Black)
                .with_title(String::from("bandwidth"))
                .with_borders(Borders::ALL, BorderType::Double, Color::Yellow)
                .with_max_entries(8)
                .with_data(&[
                    60, 80, 90, 88, 76, 101, 98, 93, 96, 102, 110, 99, 88, 75, 34, 45, 67, 102,
                ])
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Black);
        assert_eq!(component.props.background, Color::White);
        assert_eq!(component.props.visible, true);
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Double);
        assert_eq!(component.props.borders.color, Color::Yellow);
        assert_eq!(
            *component.props.own.get(PROP_MAX_ENTRIES).unwrap(),
            PropPayload::One(PropValue::U64(8))
        );
        assert!(component.props.own.get(PROP_DATA).is_some());
        // focus
        component.active();
        component.blur();
        // Commands
        assert_eq!(component.get_state(), Payload::None);
        // other keys
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::OnKey(KeyEvent::from(KeyCode::Char('a'))),
        );
        // component funcs
        assert_eq!(component.data_len(), 18);
        assert_eq!(component.data(4), vec![60, 80, 90, 88]);
        // Push
        assert_eq!(
            component.update(
                SparklinePropsBuilder::from(component.get_props())
                    .push_record_back(101)
                    .push_record_front(66)
                    .build()
            ),
            Msg::None
        );
        assert_eq!(
            component.data(100),
            vec![
                66, 60, 80, 90, 88, 76, 101, 98, 93, 96, 102, 110, 99, 88, 75, 34, 45, 67, 102, 101
            ]
        );
        // Pop
        assert_eq!(
            component.update(
                SparklinePropsBuilder::from(component.get_props())
                    .pop_record_back()
                    .pop_record_back()
                    .pop_record_back()
                    .pop_record_front()
                    .pop_record_front()
                    .build()
            ),
            Msg::None
        );
        assert_eq!(
            component.data(100),
            vec![80, 90, 88, 76, 101, 98, 93, 96, 102, 110, 99, 88, 75, 34, 45]
        );
        // Update and test empty data
        assert_eq!(
            component.update(
                SparklinePropsBuilder::from(component.get_props())
                    .with_data(&[])
                    .build()
            ),
            Msg::None
        );
        assert_eq!(component.data(8), vec![]);
    }
}
