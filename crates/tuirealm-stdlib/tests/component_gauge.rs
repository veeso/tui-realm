mod common;

use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Gauge;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, Title};
use tuirealm::state::State;

#[test]
fn test_gauge_state_is_none() {
    let component = Gauge::default().progress(0.5);
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_gauge_perform_unhandled_cmd() {
    let mut component = Gauge::default().progress(0.5);
    assert_eq!(
        component.perform(Cmd::Submit),
        CmdResult::Invalid(Cmd::Submit)
    );
}

#[test]
fn test_gauge_snapshot_default() {
    let mut component = Gauge::default()
        .borders(Borders::default())
        .title(Title::from("Progress"))
        .foreground(Color::Green)
        .label("50%")
        .progress(0.5);
    let rendered = common::render_to_string(&mut component, 40, 3);
    insta::assert_snapshot!("gauge_default", rendered);
}

#[test]
fn test_gauge_snapshot_empty() {
    let mut component = Gauge::default().borders(Borders::default()).progress(0.0);
    let rendered = common::render_to_string(&mut component, 40, 3);
    insta::assert_snapshot!("gauge_empty", rendered);
}

#[test]
fn test_gauge_snapshot_full() {
    let mut component = Gauge::default().borders(Borders::default()).progress(1.0);
    let rendered = common::render_to_string(&mut component, 40, 3);
    insta::assert_snapshot!("gauge_full", rendered);
}
