//! ## LineGauge
//!
//! `LineGauge` is a line gauge

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, SpanStatic, Style,
    TextModifiers, Title,
};
use tuirealm::ratatui::text::Span;
use tuirealm::ratatui::{layout::Rect, widgets::LineGauge as TuiLineGauge};
use tuirealm::{Frame, MockComponent, State};

// -- Component

/// ## LineGauge
///
/// provides a component which shows the progress. It is possible to set the style for the progress bar and the text shown above it.
#[derive(Default)]
#[must_use]
pub struct LineGauge {
    props: Props,
}

impl LineGauge {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    pub fn title<T: Into<Title>>(mut self, title: T) -> Self {
        self.attr(Attribute::Title, AttrValue::Title(title.into()));
        self
    }

    pub fn label<S: Into<String>>(mut self, s: S) -> Self {
        self.attr(Attribute::Text, AttrValue::String(s.into()));
        self
    }

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

impl MockComponent for LineGauge {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Text
            let label = self
                .props
                .get_or(Attribute::Text, AttrValue::String(String::default()))
                .unwrap_string();
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();
            let title = self
                .props
                .get_ref(Attribute::Title)
                .and_then(|x| x.as_title());
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

            let normal_style = Style::default()
                .fg(foreground)
                .bg(background)
                .add_modifier(modifiers);

            let div = crate::utils::get_block(borders, title, true, None);

            let mut line_guage = TuiLineGauge::default()
                .block(div)
                .style(normal_style)
                .filled_style(normal_style)
                .label(label)
                .ratio(percentage);

            if let Some(line_style) = self.get_line_style() {
                line_guage = line_guage
                    .filled_symbol(&line_style.0.content)
                    .filled_style(line_style.0.style)
                    .unfilled_symbol(&line_style.1.content)
                    .unfilled_style(line_style.1.style);
            }

            // Make progress bar
            render.render_widget(line_guage, area);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if let Attribute::Value = attr {
            if let AttrValue::Payload(p) = value.clone() {
                Self::assert_progress(p.unwrap_single().unwrap_f64());
            }
        }
        self.props.set(attr, value);
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

    use super::*;

    use pretty_assertions::assert_eq;
    use tuirealm::{
        props::{Alignment, BorderType},
        ratatui::symbols::line::{DOUBLE_HORIZONTAL, HORIZONTAL},
    };

    #[test]
    fn test_components_progress_bar() {
        let component = LineGauge::default()
            .background(Color::Red)
            .foreground(Color::White)
            .progress(0.60)
            .title(Title::from("Downloading file...").alignment(Alignment::Center))
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
            .title(Title::from("Downloading file...").alignment(Alignment::Center))
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
            .title(Title::from("Loading...").alignment(Alignment::Center))
            .line_style(
                Span::styled(HORIZONTAL, Style::new().fg(Color::Red)),
                Span::styled(HORIZONTAL, Style::new().fg(Color::Gray)),
            )
            .progress(0.0);
    }
}
