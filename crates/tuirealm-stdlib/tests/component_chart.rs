mod common;

use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::{Chart, ChartDataset};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, Style, Title};
use tuirealm::state::State;

fn make_dataset() -> ChartDataset {
    ChartDataset::default()
        .name("Series 1")
        .data(vec![(0.0, 1.0), (1.0, 3.0), (2.0, 2.0), (3.0, 5.0)])
        .style(Style::default().fg(Color::Red))
}

#[test]
fn test_chart_state_is_none() {
    let component = Chart::default().data([make_dataset()]);
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_chart_move_cursor() {
    let mut component = Chart::default().data([make_dataset()]);
    assert_eq!(
        component.perform(Cmd::Move(Direction::Right)),
        CmdResult::None
    );
    assert_eq!(
        component.perform(Cmd::Move(Direction::Left)),
        CmdResult::None
    );
}

#[test]
fn test_chart_goto() {
    let mut component = Chart::default().data([make_dataset()]);
    assert_eq!(
        component.perform(Cmd::GoTo(Position::Begin)),
        CmdResult::None
    );
    assert_eq!(component.perform(Cmd::GoTo(Position::End)), CmdResult::None);
}

#[test]
fn test_chart_disabled() {
    let mut component = Chart::default().disabled(true).data([make_dataset()]);
    assert_eq!(
        component.perform(Cmd::Move(Direction::Right)),
        CmdResult::None
    );
}

#[test]
fn test_chart_unhandled_cmd() {
    let mut component = Chart::default().data([make_dataset()]);
    assert_eq!(
        component.perform(Cmd::Delete),
        CmdResult::Invalid(Cmd::Delete)
    );
}

#[test]
fn test_chart_snapshot_default() {
    let mut component = Chart::default()
        .borders(Borders::default())
        .title(Title::from("Metrics"))
        .x_bounds((0.0, 10.0))
        .y_bounds((0.0, 10.0))
        .x_labels(&["0", "5", "10"])
        .y_labels(&["0", "5", "10"])
        .data([make_dataset()]);
    let rendered = common::render_to_string(&mut component, 60, 15);
    insta::assert_snapshot!("chart_default", rendered);
}
