#![allow(dead_code)] // practically just compile tests

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{AttrValue, Attribute, QueryResult};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::state::State;
use tuirealm_derive::Component;

struct Stub;

impl tuirealm::component::Component for Stub {
    fn view(&mut self, _frame: &mut Frame, _area: Rect) {}

    fn query(&self, _attr: Attribute) -> Option<QueryResult<'_>> {
        None
    }

    fn attr(&mut self, _attr: Attribute, _value: AttrValue) {}

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::NoChange
    }
}

#[test]
fn should_allow_default_field_name() {
    #[derive(Component)]
    struct Test1 {
        component: Stub,
    }
}

#[test]
fn should_allow_specific_names() {
    #[derive(Component)]
    #[component = "non_default_name"]
    struct Test1 {
        non_default_name: Stub,
    }

    #[derive(Component)]
    #[component("non_default_name")]
    struct Test2 {
        non_default_name: Stub,
    }
}

#[test]
fn should_allow_specific_names_via_field_attr() {
    #[derive(Component)]
    struct Test1 {
        #[component]
        non_default_name: Stub,
    }
}

#[test]
fn should_allow_unnamed_fields() {
    #[derive(Component)]
    struct Test1(Stub);

    #[derive(Component)]
    struct Test2(String, #[component] Stub);
}
