//! ## View
//!
//! This module exposes the View structure, which is the wrapper for all the components in an application.

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
use crate::tui::layout::Rect;
use crate::{AttrValue, Attribute, Component, Event, Frame, State};
// -- ext
use std::collections::HashMap;

type WrappedComponent<Msg, UserEvent> = Box<dyn Component<Msg, UserEvent>>;

/// ## View
///
/// View is the wrapper and manager for all the components.
/// A View is a container for all the components in a certain layout.
/// Each View can have only one focused component at the time. At least one component must be always focused
pub struct View<'a, Msg, UserEvent>
where
    Msg: PartialEq,
    UserEvent: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd,
{
    /// Components Mounted onto View
    components: HashMap<&'a str, WrappedComponent<Msg, UserEvent>>,
    /// Current active component
    focus: Option<&'a str>,
    /// Focus stack; used to determine which component should hold focus in case the current element is blurred
    focus_stack: Vec<&'a str>,
}

impl<'a, Msg, UserEvent> Default for View<'a, Msg, UserEvent>
where
    Msg: PartialEq,
    UserEvent: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd,
{
    fn default() -> Self {
        Self {
            components: HashMap::new(),
            focus: None,
            focus_stack: Vec::new(),
        }
    }
}

impl<'a, Msg, UserEvent> View<'a, Msg, UserEvent>
where
    Msg: PartialEq,
    UserEvent: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd,
{
    /// ### mount
    ///
    /// Mount component on View
    pub fn mount(&mut self, id: &'a str, component: Box<dyn Component<Msg, UserEvent>>) {
        self.components.insert(id, component);
    }

    /// ### umount
    ///
    /// Umount component from View
    pub fn umount(&mut self, id: &'a str) {
        if self.has_focus(id.as_ref()) {
            self.blur();
        }
        // Remove component from stack
        self.pop_from_stack(id);
        // Umount
        self.components.remove(id);
    }

    /// ### mounted
    ///
    /// Returns whether component `id` is mounted
    pub fn mounted(&self, id: &'a str) -> bool {
        self.components.contains_key(id)
    }

    /// ### view
    ///
    /// Render component called `id`
    pub fn view(&mut self, id: &'a str, f: &mut Frame, area: Rect) {
        if let Some(c) = self.components.get_mut(id) {
            c.view(f, area);
        }
    }

    /// ### forward
    ///
    /// Forward `event` (call `on()`) on component `id` and return a `Msg` if any
    pub fn forward(&mut self, id: &'a str, event: Event<UserEvent>) -> Option<Msg> {
        match self.components.get_mut(id) {
            None => None,
            Some(c) => c.on(event),
        }
    }

    /// ### query
    ///
    /// Query view component for a certain `AttrValue`
    /// Returns None if the component doesn't exist or if the attribute doesn't exist.
    pub fn query(&self, id: &'a str, query: Attribute) -> Option<AttrValue> {
        match self.components.get(id) {
            None => None,
            Some(c) => c.query(query),
        }
    }

    /// ### attr
    ///
    /// Set attribute for component `id`
    pub fn attr(&mut self, id: &'a str, attr: Attribute, value: AttrValue) {
        if let Some(c) = self.components.get_mut(id) {
            c.attr(attr, value);
        }
    }

    /// ### state
    ///
    /// Get state for component `id`.
    /// Returns `None` if component doesn't exist
    pub fn state(&self, id: &'a str) -> Option<State> {
        self.components.get(id).map(|c| c.state())
    }

    // -- shorthands

    /// ### active
    ///
    /// Shorthand for `attr(id, Attribute::Focus(AttrValue::Flag(true)))`.
    /// It also sets the component as the current one having focus.
    /// Previous active component, if any, GETS PUSHED to the STACK
    ///
    /// > NOTE: users should always use this function to give focus to components.
    /// > Panics: if component doesn't exist. Use `mounted()` to check if component exists
    pub fn active(&mut self, id: &'a str) {
        if let Some(c) = self.components.get_mut(id) {
            // Set attribute
            c.attr(Attribute::Focus, AttrValue::Flag(true));
            // Move current focus
            self.change_focus(id);
        } else {
            panic!("Component '{}' doesn't exist", id);
        }
    }

    /// ### blur
    ///
    /// Blur selected element AND DON'T PUSH CURRENT ACTIVE ELEMENT INTO THE STACK
    /// Shorthand for `attr(id, Attribute::Focus(AttrValue::Flag(false)))`.
    /// It also unset the current focus and give it to the first element in stack.
    ///
    /// > NOTE: users should always use this function to remove focus to components.
    pub fn blur(&mut self) {
        if let Some(id) = self.focus.take() {
            if let Some(c) = self.components.get_mut(id) {
                c.attr(Attribute::Focus, AttrValue::Flag(false));
            }
            self.focus_to_last();
        }
    }

    // -- private

    /// ### push_to_stack
    ///
    /// Push component `id` to focus stack
    /// In case it is already in the focus stack,
    /// it will be first removed from it.
    fn push_to_stack(&mut self, id: &'a str) {
        self.pop_from_stack(id);
        self.focus_stack.push(id);
    }

    /// ### pop_from_stack
    ///
    /// Pop component `id` from focus stack
    fn pop_from_stack(&mut self, id: &str) {
        self.focus_stack.retain(|x| *x != id);
    }

    /// ### has_focus
    ///
    /// Returns whether `who` has focus
    fn has_focus(&self, who: &str) -> bool {
        match self.focus {
            None => false,
            Some(id) => who == id,
        }
    }

    /// ### move_focus_to_stack
    ///
    /// If focus is `Some`, move it to the top of the stack and set it to `None`.
    /// Then pop from stack `new_focus` and set it to current `focus`.
    ///
    /// > Panics if `new_focus` doesn't exist in components
    fn change_focus(&mut self, new_focus: &str) {
        if let Some(focus) = self.focus.take() {
            self.push_to_stack(focus);
        }
        self.pop_from_stack(new_focus);
        // Get key from focus_stack (otherwise lifetime won't be valid)
        let key = self.components.keys().find(|x| **x == new_focus).unwrap();
        self.focus = Some(key);
    }

    /// ### focus_to_last
    ///
    /// Give focus to the last component in the stack
    fn focus_to_last(&mut self) {
        if let Some(focus) = self.take_last_from_stack() {
            self.active(focus);
        }
    }

    /// ### take_last_from_stack
    ///
    /// Take last element from stack if any
    fn take_last_from_stack(&mut self) -> Option<&'a str> {
        self.focus_stack.pop()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{
        event::{Key, KeyEvent},
        mock::{MockBarInput, MockEvent, MockFooInput, MockMsg},
        Value,
    };

    use pretty_assertions::assert_eq;

    const INPUT_FOO: &'static str = "INPUT_FOO";
    const INPUT_BAR: &'static str = "INPUT_BAR";
    const INPUT_OMAR: &'static str = "INPUT_OMAR";

    #[test]
    fn default_view_should_be_empty() {
        let view: View<MockMsg, MockEvent> = View::default();
        assert!(view.components.is_empty());
        assert_eq!(view.focus, None);
        assert!(view.focus_stack.is_empty());
    }

    #[test]
    fn view_should_mount_and_umount_components() {
        let mut view: View<MockMsg, MockEvent> = View::default();
        // Mount foo
        view.mount(INPUT_FOO, Box::new(MockFooInput::default()));
        assert_eq!(view.components.len(), 1);
        assert!(view.mounted(INPUT_FOO));
        assert_eq!(view.mounted(INPUT_BAR), false);
        // Mount bar
        view.mount(INPUT_BAR, Box::new(MockBarInput::default()));
        assert_eq!(view.components.len(), 2);
        assert!(view.mounted(INPUT_BAR));
        // Mount twice
        view.mount(INPUT_BAR, Box::new(MockBarInput::default()));
        assert_eq!(view.components.len(), 2);
        assert!(view.mounted(INPUT_BAR));
        // Umount
        view.umount(INPUT_FOO);
        assert_eq!(view.components.len(), 1);
        assert_eq!(view.mounted(INPUT_FOO), false);
        assert_eq!(view.mounted(INPUT_BAR), true);
        view.umount(INPUT_BAR);
        assert_eq!(view.components.len(), 0);
        assert_eq!(view.mounted(INPUT_BAR), false);
    }

    #[test]
    fn view_should_compile_with_dynamic_names() {
        let mut view: View<MockMsg, MockEvent> = View::default();
        let names: Vec<String> = (0..10).map(|x| format!("INPUT_{}", x)).collect();
        names.iter().for_each(|x| {
            view.mount(x, Box::new(MockBarInput::default()));
        });
        assert_eq!(view.components.len(), 10);
        names.iter().for_each(|x| assert!(view.mounted(x)));
    }

    #[test]
    fn view_should_handle_focus() {
        let mut view: View<MockMsg, MockEvent> = View::default();
        view.mount(INPUT_FOO, Box::new(MockFooInput::default()));
        view.mount(INPUT_BAR, Box::new(MockBarInput::default()));
        view.mount(INPUT_OMAR, Box::new(MockBarInput::default()));
        // Active foo
        view.active(INPUT_FOO);
        assert!(view.has_focus(INPUT_FOO));
        assert_eq!(view.focus.unwrap(), INPUT_FOO);
        assert!(view.focus_stack.is_empty());
        // Give focus to BAR
        view.active(INPUT_BAR);
        assert!(view.has_focus(INPUT_BAR));
        assert_eq!(view.focus_stack.len(), 1);
        // Give focus to OMAR
        view.active(INPUT_OMAR);
        assert!(view.has_focus(INPUT_OMAR));
        assert_eq!(view.focus_stack.len(), 2);
        // Give focus back to FOO
        view.active(INPUT_FOO);
        assert!(view.has_focus(INPUT_FOO));
        assert_eq!(view.focus_stack.len(), 2);
        // Umount FOO
        view.umount(INPUT_FOO);
        // OMAR should have focus
        assert!(view.has_focus(INPUT_OMAR));
        assert_eq!(view.focus_stack.len(), 1);
        // Umount BAR
        view.umount(INPUT_BAR);
        // OMAR should still have focus, but focus will be empty
        assert!(view.has_focus(INPUT_OMAR));
        assert_eq!(view.focus_stack.len(), 0);
        // Remount BAR
        view.mount(INPUT_BAR, Box::new(MockBarInput::default()));
        // Active BAR
        view.active(INPUT_BAR);
        // Blur
        view.blur();
        // Focus should be held by OMAR, but BAR should not be in stack
        assert!(view.has_focus(INPUT_OMAR));
        assert_eq!(view.focus_stack.len(), 0);
        assert!(view.mounted(INPUT_BAR));
    }

    #[test]
    fn view_should_forward_events() {
        let mut view: View<MockMsg, MockEvent> = View::default();
        view.mount(INPUT_FOO, Box::new(MockFooInput::default()));
        let ev: Event<MockEvent> = Event::Keyboard(KeyEvent::from(Key::Char('a')));
        assert_eq!(
            view.forward(INPUT_FOO, ev).unwrap(),
            MockMsg::FooInputChanged(String::from("a"))
        );
        // To non-existing component
        assert_eq!(view.forward(INPUT_BAR, Event::Tick), None);
    }

    #[test]
    fn view_should_read_and_write_attributes() {
        let mut view: View<MockMsg, MockEvent> = View::default();
        view.mount(INPUT_FOO, Box::new(MockFooInput::default()));
        assert_eq!(view.query(INPUT_FOO, Attribute::Focus), None);
        assert_eq!(view.query(INPUT_BAR, Attribute::Focus), None);
        view.attr(INPUT_FOO, Attribute::Focus, AttrValue::Flag(true));
        assert_eq!(
            view.query(INPUT_FOO, Attribute::Focus),
            Some(AttrValue::Flag(true))
        );
    }

    #[test]
    fn view_should_read_state() {
        let mut view: View<MockMsg, MockEvent> = View::default();
        view.mount(INPUT_FOO, Box::new(MockFooInput::default()));
        assert_eq!(
            view.state(INPUT_FOO).unwrap(),
            State::One(Value::String(String::from("")))
        );
        assert_eq!(view.state(INPUT_BAR), None);
    }
}
