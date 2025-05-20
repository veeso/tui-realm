//! ## Container
//!
//! `Container` represents an empty container where you can put other components into it.
//! It will render components based on how you defined the layout.
//! The way it updates properties is usually assigning the attributes to all the children components, but
//! when defining the component you can override these behaviours implementing `attr()` by yourself.
//! By default it will forward `Commands' to all the children and will return a `CmdResult::Batch` with all the results.

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{Alignment, AttrValue, Attribute, Borders, Color, Layout, Props};
use tuirealm::ratatui::layout::Rect;
use tuirealm::{Frame, MockComponent, State};

// -- Component

/// ## Container
///
/// represents a read-only text component without any container.
#[derive(Default)]
#[must_use]
pub struct Container {
    props: Props,
    /// Container children
    pub children: Vec<Box<dyn MockComponent>>,
}

impl Container {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn title<S: Into<String>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(Attribute::Title, AttrValue::Title((t.into(), a)));
        self
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.attr(Attribute::Layout, AttrValue::Layout(layout));
        self
    }

    pub fn children(mut self, children: Vec<Box<dyn MockComponent>>) -> Self {
        self.children = children;
        self
    }
}

impl MockComponent for Container {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Make block
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let title = self
                .props
                .get_ref(Attribute::Title)
                .and_then(|x| x.as_title());
            let div = crate::utils::get_block(borders, title, true, None);
            // Render block
            render.render_widget(div, area);
            // Render children
            if let Some(layout) = self.props.get(Attribute::Layout).map(|x| x.unwrap_layout()) {
                // make chunks
                let chunks = layout.chunks(area);
                // iter chunks
                for (i, chunk) in chunks.into_iter().enumerate() {
                    if let Some(child) = self.children.get_mut(i) {
                        child.view(render, chunk);
                    }
                }
            }
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value.clone());
        // Patch attribute to children
        self.children
            .iter_mut()
            .for_each(|x| x.attr(attr, value.clone()));
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        // Send command to children and return batch
        CmdResult::Batch(self.children.iter_mut().map(|x| x.perform(cmd)).collect())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_paragraph() {
        let component = Container::default()
            .background(Color::Blue)
            .foreground(Color::Red)
            .title("title", Alignment::Center);
        // Get value
        assert_eq!(component.state(), State::None);
    }
}
