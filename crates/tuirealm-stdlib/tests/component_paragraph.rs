use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Paragraph;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{Borders, HorizontalAlignment, Title};
use tuirealm::ratatui::layout::Size;
use tuirealm::ratatui::text::Line;
use tuirealm::state::State;
use tuirealm::testing::render_to_string;

#[test]
fn test_paragraph_state_is_none() {
    let component = Paragraph::default().text(vec![Line::from("hello")]);
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_paragraph_unhandled_cmd() {
    let mut component = Paragraph::default().text(vec![Line::from("hello")]);
    assert_eq!(
        component.perform(Cmd::Submit),
        CmdResult::Invalid(Cmd::Submit)
    );
}

#[test]
fn test_paragraph_snapshot_default() {
    let mut component = Paragraph::default()
        .borders(Borders::default())
        .title(Title::from("Info"))
        .text(vec![Line::from("Line one"), Line::from("Line two")]);
    let rendered = render_to_string(&mut component, Size::new(40, 6));
    insta::assert_snapshot!("paragraph_default", rendered);
}

#[test]
fn test_paragraph_snapshot_centered() {
    let mut component = Paragraph::default()
        .alignment_horizontal(HorizontalAlignment::Center)
        .text(vec![Line::from("Centered")]);
    let rendered = render_to_string(&mut component, Size::new(40, 3));
    insta::assert_snapshot!("paragraph_centered", rendered);
}
