//! `Container` represents an empty container where you can put other components into it.

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, Layout, Props, Style, TextModifiers, Title,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::state::State;

use crate::prop_ext::CommonProps;

// -- Component

/// `Container` represents an empty container where you can put other components into it.
///
/// It will render components based on how you defined the layout.
/// The way it updates properties is usually by assigning the attributes to all the children components, but
/// when defining the component you can override these behaviours by implementing `attr()` yourself and directly accessing `children`'s `attr()`.
/// By default it will forward `Commands' to all the children and will return a `CmdResult::Batch` with all the results.
#[must_use]
pub struct Container {
    common: CommonProps,
    props: Props,
    /// Container children
    pub children: Vec<Box<dyn Component>>,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            common: CommonProps {
                border: Some(Borders::default()),
                ..CommonProps::default()
            },
            props: Props::default(),
            children: Vec::new(),
        }
    }
}

impl Container {
    /// Set the main foreground color. This may get overwritten by individual text styles.
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    /// Set the main background color. This may get overwritten by individual text styles.
    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    /// Set the main text modifiers. This may get overwritten by individual text styles.
    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    /// Set the main style. This may get overwritten by individual text styles.
    ///
    /// This option will overwrite any previous [`foreground`](Self::foreground), [`background`](Self::background) and [`modifiers`](Self::modifiers)!
    pub fn style(mut self, style: Style) -> Self {
        self.attr(Attribute::Style, AttrValue::Style(style));
        self
    }

    /// Set a custom style for the border when the component is unfocused.
    pub fn inactive(mut self, s: Style) -> Self {
        self.attr(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    /// Add a border to the component.
    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    /// Add a title to the component.
    pub fn title<T: Into<Title>>(mut self, title: T) -> Self {
        self.attr(Attribute::Title, AttrValue::Title(title.into()));
        self
    }

    /// Set a ratatui Layout to use for all the child components.
    ///
    /// If this is unset, nothing gets drawn!
    pub fn layout(mut self, layout: Layout) -> Self {
        self.attr(Attribute::Layout, AttrValue::Layout(layout));
        self
    }

    /// Set the children Components this container contains.
    pub fn children(mut self, children: Vec<Box<dyn Component>>) -> Self {
        self.children = children;
        self
    }
}

impl Component for Container {
    fn view(&mut self, render: &mut Frame, mut area: Rect) {
        if !self.common.display {
            return;
        }

        if let Some(block) = self.common.get_block() {
            let inner = block.inner(area);
            // Render block
            render.render_widget(block, area);
            area = inner;
        }

        // Render children
        if let Some(layout) = self
            .props
            .get(Attribute::Layout)
            .and_then(AttrValue::as_layout)
        {
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

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        if let Some(value) = self.common.get(attr) {
            return Some(value);
        }

        self.props.get(attr).cloned()
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if let Some(value) = self.common.set(attr, value.clone()) {
            self.props.set(attr, value);
        }

        // dont patch borders and title to all children
        if matches!(attr, Attribute::Borders | Attribute::Title) {
            return;
        }

        // TODO: how do you control patching things to children? For example change the title / border of a specific child?
        // Patch attribute to children
        self.children
            .iter_mut()
            .for_each(|x| x.attr(attr, value.clone()));
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        // TODO: send commands to a specific child?

        // Send command to children and return batch
        CmdResult::Batch(self.children.iter_mut().map(|x| x.perform(cmd)).collect())
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;
    use tuirealm::props::HorizontalAlignment;

    use super::*;

    #[test]
    fn test_components_paragraph() {
        let component = Container::default()
            .background(Color::Blue)
            .foreground(Color::Red)
            .title(Title::from("title").alignment(HorizontalAlignment::Center));
        // Get value
        assert_eq!(component.state(), State::None);
    }
}
