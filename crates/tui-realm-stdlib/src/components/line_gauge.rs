use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, SpanStatic, Style,
    TextModifiers, Title,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::text::Span;
use tuirealm::ratatui::widgets::LineGauge as TuiLineGauge;
use tuirealm::state::State;

use crate::prop_ext::CommonProps;

// -- Component

/// `LineGauge`, also known as progress bars, provides a component which shows a line which is some percent filled.
///
/// It is possible to set the style for the progress bar and the text shown above it.
///
/// Read more in [`LineGauge`](TuiLineGauge).
///
/// If a multi-line Guage is necessary, use [`Gauge`](crate::Gauge) instead.
#[derive(Default)]
#[must_use]
pub struct LineGauge {
    common: CommonProps,
    props: Props,
}

impl LineGauge {
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

    /// Set a label text for the Gauge.
    pub fn label<S: Into<String>>(mut self, s: S) -> Self {
        self.attr(Attribute::Text, AttrValue::String(s.into()));
        self
    }

    /// Set the initial progress.
    pub fn progress(mut self, p: f64) -> Self {
        Self::assert_progress(p);
        self.attr(
            Attribute::Value,
            AttrValue::Payload(PropPayload::Single(PropValue::F64(p))),
        );
        self
    }

    /// Set custom Style & Symbols for the filled & unfilled styles.
    ///
    /// By default ratatui uses [`HORIZONTAL`](tuirealm::ratatui::symbols::line::HORIZONTAL) for *both*.
    pub fn line_style<F: Into<SpanStatic>, U: Into<SpanStatic>>(
        mut self,
        filled: F,
        unfilled: U,
    ) -> Self {
        self.attr(
            Attribute::HighlightedStr,
            AttrValue::Payload(PropPayload::Pair((
                PropValue::TextSpan(filled.into()),
                PropValue::TextSpan(unfilled.into()),
            ))),
        );

        self
    }

    fn get_line_style(&self) -> Option<(&Span<'_>, &Span<'_>)> {
        self.props
            .get_ref(Attribute::HighlightedStr)
            .and_then(AttrValue::as_payload)
            .and_then(PropPayload::as_pair)
            .and_then(|pair| Some((pair.0.as_textspan()?, pair.1.as_textspan()?)))
    }

    fn assert_progress(p: f64) {
        assert!(
            (0.0..=1.0).contains(&p),
            "Progress value must be in range [0.0, 1.0]"
        );
    }
}

impl Component for LineGauge {
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

        let mut widget = TuiLineGauge::default()
            .style(self.common.style)
            .filled_style(self.common.style)
            .label(label)
            .ratio(percentage);

        if let Some(block) = self.common.get_block() {
            widget = widget.block(block);
        }

        if let Some(line_style) = self.get_line_style() {
            widget = widget
                .filled_symbol(&line_style.0.content)
                .filled_style(line_style.0.style)
                .unfilled_symbol(&line_style.1.content)
                .unfilled_style(line_style.1.style);
        }

        // Make progress bar
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
    use tuirealm::props::{BorderType, HorizontalAlignment};
    use tuirealm::ratatui::symbols::line::{DOUBLE_HORIZONTAL, HORIZONTAL};

    use super::*;

    #[test]
    fn test_components_progress_bar() {
        let component = LineGauge::default()
            .background(Color::Red)
            .foreground(Color::White)
            .progress(0.60)
            .title(Title::from("Downloading file...").alignment(HorizontalAlignment::Center))
            .label("60% - ETA 00:20")
            .line_style(DOUBLE_HORIZONTAL, HORIZONTAL)
            .borders(Borders::default());
        // Get value
        assert_eq!(component.state(), State::None);
    }

    #[test]
    #[should_panic = "Progress value must be in range [0.0, 1.0]"]
    fn line_gauge_bad_prog() {
        let _ = LineGauge::default()
            .background(Color::Red)
            .foreground(Color::White)
            .progress(6.0)
            .title(Title::from("Downloading file...").alignment(HorizontalAlignment::Center))
            .label("60% - ETA 00:20")
            .borders(Borders::default());
    }

    #[test]
    fn should_allow_styling_line() {
        let _ = LineGauge::default()
            .borders(
                Borders::default()
                    .color(Color::Blue)
                    .modifiers(BorderType::Rounded),
            )
            .foreground(Color::Blue)
            .label("0%")
            .title(Title::from("Loading...").alignment(HorizontalAlignment::Center))
            .line_style(
                Span::styled(HORIZONTAL, Style::new().fg(Color::Red)),
                Span::styled(HORIZONTAL, Style::new().fg(Color::Gray)),
            )
            .progress(0.0);
    }
}
