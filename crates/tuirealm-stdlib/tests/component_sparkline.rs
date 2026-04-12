use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Sparkline;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, Title};
use tuirealm::state::State;
use tuirealm::testing::render_to_string;

#[test]
fn test_sparkline_state_is_none() {
    let component = Sparkline::default().data(&[1, 5, 3, 7, 2]);
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_sparkline_unhandled_cmd() {
    let mut component = Sparkline::default().data(&[1, 5, 3]);
    assert_eq!(
        component.perform(Cmd::Submit),
        CmdResult::Invalid(Cmd::Submit)
    );
}

#[test]
fn test_sparkline_snapshot_default() {
    let mut component = Sparkline::default()
        .borders(Borders::default())
        .title(Title::from("Metrics"))
        .foreground(Color::Green)
        .data(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let rendered = render_to_string(&mut component, 40, 5);
    insta::assert_snapshot!("sparkline_default", rendered);
}

#[test]
fn test_sparkline_snapshot_empty() {
    let mut component = Sparkline::default().borders(Borders::default()).data(&[]);
    let rendered = render_to_string(&mut component, 40, 5);
    insta::assert_snapshot!("sparkline_empty", rendered);
}
