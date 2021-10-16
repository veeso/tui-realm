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
use thiserror::Error;

/// ## WrappedComponent
///
/// A boxed component. Shorthand for View components map
pub(crate) type WrappedComponent<Msg, UserEvent> = Box<dyn Component<Msg, UserEvent>>;

/// ## ViewResult
///
/// Result for view methods.
/// Returns a variable Ok and a ViewError in case of error.
pub type ViewResult<T> = Result<T, ViewError>;

/// ## ViewError
///
/// An error returned by the view
#[derive(Debug, Error)]
pub enum ViewError {
    #[error("component already mounted")]
    ComponentAlreadyMounted,
    #[error("component not found")]
    ComponentNotFound,
    #[error("there's no component to blur")]
    NoComponentToBlur,
}

/// ## View
///
/// View is the wrapper and manager for all the components.
/// A View is a container for all the components in a certain layout.
/// Each View can have only one focused component at the time. At least one component must be always focused
pub struct View<Msg, UserEvent>
where
    Msg: PartialEq,
    UserEvent: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd,
{
    /// Components Mounted onto View
    components: HashMap<String, WrappedComponent<Msg, UserEvent>>,
    /// Current active component
    focus: Option<String>,
    /// Focus stack; used to determine which component should hold focus in case the current element is blurred
    focus_stack: Vec<String>,
}

impl<'a, Msg, UserEvent> Default for View<Msg, UserEvent>
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

