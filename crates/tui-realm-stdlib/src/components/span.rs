//! ## Span
//!
//! `Span` represents a read-only text component without any container, but with the possibility to define multiple text parts.
//! The main difference with `Label` is that the Span allows different styles inside the same component for the texsts.

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{
    AttrValue, Attribute, Color, HorizontalAlignment, PropPayload, PropValue, Props, SpanStatic,
    Style, TextModifiers,
};
use tuirealm::ratatui::{
    layout::Rect,
    text::{Line, Span as RSpan, Text},
    widgets::Paragraph,
};
use tuirealm::{Frame, MockComponent, State};

use crate::utils;

// -- Component

/// ## Span
///
/// Represents a read-only, single-line text component without any container, but with multi-style text parts
#[derive(Default)]
#[must_use]
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

    pub fn alignment_horizontal(mut self, a: HorizontalAlignment) -> Self {
        self.attr(
            Attribute::AlignmentHorizontal,
            AttrValue::AlignmentHorizontal(a),
        );
        self
    }

    pub fn spans<T>(mut self, s: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<SpanStatic>,
    {
        self.attr(
            Attribute::Text,
            AttrValue::Payload(PropPayload::Vec(
                s.into_iter()
                    .map(Into::into)
                    .map(PropValue::TextSpan)
                    .collect(),
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
            let text = match payload {
                Some(PropPayload::Vec(lines)) => {
                    let lines: Vec<RSpan> = lines
                        .iter()
                        // this will skip any "PropValue" that is not a "TextSpan", instead of panicing
                        .filter_map(|x| x.as_textspan())
                        .map(utils::borrow_clone_span)
                        .collect();
                    Text::from(Line::from(lines))
                }
                _ => Text::default(),
            };
            // Text properties
            let alignment: HorizontalAlignment = self
                .props
                .get_or(
                    Attribute::AlignmentHorizontal,
                    AttrValue::AlignmentHorizontal(HorizontalAlignment::Left),
                )
                .unwrap_alignment_horizontal();
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
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;
    use tuirealm::{props::SpanStatic, ratatui::style::Stylize};

    #[test]
    fn test_components_span() {
        let component = Span::default()
            .background(Color::Blue)
            .foreground(Color::Red)
            .modifiers(TextModifiers::BOLD)
            .alignment_horizontal(HorizontalAlignment::Center)
            .spans([
                SpanStatic::from("Press "),
                SpanStatic::from("<ESC>").fg(Color::Cyan).bold(),
                SpanStatic::from(" to quit"),
            ]);
        // Get value
        assert_eq!(component.state(), State::None);
    }

    #[test]
    fn various_spans_types() {
        // Vec
        let _ = Span::default().spans(vec![SpanStatic::raw("hello")]);
        // static array
        let _ = Span::default().spans([SpanStatic::raw("hello")]);
        // boxed array
        let _ = Span::default().spans(vec![SpanStatic::raw("hello")].into_boxed_slice());
        // already a iterator
        let _ = Span::default().spans(["Hello"].map(SpanStatic::raw));
    }
}
