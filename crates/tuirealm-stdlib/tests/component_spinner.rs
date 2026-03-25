mod common;

use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Spinner;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::Color;
use tuirealm::state::State;

#[test]
fn test_spinner_state_is_none() {
    let component = Spinner::default().sequence("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_spinner_perform_returns_none() {
    let mut component = Spinner::default().sequence("⠋⠙⠹⠸");
    assert_eq!(component.perform(Cmd::Submit), CmdResult::None);
    assert_eq!(component.perform(Cmd::Delete), CmdResult::None);
}

#[test]
fn test_spinner_snapshot_default() {
    let mut component = Spinner::default()
        .foreground(Color::Cyan)
        .sequence("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
    let rendered = common::render_to_string(&mut component, 10, 1);
    insta::assert_snapshot!(rendered, @"⠋");
}
