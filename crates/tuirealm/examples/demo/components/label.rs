//! ## Label
//!
//! label component

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, NoUserEvent};
use tuirealm::props::{
    AttrValue, Attribute, Color, HorizontalAlignment, Props, Style, TextModifiers,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::state::State;

use super::Msg;

/// Simple label component; just renders a text
/// NOTE: since I need just one label, I'm not going to use different object; I will directly implement Component for Label.
/// This is not ideal actually and in a real app you should differentiate Mock Components from Application Components.
#[derive(Default)]
pub struct Label {
    props: Props,
}

impl Label {
    pub fn text<S>(mut self, s: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attr(Attribute::Text, AttrValue::String(s.as_ref().to_string()));
        self
    }

    pub fn alignment(mut self, a: HorizontalAlignment) -> Self {
        self.attr(Attribute::TextAlign, AttrValue::AlignmentHorizontal(a));
        self
    }

    pub fn foreground(mut self, c: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(c));
        self
    }

    pub fn background(mut self, c: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(c));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }
}

impl Component for Label {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if matches!(
            self.props.get_ref(Attribute::Display),
            Some(AttrValue::Flag(false))
        ) {
            return;
        }

        // Get properties
        let text = self
            .props
            .get_ref(Attribute::Text)
            .and_then(AttrValue::as_string)
            .map(String::as_str)
            .unwrap_or_default();
        let alignment = self
            .props
            .get_ref(Attribute::TextAlign)
            .and_then(AttrValue::as_alignment_horizontal)
            .unwrap_or(HorizontalAlignment::Left);
        let foreground = self
            .props
            .get_ref(Attribute::Foreground)
            .and_then(AttrValue::as_color)
            .unwrap_or(Color::Reset);
        let background = self
            .props
            .get_ref(Attribute::Background)
            .and_then(AttrValue::as_color)
            .unwrap_or(Color::Reset);
        let modifiers = self
            .props
            .get_ref(Attribute::TextProps)
            .and_then(AttrValue::as_text_modifiers)
            .unwrap_or_default();
        frame.render_widget(
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

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get_ref(attr).cloned()
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl AppComponent<Msg, NoUserEvent> for Label {
    fn on(&mut self, _: &Event<NoUserEvent>) -> Option<Msg> {
        // Does nothing
        None
    }
}
