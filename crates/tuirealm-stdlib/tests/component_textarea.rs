use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Textarea;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::Component;
use tuirealm::props::{Borders, Color, Title};
use tuirealm::ratatui::layout::Size;
use tuirealm::ratatui::text::Span;
use tuirealm::state::{State, StateValue};
use tuirealm::testing::render_to_string;

#[test]
fn test_textarea_initial_state() {
    let component = Textarea::default().text_rows([
        Span::from("line1"),
        Span::from("line2"),
        Span::from("line3"),
    ]);
    assert_eq!(component.state(), State::Single(StateValue::Usize(0)));
}

#[test]
fn test_textarea_move_down() {
    let mut component =
        Textarea::default().text_rows([Span::from("A"), Span::from("B"), Span::from("C")]);
    let result = component.perform(Cmd::Move(Direction::Down));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(1)))
    );
}

#[test]
fn test_textarea_move_up_at_top() {
    let mut component = Textarea::default().text_rows([Span::from("A"), Span::from("B")]);
    let result = component.perform(Cmd::Move(Direction::Up));
    assert_eq!(result, CmdResult::NoChange);
}

#[test]
fn test_textarea_move_down_at_bottom() {
    let mut component = Textarea::default().text_rows([Span::from("A"), Span::from("B")]);
    component.perform(Cmd::Move(Direction::Down));
    let result = component.perform(Cmd::Move(Direction::Down));
    assert_eq!(result, CmdResult::NoChange);
}

#[test]
fn test_textarea_goto_end_begin() {
    let mut component =
        Textarea::default().text_rows([Span::from("A"), Span::from("B"), Span::from("C")]);
    let result = component.perform(Cmd::GoTo(Position::End));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(2)))
    );
    let result = component.perform(Cmd::GoTo(Position::Begin));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(0)))
    );
}

#[test]
fn test_textarea_scroll() {
    let mut component = Textarea::default().step(2).text_rows([
        Span::from("A"),
        Span::from("B"),
        Span::from("C"),
        Span::from("D"),
        Span::from("E"),
    ]);
    let result = component.perform(Cmd::Scroll(Direction::Down));
    assert_eq!(
        result,
        CmdResult::Changed(State::Single(StateValue::Usize(2)))
    );
}

#[test]
fn test_textarea_unhandled_cmd() {
    let mut component = Textarea::default().text_rows([Span::from("A")]);
    assert_eq!(
        component.perform(Cmd::Delete),
        CmdResult::Invalid(Cmd::Delete)
    );
    assert_eq!(
        component.perform(Cmd::Type('a')),
        CmdResult::Invalid(Cmd::Type('a'))
    );
}

#[test]
fn test_textarea_snapshot_default() {
    let mut component = Textarea::default()
        .borders(Borders::default())
        .title(Title::from("Log"))
        .foreground(Color::White)
        .highlight_str(">> ")
        .text_rows([
            Span::from("First line of text"),
            Span::from("Second line of text"),
            Span::from("Third line of text"),
        ]);
    let rendered = render_to_string(&mut component, Size::new(40, 7));
    insta::assert_snapshot!("textarea_stdlib_default", rendered);
}
