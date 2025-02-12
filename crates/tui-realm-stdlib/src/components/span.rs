//! ## Span
//!
//! `Span` represents a read-only text component without any container, but with the possibility to define multiple text parts.
//! The main difference with `Label` is that the Span allows different styles inside the same component for the texsts.

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Color, PropPayload, PropValue, Props, Style, TextModifiers,
    TextSpan,
};
use tuirealm::ratatui::text::Line;
use tuirealm::ratatui::{
    layout::Rect,
    text::{Span as TuiSpan, Text},
    widgets::Paragraph,
};
use tuirealm::{Frame, MockComponent, State};

// -- Component

/// ## Span
///
/// represents a read-only text component without any container, but with multy-style text parts
#[derive(Default)]
pub struct Span {
    props: Props,
}

impl Span {
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

    pub fn alignment(mut self, a: Alignment) -> Self {
        self.attr(Attribute::Alignment, AttrValue::Alignment(a));
        self
    }

    pub fn spans(mut self, s: &[TextSpan]) -> Self {
        self.attr(
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(
                s.iter().cloned().map(PropValue::TextSpan).collect(),
            )),
        );
        self
    }
}

impl MockComponent for Span {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Make text
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            // binding required as "spans" is a reference and otherwise would not live long enough
            let payload = self
                .props
                .get_ref(Attribute::Text)
                .and_then(|x| x.as_payload());
            let spans: Vec<TuiSpan> = match payload {
                Some(PropPayload::Vec(ref spans)) => spans
                    .iter()
                    // this will skip any "PropValue" that is not a "TextSpan", instead of panicing
                    .flat_map(|x| x.as_text_span())
                    .map(|x| {
                        // Keep colors and modifiers, or use default
                        let (fg, bg, modifiers) =
                            crate::utils::use_or_default_styles(&self.props, x);
                        TuiSpan::styled(
                            &x.content,
                            Style::default().add_modifier(modifiers).fg(fg).bg(bg),
                        )
                    })
                    .collect(),
                _ => Vec::new(),
            };
            let text: Text = Text::from(Line::from(spans));
            // Text properties
            let alignment: Alignment = self
                .props
                .get_or(Attribute::Alignment, AttrValue::Alignment(Alignment::Left))
                .unwrap_alignment();
            render.render_widget(
                Paragraph::new(text)
                    .alignment(alignment)
                    .style(Style::default().bg(background).fg(foreground)),
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
    fn test_components_span() {
        let component = Span::default()
            .background(Color::Blue)
            .foreground(Color::Red)
            .modifiers(TextModifiers::BOLD)
            .alignment(Alignment::Center)
            .spans(&[
                TextSpan::from("Press "),
                TextSpan::from("<ESC>").fg(Color::Cyan).bold(),
                TextSpan::from(" to quit"),
            ]);
        // Get value
        assert_eq!(component.state(), State::None);
    }
}
