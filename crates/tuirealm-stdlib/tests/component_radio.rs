mod common;

use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Radio;
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, HorizontalAlignment, Title};
use tuirealm::state::{State, StateValue};

#[test]
fn test_radio_initial_state() {
    let component = Radio::default().choices(["A", "B", "C"]).value(1);
    assert_eq!(component.state(), State::Single(StateValue::Usize(1)));
}

#[test]
fn test_radio_move_right() {
    let mut component = Radio::default().choices(["A", "B", "C"]).value(0);
    let result = component.perform(Cmd::Move(Direction::Right));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(1)))
    );
}

#[test]
fn test_radio_move_left() {
    let mut component = Radio::default().choices(["A", "B", "C"]).value(1);
    let result = component.perform(Cmd::Move(Direction::Left));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(0)))
    );
}

#[test]
fn test_radio_boundary_no_rewind() {
    let mut component = Radio::default()
        .choices(["A", "B", "C"])
        .value(2)
        .rewind(false);
    let result = component.perform(Cmd::Move(Direction::Right));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(2)))
    );
}

#[test]
fn test_radio_rewind() {
    let mut component = Radio::default()
        .choices(["A", "B", "C"])
        .value(2)
        .rewind(true);
    let result = component.perform(Cmd::Move(Direction::Right));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(0)))
    );
}

#[test]
fn test_radio_submit() {
    let mut component = Radio::default().choices(["A", "B", "C"]).value(1);
    let result = component.perform(Cmd::Submit);
    assert_eq!(
        result,
        CmdResult::Submit(State::Single(StateValue::Usize(1)))
    );
}

#[test]
fn test_radio_unhandled_cmd() {
    let mut component = Radio::default().choices(["A", "B"]);
    assert_eq!(component.perform(Cmd::Delete), CmdResult::None);
    assert_eq!(component.perform(Cmd::Toggle), CmdResult::None);
}

#[test]
fn test_radio_snapshot_default() {
    let mut component = Radio::default()
        .borders(Borders::default())
        .title(Title::from("Pick one").alignment(HorizontalAlignment::Center))
        .foreground(Color::Cyan)
        .choices(["Option A", "Option B", "Option C"])
        .value(0);
    let rendered = common::render_to_string(&mut component, 40, 3);
    insta::assert_snapshot!("radio_default", rendered);
}
