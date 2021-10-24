//! ## LineGauge
//!
//! `LineGauge` is a line gauge

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
use tuirealm::props::{
    Alignment, BlockTitle, BordersProps, PropPayload, PropValue, Props, PropsBuilder,
};
use tuirealm::tui::{
    layout::Rect,
    style::{Color, Style},
    symbols::line::{Set, DOUBLE, NORMAL, ROUNDED, THICK},
    widgets::{Block, BorderType, Borders, LineGauge as TuiLineGauge},
};
use tuirealm::{event::Event, CmdResult, Component, Frame, Payload};

// -- Props

const PROP_PROGRESS: &str = "progress";
const PROP_LABEL: &str = "label";
const PROP_LINE: &str = "line";

// -- line style
const LINE_NORMAL: u8 = 0;
const LINE_DOUBLE: u8 = 1;
const LINE_ROUND: u8 = 2;
const LINE_THICK: u8 = 3;

pub struct LineGaugePropsBuilder {
    props: Option<Props>,
}

impl Default for LineGaugePropsBuilder {
    fn default() -> Self {
        LineGaugePropsBuilder {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for LineGaugePropsBuilder {
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

impl From<Props> for LineGaugePropsBuilder {
    fn from(props: Props) -> Self {
        LineGaugePropsBuilder { props: Some(props) }
    }
}

impl LineGaugePropsBuilder {
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
    pub fn with_title<S: AsRef<str>>(&mut self, title: S, alignment: Alignment) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.title = Some(BlockTitle::new(title, alignment));
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

    /// ### with_line_doubled
    ///
    /// Set line style to `DOUBLE`
    pub fn with_line_doubled(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_LINE, PropPayload::One(PropValue::U8(LINE_DOUBLE)));
        }
        self
    }

    /// ### with_line_normal
    ///
    /// Set line style to `NORMAL`
    pub fn with_line_normal(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_LINE, PropPayload::One(PropValue::U8(LINE_NORMAL)));
        }
        self
    }

    /// ### with_line_rounded
    ///
    /// Set line style to `ROUND`
    pub fn with_line_rounded(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_LINE, PropPayload::One(PropValue::U8(LINE_ROUND)));
        }
        self
    }

    /// ### with_line_thick
    ///
    /// Set line style to `TICK`
    pub fn with_line_thick(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props
                .own
                .insert(PROP_LINE, PropPayload::One(PropValue::U8(LINE_THICK)));
        }
        self
    }
}

// -- Component

/// ## LineGauge
///
/// provides a component which shows the progress. It is possible to set the style for the progress bar and the text shown above it.
pub struct LineGauge {
    props: Props,
}

impl LineGauge {
    /// ### new
    ///
    /// Instantiates a new `LineGauge` component.
    pub fn new(props: Props) -> Self {
        LineGauge { props }
    }

    /// ### set
    ///
    /// Get set associated to prop
    fn set(&self) -> Set {
        match self.props.own.get(PROP_LINE) {
            Some(PropPayload::One(PropValue::U8(LINE_DOUBLE))) => DOUBLE,
            Some(PropPayload::One(PropValue::U8(LINE_ROUND))) => ROUNDED,
            Some(PropPayload::One(PropValue::U8(LINE_THICK))) => THICK,
            _ => NORMAL,
        }
    }
}

impl MockComponent for LineGauge {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    fn render(&self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Text
            let label: String = match self.props.own.get(PROP_LABEL).as_ref() {
                Some(PropPayload::One(PropValue::Str(t))) => t.to_string(),
                _ => String::default(),
            };
            // Get percentage
            let percentage: f64 = match self.props.own.get(PROP_PROGRESS) {
                Some(PropPayload::One(PropValue::F64(ratio))) => *ratio,
                _ => 0.0,
            };
            let div: Block = crate::utils::get_block(&borders, title.as_ref(), true);
            // Make progress bar
            render.render_widget(
                TuiLineGauge::default()
                    .block(div)
                    .gauge_style(
                        Style::default()
                            .fg(foreground)
                            .bg(background)
                            .add_modifier(self.props.modifiers),
                    )
                    .line_set(self.set())
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
mod test {

    use super::*;

    use crossterm::event::{KeyCode, KeyEvent};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_progress_bar() {
        let mut component: LineGauge = LineGauge::new(
            LineGaugePropsBuilder::default()
                .hidden()
                .visible()
                .with_progress(0.60)
                .with_progbar_color(Color::Red)
                .with_background(Color::Blue)
                .with_title("Downloading file...", Alignment::Center)
                .with_label("60% - ETA: 00:20")
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .with_line_doubled()
                .build(),
        );
        assert_eq!(
            *component.props.own.get(PROP_LABEL).unwrap(),
            PropPayload::One(PropValue::Str(String::from("60% - ETA: 00:20")))
        );
        assert_eq!(
            component.props.title.as_ref().unwrap().text(),
            "Downloading file..."
        );
        assert_eq!(
            component.props.title.as_ref().unwrap().alignment(),
            Alignment::Center
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
        assert_eq!(
            *component.props.own.get(PROP_LINE).unwrap(),
            PropPayload::One(PropValue::U8(LINE_DOUBLE))
        );
        // Get value
        assert_eq!(component.state(), State::None);
        component.active();
        component.blur();
        // Update
        let props = LineGaugePropsBuilder::from(component.get_props())
            .with_progbar_color(Color::Yellow)
            .hidden()
            .build();
        assert_eq!(component.update(props), CmdResult::None);
        assert_eq!(component.props.foreground, Color::Yellow);
        assert_eq!(component.props.visible, false);
        // Event
        assert_eq!(
            component.on(Cmd::Key(KeyCmd::from(KeyCode::Delete))),
            Cmd::None(KeyCmd::from(KeyCode::Delete))
        );
        assert_eq!(component.on(Cmd::Resize(0, 0)), CmdResult::None);
    }

    #[test]
    #[should_panic]
    fn test_components_progress_bar_bad_prog() {
        LineGauge::new(LineGaugePropsBuilder::default().with_progress(60.0).build());
    }
}
