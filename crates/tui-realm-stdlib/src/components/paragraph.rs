//! ## Paragraph
//!
//! `Paragraph` represents a read-only text component inside a container, the text is wrapped inside the container automatically
//! using the [textwrap](https://docs.rs/textwrap/0.13.4/textwrap/) crate.
//! The textarea supports multi-style spans.
//! The component is not scrollable and doesn't handle any input. The text must then fit into the area.
//! If you want scroll support, use a `Textarea` instead.

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, Style,
    TextModifiers, TextSpan,
};
use tuirealm::ratatui::text::Line as Spans;
use tuirealm::ratatui::{
    layout::Rect,
    text::Span,
    widgets::{Paragraph as TuiParagraph, Wrap},
};
use tuirealm::{Frame, MockComponent, State};

// -- Component

/// ## Paragraph
///
/// represents a read-only text component without any container.
#[derive(Default)]
pub struct Paragraph {
    props: Props,
}

impl Paragraph {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn alignment(mut self, a: Alignment) -> Self {
        self.attr(Attribute::Alignment, AttrValue::Alignment(a));
        self
    }

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }

    pub fn text(mut self, s: &[TextSpan]) -> Self {
        self.attr(
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(
                s.iter().cloned().map(PropValue::TextSpan).collect(),
            )),
        );
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.attr(Attribute::TextWrap, AttrValue::Flag(wrap));
        self
    }
}

impl MockComponent for Paragraph {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Make text items
            let text: Vec<Spans> = match self.props.get(Attribute::Text).map(|x| x.unwrap_payload())
            {
                Some(PropPayload::Vec(spans)) => spans
                    .iter()
                    .cloned()
                    .map(|x| x.unwrap_text_span())
                    .map(|x| {
                        let (fg, bg, modifiers) =
                            crate::utils::use_or_default_styles(&self.props, &x);
                        Spans::from(vec![Span::styled(
                            x.content,
                            Style::default().add_modifier(modifiers).fg(fg).bg(bg),
                        )])
                    })
                    .collect(),
                _ => Vec::new(),
            };
            // Text properties
            let alignment: Alignment = self
                .props
                .get_or(Attribute::Alignment, AttrValue::Alignment(Alignment::Left))
                .unwrap_alignment();
            // Wrap
            let trim = self
                .props
                .get_or(Attribute::TextWrap, AttrValue::Flag(false))
                .unwrap_flag();
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let title = self.props.get(Attribute::Title).map(|x| x.unwrap_title());
            let div = crate::utils::get_block(borders, title, true, None);
            render.render_widget(
                TuiParagraph::new(text)
                    .block(div)
                    .style(
                        Style::default()
                            .fg(foreground)
                            .bg(background)
                            .add_modifier(modifiers),
                    )
                    .alignment(alignment)
                    .wrap(Wrap { trim }),
                area,
            );
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value)
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_paragraph() {
        let component = Paragraph::default()
            .background(Color::Blue)
            .foreground(Color::Red)
            .modifiers(TextModifiers::BOLD)
            .alignment(Alignment::Center)
            .text(&[
                TextSpan::from("Press "),
                TextSpan::from("<ESC>").fg(Color::Cyan).bold(),
                TextSpan::from(" to quit"),
            ])
            .wrap(true)
            .title("title", Alignment::Center);
        // Get value
        assert_eq!(component.state(), State::None);
    }
}
