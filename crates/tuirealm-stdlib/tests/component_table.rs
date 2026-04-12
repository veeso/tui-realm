use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Table;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, Style, TableBuilder, Title};
use tuirealm::ratatui::text::Line;
use tuirealm::state::{State, StateValue};
use tuirealm::testing::render_to_string;

fn make_table_data() -> tuirealm::props::Table {
    TableBuilder::default()
        .add_col(Line::from("Alice"))
        .add_col(Line::from("30"))
        .add_row()
        .add_col(Line::from("Bob"))
        .add_col(Line::from("25"))
        .add_row()
        .add_col(Line::from("Charlie"))
        .add_col(Line::from("35"))
        .build()
}

#[test]
fn test_table_initial_state_scrollable() {
    let component = Table::default()
        .scroll(true)
        .headers(["Name", "Age"])
        .widths(&[20, 10])
        .table(make_table_data());
    assert_eq!(component.state(), State::Single(StateValue::Usize(0)));
}

#[test]
fn test_table_move_down() {
    let mut component = Table::default()
        .scroll(true)
        .headers(["Name", "Age"])
        .widths(&[20, 10])
        .table(make_table_data());
    let result = component.perform(Cmd::Move(Direction::Down));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(1)))
    );
}

#[test]
fn test_table_move_up_at_top() {
    let mut component = Table::default()
        .scroll(true)
        .rewind(false)
        .headers(["Name", "Age"])
        .widths(&[20, 10])
        .table(make_table_data());
    let result = component.perform(Cmd::Move(Direction::Up));
    assert_eq!(result, CmdResult::NoChange);
}

#[test]
fn test_table_goto_end_begin() {
    let mut component = Table::default()
        .scroll(true)
        .headers(["Name", "Age"])
        .widths(&[20, 10])
        .table(make_table_data());
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
fn test_table_scroll() {
    let mut component = Table::default()
        .scroll(true)
        .step(2)
        .headers(["Name", "Age"])
        .widths(&[20, 10])
        .table(make_table_data());
    let result = component.perform(Cmd::Scroll(Direction::Down));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(2)))
    );
}

#[test]
fn test_table_unhandled_cmd() {
    let mut component = Table::default().scroll(true).table(make_table_data());
    assert_eq!(
        component.perform(Cmd::Delete),
        CmdResult::Invalid(Cmd::Delete)
    );
}

#[test]
fn test_table_snapshot_default() {
    let mut component = Table::default()
        .borders(Borders::default())
        .title(Title::from("Users"))
        .foreground(Color::White)
        .scroll(true)
        .highlight_style(Style::new().fg(Color::Yellow))
        .headers(["Name", "Age"])
        .widths(&[20, 10])
        .table(make_table_data());
    let rendered = render_to_string(&mut component, 50, 8);
    insta::assert_snapshot!("table_default", rendered);
}