impl<'a, Msg, UserEvent> View<Msg, UserEvent>
where
    Msg: PartialEq,
    UserEvent: std::fmt::Debug + Eq + PartialEq + Clone + PartialOrd,
{
    /// ### mount
    ///
    /// Mount component on View.
    /// Returns error if the component is already mounted
    pub fn mount<S>(&mut self, id: S, component: WrappedComponent<Msg, UserEvent>) -> ViewResult<()>
    where
        S: AsRef<str>,
    {
        if self.mounted(id.as_ref()) {
            Err(ViewError::ComponentAlreadyMounted)
        } else {
            self.components.insert(id.as_ref().to_string(), component);
            Ok(())
        }
    }

    /// ### umount
    ///
    /// Umount component from View
    pub fn umount<S>(&mut self, id: S) -> ViewResult<()>
    where
        S: AsRef<str>,
    {
        if !self.mounted(id.as_ref()) {
            return Err(ViewError::ComponentNotFound);
        }
        if self.has_focus(id.as_ref()) {
            let _ = self.blur();
        }
        // Remove component from stack
        self.pop_from_stack(id.as_ref());
        // Umount
        self.components.remove(id.as_ref());
        Ok(())
    }

    /// ### mounted
    ///
    /// Returns whether component `id` is mounted
    pub fn mounted<S>(&self, id: S) -> bool
    where
        S: AsRef<str>,
    {
        self.components.contains_key(id.as_ref())
    }

    /// ### focus
    ///
    /// Returns current active element (if any)
    pub(crate) fn focus(&self) -> Option<&str> {
        self.focus.as_deref()
    }

    /// ### component
    ///
    /// Returns reference to component associated to `id`
    pub(crate) fn component<S>(&self, id: S) -> Option<&dyn Component<Msg, UserEvent>>
    where
        S: AsRef<str>,
    {
        self.components.get(id.as_ref()).map(|x| x.as_ref())
    }

    /// ### view
    ///
    /// Render component called `id`
    pub fn view<S>(&mut self, id: S, f: &mut Frame, area: Rect)
    where
        S: AsRef<str>,
    {
        if let Some(c) = self.components.get_mut(id.as_ref()) {
            c.view(f, area);
        }
    }

    /// ### forward
    ///
    /// Forward `event` (call `on()`) on component `id` and return a `Msg` if any.
    /// Returns error if the component doesn't exist
    pub(crate) fn forward<S>(&mut self, id: S, event: Event<UserEvent>) -> ViewResult<Option<Msg>>
    where
        S: AsRef<str>,
    {
        match self.components.get_mut(id.as_ref()) {
            None => Err(ViewError::ComponentNotFound),
            Some(c) => Ok(c.on(event)),
        }
    }

    /// ### query
    ///
    /// Query view component for a certain `AttrValue`
    /// Returns error if the component doesn't exist
    /// Returns None if the attribute doesn't exist.
    pub fn query<S>(&self, id: S, query: Attribute) -> ViewResult<Option<AttrValue>>
    where
        S: AsRef<str>,
    {
        match self.components.get(id.as_ref()) {
            None => Err(ViewError::ComponentNotFound),
            Some(c) => Ok(c.query(query)),
        }
    }

    /// ### attr
    ///
    /// Set attribute for component `id`
    /// Returns error if the component doesn't exist
    pub fn attr<S>(&mut self, id: S, attr: Attribute, value: AttrValue) -> ViewResult<()>
    where
        S: AsRef<str>,
    {
        if let Some(c) = self.components.get_mut(id.as_ref()) {
            c.attr(attr, value);
            Ok(())
        } else {
            Err(ViewError::ComponentNotFound)
        }
    }

    /// ### state
    ///
    /// Get state for component `id`.
    /// Returns `Err` if component doesn't exist
    pub fn state<S>(&self, id: S) -> ViewResult<State>
    where
        S: AsRef<str>,
    {
        self.components
            .get(id.as_ref())
            .map(|c| c.state())
            .ok_or(ViewError::ComponentNotFound)
    }

    // -- shorthands

    /// ### active
    ///
    /// Shorthand for `attr(id, Attribute::Focus(AttrValue::Flag(true)))`.
    /// It also sets the component as the current one having focus.
    /// Previous active component, if any, GETS PUSHED to the STACK
    /// Returns error: if component doesn't exist. Use `mounted()` to check if component exists
    ///
    /// > NOTE: users should always use this function to give focus to components.
    pub fn active<S>(&mut self, id: S) -> ViewResult<()>
    where
        S: AsRef<str>,
    {
        if let Some(c) = self.components.get_mut(id.as_ref()) {
            // Set attribute
            c.attr(Attribute::Focus, AttrValue::Flag(true));
            // Move current focus
            self.change_focus(id.as_ref());
            Ok(())
        } else {
            Err(ViewError::ComponentNotFound)
        }
    }

    /// ### blur
    ///
    /// Blur selected element AND DON'T PUSH CURRENT ACTIVE ELEMENT INTO THE STACK
    /// Shorthand for `attr(id, Attribute::Focus(AttrValue::Flag(false)))`.
    /// It also unset the current focus and give it to the first element in stack.
    /// Returns error: if no component has focus
    ///
    /// > NOTE: users should always use this function to remove focus to components.
    pub fn blur(&mut self) -> ViewResult<()> {
        if let Some(id) = self.focus.take() {
            if let Some(c) = self.components.get_mut(id.as_str()) {
                c.attr(Attribute::Focus, AttrValue::Flag(false));
            }
            self.focus_to_last();
            Ok(())
        } else {
            Err(ViewError::NoComponentToBlur)
        }
    }

    // -- private

    /// ### push_to_stack
    ///
    /// Push component `id` to focus stack
    /// In case it is already in the focus stack,
    /// it will be first removed from it.
    fn push_to_stack<S>(&mut self, id: S)
    where
        S: AsRef<str>,
    {
        self.pop_from_stack(id.as_ref());
        self.focus_stack.push(id.as_ref().to_string());
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
        match self.focus.as_ref() {
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
        self.focus = Some(key.to_string());
    }

    /// ### focus_to_last
    ///
    /// Give focus to the last component in the stack
    fn focus_to_last(&mut self) {
        if let Some(focus) = self.take_last_from_stack() {
            let _ = self.active(focus);
        }
    }

    /// ### take_last_from_stack
    ///
    /// Take last element from stack if any
    fn take_last_from_stack(&mut self) -> Option<String> {
        self.focus_stack.pop()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{
        event::{Key, KeyEvent},
        mock::{MockBarInput, MockEvent, MockFooInput, MockMsg},
        StateValue,
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
        assert!(view
            .mount(INPUT_FOO, Box::new(MockFooInput::default()))
            .is_ok());
        assert_eq!(view.components.len(), 1);
        assert!(view.mounted(INPUT_FOO));
        assert!(view.component(INPUT_FOO).is_some());
        assert!(view.component(INPUT_BAR).is_none());
        assert_eq!(view.mounted(INPUT_BAR), false);
        // Mount bar
        assert!(view
            .mount(INPUT_BAR, Box::new(MockBarInput::default()))
            .is_ok());
        assert_eq!(view.components.len(), 2);
        assert!(view.mounted(INPUT_BAR));
        // Mount twice
        assert!(view
            .mount(INPUT_BAR, Box::new(MockBarInput::default()))
            .is_err());
        assert_eq!(view.components.len(), 2);
        assert!(view.mounted(INPUT_BAR));
        // Umount
        assert!(view.umount(INPUT_FOO).is_ok());
        assert_eq!(view.components.len(), 1);
        assert_eq!(view.mounted(INPUT_FOO), false);
        assert_eq!(view.mounted(INPUT_BAR), true);
        assert!(view.umount(INPUT_BAR).is_ok());
        assert_eq!(view.components.len(), 0);
        assert_eq!(view.mounted(INPUT_BAR), false);
        // Umount twice
        assert!(view.umount(INPUT_BAR).is_err());
    }

    #[test]
    fn view_should_compile_with_dynamic_names() {
        let mut view: View<MockMsg, MockEvent> = View::default();
        let names: Vec<String> = (0..10).map(|x| format!("INPUT_{}", x)).collect();
        names.iter().for_each(|x| {
            assert!(view.mount(x, Box::new(MockBarInput::default())).is_ok());
        });
        assert_eq!(view.components.len(), 10);
        names.iter().for_each(|x| assert!(view.mounted(x)));
    }

    #[test]
    fn view_should_handle_focus() {
        let mut view: View<MockMsg, MockEvent> = View::default();
        assert!(view
            .mount(INPUT_FOO, Box::new(MockFooInput::default()))
            .is_ok());
        assert!(view
            .mount(INPUT_BAR, Box::new(MockBarInput::default()))
            .is_ok());
        assert!(view
            .mount(INPUT_OMAR, Box::new(MockBarInput::default()))
            .is_ok());
        // Active foo
        assert!(view.active(INPUT_FOO).is_ok());
        assert_eq!(view.focus(), Some(INPUT_FOO));
        assert!(view.has_focus(INPUT_FOO));
        assert_eq!(view.focus.as_deref().unwrap(), INPUT_FOO);
        assert!(view.focus_stack.is_empty());
        // Give focus to BAR
        assert!(view.active(INPUT_BAR).is_ok());
        assert!(view.has_focus(INPUT_BAR));
        assert_eq!(view.focus_stack.len(), 1);
        // Give focus to OMAR
        assert!(view.active(INPUT_OMAR).is_ok());
        assert!(view.has_focus(INPUT_OMAR));
        assert_eq!(view.focus_stack.len(), 2);
        // Give focus back to FOO
        assert!(view.active(INPUT_FOO).is_ok());
        assert!(view.has_focus(INPUT_FOO));
        assert_eq!(view.focus_stack.len(), 2);
        // Umount FOO
        assert!(view.umount(INPUT_FOO).is_ok());
        // OMAR should have focus
        assert!(view.has_focus(INPUT_OMAR));
        assert_eq!(view.focus_stack.len(), 1);
        // Umount BAR
        assert!(view.umount(INPUT_BAR).is_ok());
        // Give focus to unexisting component
        assert!(view.active(INPUT_BAR).is_err());
        // OMAR should still have focus, but focus will be empty
        assert!(view.has_focus(INPUT_OMAR));
        assert_eq!(view.focus_stack.len(), 0);
        // Remount BAR
        assert!(view
            .mount(INPUT_BAR, Box::new(MockBarInput::default()))
            .is_ok());
        // Active BAR
        assert!(view.active(INPUT_BAR).is_ok());
        // Blur
        assert!(view.blur().is_ok());
        // Focus should be held by OMAR, but BAR should not be in stack
        assert!(view.has_focus(INPUT_OMAR));
        assert_eq!(view.focus_stack.len(), 0);
        assert!(view.mounted(INPUT_BAR));
        // Blur again
        assert!(view.blur().is_ok());
        // None has focus
        assert!(view.blur().is_err());
    }

    #[test]
    fn view_should_forward_events() {
        let mut view: View<MockMsg, MockEvent> = View::default();
        assert!(view
            .mount(INPUT_FOO, Box::new(MockFooInput::default()))
            .is_ok());
        let ev: Event<MockEvent> = Event::Keyboard(KeyEvent::from(Key::Char('a')));
        assert_eq!(
            view.forward(INPUT_FOO, ev).ok().unwrap().unwrap(),
            MockMsg::FooInputChanged(String::from("a"))
        );
        // To non-existing component
        assert!(view.forward(INPUT_BAR, Event::Tick).is_err());
    }

    #[test]
    fn view_should_read_and_write_attributes() {
        let mut view: View<MockMsg, MockEvent> = View::default();
        assert!(view
            .mount(INPUT_FOO, Box::new(MockFooInput::default()))
            .is_ok());
        assert_eq!(view.query(INPUT_FOO, Attribute::Focus).ok().unwrap(), None);
        assert!(view.query(INPUT_BAR, Attribute::Focus).is_err());
        assert!(view
            .attr(INPUT_FOO, Attribute::Focus, AttrValue::Flag(true))
            .is_ok());
        assert_eq!(
            view.query(INPUT_FOO, Attribute::Focus).ok().unwrap(),
            Some(AttrValue::Flag(true))
        );
    }

    #[test]
    fn view_should_read_state() {
        let mut view: View<MockMsg, MockEvent> = View::default();
        assert!(view
            .mount(INPUT_FOO, Box::new(MockFooInput::default()))
            .is_ok());
        assert_eq!(
            view.state(INPUT_FOO).unwrap(),
            State::One(StateValue::String(String::from("")))
        );
        assert!(view.state(INPUT_BAR).is_err());
    }
}
