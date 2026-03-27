mod common;

use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::LineGauge;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, Title};
use tuirealm::state::State;

#[test]
fn test_line_gauge_state_is_none() {
    let component = LineGauge::default().progress(0.5);
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_line_gauge_unhandled_cmd() {
    let mut component = LineGauge::default().progress(0.5);
    assert_eq!(
        component.perform(Cmd::Submit),
        CmdResult::Invalid(Cmd::Submit)
    );
}

#[test]
fn test_line_gauge_snapshot_default() {
    let mut component = LineGauge::default()
        .borders(Borders::default())
        .title(Title::from("Loading"))
        .foreground(Color::Cyan)
        .label("75%")
        .progress(0.75);
    let rendered = common::render_to_string(&mut component, 40, 3);
    insta::assert_snapshot!("line_gauge_default", rendered);
}

#[test]
fn test_line_gauge_snapshot_empty() {
    let mut component = LineGauge::default()
        .borders(Borders::default())
        .progress(0.0);
    let rendered = common::render_to_string(&mut component, 40, 3);
    insta::assert_snapshot!("line_gauge_empty", rendered);
}
