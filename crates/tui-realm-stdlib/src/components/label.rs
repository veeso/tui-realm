use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::MockComponent;
use tuirealm::props::{
    AttrValue, Attribute, Color, HorizontalAlignment, Props, Style, TextModifiers,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::state::State;

use crate::prop_ext::CommonProps;

/// A Label. It represents a single-line, single-style text without any container support.
///
/// If multi-style text is wanted, use [`Span`](super::Span).
/// If multi-style, mutli-line, with container support is wanted, use [`Paragraph`](super::Paragraph).
#[derive(Default)]
#[must_use]
pub struct Label {
    common: CommonProps,
    props: Props,
}

impl Label {
    /// Set the main foreground color.
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    /// Set the main background color.
    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    /// Set the main text modifiers.
    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    /// Set the main style.
    ///
    /// This option will overwrite any previous [`foreground`](Self::foreground), [`background`](Self::background) and [`modifiers`](Self::modifiers)!
    pub fn style(mut self, style: Style) -> Self {
        self.attr(Attribute::Style, AttrValue::Style(style));
        self
    }

    /// Set the Text content.
    pub fn text<S: Into<String>>(mut self, t: S) -> Self {
        self.attr(Attribute::Text, AttrValue::String(t.into()));
        self
    }

    /// Set the horizontal text alignment.
    pub fn alignment_horizontal(mut self, alignment: HorizontalAlignment) -> Self {
        self.attr(
            Attribute::AlignmentHorizontal,
            AttrValue::AlignmentHorizontal(alignment),
        );
        self
    }
}

impl MockComponent for Label {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if !self.common.display {
            return;
        }

        // Make text
        let text = self
            .props
            .get_ref(Attribute::Text)
            .and_then(|v| v.as_string())
            .map_or("", |v| v.as_str());
        let alignment: HorizontalAlignment = self
            .props
            .get_or(
                Attribute::AlignmentHorizontal,
                AttrValue::AlignmentHorizontal(HorizontalAlignment::Left),
            )
            .unwrap_alignment_horizontal();
        render.render_widget(
            Paragraph::new(text)
                .style(self.common.style)
                .alignment(alignment),
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

    use super::*;

    #[test]
    fn test_components_label() {
        let component: Label = Label::default()
            .alignment_horizontal(HorizontalAlignment::Center)
            .background(Color::Red)
            .foreground(Color::Yellow)
            .modifiers(TextModifiers::BOLD)
            .text("foobar");

        assert_eq!(component.state(), State::None);
    }

    #[test]
    fn test_various_text_inputs() {
        let _ = Label::default().text("str");
        let _ = Label::default().text(String::from("String"));
        // explicitly test references to string working
        #[allow(clippy::needless_borrows_for_generic_args)]
        let _ = Label::default().text(&String::from("&String"));
    }
}
