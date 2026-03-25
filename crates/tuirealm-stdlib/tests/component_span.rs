mod common;

use pretty_assertions::assert_eq;
use tui_realm_stdlib::components::Span;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::HorizontalAlignment;
use tuirealm::ratatui::text::Span as RatatuiSpan;
use tuirealm::state::State;

#[test]
fn test_span_state_is_none() {
    let component = Span::default().spans([RatatuiSpan::raw("hello")]);
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_span_perform_returns_none() {
    let mut component = Span::default().spans([RatatuiSpan::raw("hello")]);
    assert_eq!(component.perform(Cmd::Submit), CmdResult::None);
}

#[test]
fn test_span_snapshot_default() {
    let mut component =
        Span::default().spans([RatatuiSpan::raw("Hello "), RatatuiSpan::raw("World")]);
    let rendered = common::render_to_string(&mut component, 40, 3);
    insta::assert_snapshot!("span_default", rendered);
}

#[test]
fn test_span_snapshot_centered() {
    let mut component = Span::default()
        .alignment_horizontal(HorizontalAlignment::Center)
        .spans([RatatuiSpan::raw("Centered")]);
    let rendered = common::render_to_string(&mut component, 40, 3);
    insta::assert_snapshot!("span_centered", rendered);
}
