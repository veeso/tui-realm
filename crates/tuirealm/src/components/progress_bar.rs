//! ## ProgressBar
//!
//! `ProgressBar` provides a component which shows the progress. It is possible to set the style for the progress bar and the text shown above it.

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
use crate::props::{BordersProps, PropPayload, PropValue, Props, PropsBuilder};
use crate::tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Gauge},
};
use crate::{Component, Event, Frame, Msg, Payload};

// -- Props

const PROP_PROGRESS: &str = "progress";
const PROP_LABEL: &str = "label";
const PROP_TITLE: &str = "title";

pub struct ProgressBarPropsBuilder {
    props: Option<Props>,
}

impl Default for ProgressBarPropsBuilder {
    fn default() -> Self {
        ProgressBarPropsBuilder {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for ProgressBarPropsBuilder {
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

impl From<Props> for ProgressBarPropsBuilder {
    fn from(props: Props) -> Self {
        ProgressBarPropsBuilder { props: Some(props) }
    }
}

impl ProgressBarPropsBuilder {
    /// ### with_progbar_color
    ///
    /// Set progbar color for component
    pub fn with_progbar_color(&mut self, color: Color) -> &mut Self {
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

    /// ### with_label
    ///
    /// Set label to display on progress bar
    pub fn with_label<S: AsRef<str>>(&mut self, label: S) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.own.insert(
                PROP_LABEL,
                PropPayload::One(PropValue::Str(label.as_ref().to_string())),
            );
        }
        self
    }

    /// ### with_progress
    ///
    /// Set progress percentage
    /// Progress must be in range [0.0,1.0] or will panic
    pub fn with_progress(&mut self, prog: f64) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            assert!(
                (0.0..=1.0).contains(&prog),
                "Progress must be in range [0.0,1.0]"
            );
            props
                .own
                .insert(PROP_PROGRESS, PropPayload::One(PropValue::F64(prog)));
        }
        self
    }
}

// -- Component

/// ## ProgressBar
///
/// provides a component which shows the progress. It is possible to set the style for the progress bar and the text shown above it.
pub struct ProgressBar {
    props: Props,
}

impl ProgressBar {
    /// ### new
    ///
    /// Instantiates a new `ProgressBar` component.
    pub fn new(props: Props) -> Self {
        ProgressBar { props }
    }
}

impl Component for ProgressBar {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    fn render(&self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.visible {
            // Text
            let title: Option<&str> = match self.props.own.get(PROP_TITLE).as_ref() {
                Some(PropPayload::One(PropValue::Str(t))) => Some(t),
                _ => None,
            };
            let label: String = match self.props.own.get(PROP_LABEL).as_ref() {
                Some(PropPayload::One(PropValue::Str(t))) => t.to_string(),
                _ => String::default(),
            };
            // Get percentage
            let percentage: f64 = match self.props.own.get(PROP_PROGRESS) {
                Some(PropPayload::One(PropValue::F64(ratio))) => *ratio,
                _ => 0.0,
            };
            let div: Block = super::utils::get_block(&self.props.borders, title, true);
            // Make progress bar
            render.render_widget(
                Gauge::default()
                    .block(div)
                    .gauge_style(
                        Style::default()
                            .fg(self.props.foreground)
                            .bg(self.props.background)
                            .add_modifier(self.props.modifiers),
                    )
                    .label(label)
                    .ratio(percentage),
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
mod test {

    use super::*;

    use crossterm::event::{KeyCode, KeyEvent};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_progress_bar() {
        let mut component: ProgressBar = ProgressBar::new(
            ProgressBarPropsBuilder::default()
                .hidden()
                .visible()
                .with_progress(0.60)
                .with_progbar_color(Color::Red)
                .with_background(Color::Blue)
                .with_title("Downloading file...")
                .with_label("60% - ETA: 00:20")
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .build(),
        );
        assert_eq!(
            *component.props.own.get(PROP_LABEL).unwrap(),
            PropPayload::One(PropValue::Str(String::from("60% - ETA: 00:20")))
        );
        assert_eq!(
            *component.props.own.get(PROP_TITLE).unwrap(),
            PropPayload::One(PropValue::Str(String::from("Downloading file...")))
        );
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.background, Color::Blue);
        assert_eq!(component.props.visible, true);
        assert_eq!(component.props.borders.borders, Borders::ALL);
        assert_eq!(component.props.borders.variant, BorderType::Double);
        assert_eq!(component.props.borders.color, Color::Red);
        assert_eq!(
            *component.props.own.get(PROP_PROGRESS).unwrap(),
            PropPayload::One(PropValue::F64(0.60))
        );
        // Get value
        assert_eq!(component.get_state(), Payload::None);
        component.active();
        component.blur();
        // Update
        let props = ProgressBarPropsBuilder::from(component.get_props())
            .with_progbar_color(Color::Yellow)
            .hidden()
            .build();
        assert_eq!(component.update(props), Msg::None);
        assert_eq!(component.props.foreground, Color::Yellow);
        assert_eq!(component.props.visible, false);
        // Event
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Delete))),
            Msg::OnKey(KeyEvent::from(KeyCode::Delete))
        );
        assert_eq!(component.on(Event::Resize(0, 0)), Msg::None);
    }

    #[test]
    #[should_panic]
    fn test_components_progress_bar_bad_prog() {
        ProgressBar::new(
            ProgressBarPropsBuilder::default()
                .with_progress(60.0)
                .build(),
        );
    }
}
