mod common;

use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::List;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, Title};
use tuirealm::ratatui::text::Line;
use tuirealm::state::{State, StateValue};

#[test]
fn test_list_initial_state_scrollable() {
    let component =
        List::default()
            .scroll(true)
            .rows(vec![Line::from("A"), Line::from("B"), Line::from("C")]);
    assert_eq!(component.state(), State::Single(StateValue::Usize(0)));
}

#[test]
fn test_list_initial_state_not_scrollable() {
    let component = List::default().scroll(false).rows(vec![Line::from("A")]);
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_list_move_down() {
    let mut component =
        List::default()
            .scroll(true)
            .rows(vec![Line::from("A"), Line::from("B"), Line::from("C")]);
    let result = component.perform(Cmd::Move(Direction::Down));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(1)))
    );
}

#[test]
fn test_list_move_up_at_top() {
    let mut component = List::default()
        .scroll(true)
        .rewind(false)
        .rows(vec![Line::from("A"), Line::from("B")]);
    let result = component.perform(Cmd::Move(Direction::Up));
    assert_eq!(result, CmdResult::None);
}

#[test]
fn test_list_move_down_at_bottom_no_rewind() {
    let mut component = List::default()
        .scroll(true)
        .rewind(false)
        .rows(vec![Line::from("A"), Line::from("B")])
        .selected_line(1);
    let result = component.perform(Cmd::Move(Direction::Down));
    assert_eq!(result, CmdResult::None);
}

#[test]
fn test_list_goto_begin_end() {
    let mut component = List::default()
        .scroll(true)
        .rows(vec![Line::from("A"), Line::from("B"), Line::from("C")])
        .selected_line(1);
    let result = component.perform(Cmd::GoTo(Position::End));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(2)))
    );
    let result = component.perform(Cmd::GoTo(Position::Begin));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(0)))
    );
}

#[test]
fn test_list_scroll_down() {
    let mut component = List::default().scroll(true).step(2).rows(vec![
        Line::from("A"),
        Line::from("B"),
        Line::from("C"),
        Line::from("D"),
        Line::from("E"),
    ]);
    let result = component.perform(Cmd::Scroll(Direction::Down));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(2)))
    );
}

#[test]
fn test_list_unhandled_cmd() {
    let mut component = List::default().scroll(true).rows(vec![Line::from("A")]);
    assert_eq!(component.perform(Cmd::Delete), CmdResult::None);
    assert_eq!(component.perform(Cmd::Type('a')), CmdResult::None);
}

#[test]
fn test_list_snapshot_default() {
    let mut component = List::default()
        .borders(Borders::default())
        .title(Title::from("Items"))
        .foreground(Color::White)
        .scroll(true)
        .highlighted_color(Color::Yellow)
        .rows(vec![
            Line::from("First item"),
            Line::from("Second item"),
            Line::from("Third item"),
        ]);
    let rendered = common::render_to_string(&mut component, 40, 7);
    insta::assert_snapshot!("list_default", rendered);
}

#[test]
fn test_list_snapshot_empty() {
    let mut component = List::default()
        .borders(Borders::default())
        .title(Title::from("Empty List"))
        .scroll(true)
        .rows(Vec::<Line>::new());
    let rendered = common::render_to_string(&mut component, 40, 5);
    insta::assert_snapshot!("list_empty", rendered);
}
