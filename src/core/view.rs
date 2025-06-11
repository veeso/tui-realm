//! ## View
//!
//! This module exposes the View structure, which is the wrapper for all the components in an application.

// -- ext
use std::collections::HashMap;
use std::hash::Hash;

use ratatui::Frame;
use thiserror::Error;

use crate::ratatui::layout::Rect;
use crate::{AttrValue, Attribute, Component, Event, Injector, State};

/// A boxed component. Shorthand for View components map
pub(crate) type WrappedComponent<Msg, UserEvent> = Box<dyn Component<Msg, UserEvent>>;

/// Result for view methods.
/// Returns a variable Ok and a ViewError in case of error.
pub type ViewResult<T> = Result<T, ViewError>;

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

/// View is the wrapper and manager for all the components.
/// A View is a container for all the components in a certain layout.
/// Each View can have only one focused component at the time. At least one component must be always focused
pub struct View<ComponentId, Msg, UserEvent>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone,
{
    /// Components Mounted onto View
    components: HashMap<ComponentId, WrappedComponent<Msg, UserEvent>>,
    /// Current active component
    focus: Option<ComponentId>,
    /// Focus stack; used to determine which component should hold focus in case the current element is blurred
    focus_stack: Vec<ComponentId>,
    /// Property injectors
    injectors: Vec<Box<dyn Injector<ComponentId>>>,
}

impl<K, Msg, UserEvent> Default for View<K, Msg, UserEvent>
where
    K: Eq + PartialEq + Clone + Hash,
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone,
{
    fn default() -> Self {
        Self {
            components: HashMap::new(),
            focus: None,
            focus_stack: Vec::new(),
            injectors: Vec::new(),
        }
    }
}

