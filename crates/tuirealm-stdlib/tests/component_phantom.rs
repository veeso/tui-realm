use tui_realm_stdlib::components::Phantom;
use tuirealm::component::Component;
use tuirealm::ratatui::layout::Size;
use tuirealm::state::State;
use tuirealm::testing::render_to_string;

#[test]
fn test_phantom_state_is_none() {
    let component = Phantom::default();
    assert_eq!(component.state(), State::None);
}

#[test]
fn test_phantom_snapshot_default() {
    let mut component = Phantom::default();
    let rendered = render_to_string(&mut component, Size::new(40, 5));
    insta::assert_snapshot!(rendered, @r"




");
}
