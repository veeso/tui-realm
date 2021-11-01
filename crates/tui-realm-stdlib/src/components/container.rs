//! ## Container
//!
//! `Container` represents an empty container where you can put other components into it.
//! It will render components based on how you defined the layout.
//! The way it updates properties is usually assigning the attributes to all the children components, but
//! when defining the component you can override these behaviours implementing `attr()` by yourself.
//! By default it will forward `Commands' to all the children and will return a `CmdResult::Batch` with all the results.

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{Alignment, AttrValue, Attribute, Borders, Color, Layout, Props};
use tuirealm::tui::layout::Rect;
use tuirealm::{Frame, MockComponent, State};

// -- Component

/// ## Container
///
/// represents a read-only text component without any container.
pub struct Container {
    props: Props,
    /// Container children
    pub children: Vec<Box<dyn MockComponent>>,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            props: Props::default(),
            children: Vec::new(),
        }
    }
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

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
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
            let title = self.props.get(Attribute::Title).map(|x| x.unwrap_title());
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
