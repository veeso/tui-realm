mod common;

use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Checkbox;
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, HorizontalAlignment, Title};
use tuirealm::state::{State, StateValue};

#[test]
fn test_checkbox_initial_state_empty() {
    let component = Checkbox::default().choices(["A", "B", "C"]);
    assert_eq!(component.state(), State::Vec(vec![]));
}

#[test]
fn test_checkbox_initial_state_with_values() {
    let component = Checkbox::default().choices(["A", "B", "C"]).values(&[0, 2]);
    assert_eq!(
        component.state(),
        State::Vec(vec![StateValue::Usize(0), StateValue::Usize(2)])
    );
}

#[test]
fn test_checkbox_toggle() {
    let mut component = Checkbox::default().choices(["A", "B", "C"]);
    let result = component.perform(Cmd::Toggle);
    assert!(matches!(result, CmdResult::Changed(_)));
    assert_eq!(component.state(), State::Vec(vec![StateValue::Usize(0)]));
}

#[test]
fn test_checkbox_toggle_off() {
    let mut component = Checkbox::default().choices(["A", "B", "C"]).values(&[0]);
    let result = component.perform(Cmd::Toggle);
    assert!(matches!(result, CmdResult::Changed(_)));
    assert_eq!(component.state(), State::Vec(vec![]));
}

#[test]
fn test_checkbox_move_and_toggle() {
    let mut component = Checkbox::default().choices(["A", "B", "C"]);
    component.perform(Cmd::Move(Direction::Right));
    component.perform(Cmd::Toggle);
    assert_eq!(component.state(), State::Vec(vec![StateValue::Usize(1)]));
}

#[test]
fn test_checkbox_submit() {
    let mut component = Checkbox::default().choices(["A", "B"]).values(&[1]);
    let result = component.perform(Cmd::Submit);
    assert_eq!(
        result,
        CmdResult::Submit(State::Vec(vec![StateValue::Usize(1)]))
    );
}

#[test]
fn test_checkbox_unhandled_cmd() {
    let mut component = Checkbox::default().choices(["A", "B"]);
    assert_eq!(
        component.perform(Cmd::Delete),
        CmdResult::Invalid(Cmd::Delete)
    );
}

#[test]
fn test_checkbox_snapshot_default() {
    let mut component = Checkbox::default()
        .borders(Borders::default())
        .title(Title::from("Select items").alignment(HorizontalAlignment::Center))
        .foreground(Color::Yellow)
        .choices(["Alpha", "Beta", "Gamma"])
        .values(&[1]);
    let rendered = common::render_to_string(&mut component, 50, 3);
    insta::assert_snapshot!("checkbox_default", rendered);
}
