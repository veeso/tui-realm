mod common;

use pretty_assertions::assert_eq;
use tui_realm_treeview::mock::mock_tree;
use tui_realm_treeview::{TREE_CMD_CLOSE, TREE_CMD_OPEN, TreeView};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, Title};
use tuirealm::state::{State, StateValue};

#[test]
fn test_treeview_initial_state() {
    let component = TreeView::default()
        .borders(Borders::default())
        .with_tree(mock_tree())
        .initial_node("/");
    assert_eq!(
        component.state(),
        State::Single(StateValue::String("/".to_string()))
    );
}

#[test]
fn test_treeview_move_down() {
    let mut component = TreeView::default()
        .borders(Borders::default())
        .with_tree(mock_tree())
        .initial_node("a"); // selecting "a" opens its ancestor "/" so move_down works
    let result = component.perform(Cmd::Move(Direction::Down));
    assert!(matches!(result, CmdResult::Changed(_)));
}

#[test]
fn test_treeview_move_up_at_root() {
    let mut component = TreeView::default()
        .borders(Borders::default())
        .with_tree(mock_tree())
        .initial_node("/");
    let result = component.perform(Cmd::Move(Direction::Up));
    assert_eq!(result, CmdResult::None);
}

#[test]
fn test_treeview_open_close_node() {
    let mut component = TreeView::default()
        .borders(Borders::default())
        .with_tree(mock_tree())
        .initial_node("/");
    assert_eq!(
        component.perform(Cmd::Custom(TREE_CMD_CLOSE)),
        CmdResult::Visual
    );
    assert_eq!(
        component.perform(Cmd::Custom(TREE_CMD_OPEN)),
        CmdResult::Visual
    );
}

#[test]
fn test_treeview_scroll() {
    let mut component = TreeView::default()
        .borders(Borders::default())
        .scroll_step(3)
        .with_tree(mock_tree())
        .initial_node("a"); // selecting "a" opens its ancestor "/" so scroll works
    let result = component.perform(Cmd::Scroll(Direction::Down));
    assert!(matches!(result, CmdResult::Changed(_)));
}

#[test]
fn test_treeview_goto() {
    let mut component = TreeView::default()
        .borders(Borders::default())
        .with_tree(mock_tree())
        .initial_node("/");
    component.perform(Cmd::GoTo(Position::End));
    component.perform(Cmd::GoTo(Position::Begin));
}

#[test]
fn test_treeview_submit() {
    let mut component = TreeView::default()
        .borders(Borders::default())
        .with_tree(mock_tree())
        .initial_node("/");
    let result = component.perform(Cmd::Submit);
    assert!(matches!(result, CmdResult::Submit(_)));
}

#[test]
fn test_treeview_unhandled_cmd() {
    let mut component = TreeView::default()
        .borders(Borders::default())
        .with_tree(mock_tree())
        .initial_node("/");
    assert_eq!(
        component.perform(Cmd::Delete),
        CmdResult::Invalid(Cmd::Delete)
    );
    assert_eq!(
        component.perform(Cmd::Type('a')),
        CmdResult::Invalid(Cmd::Type('a'))
    );
}

// Snapshot tests

#[test]
fn test_treeview_snapshot_default() {
    let mut component = TreeView::default()
        .borders(Borders::default())
        .title(Title::from("Files"))
        .foreground(Color::White)
        .highlighted_color(Color::Yellow)
        .indent_size(3)
        .with_tree(mock_tree())
        .initial_node("/");
    let rendered = common::render_to_string(&mut component, 40, 15);
    insta::assert_snapshot!("treeview_default", rendered);
}

#[test]
fn test_treeview_snapshot_collapsed() {
    let mut component = TreeView::default()
        .borders(Borders::default())
        .title(Title::from("Files"))
        .indent_size(3)
        .with_tree(mock_tree())
        .initial_node("/");
    component.perform(Cmd::Custom(TREE_CMD_CLOSE));
    let rendered = common::render_to_string(&mut component, 40, 15);
    insta::assert_snapshot!("treeview_collapsed", rendered);
}
