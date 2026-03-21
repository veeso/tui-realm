use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{
    AttrValue, Attribute, Color, HorizontalAlignment, PropPayload, PropValue, Props, SpanStatic,
    Style, TextModifiers,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::text::{Line, Span as RSpan, Text};
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::state::State;

use crate::prop_ext::CommonProps;
use crate::utils;

/// A Span represents single-line, multi-style text, without any container support.
///
/// If single-style text is wanted, use [`Label`](super::Label).
/// If multi-style, mutli-line, with container support is wanted, use [`Paragraph`](super::Paragraph).
#[derive(Default)]
#[must_use]
pub struct Span {
    common: CommonProps,
    props: Props,
}

impl Span {
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

    /// Set the horizontal text alignment.
    pub fn alignment_horizontal(mut self, a: HorizontalAlignment) -> Self {
        self.attr(
            Attribute::AlignmentHorizontal,
            AttrValue::AlignmentHorizontal(a),
        );
        self
    }

    /// Set the Text content.
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

impl Component for Span {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if !self.common.display {
            return;
        }

        // Make text
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
                .style(self.common.style),
            area,
        );
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        if let Some(value) = self.common.get(attr) {
            return Some(value);
        }

        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if let Some(value) = self.common.set(attr, value) {
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
mod tests {

    use pretty_assertions::assert_eq;
    use tuirealm::props::SpanStatic;
    use tuirealm::ratatui::style::Stylize;

    use super::*;

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
