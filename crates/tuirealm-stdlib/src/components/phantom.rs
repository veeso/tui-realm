use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{AttrValue, Attribute, Props, QueryResult};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::state::State;

/// [`Phantom`] is a component which is not rendered. It's only purpose is to become a global listener in a tui-realm application
/// for some kind of events using subscriptions.
///
/// An example would be a listener for `<ESC>` key to terminate the application.
/// The [`Phantom`] allows you not to write a listener for each component for the `ESC` key, but just to subscribe the phantom to it.
#[derive(Default)]
pub struct Phantom {
    props: Props,
}

impl Component for Phantom {
    fn view(&mut self, _render: &mut Frame, _area: Rect) {}

    fn query<'a>(&'a self, attr: Attribute) -> Option<QueryResult<'a>> {
        self.props.get_for_query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        CmdResult::Invalid(cmd)
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_components_phantom() {
        let component = Phantom::default();
        // Get value
        assert_eq!(component.state(), State::None);
    }
}
