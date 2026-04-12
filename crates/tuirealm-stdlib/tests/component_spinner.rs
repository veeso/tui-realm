use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Spinner;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::Color;
use tuirealm::ratatui::layout::Size;
use tuirealm::state::State;
use tuirealm::testing::render_to_string;

#[test]
fn test_spinner_state_is_none() {
    let component = Spinner::default().sequence("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_spinner_unhandled_cmd() {
    let mut component = Spinner::default().sequence("⠋⠙⠹⠸");
    assert_eq!(
        component.perform(Cmd::Submit),
        CmdResult::Invalid(Cmd::Submit)
    );
    assert_eq!(
        component.perform(Cmd::Delete),
        CmdResult::Invalid(Cmd::Delete)
    );
}

#[test]
fn test_spinner_snapshot_default() {
    let mut component = Spinner::default()
        .foreground(Color::Cyan)
        .sequence("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
    let rendered = render_to_string(&mut component, Size::new(10, 1));
    insta::assert_snapshot!(rendered, @"⠋");
}
