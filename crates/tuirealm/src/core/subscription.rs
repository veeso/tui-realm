//! # Subscription
//!
//! This module defines the model for the Subscriptions

use crate::event::KeyEvent;
use crate::{AttrValue, Attribute, Event, State};

use std::hash::Hash;

/// Public type to define a subscription.
pub struct Sub<ComponentId, UserEvent>(EventClause<UserEvent>, SubClause<ComponentId>)
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    UserEvent: Eq + PartialEq + Clone + PartialOrd;

impl<K, U> Sub<K, U>
where
    K: Eq + PartialEq + Clone + Hash,
    U: Eq + PartialEq + Clone + PartialOrd,
{
    /// Creates a new `Sub`
    pub fn new(event_clause: EventClause<U>, sub_clause: SubClause<K>) -> Self {
        Self(event_clause, sub_clause)
    }
}

/// Defines a subscription for a component.
/// A subscription tells the application to forward an event to the `target` component, when an event of type `ev`
/// is received by the listener. In order to forward the event, the `where` clause must also be satisfied.
///
/// > NOTE: Remember that "Component has focus" is NOT a subscription. Events are ALWAYS FORWARDED to components that have
/// > FOCUS
///
/// A subscription is defined by 3 attributes:
///     - target: the id of the target component
///     - ev: the event it listens for
///     - when: a clause that must be satisfied to forward the event to the component.
///
///
pub(crate) struct Subscription<ComponentId, UserEvent>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    UserEvent: Eq + PartialEq + Clone + PartialOrd,
{
    /// Target component
    target: ComponentId,
    /// Event to forward and listen to
    ev: EventClause<UserEvent>,
    /// Restrict forwarding clauses
    when: SubClause<ComponentId>,
}

impl<K, U> Subscription<K, U>
where
    K: Eq + PartialEq + Clone + Hash,
    U: Eq + PartialEq + Clone + PartialOrd + Send,
{
    /// Instantiates a new `Subscription`
    pub fn new(target: K, sub: Sub<K, U>) -> Self {
        Self {
            target,
            ev: sub.0,
            when: sub.1,
        }
    }

    /// Returns sub target
    pub(crate) fn target(&self) -> &K {
        &self.target
    }

    /// Returns reference to subscription event clause
    pub(crate) fn event(&self) -> &EventClause<U> {
        &self.ev
    }

    /// Returns whether to forward event to component
    pub(crate) fn forward<HasAttrFn, GetStateFn, MountedFn>(
        &self,
        ev: &Event<U>,
        has_attr_fn: HasAttrFn,
        get_state_fn: GetStateFn,
        mounted_fn: MountedFn,
    ) -> bool
    where
        HasAttrFn: Fn(&K, Attribute) -> Option<AttrValue>,
        GetStateFn: Fn(&K) -> Option<State>,
        MountedFn: Fn(&K) -> bool,
    {
        self.ev.forward(ev) && self.when.forward(has_attr_fn, get_state_fn, mounted_fn)
    }
}

#[derive(Debug, PartialEq)]

/// An event clause indicates on which kind of event the event must be forwarded to the `target` component.
pub enum EventClause<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone + PartialOrd,
{
    /// Forward, no matter what kind of event
    Any,
    /// Check whether a certain key has been pressed
    Keyboard(KeyEvent),
    /// Check whether window has been resized
    WindowResize,
    /// The event will be forwarded on a tick
    Tick,
    /// Event will be forwarded on this specific user event.
    /// The way user event is matched, depends on its partialEq implementation
    User(UserEvent),
}

impl<U> EventClause<U>
where
    U: Eq + PartialEq + Clone + PartialOrd,
{
    /// Check whether to forward based on even type and event clause.
    ///
    /// This is how events are forwarded:
    ///
    /// - Any: Forward, no matter what kind of event
    /// - Keyboard: everything must match
    /// - WindowResize: matches only event type, not sizes
    /// - Tick: matches tick event
    /// - None: matches None event
    /// - UserEvent: depends on UserEvent PartialEq
    fn forward(&self, ev: &Event<U>) -> bool {
        match self {
            EventClause::Any => true,
            EventClause::Keyboard(k) => Some(k) == ev.is_keyboard(),
            EventClause::WindowResize => ev.is_window_resize(),
            EventClause::Tick => ev.is_tick(),
            EventClause::User(u) => Some(u) == ev.is_user(),
        }
    }
}

