//! ## Label
//!
//! label component

use std::time::UNIX_EPOCH;

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent};
use tuirealm::props::{
    AttrValue, Attribute, Color, HorizontalAlignment, Props, QueryResult, Style,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::state::State;

use super::{Msg, UserEvent};

/// Simple label component; just renders a text
/// NOTE: since I need just one label, I'm not going to use different object; I will directly implement Component for Label.
/// This is not ideal actually and in a real app you should differentiate Components from Application Components.
#[derive(Default)]
pub struct Label {
    props: Props,
}

impl Component for Label {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if matches!(
            self.props.get(Attribute::Display),
            Some(AttrValue::Flag(false))
        ) {
            return;
        }

        // Get properties
        let text = self
            .props
            .get(Attribute::Text)
            .and_then(AttrValue::as_string)
            .map(String::as_str)
            .unwrap_or_default();
        let alignment = self
            .props
            .get(Attribute::TextAlign)
            .and_then(AttrValue::as_alignment_horizontal)
            .unwrap_or(HorizontalAlignment::Left);
        let foreground = self
            .props
            .get(Attribute::Foreground)
            .and_then(AttrValue::as_color)
            .unwrap_or(Color::Reset);
        let background = self
            .props
            .get(Attribute::Background)
            .and_then(AttrValue::as_color)
            .unwrap_or(Color::Reset);
        let modifiers = self
            .props
            .get(Attribute::TextProps)
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

    fn query<'a>(&'a self, attr: Attribute) -> Option<QueryResult<'a>> {
        self.props.get_for_query(attr)
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

impl AppComponent<Msg, UserEvent> for Label {
    fn on(&mut self, ev: &Event<UserEvent>) -> Option<Msg> {
        // Does nothing
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => Some(Msg::AppClose),
            Event::User(UserEvent::GotData(time)) => {
                // set text
                self.attr(
                    Attribute::Text,
                    AttrValue::String(
                        time.duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                            .to_string(),
                    ),
                );

                Some(Msg::None)
            }
            _ => None,
        }
    }
}
