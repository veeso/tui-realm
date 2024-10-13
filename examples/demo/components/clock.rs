//! ## Label
//!
//! label component

use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{Alignment, Color, TextModifiers};
use tuirealm::ratatui::layout::Rect;
use tuirealm::{
    AttrValue, Attribute, Component, Event, Frame, MockComponent, NoUserEvent, State, StateValue,
};

use super::{Label, Msg};

/// Simple clock component which displays current time
pub struct Clock {
    component: Label,
    states: OwnStates,
}

impl Clock {
    pub fn new(initial_time: SystemTime) -> Self {
        Self {
            component: Label::default(),
            states: OwnStates::new(initial_time),
        }
    }

    pub fn alignment(mut self, a: Alignment) -> Self {
        self.component
            .attr(Attribute::TextAlign, AttrValue::Alignment(a));
        self
    }

    pub fn foreground(mut self, c: Color) -> Self {
        self.component
            .attr(Attribute::Foreground, AttrValue::Color(c));
        self
    }

    pub fn background(mut self, c: Color) -> Self {
        self.component
            .attr(Attribute::Background, AttrValue::Color(c));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    fn time_to_str(&self) -> String {
        let since_the_epoch = self.get_epoch_time();
        let hours = (since_the_epoch / 3600) % 24;
        let minutes = (since_the_epoch / 60) % 60;
        let seconds = since_the_epoch % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }

    fn get_epoch_time(&self) -> u64 {
        self.states
            .time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
}

impl MockComponent for Clock {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Render
        self.component.view(frame, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.component.attr(attr, value);
    }

    fn state(&self) -> State {
        // Return current time
        State::One(StateValue::U64(self.get_epoch_time()))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}

impl Component<Msg, NoUserEvent> for Clock {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        if let Event::Tick = ev {
            self.states.tick();
            // Set text
            self.attr(Attribute::Text, AttrValue::String(self.time_to_str()));
            Some(Msg::Clock)
        } else {
            None
        }
    }
}

struct OwnStates {
    time: SystemTime,
}

impl OwnStates {
    pub fn new(time: SystemTime) -> Self {
        Self { time }
    }

    pub fn tick(&mut self) {
        self.time = self.time.add(Duration::from_secs(1));
    }
}