/// A subclause indicates the condition that must be satisfied in order to forward `ev` to `target`.
/// Usually clauses are single conditions, but there are also some special condition, to create "ligatures", which are:
///
/// - `Not(SubClause)`: Negates inner condition
/// - `And(SubClause, SubClause)`: the AND of the two clauses must be `true`
/// - `Or(SubClause, SubClause)`: the OR of the two clauses must be `true`
#[derive(Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum SubClause<Id>
where
    Id: Eq + PartialEq + Clone + Hash,
{
    /// Always forward event to component
    Always,
    /// Forward event if target component has provided attribute with the provided value
    /// If the attribute doesn't exist on component, result is always `false`.
    HasAttrValue(Id, Attribute, AttrValue),
    /// Forward event if target component has provided state
    HasState(Id, State),
    /// Forward event if target component is mounted
    IsMounted(Id),
    /// Forward event if the inner clause is `false`
    Not(Box<SubClause<Id>>),
    /// Forward event if both the inner clauses are `true`
    And(Box<SubClause<Id>>, Box<SubClause<Id>>),
    /// Forward event if at least one of the inner clauses is `true`
    Or(Box<SubClause<Id>>, Box<SubClause<Id>>),
}

impl<Id> SubClause<Id>
where
    Id: Eq + PartialEq + Clone + Hash,
{
    /// Shortcut for `SubClause::Not` without specifying `Box::new(...)`
    #[allow(clippy::should_implement_trait)]
    pub fn not(clause: Self) -> Self {
        Self::Not(Box::new(clause))
    }

    /// Shortcut for `SubClause::And` without specifying `Box::new(...)`
    pub fn and(a: Self, b: Self) -> Self {
        Self::And(Box::new(a), Box::new(b))
    }

    /// Shortcut for `SubClause::Or` without specifying `Box::new(...)`
    pub fn or(a: Self, b: Self) -> Self {
        Self::Or(Box::new(a), Box::new(b))
    }

    /// Returns whether the subscription clause is satisfied
    pub(crate) fn forward<HasAttrFn, GetStateFn, MountedFn>(
        &self,
        has_attr_fn: HasAttrFn,
        get_state_fn: GetStateFn,
        mounted_fn: MountedFn,
    ) -> bool
    where
        HasAttrFn: Fn(&Id, Attribute) -> Option<AttrValue>,
        GetStateFn: Fn(&Id) -> Option<State>,
        MountedFn: Fn(&Id) -> bool,
    {
        self.check_forwarding(has_attr_fn, get_state_fn, mounted_fn)
            .0
    }

    fn check_forwarding<HasAttrFn, GetStateFn, MountedFn>(
        &self,
        has_attr_fn: HasAttrFn,
        get_state_fn: GetStateFn,
        mounted_fn: MountedFn,
    ) -> (bool, HasAttrFn, GetStateFn, MountedFn)
    where
        HasAttrFn: Fn(&Id, Attribute) -> Option<AttrValue>,
        GetStateFn: Fn(&Id) -> Option<State>,
        MountedFn: Fn(&Id) -> bool,
    {
        match self {
            Self::Always => (true, has_attr_fn, get_state_fn, mounted_fn),
            Self::HasAttrValue(id, query, value) => {
                let (fwd, has_attr_fn) = Self::has_attribute(id, query, value, has_attr_fn);
                (fwd, has_attr_fn, get_state_fn, mounted_fn)
            }
            Self::HasState(id, state) => {
                let (fwd, get_state_fn) = Self::has_state(id, state, get_state_fn);
                (fwd, has_attr_fn, get_state_fn, mounted_fn)
            }
            Self::IsMounted(id) => {
                let (fwd, mounted_fn) = Self::is_mounted(id, mounted_fn);
                (fwd, has_attr_fn, get_state_fn, mounted_fn)
            }
            Self::Not(clause) => {
                let (fwd, has_attr_fn, get_state_fn, mounted_fn) =
                    clause.check_forwarding(has_attr_fn, get_state_fn, mounted_fn);
                (!fwd, has_attr_fn, get_state_fn, mounted_fn)
            }
            Self::And(a, b) => {
                let (fwd_a, has_attr_fn, get_state_fn, mounted_fn) =
                    a.check_forwarding(has_attr_fn, get_state_fn, mounted_fn);
                let (fwd_b, has_attr_fn, get_state_fn, mounted_fn) =
                    b.check_forwarding(has_attr_fn, get_state_fn, mounted_fn);
                (fwd_a && fwd_b, has_attr_fn, get_state_fn, mounted_fn)
            }
            Self::Or(a, b) => {
                let (fwd_a, has_attr_fn, get_state_fn, mounted_fn) =
                    a.check_forwarding(has_attr_fn, get_state_fn, mounted_fn);
                let (fwd_b, has_attr_fn, get_state_fn, mounted_fn) =
                    b.check_forwarding(has_attr_fn, get_state_fn, mounted_fn);
                (fwd_a || fwd_b, has_attr_fn, get_state_fn, mounted_fn)
            }
        }
    }

    // -- privates

    fn has_attribute<HasAttrFn>(
        id: &Id,
        query: &Attribute,
        value: &AttrValue,
        has_attr_fn: HasAttrFn,
    ) -> (bool, HasAttrFn)
    where
        HasAttrFn: Fn(&Id, Attribute) -> Option<AttrValue>,
    {
        (
            match has_attr_fn(id, *query) {
                None => false,
                Some(v) => *value == v,
            },
            has_attr_fn,
        )
    }

    fn has_state<GetStateFn>(id: &Id, state: &State, get_state_fn: GetStateFn) -> (bool, GetStateFn)
    where
        GetStateFn: Fn(&Id) -> Option<State>,
    {
        (
            match get_state_fn(id) {
                Some(s) => s == *state,
                None => false,
            },
            get_state_fn,
        )
    }

    fn is_mounted<MountedFn>(id: &Id, mounted_fn: MountedFn) -> (bool, MountedFn)
    where
        MountedFn: Fn(&Id) -> bool,
    {
        (mounted_fn(id), mounted_fn)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::event::Key;
    use crate::mock::{MockComponentId, MockEvent, MockFooInput};
    use crate::{command::Cmd, MockComponent, StateValue};

    use pretty_assertions::assert_eq;

    #[test]
    fn subscription_should_forward() {
        let ev: Event<MockEvent> = Event::WindowResize(1024, 512);
        let mut component = MockFooInput::default();
        component.attr(Attribute::Focus, AttrValue::Flag(true));
        let sub = Subscription::new(
            MockComponentId::InputFoo,
            Sub(
                EventClause::WindowResize,
                SubClause::HasAttrValue(
                    MockComponentId::InputBar,
                    Attribute::Focus,
                    AttrValue::Flag(true),
                ),
            ),
        );
        assert_eq!(sub.target(), &MockComponentId::InputFoo);
        assert_eq!(sub.event(), &EventClause::<MockEvent>::WindowResize);
        assert_eq!(
            sub.when,
            SubClause::HasAttrValue(
                MockComponentId::InputBar,
                Attribute::Focus,
                AttrValue::Flag(true)
            )
        );
        assert_eq!(
            sub.forward(
                &ev,
                |_: &MockComponentId, q| component.query(q),
                |_: &MockComponentId| Some(component.state()),
                |_: &MockComponentId| true
            ),
            true
        );
        // False clause
        component.attr(Attribute::Focus, AttrValue::Flag(false));
        assert_eq!(
            sub.forward(
                &ev,
                |_: &MockComponentId, q| component.query(q),
                |_: &MockComponentId| Some(component.state()),
                |_: &MockComponentId| true
            ),
            false
        );
        // False event
        assert_eq!(
            sub.forward(
                &Event::User(MockEvent::Foo),
                |_: &MockComponentId, q| component.query(q),
                |_: &MockComponentId| Some(component.state()),
                |_: &MockComponentId| true
            ),
            false
        );
        // False id
        assert_eq!(
            sub.forward(
                &Event::WindowResize(0, 0),
                |_: &MockComponentId, q| component.query(q),
                |_: &MockComponentId| Some(component.state()),
                |_: &MockComponentId| true
            ),
            false
        );
    }

    #[test]
    fn event_clause_any_should_forward() {
        assert!(EventClause::<MockEvent>::Any.forward(&Event::Tick));
    }

    #[test]
    fn event_clause_keyboard_should_forward() {
        assert_eq!(
            EventClause::<MockEvent>::Keyboard(KeyEvent::from(Key::Enter))
                .forward(&Event::Keyboard(KeyEvent::from(Key::Enter))),
            true
        );
        assert_eq!(
            EventClause::<MockEvent>::Keyboard(KeyEvent::from(Key::Enter))
                .forward(&Event::Keyboard(KeyEvent::from(Key::Backspace))),
            false
        );
        assert_eq!(
            EventClause::<MockEvent>::Keyboard(KeyEvent::from(Key::Enter)).forward(&Event::Tick),
            false
        );
    }

    #[test]
    fn event_clause_window_resize_should_forward() {
        assert_eq!(
            EventClause::<MockEvent>::WindowResize.forward(&Event::WindowResize(0, 0)),
            true
        );
        assert_eq!(
            EventClause::<MockEvent>::WindowResize.forward(&Event::Tick),
            false
        );
    }

    #[test]
    fn event_clause_tick_should_forward() {
        assert_eq!(EventClause::<MockEvent>::Tick.forward(&Event::Tick), true);
        assert_eq!(
            EventClause::<MockEvent>::Tick.forward(&Event::WindowResize(0, 0)),
            false
        );
    }

    #[test]
    fn event_clause_user_should_forward() {
        assert_eq!(
            EventClause::<MockEvent>::User(MockEvent::Foo).forward(&Event::User(MockEvent::Foo)),
            true
        );
        assert_eq!(
            EventClause::<MockEvent>::User(MockEvent::Foo).forward(&Event::Tick),
            false
        );
    }

    #[test]
    fn clause_always_should_forward() {
        let component = MockFooInput::default();
        let clause = SubClause::Always;
        assert_eq!(
            clause.forward(
                |_: &MockComponentId, q| component.query(q),
                |_: &MockComponentId| Some(component.state()),
                |_: &MockComponentId| true
            ),
            true
        );
    }

    #[test]
    fn clause_has_attribute_should_forward() {
        let mut component = MockFooInput::default();
        let clause = SubClause::HasAttrValue(
            MockComponentId::InputBar,
            Attribute::Focus,
            AttrValue::Flag(true),
        );
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            false
        ); // Has no focus
        component.attr(Attribute::Focus, AttrValue::Flag(true));
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            true
        ); // Has focus
    }

    #[test]
    fn clause_has_state_should_forward() {
        let mut component = MockFooInput::default();
        let clause = SubClause::HasState(
            MockComponentId::InputBar,
            State::One(StateValue::String(String::from("a"))),
        );
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            false
        ); // Has no state 'a'
        component.perform(Cmd::Type('a'));
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            true
        ); // Has state 'a'
    }

    #[test]
    fn clause_is_mounted_should_forward() {
        let component = MockFooInput::default();
        let clause = SubClause::IsMounted(MockComponentId::InputBar);
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |id| *id == MockComponentId::InputBar
            ),
            true
        );
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |id| *id == MockComponentId::InputFoo
            ),
            false
        );
    }

    #[test]
    fn clause_not_should_forward() {
        let mut component = MockFooInput::default();
        let clause = SubClause::not(SubClause::HasAttrValue(
            MockComponentId::InputBar,
            Attribute::Focus,
            AttrValue::Flag(true),
        ));
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            true
        ); // Has no focus
        component.attr(Attribute::Focus, AttrValue::Flag(true));
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            false
        ); // Has focus
    }

    #[test]
    fn clause_and_should_forward() {
        let mut component = MockFooInput::default();
        let clause = SubClause::and(
            SubClause::HasAttrValue(
                MockComponentId::InputBar,
                Attribute::Focus,
                AttrValue::Flag(true),
            ),
            SubClause::HasState(
                MockComponentId::InputBar,
                State::One(StateValue::String(String::from("a"))),
            ),
        );
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            false
        ); // Has no focus and has no state 'a'
        component.attr(Attribute::Focus, AttrValue::Flag(true));
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            false
        ); // Has focus and has no state 'a'
        component.perform(Cmd::Type('a'));
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            true
        ); // Has focus and has state 'a'
        component.attr(Attribute::Focus, AttrValue::Flag(false));
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            false
        ); // Has no focus and has state 'a'
    }

    #[test]
    fn clause_or_should_forward() {
        let mut component = MockFooInput::default();
        let clause = SubClause::or(
            SubClause::HasAttrValue(
                MockComponentId::InputBar,
                Attribute::Focus,
                AttrValue::Flag(true),
            ),
            SubClause::HasState(
                MockComponentId::InputBar,
                State::One(StateValue::String(String::from("a"))),
            ),
        );
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            false
        ); // Has no focus and has no state 'a'
        component.attr(Attribute::Focus, AttrValue::Flag(true));
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            true
        ); // Has focus and has no state 'a'
        component.perform(Cmd::Type('a'));
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            true
        ); // Has focus and has state 'a'
        component.attr(Attribute::Focus, AttrValue::Flag(false));
        assert_eq!(
            clause.forward(
                |_, q| component.query(q),
                |_| Some(component.state()),
                |_| true
            ),
            true
        ); // Has no focus and has state 'a'
    }

    #[test]
    fn should_create_a_sub() {
        let actual: Sub<MockComponentId, MockEvent> =
            Sub::new(EventClause::Tick, SubClause::Always);
        let expected: Sub<MockComponentId, MockEvent> = Sub(EventClause::Tick, SubClause::Always);
        assert_eq!(actual.0, expected.0);
        assert_eq!(actual.1, expected.1);
    }
}
