mod common;

use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::BarChart;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, Title};
use tuirealm::state::State;

#[test]
fn test_bar_chart_state_is_none() {
    let component = BarChart::default().data(&[("Q1", 100), ("Q2", 200)]);
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_bar_chart_move_cursor() {
    let mut component = BarChart::default().data(&[("Q1", 100), ("Q2", 200), ("Q3", 300)]);
    assert_eq!(
        component.perform(Cmd::Move(Direction::Right)),
        CmdResult::Visual
    );
    assert_eq!(
        component.perform(Cmd::Move(Direction::Left)),
        CmdResult::Visual
    );
}

#[test]
fn test_bar_chart_goto() {
    let mut component = BarChart::default().data(&[("Q1", 100), ("Q2", 200), ("Q3", 300)]);
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
fn test_bar_chart_disabled() {
    let mut component = BarChart::default()
        .disabled(true)
        .data(&[("Q1", 100), ("Q2", 200)]);
    assert_eq!(
        component.perform(Cmd::Move(Direction::Right)),
        CmdResult::None
    );
}

#[test]
fn test_bar_chart_unhandled_cmd() {
    let mut component = BarChart::default().data(&[("Q1", 100)]);
    assert_eq!(
        component.perform(Cmd::Delete),
        CmdResult::Invalid(Cmd::Delete)
    );
    assert_eq!(
        component.perform(Cmd::Submit),
        CmdResult::Invalid(Cmd::Submit)
    );
}

#[test]
fn test_bar_chart_snapshot_default() {
    let mut component = BarChart::default()
        .borders(Borders::default())
        .title(Title::from("Sales"))
        .foreground(Color::Green)
        .data(&[("Q1", 100), ("Q2", 150), ("Q3", 200), ("Q4", 175)]);
    let rendered = common::render_to_string(&mut component, 50, 12);
    insta::assert_snapshot!("bar_chart_default", rendered);
}
