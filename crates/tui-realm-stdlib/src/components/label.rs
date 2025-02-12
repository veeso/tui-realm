//! ## Label
//!
//! `Label` represents a read-only text component without any container.

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{Alignment, AttrValue, Attribute, Color, Props, Style, TextModifiers};
use tuirealm::ratatui::{layout::Rect, widgets::Paragraph};
use tuirealm::{Frame, MockComponent, State};

// -- Component

/// ## Label
///
/// represents a read-only text component without any container.
#[derive(Default)]
pub struct Label {
    props: Props,
}

impl Label {
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

    pub fn text<S: Into<String>>(mut self, t: S) -> Self {
        self.attr(Attribute::Text, AttrValue::String(t.into()));
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.attr(Attribute::Alignment, AttrValue::Alignment(alignment));
        self
    }
}

impl MockComponent for Label {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Make text
            let text = self
                .props
                .get_ref(Attribute::Text)
                .and_then(|v| v.as_string())
                .map(|v| v.as_str())
                .unwrap_or("");
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let alignment: Alignment = self
                .props
                .get_or(Attribute::Alignment, AttrValue::Alignment(Alignment::Left))
                .unwrap_alignment();
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();
            render.render_widget(
                Paragraph::new(text)
                    .style(
                        Style::default()
                            .fg(foreground)
                            .bg(background)
                            .add_modifier(modifiers),
                    )
                    .alignment(alignment),
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
    fn test_components_label() {
        let component: Label = Label::default()
            .alignment(Alignment::Center)
            .background(Color::Red)
            .foreground(Color::Yellow)
            .modifiers(TextModifiers::BOLD)
            .text("foobar");

        assert_eq!(component.state(), State::None);
    }

    #[test]
    fn test_various_text_inputs() {
        let _ = Label::default().text("str");
        let _ = Label::default().text(*&"*&str");
        let _ = Label::default().text(String::from("String"));
        let _ = Label::default().text(&String::from("&String"));
        let _ = Label::default().text(format!("Format"));
    }
}