impl<K, Msg, UserEvent> View<K, Msg, UserEvent>
where
    K: Eq + PartialEq + Clone + Hash,
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone,
{
    /// Mount component on View.
    /// Returns error if the component is already mounted
    pub fn mount(&mut self, id: &K, component: WrappedComponent<Msg, UserEvent>) -> ViewResult<()> {
        if self.mounted(id) {
            Err(ViewError::ComponentAlreadyMounted)
        } else {
            // Insert
            self.components.insert(id.clone(), component);
            // Inject properties
            self.inject(id)
        }
    }

    /// Umount component from View
    pub fn umount(&mut self, id: &K) -> ViewResult<()> {
        if !self.mounted(id) {
            return Err(ViewError::ComponentNotFound);
        }
        if self.has_focus(id) {
            let _ = self.blur();
        }
        // Remove component from stack
        self.pop_from_stack(id);
        // Umount
        self.components.remove(id);
        Ok(())
    }

    /// Remount component. This method WON'T change the focus stack
    pub fn remount(
        &mut self,
        id: &K,
        component: WrappedComponent<Msg, UserEvent>,
    ) -> ViewResult<()> {
        // Umount, but keep focus
        let had_focus = self.has_focus(id);
        if self.mounted(id) {
            self.components.remove(id);
        }
        // remount
        self.components.insert(id.clone(), component);
        // Inject properties
        self.inject(id)?;
        // give focus if needed
        if had_focus { self.active(id) } else { Ok(()) }
    }

    /// Umount all components in the view and clear focus stack and state
    pub fn umount_all(&mut self) {
        self.components.clear();
        self.focus_stack.clear();
        self.focus = None;
    }

    /// Returns whether component `id` is mounted
    pub fn mounted(&self, id: &K) -> bool {
        self.components.contains_key(id)
    }

    /// Returns current active element (if any)
    pub(crate) fn focus(&self) -> Option<&K> {
        self.focus.as_ref()
    }

    /// Render component called `id`
    pub fn view(&mut self, id: &K, f: &mut Frame, area: Rect) {
        if let Some(c) = self.components.get_mut(id) {
            c.view(f, area);
        }
    }

    /// Forward `event` (call `on()`) on component `id` and return a `Msg` if any.
    /// Returns error if the component doesn't exist
    pub(crate) fn forward(&mut self, id: &K, event: Event<UserEvent>) -> ViewResult<Option<Msg>> {
        match self.components.get_mut(id) {
            None => Err(ViewError::ComponentNotFound),
            Some(c) => Ok(c.on(event)),
        }
    }

    /// Query view component for a certain `AttrValue`
    /// Returns error if the component doesn't exist
    /// Returns None if the attribute doesn't exist.
    pub fn query(&self, id: &K, query: Attribute) -> ViewResult<Option<AttrValue>> {
        match self.components.get(id) {
            None => Err(ViewError::ComponentNotFound),
            Some(c) => Ok(c.query(query)),
        }
    }

    /// Set attribute for component `id`
    /// Returns error if the component doesn't exist
    pub fn attr(&mut self, id: &K, attr: Attribute, value: AttrValue) -> ViewResult<()> {
        if let Some(c) = self.components.get_mut(id) {
            c.attr(attr, value);
            Ok(())
        } else {
            Err(ViewError::ComponentNotFound)
        }
    }

    /// Get state for component `id`.
    /// Returns `Err` if component doesn't exist
    pub fn state(&self, id: &K) -> ViewResult<State> {
        self.components
            .get(id)
            .map(|c| c.state())
            .ok_or(ViewError::ComponentNotFound)
    }

    // -- shorthands

    /// Shorthand for `attr(id, Attribute::Focus(AttrValue::Flag(true)))`.
    /// It also sets the component as the current one having focus.
    /// Previous active component, if any, GETS PUSHED to the STACK
    /// Returns error: if component doesn't exist. Use `mounted()` to check if component exists
    ///
    /// > NOTE: users should always use this function to give focus to components.
    pub fn active(&mut self, id: &K) -> ViewResult<()> {
        self.set_focus(id, true)?;
        self.change_focus(id);
        Ok(())
    }

    /// Blur selected element AND DON'T PUSH CURRENT ACTIVE ELEMENT INTO THE STACK
    /// Shorthand for `attr(id, Attribute::Focus(AttrValue::Flag(false)))`.
    /// It also unset the current focus and give it to the first element in stack.
    /// Returns error: if no component has focus
    ///
    /// > NOTE: users should always use this function to remove focus to components.
    pub fn blur(&mut self) -> ViewResult<()> {
        if let Some(id) = self.focus.take() {
            self.set_focus(&id, false)?;
            self.focus_to_last();
            Ok(())
        } else {
            Err(ViewError::NoComponentToBlur)
        }
    }

    // -- injectors

    /// Add an injector to the view
    pub fn add_injector(&mut self, injector: Box<dyn Injector<K>>) {
        self.injectors.push(injector);
    }

    // -- private

    /// Push component `id` to focus stack
    /// In case it is already in the focus stack,
    /// it will be first removed from it.
    fn push_to_stack(&mut self, id: K) {
        self.pop_from_stack(&id);
        self.focus_stack.push(id);
    }

    /// Pop component `id` from focus stack
    fn pop_from_stack(&mut self, id: &K) {
        self.focus_stack.retain(|x| x != id);
    }

    /// Returns whether `who` has focus
    pub(crate) fn has_focus(&self, who: &K) -> bool {
        match self.focus.as_ref() {
            None => false,
            Some(id) => who == id,
        }
    }

    /// If focus is `Some`, move it to the top of the stack and set it to `None`.
    /// Then pop from stack `new_focus` and set it to current `focus`.
    ///
    /// > Panics if `new_focus` doesn't exist in components
    fn change_focus(&mut self, new_focus: &K) {
        if let Some(focus) = self.focus.take() {
            // Remove focus (can't return error)
            let _ = self.set_focus(&focus, false);
            // Push to stack
            self.push_to_stack(focus);
        }
        self.pop_from_stack(new_focus);
        // Get key from focus_stack (otherwise lifetime won't be valid)
        let key = self.components.keys().find(|x| *x == new_focus).unwrap();
        self.focus = Some(key.clone());
    }

    /// Give focus to the last component in the stack
    fn focus_to_last(&mut self) {
        if let Some(focus) = self.take_last_from_stack() {
            let _ = self.active(&focus);
        }
    }

    /// Take last element from stack if any
    fn take_last_from_stack(&mut self) -> Option<K> {
        self.focus_stack.pop()
    }

    /// Set focus value for component
    fn set_focus(&mut self, id: &K, value: bool) -> ViewResult<()> {
        if let Some(c) = self.components.get_mut(id) {
            c.attr(Attribute::Focus, AttrValue::Flag(value));
            Ok(())
        } else {
            Err(ViewError::ComponentNotFound)
        }
    }

    /// Inject properties for `id` using view injectors
    fn inject(&mut self, id: &K) -> ViewResult<()> {
        for (attr, value) in self.properties_to_inject(id) {
            self.attr(id, attr, value)?;
        }
        Ok(())
    }

    /// Collect properties to inject for component `K`
    fn properties_to_inject(&self, id: &K) -> Vec<(Attribute, AttrValue)> {
        self.injectors.iter().flat_map(|x| x.inject(id)).collect()
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::StateValue;
    use crate::event::{Key, KeyEvent};
    use crate::mock::{
        MockBarInput, MockComponentId, MockEvent, MockFooInput, MockInjector, MockMsg,
    };

    #[test]
    fn default_view_should_be_empty() {
        let view: View<MockComponentId, MockMsg, MockEvent> = View::default();
        assert!(view.components.is_empty());
        assert_eq!(view.focus, None);
        assert!(view.focus_stack.is_empty());
    }

    #[test]
    fn view_should_mount_and_umount_components() {
        let mut view: View<MockComponentId, MockMsg, MockEvent> = View::default();
        // Mount foo
        assert!(
            view.mount(
                &MockComponentId::InputFoo,
                Box::new(MockFooInput::default())
            )
            .is_ok()
        );
        assert_eq!(view.components.len(), 1);
        assert!(view.mounted(&MockComponentId::InputFoo));
        assert_eq!(view.mounted(&MockComponentId::InputBar), false);
        // Mount bar
        assert!(
            view.mount(
                &MockComponentId::InputBar,
                Box::new(MockBarInput::default())
            )
            .is_ok()
        );
        assert_eq!(view.components.len(), 2);
        assert!(view.mounted(&MockComponentId::InputBar));
        // Mount twice
        assert!(
            view.mount(
                &MockComponentId::InputBar,
                Box::new(MockBarInput::default())
            )
            .is_err()
        );
        assert_eq!(view.components.len(), 2);
        assert!(view.mounted(&MockComponentId::InputBar));
        // Umount
        assert!(view.umount(&MockComponentId::InputFoo).is_ok());
        assert_eq!(view.components.len(), 1);
        assert_eq!(view.mounted(&MockComponentId::InputFoo), false);
        assert_eq!(view.mounted(&MockComponentId::InputBar), true);
        assert!(view.umount(&MockComponentId::InputBar).is_ok());
        assert_eq!(view.components.len(), 0);
        assert_eq!(view.mounted(&MockComponentId::InputBar), false);
        // Umount twice
        assert!(view.umount(&MockComponentId::InputBar).is_err());
    }

    #[test]
    fn view_should_remount_component_without_losing_focus_stack() {
        let mut view: View<MockComponentId, MockMsg, MockEvent> = View::default();
        // Mount foo
        assert!(
            view.mount(
                &MockComponentId::InputFoo,
                Box::new(MockFooInput::default())
            )
            .is_ok()
        );
        assert!(view.active(&MockComponentId::InputFoo).is_ok());
        // mount another component
        assert!(
            view.mount(
                &MockComponentId::InputBar,
                Box::new(MockBarInput::default())
            )
            .is_ok()
        );
        assert!(view.active(&MockComponentId::InputBar).is_ok());
        // Remount foo
        assert!(
            view.remount(
                &MockComponentId::InputFoo,
                Box::new(MockFooInput::default())
            )
            .is_ok()
        );
        // Blur bar
        assert!(view.blur().is_ok());
        // Foo MUST have focus now
        assert!(view.has_focus(&MockComponentId::InputFoo));
    }

    #[test]
    fn view_should_umount_all() {
        let mut view: View<MockComponentId, MockMsg, MockEvent> = View::default();
        // Mount foo
        assert!(
            view.mount(
                &MockComponentId::InputFoo,
                Box::new(MockFooInput::default())
            )
            .is_ok()
        );
        assert_eq!(view.components.len(), 1);
        assert!(view.mounted(&MockComponentId::InputFoo));
        assert_eq!(view.mounted(&MockComponentId::InputBar), false);
        // Mount bar
        assert!(
            view.mount(
                &MockComponentId::InputBar,
                Box::new(MockBarInput::default())
            )
            .is_ok()
        );
        assert_eq!(view.components.len(), 2);
        assert!(view.mounted(&MockComponentId::InputBar));
        // Mount twice
        assert!(
            view.mount(
                &MockComponentId::InputBar,
                Box::new(MockBarInput::default())
            )
            .is_err()
        );
        assert_eq!(view.components.len(), 2);
        // Give focus
        assert!(view.active(&MockComponentId::InputFoo).is_ok());
        assert!(view.active(&MockComponentId::InputBar).is_ok());
        // Umount all
        view.umount_all();
        assert!(view.components.is_empty());
        assert!(view.focus_stack.is_empty());
        assert!(view.focus.is_none());
    }

    #[test]
    fn view_should_compile_with_dynamic_names() {
        let mut view: View<MockComponentId, MockMsg, MockEvent> = View::default();
        let names: Vec<MockComponentId> = (0..10)
            .map(|x| MockComponentId::Dyn(format!("INPUT_{}", x)))
            .collect();
        names.iter().for_each(|x| {
            assert!(view.mount(x, Box::new(MockBarInput::default())).is_ok());
        });
        assert_eq!(view.components.len(), 10);
        names.iter().for_each(|x| assert!(view.mounted(x)));
    }

    #[test]
    fn view_should_handle_focus() {
        let mut view: View<MockComponentId, MockMsg, MockEvent> = View::default();
        assert!(
            view.mount(
                &MockComponentId::InputFoo,
                Box::new(MockFooInput::default())
            )
            .is_ok()
        );
        assert!(
            view.mount(
                &MockComponentId::InputBar,
                Box::new(MockBarInput::default())
            )
            .is_ok()
        );
        assert!(
            view.mount(
                &MockComponentId::InputOmar,
                Box::new(MockBarInput::default())
            )
            .is_ok()
        );
        // Active foo
        assert!(view.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(view.focus(), Some(&MockComponentId::InputFoo));
        assert!(view.has_focus(&MockComponentId::InputFoo));
        assert_eq!(
            view.query(&MockComponentId::InputFoo, Attribute::Focus)
                .ok()
                .unwrap()
                .unwrap(),
            AttrValue::Flag(true)
        );
        assert_eq!(view.focus.to_owned().unwrap(), MockComponentId::InputFoo);
        assert!(view.focus_stack.is_empty());
        // Give focus to BAR
        assert!(view.active(&MockComponentId::InputBar).is_ok());
        assert_eq!(
            view.query(&MockComponentId::InputBar, Attribute::Focus)
                .ok()
                .unwrap()
                .unwrap(),
            AttrValue::Flag(true)
        );
        assert_eq!(
            view.query(&MockComponentId::InputFoo, Attribute::Focus)
                .ok()
                .unwrap()
                .unwrap(),
            AttrValue::Flag(false)
        );
        assert!(view.has_focus(&MockComponentId::InputBar));
        assert_eq!(view.focus_stack.len(), 1);
        // Give focus to OMAR
        assert!(view.active(&MockComponentId::InputOmar).is_ok());
        assert!(view.has_focus(&MockComponentId::InputOmar));
        assert_eq!(view.focus_stack.len(), 2);
        // Give focus back to FOO
        assert!(view.active(&MockComponentId::InputFoo).is_ok());
        assert!(view.has_focus(&MockComponentId::InputFoo));
        assert_eq!(view.focus_stack.len(), 2);
        // Umount FOO
        assert!(view.umount(&MockComponentId::InputFoo).is_ok());
        // OMAR should have focus
        assert!(view.has_focus(&MockComponentId::InputOmar));
        assert_eq!(view.focus_stack.len(), 1);
        // Umount BAR
        assert!(view.umount(&MockComponentId::InputBar).is_ok());
        // Give focus to unexisting component
        assert!(view.active(&MockComponentId::InputBar).is_err());
        // OMAR should still have focus, but focus will be empty
        assert!(view.has_focus(&MockComponentId::InputOmar));
        assert_eq!(view.focus_stack.len(), 0);
        // Remount BAR
        assert!(
            view.mount(
                &MockComponentId::InputBar,
                Box::new(MockBarInput::default())
            )
            .is_ok()
        );
        // Active BAR
        assert!(view.active(&MockComponentId::InputBar).is_ok());
        // Blur
        assert!(view.blur().is_ok());
        // Focus should be held by OMAR, but BAR should not be in stack
        assert!(view.has_focus(&MockComponentId::InputOmar));
        assert_eq!(view.focus_stack.len(), 0);
        assert!(view.mounted(&MockComponentId::InputBar));
        // Blur again
        assert!(view.blur().is_ok());
        // None has focus
        assert!(view.blur().is_err());
    }

    #[test]
    fn view_should_forward_events() {
        let mut view: View<MockComponentId, MockMsg, MockEvent> = View::default();
        assert!(
            view.mount(
                &MockComponentId::InputFoo,
                Box::new(MockFooInput::default())
            )
            .is_ok()
        );
        let ev: Event<MockEvent> = Event::Keyboard(KeyEvent::from(Key::Char('a')));
        assert_eq!(
            view.forward(&MockComponentId::InputFoo, ev)
                .ok()
                .unwrap()
                .unwrap(),
            MockMsg::FooInputChanged(String::from("a"))
        );
        // To non-existing component
        assert!(
            view.forward(&MockComponentId::InputBar, Event::Tick)
                .is_err()
        );
    }

    #[test]
    fn view_should_read_and_write_attributes() {
        let mut view: View<MockComponentId, MockMsg, MockEvent> = View::default();
        assert!(
            view.mount(
                &MockComponentId::InputFoo,
                Box::new(MockFooInput::default())
            )
            .is_ok()
        );
        assert_eq!(
            view.query(&MockComponentId::InputFoo, Attribute::Focus)
                .ok()
                .unwrap(),
            None
        );
        assert!(
            view.query(&MockComponentId::InputBar, Attribute::Focus)
                .is_err()
        );
        assert!(
            view.attr(
                &MockComponentId::InputFoo,
                Attribute::Focus,
                AttrValue::Flag(true)
            )
            .is_ok()
        );
        assert_eq!(
            view.query(&MockComponentId::InputFoo, Attribute::Focus)
                .ok()
                .unwrap(),
            Some(AttrValue::Flag(true))
        );
    }

    #[test]
    fn view_should_read_state() {
        let mut view: View<MockComponentId, MockMsg, MockEvent> = View::default();
        assert!(
            view.mount(
                &MockComponentId::InputFoo,
                Box::new(MockFooInput::default())
            )
            .is_ok()
        );
        assert_eq!(
            view.state(&MockComponentId::InputFoo).unwrap(),
            State::One(StateValue::String(String::new()))
        );
        assert!(view.state(&MockComponentId::InputBar).is_err());
    }

    #[test]
    fn view_should_inject_properties() {
        let mut view: View<MockComponentId, MockMsg, MockEvent> = View::default();
        view.add_injector(Box::new(MockInjector));
        assert!(
            view.mount(
                &MockComponentId::InputBar,
                Box::new(MockBarInput::default())
            )
            .is_ok()
        );
        // Check if property has been injected
        assert_eq!(
            view.query(&MockComponentId::InputBar, Attribute::Text)
                .ok()
                .unwrap()
                .unwrap(),
            AttrValue::String(String::from("hello, world!"))
        );
    }
}
