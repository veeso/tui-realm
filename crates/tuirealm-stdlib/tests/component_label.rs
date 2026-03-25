mod common;

use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Label;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{Color, HorizontalAlignment};
use tuirealm::state::State;

#[test]
fn test_label_state_is_none() {
    let component = Label::default().text("hello");
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_label_perform_returns_none() {
    let mut component = Label::default().text("hello");
    assert_eq!(component.perform(Cmd::Submit), CmdResult::None);
    assert_eq!(component.perform(Cmd::Type('a')), CmdResult::None);
    assert_eq!(component.perform(Cmd::Delete), CmdResult::None);
}

#[test]
fn test_label_snapshot_default() {
    let mut component = Label::default()
        .foreground(Color::Yellow)
        .text("Hello, World!");
    let rendered = common::render_to_string(&mut component, 40, 3);
    insta::assert_snapshot!("label_default", rendered);
}

#[test]
fn test_label_snapshot_centered() {
    let mut component = Label::default()
        .text("Centered Text")
        .alignment_horizontal(HorizontalAlignment::Center);
    let rendered = common::render_to_string(&mut component, 40, 3);
    insta::assert_snapshot!("label_centered", rendered);
}
