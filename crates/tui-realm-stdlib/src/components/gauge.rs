//! `ProgressBar` provides a component which shows the progress. It is possible to set the style for the progress bar and the text shown above it.

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, Style, TextModifiers,
    Title,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Gauge as TuiGauge;
use tuirealm::state::State;

use crate::prop_ext::CommonProps;

// -- Component

/// `Gauge` provides a multi-line component which shows the progress. It is possible to set the style for the progress bar and the text shown above it.
///
/// Read more in [`Gauge`](TuiGauge).
///
/// If only a single-line Gauge is necessary, use [`LineGauge`](crate::LineGauge) instead.
#[derive(Default)]
#[must_use]
pub struct Gauge {
    common: CommonProps,
    props: Props,
}

impl Gauge {
    /// Set the main foreground color. This may get overwritten by individual text styles.
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    /// Set the main background color. This may get overwritten by individual text styles.
    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    /// Set the main text modifiers. This may get overwritten by individual text styles.
    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    /// Set the main style. This may get overwritten by individual text styles.
    ///
    /// This option will overwrite any previous [`foreground`](Self::foreground), [`background`](Self::background) and [`modifiers`](Self::modifiers)!
    pub fn style(mut self, style: Style) -> Self {
        self.attr(Attribute::Style, AttrValue::Style(style));
        self
    }

    /// Set a custom style for the border when the component is unfocused.
    pub fn inactive(mut self, s: Style) -> Self {
        self.attr(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    /// Add a border to the component.
    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    /// Add a title to the component.
    pub fn title<T: Into<Title>>(mut self, title: T) -> Self {
        self.attr(Attribute::Title, AttrValue::Title(title.into()));
        self
    }

    /// Set a label text for the Bar.
    pub fn label<S: Into<String>>(mut self, s: S) -> Self {
        self.attr(Attribute::Text, AttrValue::String(s.into()));
        self
    }

    /// Set the initial progress value.
    pub fn progress(mut self, p: f64) -> Self {
        Self::assert_progress(p);
        self.attr(
            Attribute::Value,
            AttrValue::Payload(PropPayload::Single(PropValue::F64(p))),
        );
        self
    }

    fn assert_progress(p: f64) {
        assert!(
            (0.0..=1.0).contains(&p),
            "Progress value must be in range [0.0, 1.0]"
        );
    }
}

impl Component for Gauge {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if !self.common.display {
            return;
        }

        // Text
        let label = self
            .props
            .get_or(Attribute::Text, AttrValue::String(String::default()))
            .unwrap_string();
        // Get percentage
        let percentage = self
            .props
            .get_or(
                Attribute::Value,
                AttrValue::Payload(PropPayload::Single(PropValue::F64(0.0))),
            )
            .unwrap_payload()
            .unwrap_single()
            .unwrap_f64();

        // Make progress bar
        let mut widget = TuiGauge::default()
            .style(self.common.style)
            .gauge_style(self.common.style)
            .label(label)
            .ratio(percentage)
            .use_unicode(true);

        if let Some(block) = self.common.get_block() {
            widget = widget.block(block);
        }

        render.render_widget(widget, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        if let Some(value) = self.common.get(attr) {
            return Some(value);
        }

        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if let Some(value) = self.common.set(attr, value) {
            if let Attribute::Value = attr {
                if let AttrValue::Payload(p) = value.clone() {
                    Self::assert_progress(p.unwrap_single().unwrap_f64());
                }
            }
            self.props.set(attr, value);
        }
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;
    use tuirealm::props::HorizontalAlignment;

    use super::*;

    #[test]
    fn test_components_progress_bar() {
        let component = Gauge::default()
            .background(Color::Red)
            .foreground(Color::White)
            .progress(0.60)
            .title(Title::from("Downloading file...").alignment(HorizontalAlignment::Center))
            .label("60% - ETA 00:20")
            .borders(Borders::default());
        // Get value
        assert_eq!(component.state(), State::None);
    }

    #[test]
    #[should_panic = "Progress value must be in range [0.0, 1.0]"]
    fn test_components_progress_bar_bad_prog() {
        let _ = Gauge::default()
            .background(Color::Red)
            .foreground(Color::White)
            .progress(6.0)
            .title(Title::from("Downloading file...").alignment(HorizontalAlignment::Center))
            .label("60% - ETA 00:20")
            .borders(Borders::default());
    }
}
