//! This module defines the model for the Subscriptions
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::hash::Hash;
use core::ops::Range;

use crate::event::{KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use crate::{AttrValue, Attribute, Event, State};

/// Public type to define a subscription.
pub struct Sub<ComponentId, UserEvent>(EventClause<UserEvent>, Arc<SubClause<ComponentId>>)
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    UserEvent: Eq + PartialEq + Clone;

impl<ComponentId, UserEvent> Sub<ComponentId, UserEvent>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    UserEvent: Eq + PartialEq + Clone,
{
    /// Creates a new `Sub`
    #[must_use]
    pub fn new<SC: Into<Arc<SubClause<ComponentId>>>>(
        event_clause: EventClause<UserEvent>,
        sub_clause: SC,
    ) -> Self {
        Self(event_clause, sub_clause.into())
    }
}

/// Defines a subscription for a component.
/// A subscription tells the application to forward an event to the `target` component, when an event of type `ev`
/// is received by the listener, regardless if it has focus or not. In order to forward the event, the `when` clause must also be satisfied.
///
/// > NOTE: Remember that "Component has focus" is NOT a subscription. Events are ALWAYS FORWARDED to components that have
/// > FOCUS
///
/// A subscription is defined by 3 attributes:
///     - target: the id of the target component
///     - ev: the event it listens for
///     - when: a clause that must be satisfied to forward the event to the component.
pub(crate) struct Subscription<ComponentId, UserEvent>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    UserEvent: Eq + PartialEq + Clone,
{
    /// Target component
    target: ComponentId,
    /// Event to forward and listen to
    ev: EventClause<UserEvent>,
    /// Restrict forwarding clauses
    when: Arc<SubClause<ComponentId>>,
}

impl<ComponentId, UserEvent> Subscription<ComponentId, UserEvent>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    UserEvent: Eq + PartialEq + Clone + Send,
{
    /// Instantiates a new [`Subscription`]
    #[must_use]
    pub fn new(target: ComponentId, sub: Sub<ComponentId, UserEvent>) -> Self {
        Self {
            target,
            ev: sub.0,
            when: sub.1,
        }
    }

    /// Returns sub target
    #[must_use]
    pub(crate) fn target(&self) -> &ComponentId {
        &self.target
    }

    /// Returns reference to subscription event clause
    #[must_use]
    pub(crate) fn event(&self) -> &EventClause<UserEvent> {
        &self.ev
    }

    /// Returns whether to forward event to component
    #[must_use]
    pub(crate) fn forward<HasAttrFn, GetStateFn, MountedFn>(
        &self,
        ev: &Event<UserEvent>,
        has_attr_fn: HasAttrFn,
        get_state_fn: GetStateFn,
        mounted_fn: MountedFn,
    ) -> bool
    where
        HasAttrFn: Fn(&ComponentId, Attribute) -> Option<AttrValue>,
        GetStateFn: Fn(&ComponentId) -> Option<State>,
        MountedFn: Fn(&ComponentId) -> bool,
    {
        self.ev.forward(ev) && self.when.forward(has_attr_fn, get_state_fn, mounted_fn)
    }
}

/// A event clause for [`MouseEvent`]s
#[derive(Debug, PartialEq, Eq)]
pub struct MouseEventClause {
    /// The kind of mouse event that was caused
    pub kind: MouseEventKind,
    /// The key modifiers active when the event occurred
    pub modifiers: KeyModifiers,
    /// The column that the event occurred on
    pub column: Range<u16>,
    /// The row that the event occurred on
    pub row: Range<u16>,
}

impl MouseEventClause {
    fn is_in_range(&self, ev: MouseEvent) -> bool {
        self.column.contains(&ev.column) && self.row.contains(&ev.row)
    }
}

#[derive(Debug, PartialEq, Eq)]
/// An event clause indicates on which kind of event the event must be forwarded to the `target` component.
pub enum EventClause<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone,
{
    /// Forward, no matter what kind of event
    Any,
    /// Check whether a certain key has been pressed
    Keyboard(KeyEvent),
    /// Check whether a certain key has been pressed
    Mouse(MouseEventClause),
    /// Check whether window has been resized
    WindowResize,
    /// The event will be forwarded on a tick
    Tick,
    /// Event will be forwarded on this specific user event.
    /// The way user event is matched, depends on its [`PartialEq`] implementation
    User(UserEvent),
    /// Event will be forwarded on this specific user event if the discriminant is the same.
    /// The event is matched by its discriminant only. See [`std::mem::discriminant`]
    Discriminant(UserEvent),
}

impl<UserEvent> EventClause<UserEvent>
where
    UserEvent: Eq + PartialEq + Clone,
{
    /// Check whether to forward based on even type and event clause.
    ///
    /// This is how events are forwarded:
    ///
    /// - [`EventClause::Any`]: Forward, no matter what kind of event
    /// - [`EventClause::Keyboard`]: everything must match
    /// - [`EventClause::Mouse`]: everything must match, column and row need to be within range
    /// - [`EventClause::WindowResize`]: matches only event type, not sizes
    /// - [`EventClause::Tick`]: matches tick event
    /// - [`EventClause::User`]: depends on UserEvent [`PartialEq`]
    /// - [`EventClause::Discriminant`]: matches only event type, not values
    fn forward(&self, ev: &Event<UserEvent>) -> bool {
        match self {
            EventClause::Any => true,
            EventClause::Keyboard(k) => Some(k) == ev.as_keyboard(),
            EventClause::Mouse(m) => ev.as_mouse().is_some_and(|ev| m.is_in_range(*ev)),
            EventClause::WindowResize => ev.as_window_resize(),
            EventClause::Tick => ev.as_tick(),
            EventClause::User(u) => Some(u) == ev.as_user(),
            EventClause::Discriminant(u) => {
                Some(core::mem::discriminant(u)) == ev.as_user().map(|u| core::mem::discriminant(u))
            }
        }
    }
}

/// A subclause indicates the condition that must be satisfied in order to forward `ev` to `target`.
/// Usually clauses are single conditions, but there are also some special condition, to create "ligatures", which are:
///
/// - [`SubClause::Not`]: Negates inner condition
/// - [`SubClause::And`]: the AND of the two clauses must be `true`
/// - [`SubClause::Or`]: the OR of the two clauses must be `true`
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum SubClause<ComponentId>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
{
    /// Always forward event to component
    Always,
    /// Forward event if target component has provided attribute with the provided value
    /// If the attribute doesn't exist on component, result is always `false`.
    HasAttrValue(ComponentId, Attribute, AttrValue),
    /// Forward event if target component has provided state
    HasState(ComponentId, State),
    /// Forward event if target component is mounted
    IsMounted(ComponentId),
    /// Forward event if the inner clause is `false`
    Not(Box<SubClause<ComponentId>>),
    /// Forward event if both the inner clauses are `true`
    And(Box<SubClause<ComponentId>>, Box<SubClause<ComponentId>>),
    /// Forward event if all the inner clauses are `true`.
    ///
    /// Short-circuits on first `false`.
    ///
    /// If empty will always return `false`.
    AndMany(Vec<SubClause<ComponentId>>),
    /// Forward event if at least one of the inner clauses is `true`
    Or(Box<SubClause<ComponentId>>, Box<SubClause<ComponentId>>),
    /// Forward event if at least one of the inner clauses is `true`.
    ///
    /// Short-circuits on first `true`.
    ///
    /// If empty will always return `false`.
    OrMany(Vec<SubClause<ComponentId>>),
}

impl<ComponentId> SubClause<ComponentId>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
{
    /// Shortcut for [`SubClause::Not`] without specifying `Box::new(...)`
    #[allow(clippy::should_implement_trait)]
    #[must_use]
    pub fn not(clause: Self) -> Self {
        Self::Not(Box::new(clause))
    }

    /// Shortcut for [`SubClause::And`] without specifying `Box::new(...)`
    #[must_use]
    pub fn and(a: Self, b: Self) -> Self {
        Self::And(Box::new(a), Box::new(b))
    }

    /// Shortcut for [`SubClause::Or`] without specifying `Box::new(...)`
    #[must_use]
    pub fn or(a: Self, b: Self) -> Self {
        Self::Or(Box::new(a), Box::new(b))
    }

    /// Returns whether the subscription clause is satisfied
    #[must_use]
    pub(crate) fn forward<HasAttrFn, GetStateFn, MountedFn>(
        &self,
        has_attr_fn: HasAttrFn,
        get_state_fn: GetStateFn,
        mounted_fn: MountedFn,
    ) -> bool
    where
        HasAttrFn: Fn(&ComponentId, Attribute) -> Option<AttrValue>,
        GetStateFn: Fn(&ComponentId) -> Option<State>,
        MountedFn: Fn(&ComponentId) -> bool,
    {
        self.check_forwarding(has_attr_fn, get_state_fn, mounted_fn)
            .0
    }

    /// Function to recursively check forwarding.
    #[must_use]
    fn check_forwarding<HasAttrFn, GetStateFn, MountedFn>(
        &self,
        has_attr_fn: HasAttrFn,
        get_state_fn: GetStateFn,
        mounted_fn: MountedFn,
    ) -> (bool, HasAttrFn, GetStateFn, MountedFn)
    where
        HasAttrFn: Fn(&ComponentId, Attribute) -> Option<AttrValue>,
        GetStateFn: Fn(&ComponentId) -> Option<State>,
        MountedFn: Fn(&ComponentId) -> bool,
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
            Self::AndMany(clauses) => {
                let mut has_attr_fn = has_attr_fn;
                let mut get_state_fn = get_state_fn;
                let mut mounted_fn = mounted_fn;
                for clause in clauses {
                    let res = clause.check_forwarding(has_attr_fn, get_state_fn, mounted_fn);
                    has_attr_fn = res.1;
                    get_state_fn = res.2;
                    mounted_fn = res.3;
                    // short-circuit on any "false" value, which would correspond to "false && true && true" -> "false"
                    if !res.0 {
                        return (false, has_attr_fn, get_state_fn, mounted_fn);
                    }
                }
                // return "false" on empty as anything else would not make sense to always forward if the condition is empty
                (!clauses.is_empty(), has_attr_fn, get_state_fn, mounted_fn)
            }
            Self::Or(a, b) => {
                let (fwd_a, has_attr_fn, get_state_fn, mounted_fn) =
                    a.check_forwarding(has_attr_fn, get_state_fn, mounted_fn);
                let (fwd_b, has_attr_fn, get_state_fn, mounted_fn) =
                    b.check_forwarding(has_attr_fn, get_state_fn, mounted_fn);
                (fwd_a || fwd_b, has_attr_fn, get_state_fn, mounted_fn)
            }
            Self::OrMany(clauses) => {
                let mut has_attr_fn = has_attr_fn;
                let mut get_state_fn = get_state_fn;
                let mut mounted_fn = mounted_fn;
                for clause in clauses {
                    let res = clause.check_forwarding(has_attr_fn, get_state_fn, mounted_fn);
                    has_attr_fn = res.1;
                    get_state_fn = res.2;
                    mounted_fn = res.3;
                    // short-circuit on any "true" value, which would correspond to "false || true || true" -> "true"
                    if res.0 {
                        return (true, has_attr_fn, get_state_fn, mounted_fn);
                    }
                }
                (false, has_attr_fn, get_state_fn, mounted_fn)
            }
        }
    }

    // -- privates

    #[must_use]
    fn has_attribute<HasAttrFn>(
        id: &ComponentId,
        query: &Attribute,
        value: &AttrValue,
        has_attr_fn: HasAttrFn,
    ) -> (bool, HasAttrFn)
    where
        HasAttrFn: Fn(&ComponentId, Attribute) -> Option<AttrValue>,
    {
        (
            match has_attr_fn(id, *query) {
                None => false,
                Some(v) => *value == v,
            },
            has_attr_fn,
        )
    }

    #[must_use]
    fn has_state<GetStateFn>(
        id: &ComponentId,
        state: &State,
        get_state_fn: GetStateFn,
    ) -> (bool, GetStateFn)
    where
        GetStateFn: Fn(&ComponentId) -> Option<State>,
    {
        (
            match get_state_fn(id) {
                Some(s) => s == *state,
                None => false,
            },
            get_state_fn,
        )
    }

    #[must_use]
    fn is_mounted<MountedFn>(id: &ComponentId, mounted_fn: MountedFn) -> (bool, MountedFn)
    where
        MountedFn: Fn(&ComponentId) -> bool,
    {
        (mounted_fn(id), mounted_fn)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::command::Cmd;
    use crate::event::{Key, KeyModifiers, MouseEventKind};
    use crate::mock::{MockComponentId, MockEvent, MockFooInput};
    use crate::{MockComponent, NoUserEvent, StateValue};

    #[test]
    fn subscription_should_forward() {
        let ev: Event<MockEvent> = Event::WindowResize(1024, 512);
        let mut component = MockFooInput::default();
        component.attr(Attribute::Focus, AttrValue::Flag(true));
        let sub = Subscription::new(
            MockComponentId::InputFoo,
            Sub::new(
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
            *sub.when,
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
    fn forward_many() {
        let ev: Event<MockEvent> = Event::Keyboard(Key::Char('q').into());
        let mut component = MockFooInput::default();
        component.attr(Attribute::Focus, AttrValue::Flag(true));

        // AndMany all "true", returns "true"
        let sub = Subscription::new(
            MockComponentId::InputFoo,
            Sub::new(
                EventClause::Keyboard(Key::Char('q').into()),
                SubClause::AndMany(vec![
                    SubClause::IsMounted(MockComponentId::InputFoo),
                    SubClause::IsMounted(MockComponentId::InputBar),
                    SubClause::IsMounted(MockComponentId::InputOmar),
                ]),
            ),
        );
        assert_eq!(sub.target(), &MockComponentId::InputFoo);
        assert_eq!(
            sub.event(),
            &EventClause::<MockEvent>::Keyboard(Key::Char('q').into())
        );
        assert_eq!(
            *sub.when,
            SubClause::AndMany(vec![
                SubClause::IsMounted(MockComponentId::InputFoo),
                SubClause::IsMounted(MockComponentId::InputBar),
                SubClause::IsMounted(MockComponentId::InputOmar),
            ])
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

        // AndMany one "false", returns "false"
        let sub = Subscription::new(
            MockComponentId::InputFoo,
            Sub::new(
                EventClause::Keyboard(Key::Char('q').into()),
                SubClause::AndMany(vec![
                    SubClause::IsMounted(MockComponentId::InputFoo),
                    SubClause::IsMounted(MockComponentId::InputBar),
                    SubClause::not(SubClause::IsMounted(MockComponentId::InputOmar)),
                ]),
            ),
        );
        assert_eq!(sub.target(), &MockComponentId::InputFoo);
        assert_eq!(
            sub.event(),
            &EventClause::<MockEvent>::Keyboard(Key::Char('q').into())
        );
        assert_eq!(
            *sub.when,
            SubClause::AndMany(vec![
                SubClause::IsMounted(MockComponentId::InputFoo),
                SubClause::IsMounted(MockComponentId::InputBar),
                SubClause::not(SubClause::IsMounted(MockComponentId::InputOmar)),
            ])
        );
        assert_eq!(
            sub.forward(
                &ev,
                |_: &MockComponentId, q| component.query(q),
                |_: &MockComponentId| Some(component.state()),
                |_: &MockComponentId| true
            ),
            false
        );

        // OrMany one "false", returns "true"
        let sub = Subscription::new(
            MockComponentId::InputFoo,
            Sub::new(
                EventClause::Keyboard(Key::Char('q').into()),
                SubClause::OrMany(vec![
                    SubClause::IsMounted(MockComponentId::InputFoo),
                    SubClause::IsMounted(MockComponentId::InputBar),
                    SubClause::not(SubClause::IsMounted(MockComponentId::InputOmar)),
                ]),
            ),
        );
        assert_eq!(sub.target(), &MockComponentId::InputFoo);
        assert_eq!(
            sub.event(),
            &EventClause::<MockEvent>::Keyboard(Key::Char('q').into())
        );
        assert_eq!(
            *sub.when,
            SubClause::OrMany(vec![
                SubClause::IsMounted(MockComponentId::InputFoo),
                SubClause::IsMounted(MockComponentId::InputBar),
                SubClause::not(SubClause::IsMounted(MockComponentId::InputOmar)),
            ])
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

        // OrMany all "false", returns "false"
        let sub = Subscription::new(
            MockComponentId::InputFoo,
            Sub::new(
                EventClause::Keyboard(Key::Char('q').into()),
                SubClause::OrMany(vec![
                    SubClause::not(SubClause::IsMounted(MockComponentId::InputFoo)),
                    SubClause::not(SubClause::IsMounted(MockComponentId::InputBar)),
                    SubClause::not(SubClause::IsMounted(MockComponentId::InputOmar)),
                ]),
            ),
        );
        assert_eq!(sub.target(), &MockComponentId::InputFoo);
        assert_eq!(
            sub.event(),
            &EventClause::<MockEvent>::Keyboard(Key::Char('q').into())
        );
        assert_eq!(
            *sub.when,
            SubClause::OrMany(vec![
                SubClause::not(SubClause::IsMounted(MockComponentId::InputFoo)),
                SubClause::not(SubClause::IsMounted(MockComponentId::InputBar)),
                SubClause::not(SubClause::IsMounted(MockComponentId::InputOmar)),
            ])
        );
        assert_eq!(
            sub.forward(
                &ev,
                |_: &MockComponentId, q| component.query(q),
                |_: &MockComponentId| Some(component.state()),
                |_: &MockComponentId| true
            ),
            false
        );
    }

    #[test]
    fn forward_many_zero_elements() {
        let ev: Event<MockEvent> = Event::Keyboard(Key::Char('q').into());
        let mut component = MockFooInput::default();
        component.attr(Attribute::Focus, AttrValue::Flag(true));

        // AndMany returns "true"
        let sub = Subscription::new(
            MockComponentId::InputFoo,
            Sub::new(
                EventClause::Keyboard(Key::Char('q').into()),
                SubClause::AndMany(vec![]),
            ),
        );
        assert_eq!(sub.target(), &MockComponentId::InputFoo);
        assert_eq!(
            sub.event(),
            &EventClause::<MockEvent>::Keyboard(Key::Char('q').into())
        );
        assert_eq!(*sub.when, SubClause::AndMany(vec![]));
        assert_eq!(
            sub.forward(
                &ev,
                |_: &MockComponentId, q| component.query(q),
                |_: &MockComponentId| Some(component.state()),
                |_: &MockComponentId| true
            ),
            false
        );

        // OrMany returns "true"
        let sub = Subscription::new(
            MockComponentId::InputFoo,
            Sub::new(
                EventClause::Keyboard(Key::Char('q').into()),
                SubClause::OrMany(vec![]),
            ),
        );
        assert_eq!(sub.target(), &MockComponentId::InputFoo);
        assert_eq!(
            sub.event(),
            &EventClause::<MockEvent>::Keyboard(Key::Char('q').into())
        );
        assert_eq!(*sub.when, SubClause::OrMany(vec![]));
        assert_eq!(
            sub.forward(
                &ev,
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
        assert_eq!(
            EventClause::<MockEvent>::Keyboard(KeyEvent::from(Key::Enter)).forward(&Event::Mouse(
                MouseEvent {
                    kind: MouseEventKind::Moved,
                    modifiers: KeyModifiers::NONE,
                    column: 0,
                    row: 0
                }
            )),
            false
        );
    }

    #[test]
    fn event_clause_mouse_should_forward() {
        assert_eq!(
            EventClause::<MockEvent>::Mouse(MouseEventClause {
                kind: MouseEventKind::Moved,
                modifiers: KeyModifiers::NONE,
                column: 0..10,
                row: 0..10
            })
            .forward(&Event::Mouse(MouseEvent {
                kind: MouseEventKind::Moved,
                modifiers: KeyModifiers::NONE,
                column: 0,
                row: 0
            })),
            true
        );
        assert_eq!(
            EventClause::<MockEvent>::Mouse(MouseEventClause {
                kind: MouseEventKind::Moved,
                modifiers: KeyModifiers::NONE,
                column: 0..10,
                row: 0..10
            })
            .forward(&Event::Mouse(MouseEvent {
                kind: MouseEventKind::Moved,
                modifiers: KeyModifiers::NONE,
                column: 20,
                row: 20
            })),
            false
        );
        assert_eq!(
            EventClause::<MockEvent>::Mouse(MouseEventClause {
                kind: MouseEventKind::Moved,
                modifiers: KeyModifiers::NONE,
                column: 0..10,
                row: 0..10
            })
            .forward(&Event::Keyboard(KeyEvent::from(Key::Backspace))),
            false
        );
        assert_eq!(
            EventClause::<MockEvent>::Mouse(MouseEventClause {
                kind: MouseEventKind::Moved,
                modifiers: KeyModifiers::NONE,
                column: 0..10,
                row: 0..10
            })
            .forward(&Event::Tick),
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
    fn event_clause_discriminant_should_forward() {
        assert_eq!(
            EventClause::<MockEvent>::Discriminant(MockEvent::Foo)
                .forward(&Event::User(MockEvent::Foo)),
            true
        );
        assert_eq!(
            EventClause::<MockEvent>::Discriminant(MockEvent::Hello("foo".to_string()))
                .forward(&Event::User(MockEvent::Hello("bar".to_string()))),
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
            State::Single(StateValue::String(String::from("a"))),
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
                State::Single(StateValue::String(String::from("a"))),
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
                State::Single(StateValue::String(String::from("a"))),
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
        let expected: Sub<MockComponentId, MockEvent> =
            Sub::new(EventClause::Tick, SubClause::Always);
        assert_eq!(actual.0, expected.0);
        assert_eq!(actual.1, expected.1);
    }

    #[test]
    fn should_share_subs() {
        // image this is one big clause with "OrMany" or something
        let no_popup_clause = Arc::new(SubClause::<MockComponentId>::Not(Box::new(
            SubClause::Always,
        )));

        let subscriptions: Vec<Sub<MockComponentId, NoUserEvent>> = vec![
            Sub::new(
                EventClause::Keyboard(KeyEvent::new(Key::Enter, KeyModifiers::NONE)),
                no_popup_clause.clone(),
            ),
            Sub::new(
                EventClause::Keyboard(KeyEvent::new(Key::Esc, KeyModifiers::NONE)),
                no_popup_clause.clone(),
            ),
            Sub::new(
                EventClause::Keyboard(KeyEvent::new(Key::Backspace, KeyModifiers::NONE)),
                no_popup_clause.clone(),
            ),
            Sub::new(EventClause::Tick, SubClause::Always),
        ];

        // assert that all shared subs point to the same memory location
        assert!(Arc::ptr_eq(&no_popup_clause, &subscriptions[0].1));
        assert!(Arc::ptr_eq(&no_popup_clause, &subscriptions[1].1));
        assert!(Arc::ptr_eq(&no_popup_clause, &subscriptions[2].1));
        assert!(!Arc::ptr_eq(&no_popup_clause, &subscriptions[3].1));
    }

    #[test]
    fn should_allow_creation_without_arc() {
        let sub1 = Sub::<MockComponentId, NoUserEvent>::new(EventClause::Tick, SubClause::Always);
        let sub2 = Sub::<MockComponentId, NoUserEvent>::new(EventClause::Tick, SubClause::Always);

        assert!(!Arc::ptr_eq(&sub1.1, &sub2.1));
    }
}
