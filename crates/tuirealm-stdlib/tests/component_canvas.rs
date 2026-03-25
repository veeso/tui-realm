mod common;

use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Canvas;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, Shape, Title};
use tuirealm::ratatui::widgets::canvas::Line as CanvasLine;
use tuirealm::state::State;

#[test]
fn test_canvas_state_is_none() {
    let component = Canvas::default();
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_canvas_perform_returns_none() {
    let mut component = Canvas::default();
    assert_eq!(component.perform(Cmd::Submit), CmdResult::None);
}

#[test]
fn test_canvas_snapshot_default() {
    let mut component = Canvas::default()
        .borders(Borders::default())
        .title(Title::from("Canvas"))
        .background(Color::Black)
        .x_bounds((0.0, 100.0))
        .y_bounds((0.0, 100.0))
        .data([Shape::Line(CanvasLine {
            x1: 0.0,
            y1: 0.0,
            x2: 100.0,
            y2: 100.0,
            color: Color::White,
        })]);
    let rendered = common::render_to_string(&mut component, 40, 15);
    insta::assert_snapshot!("canvas_default", rendered);
}
