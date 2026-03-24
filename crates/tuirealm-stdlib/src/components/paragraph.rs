use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, HorizontalAlignment, Props, Style, TextModifiers,
    TextStatic, Title,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::{
    layout::Rect,
    widgets::{Paragraph as TuiParagraph, Wrap},
};
use tuirealm::state::State;

use crate::prop_ext::CommonProps;
use crate::utils;

/// A Paragraph represents multi-line, multi-style, automatically wrapped text, with container support.
///
/// This component does not scroll. If scrolling is additionally wanted, use [`Textarea`](super::Textarea).
///
/// If single-style, single-line text is wanted, use [`Label`](super::Label).
/// If multi-style, single-line text is wanted, use [`Span`](super::Span).
#[derive(Debug, Default)]
#[must_use]
pub struct Paragraph {
    common: CommonProps,
    props: Props,
}

impl Paragraph {
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

    /// Add a border to the component.
    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
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

    /// Add a title to the component.
    pub fn title<T: Into<Title>>(mut self, title: T) -> Self {
        self.attr(Attribute::Title, AttrValue::Title(title.into()));
        self
    }

    /// Set the text of the [`Paragraph`].
    pub fn text<T>(mut self, text: T) -> Self
    where
        T: Into<TextStatic>,
    {
        self.attr(Attribute::Text, AttrValue::Text(text.into()));
        self
    }

    /// Set wheter automatic wrap whitespace trimming should be enabled.
    ///
    /// Default: `false`
    pub fn wrap_trim(mut self, wrap: bool) -> Self {
        self.attr(Attribute::TextWrap, AttrValue::Flag(wrap));
        self
    }
}

impl Component for Paragraph {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if !self.common.display {
            return;
        }

        // Make text items
        let text = self
            .props
            .get_ref(Attribute::Text)
            .and_then(AttrValue::as_text)
            .map(utils::borrow_clone_text)
            .unwrap_or_default();

        // Text properties
        let alignment: HorizontalAlignment = self
            .props
            .get_ref(Attribute::AlignmentHorizontal)
            .and_then(AttrValue::as_alignment_horizontal)
            .unwrap_or(HorizontalAlignment::Left);
        // Wrap
        let trim = self
            .props
            .get_ref(Attribute::TextWrap)
            .and_then(AttrValue::as_flag)
            .unwrap_or_default();

        let mut paragraph = TuiParagraph::new(text)
            .style(self.common.style)
            .alignment(alignment)
            .wrap(Wrap { trim });

        if let Some(block) = self.common.get_block() {
            paragraph = paragraph.block(block);
        }

        render.render_widget(paragraph, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        if let Some(value) = self.common.get(attr) {
            return Some(value);
        }

        self.props.get_ref(attr).cloned()
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

    use super::*;

    use pretty_assertions::assert_eq;
    use tuirealm::ratatui::{
        style::Stylize,
        text::{Line, Text},
    };

    #[test]
    fn test_components_paragraph() {
        let component = Paragraph::default()
            .background(Color::Blue)
            .foreground(Color::Red)
            .modifiers(TextModifiers::BOLD)
            .alignment_horizontal(HorizontalAlignment::Center)
            .text(vec![
                Line::from("Press "),
                Line::from("<ESC>").fg(Color::Cyan).bold(),
                Line::from(" to quit"),
            ])
            .wrap_trim(true)
            .title(Title::from("title").alignment(HorizontalAlignment::Center));
        // Get value
        assert_eq!(component.state(), State::None);
    }

    #[test]
    fn various_text_types() {
        // Vec of Lines
        let _ = Paragraph::default().text(vec![Line::raw("hello")]);
        // Direct text
        let _ = Paragraph::default().text(Text::from("hello"));
    }
}
