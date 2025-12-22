//! ## Paragraph
//!
//! `Paragraph` represents a read-only text component inside a container, the text is wrapped inside the container automatically
//! using the [textwrap](https://docs.rs/textwrap/0.13.4/textwrap/) crate.
//! The textarea supports multi-style spans.
//! The component is not scrollable and doesn't handle any input. The text must then fit into the area.
//! If you want scroll support, use a `Textarea` instead.

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, Props, Style, TextModifiers, TextStatic, Title,
};
use tuirealm::ratatui::{
    layout::Rect,
    widgets::{Paragraph as TuiParagraph, Wrap},
};
use tuirealm::{Frame, MockComponent, State};

use crate::utils;

// -- Component

/// ## Paragraph
///
/// represents a read-only text component without any container.
#[derive(Default)]
#[must_use]
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
            let text = self
                .props
                .get_ref(Attribute::Text)
                .and_then(AttrValue::as_text)
                .map(utils::borrow_clone_text)
                .unwrap_or_default();

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
            let title = self
                .props
                .get_ref(Attribute::Title)
                .and_then(|x| x.as_title());
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
            .alignment(Alignment::Center)
            .text(vec![
                Line::from("Press "),
                Line::from("<ESC>").fg(Color::Cyan).bold(),
                Line::from(" to quit"),
            ])
            .wrap(true)
            .title(Title::from("title").alignment(Alignment::Center));
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
