use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::{Container, Label};
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Layout, Title};
use tuirealm::ratatui::layout::{Constraint, Direction, Size};
use tuirealm::state::State;
use tuirealm::testing::render_to_string;

#[test]
fn test_container_state_is_none() {
    let component = Container::default();
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_container_perform_forwards_to_children() {
    let child1 = Label::default().text("A");
    let child2 = Label::default().text("B");
    let mut component = Container::default().children(vec![Box::new(child1), Box::new(child2)]);
    let result = component.perform(Cmd::Submit);
    assert!(matches!(result, CmdResult::Batch(ref v) if v.len() == 2));
}

#[test]
fn test_container_snapshot_with_children() {
    let child1 = Label::default().text("Child 1");
    let child2 = Label::default().text("Child 2");
    let mut component = Container::default()
        .borders(Borders::default())
        .title(Title::from("Container"))
        .layout(
            Layout::default()
                .direction(Direction::Vertical)
                .constraints(&[Constraint::Percentage(50), Constraint::Percentage(50)]),
        )
        .children(vec![Box::new(child1), Box::new(child2)]);
    let rendered = render_to_string(&mut component, Size::new(40, 8));
    insta::assert_snapshot!("container_with_children", rendered);
}
