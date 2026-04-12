use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Select;
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, Title};
use tuirealm::ratatui::layout::Size;
use tuirealm::state::{State, StateValue};
use tuirealm::testing::render_to_string;

#[test]
fn test_select_initial_state() {
    let component = Select::default().choices(["A", "B", "C"]).value(0);
    assert_eq!(component.state(), State::Single(StateValue::Usize(0)));
}

#[test]
fn test_select_open_tab() {
    let mut component = Select::default().choices(["A", "B", "C"]).value(0);
    let result = component.perform(Cmd::Submit);
    assert_eq!(result, CmdResult::Visual);
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_select_navigate_and_select() {
    let mut component = Select::default().choices(["A", "B", "C"]).value(0);
    component.perform(Cmd::Submit); // Open tab
    component.perform(Cmd::Move(Direction::Down)); // Move to B
    let result = component.perform(Cmd::Submit); // Select B
    assert_eq!(
        result,
        CmdResult::Submit(State::Single(StateValue::Usize(1)))
    );
}

#[test]
fn test_select_cancel_closes_tab() {
    let mut component = Select::default().choices(["A", "B", "C"]).value(0);
    component.perform(Cmd::Submit); // Open tab
    assert_eq!(component.state(), State::None);
    let result = component.perform(Cmd::Cancel);
    assert!(matches!(result, CmdResult::Changed(_)));
    assert_eq!(component.state(), State::Single(StateValue::Usize(0)));
}

#[test]
fn test_select_unhandled_cmd() {
    let mut component = Select::default().choices(["A", "B"]);
    assert_eq!(
        component.perform(Cmd::Delete),
        CmdResult::Invalid(Cmd::Delete)
    );
    assert_eq!(
        component.perform(Cmd::Toggle),
        CmdResult::Invalid(Cmd::Toggle)
    );
}

#[test]
fn test_select_snapshot_closed() {
    let mut component = Select::default()
        .borders(Borders::default())
        .title(Title::from("Choose"))
        .foreground(Color::Cyan)
        .choices(["Option A", "Option B", "Option C"])
        .value(0);
    let rendered = render_to_string(&mut component, Size::new(40, 3));
    insta::assert_snapshot!("select_closed", rendered);
}

#[test]
fn test_select_snapshot_open() {
    let mut component = Select::default()
        .borders(Borders::default())
        .title(Title::from("Choose"))
        .foreground(Color::Cyan)
        .choices(["Option A", "Option B", "Option C"])
        .value(0);
    component.perform(Cmd::Submit); // Open tab
    let rendered = render_to_string(&mut component, Size::new(40, 8));
    insta::assert_snapshot!("select_open", rendered);
}
