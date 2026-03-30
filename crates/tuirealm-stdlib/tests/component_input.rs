mod common;

use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Input;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, InputType, Title};
use tuirealm::state::{State, StateValue};

#[test]
fn test_input_initial_state() {
    let component = Input::default().input_type(InputType::Text).value("hello");
    assert_eq!(
        component.state(),
        State::Single(StateValue::String("hello".to_string()))
    );
}

#[test]
fn test_input_type_char() {
    let mut component = Input::default().input_type(InputType::Text);
    let result = component.perform(Cmd::Type('a'));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::String("a".to_string())))
    );
    assert_eq!(
        component.state(),
        State::Single(StateValue::String("a".to_string()))
    );
}

#[test]
fn test_input_type_multiple_chars() {
    let mut component = Input::default().input_type(InputType::Text);
    component.perform(Cmd::Type('a'));
    component.perform(Cmd::Type('b'));
    component.perform(Cmd::Type('c'));
    assert_eq!(
        component.state(),
        State::Single(StateValue::String("abc".to_string()))
    );
}

#[test]
fn test_input_delete_backspace() {
    let mut component = Input::default().input_type(InputType::Text).value("abc");
    let result = component.perform(Cmd::Delete);
    assert!(matches!(result, CmdResult::Changed(_)));
    assert_eq!(
        component.state(),
        State::Single(StateValue::String("ab".to_string()))
    );
}

#[test]
fn test_input_delete_on_empty() {
    let mut component = Input::default().input_type(InputType::Text);
    let result = component.perform(Cmd::Delete);
    assert_eq!(result, CmdResult::NoChange);
}

#[test]
fn test_input_cancel_deletes_forward() {
    let mut component = Input::default().input_type(InputType::Text).value("abc");
    component.perform(Cmd::GoTo(Position::Begin));
    let result = component.perform(Cmd::Cancel);
    assert!(matches!(result, CmdResult::Changed(_)));
}

#[test]
fn test_input_move_cursor() {
    let mut component = Input::default().input_type(InputType::Text).value("abc");
    assert_eq!(
        component.perform(Cmd::Move(Direction::Left)),
        CmdResult::Visual
    );
    assert_eq!(
        component.perform(Cmd::Move(Direction::Right)),
        CmdResult::Visual
    );
}

#[test]
fn test_input_goto_begin_end() {
    let mut component = Input::default().input_type(InputType::Text).value("hello");
    assert_eq!(
        component.perform(Cmd::GoTo(Position::Begin)),
        CmdResult::Visual
    );
    assert_eq!(
        component.perform(Cmd::GoTo(Position::End)),
        CmdResult::Visual
    );
}

#[test]
fn test_input_submit() {
    let mut component = Input::default().input_type(InputType::Text).value("test");
    let result = component.perform(Cmd::Submit);
    assert_eq!(
        result,
        CmdResult::Submit(State::Single(StateValue::String("test".to_string())))
    );
}

#[test]
fn test_input_with_max_length() {
    let mut component = Input::default()
        .input_type(InputType::Text)
        .input_len(3)
        .value("ab");
    component.perform(Cmd::Type('c'));
    let result = component.perform(Cmd::Type('d'));
    assert_eq!(result, CmdResult::NoChange);
    assert_eq!(
        component.state(),
        State::Single(StateValue::String("abc".to_string()))
    );
}

#[test]
fn test_input_unhandled_cmd() {
    let mut component = Input::default().input_type(InputType::Text);
    assert_eq!(
        component.perform(Cmd::Toggle),
        CmdResult::Invalid(Cmd::Toggle)
    );
    assert_eq!(
        component.perform(Cmd::Scroll(Direction::Down)),
        CmdResult::Invalid(Cmd::Scroll(Direction::Down))
    );
}

#[test]
fn test_input_snapshot_default() {
    let mut component = Input::default()
        .borders(Borders::default())
        .title(Title::from("Username"))
        .foreground(Color::Cyan)
        .input_type(InputType::Text)
        .value("john_doe");
    let rendered = common::render_to_string(&mut component, 40, 3);
    insta::assert_snapshot!("input_default", rendered);
}

#[test]
fn test_input_snapshot_empty() {
    let mut component = Input::default()
        .borders(Borders::default())
        .title(Title::from("Input"))
        .input_type(InputType::Text);
    let rendered = common::render_to_string(&mut component, 40, 3);
    insta::assert_snapshot!("input_empty", rendered);
}

#[test]
fn test_input_snapshot_password() {
    let mut component = Input::default()
        .borders(Borders::default())
        .title(Title::from("Password"))
        .input_type(InputType::Password('*'))
        .value("secret");
    let rendered = common::render_to_string(&mut component, 40, 3);
    insta::assert_snapshot!("input_password", rendered);
}
