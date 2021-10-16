//! # Subscription
//!
//! This module defines the model for the Subscriptions

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
use crate::event::KeyEvent;
use crate::{AttrValue, Attribute, Event, State};
use std::fmt;

/// ## Sub
///
/// Public type to define a subscription.
pub struct Sub<UserEvent>(EventClause<UserEvent>, SubClause)
where
    UserEvent: fmt::Debug + Eq + PartialEq + Clone + PartialOrd;

impl<U> Sub<U>
where
    U: fmt::Debug + Eq + PartialEq + Clone + PartialOrd,
{
    /// ### new
    ///
    /// Creates a new `Sub`
    pub fn new(event_clause: EventClause<U>, sub_clause: SubClause) -> Self {
        Self(event_clause, sub_clause)
    }
}

/// ## Subscription
///
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
pub(crate) struct Subscription<UserEvent>
where
    UserEvent: fmt::Debug + Eq + PartialEq + Clone + PartialOrd,
{
    /// Target component
    target: String,
    /// Event to forward and listen to
    ev: EventClause<UserEvent>,
    /// Restrict forwarding clauses
    when: SubClause,
}

impl<U> Subscription<U>
where
    U: fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send,
{
    /// ### new
    ///
    /// Instantiates a new `Subscription`
    pub fn new(target: &str, sub: Sub<U>) -> Self {
        Self {
            target: target.to_string(),
            ev: sub.0,
            when: sub.1,
        }
    }

    /// ### target
    ///
    /// Returns sub target
    pub(crate) fn target(&self) -> &str {
        self.target.as_str()
    }

    /// ### event
    ///
    /// Returns reference to subscription event clause
    pub(crate) fn event(&self) -> &EventClause<U> {
        &self.ev
    }

    /// ### forward
    ///
    /// Returns whether to forward event to component
    pub(crate) fn forward<HasAttrFn, GetStateFn>(
        &self,
        ev: &Event<U>,
        has_attr_fn: HasAttrFn,
        get_state_fn: GetStateFn,
    ) -> bool
    where
        HasAttrFn: Fn(Attribute) -> Option<AttrValue>,
        GetStateFn: Fn() -> State,
    {
        self.match_event(ev) && self.when.forward(has_attr_fn, get_state_fn)
    }

    /// ### match_event
    ///
    /// Check whether event matches.
    /// This is how events are matched:
    ///
    /// - Keyboard: everything must match
    /// - WindowResize: matches only event type, not sizes
    /// - Tick: matches tick event
    /// - None: matches None event
    /// - UserEvent: depends on UserEvent PartialEq
    fn match_event(&self, ev: &Event<U>) -> bool {
        match &self.ev {
            EventClause::Keyboard(k) => Some(k) == ev.is_keyboard(),
            EventClause::WindowResize => ev.is_window_resize(),
            EventClause::Tick => ev.is_tick(),
            EventClause::User(u) => Some(u) == ev.is_user(),
        }
    }
}

#[derive(Debug, PartialEq)]
/// ## EventClause
///
/// An event clause indicates on which kind of event the event must be forwarded to the `target` component.
pub enum EventClause<UserEvent>
where
    UserEvent: fmt::Debug + Eq + PartialEq + Clone + PartialOrd,
{
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

/// ## SubClause
///
/// A subclause indicates the condition that must be satisfied in order to forward `ev` to `target`.
/// Usually clauses are single conditions, but there are also some special condition, to create "ligatures", which are:
///
/// - `Not(SubClause)`: Negates inner condition
/// - `And(SubClause, SubClause)`: the AND of the two clauses must be `true`
/// - `Or(SubClause, SubClause)`: the OR of the two clauses must be `true`
#[derive(Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum SubClause {
    /// Always forward event to component
    Always,
    /// Forward event if target component has provided attribute with the provided value
    /// If the attribute doesn't exist on component, result is always `false`.
    HasAttrValue(Attribute, AttrValue),
    /// Forward event if target component has provided state
    HasState(State),
    /// Forward event if the inner clause is `false`
    Not(Box<SubClause>),
    /// Forward event if both the inner clauses are `true`
    And(Box<SubClause>, Box<SubClause>),
    /// Forward event if at least one of the inner clauses is `true`
    Or(Box<SubClause>, Box<SubClause>),
}

impl SubClause {
    /// ### not
    ///
    /// Shortcut for `SubClause::Not` without specifying `Box::new(...)`
    #[allow(clippy::should_implement_trait)]
    pub fn not(clause: SubClause) -> Self {
        Self::Not(Box::new(clause))
    }

    /// ### and
    ///
    /// Shortcut for `SubClause::And` without specifying `Box::new(...)`
    pub fn and(a: SubClause, b: SubClause) -> Self {
        Self::And(Box::new(a), Box::new(b))
    }

    /// ### or
    ///
    /// Shortcut for `SubClause::Or` without specifying `Box::new(...)`
    pub fn or(a: SubClause, b: SubClause) -> Self {
        Self::Or(Box::new(a), Box::new(b))
    }

    /// ### forward
    ///
    /// Returns whether the subscription clause is satisfied
    pub(crate) fn forward<HasAttrFn, GetStateFn>(
        &self,
        has_attr_fn: HasAttrFn,
        get_state_fn: GetStateFn,
    ) -> bool
    where
        HasAttrFn: Fn(Attribute) -> Option<AttrValue>,
        GetStateFn: Fn() -> State,
    {
        self.check_forwarding(has_attr_fn, get_state_fn).0
    }

    fn check_forwarding<HasAttrFn, GetStateFn>(
        &self,
        has_attr_fn: HasAttrFn,
        get_state_fn: GetStateFn,
    ) -> (bool, HasAttrFn, GetStateFn)
    where
        HasAttrFn: Fn(Attribute) -> Option<AttrValue>,
        GetStateFn: Fn() -> State,
    {
        match self {
            Self::Always => (true, has_attr_fn, get_state_fn),
            Self::HasAttrValue(query, value) => {
                let (fwd, has_attr_fn) = Self::has_attribute(query, value, has_attr_fn);
                (fwd, has_attr_fn, get_state_fn)
            }
            Self::HasState(state) => {
                let (fwd, get_state_fn) = Self::has_state(state, get_state_fn);
                (fwd, has_attr_fn, get_state_fn)
            }
            Self::Not(clause) => {
                let (fwd, has_attr_fn, get_state_fn) =
                    clause.check_forwarding(has_attr_fn, get_state_fn);
                (!fwd, has_attr_fn, get_state_fn)
            }
            Self::And(a, b) => {
                let (fwd_a, has_attr_fn, get_state_fn) =
                    a.check_forwarding(has_attr_fn, get_state_fn);
                let (fwd_b, has_attr_fn, get_state_fn) =
                    b.check_forwarding(has_attr_fn, get_state_fn);
                (fwd_a && fwd_b, has_attr_fn, get_state_fn)
            }
            Self::Or(a, b) => {
                let (fwd_a, has_attr_fn, get_state_fn) =
                    a.check_forwarding(has_attr_fn, get_state_fn);
                let (fwd_b, has_attr_fn, get_state_fn) =
                    b.check_forwarding(has_attr_fn, get_state_fn);
                (fwd_a || fwd_b, has_attr_fn, get_state_fn)
            }
        }
    }

    // -- privates

    fn has_attribute<HasAttrFn>(
        query: &Attribute,
        value: &AttrValue,
        has_attr_fn: HasAttrFn,
    ) -> (bool, HasAttrFn)
    where
        HasAttrFn: Fn(Attribute) -> Option<AttrValue>,
    {
        (
            match has_attr_fn(*query) {
                None => false,
                Some(v) => *value == v,
            },
            has_attr_fn,
        )
    }

    fn has_state<GetStateFn>(state: &State, get_state_fn: GetStateFn) -> (bool, GetStateFn)
    where
        GetStateFn: Fn() -> State,
    {
        (get_state_fn() == *state, get_state_fn)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::{MockEvent, MockFooInput};
    use crate::{command::Cmd, MockComponent, StateValue};

    use pretty_assertions::assert_eq;

    #[test]
    fn subscription_should_forward() {
        let ev: Event<MockEvent> = Event::WindowResize(1024, 512);
        let mut component = MockFooInput::default();
        component.attr(Attribute::Focus, AttrValue::Flag(true));
        let sub = Subscription::new(
            "foo",
            Sub(
                EventClause::WindowResize,
                SubClause::HasAttrValue(Attribute::Focus, AttrValue::Flag(true)),
            ),
        );
        assert_eq!(sub.target(), "foo");
        assert_eq!(sub.event(), &EventClause::<MockEvent>::WindowResize);
        assert_eq!(
            sub.when,
            SubClause::HasAttrValue(Attribute::Focus, AttrValue::Flag(true))
        );
        assert_eq!(
            sub.forward(&ev, |q| component.query(q), || component.state()),
            true
        );
        // False clause
        component.attr(Attribute::Focus, AttrValue::Flag(false));
        assert_eq!(
            sub.forward(&ev, |q| component.query(q), || component.state()),
            false
        );
        // False event
        assert_eq!(
            sub.forward(
                &Event::User(MockEvent::Foo),
                |q| component.query(q),
                || component.state()
            ),
            false
        );
        // False id
        assert_eq!(
            sub.forward(
                &Event::WindowResize(0, 0),
                |q| component.query(q),
                || component.state()
            ),
            false
        );
    }

    #[test]
    fn clause_always_should_forward() {
        let component = MockFooInput::default();
        let clause = SubClause::Always;
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            true
        );
    }

    #[test]
    fn clause_has_attribute_should_forward() {
        let mut component = MockFooInput::default();
        let clause = SubClause::HasAttrValue(Attribute::Focus, AttrValue::Flag(true));
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            false
        ); // Has no focus
        component.attr(Attribute::Focus, AttrValue::Flag(true));
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            true
        ); // Has focus
    }

    #[test]
    fn clause_has_state_should_forward() {
        let mut component = MockFooInput::default();
        let clause = SubClause::HasState(State::One(StateValue::String(String::from("a"))));
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            false
        ); // Has no state 'a'
        component.perform(Cmd::Type('a'));
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            true
        ); // Has state 'a'
    }

    #[test]
    fn clause_not_should_forward() {
        let mut component = MockFooInput::default();
        let clause = SubClause::not(SubClause::HasAttrValue(
            Attribute::Focus,
            AttrValue::Flag(true),
        ));
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            true
        ); // Has no focus
        component.attr(Attribute::Focus, AttrValue::Flag(true));
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            false
        ); // Has focus
    }

    #[test]
    fn clause_and_should_forward() {
        let mut component = MockFooInput::default();
        let clause = SubClause::and(
            SubClause::HasAttrValue(Attribute::Focus, AttrValue::Flag(true)),
            SubClause::HasState(State::One(StateValue::String(String::from("a")))),
        );
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            false
        ); // Has no focus and has no state 'a'
        component.attr(Attribute::Focus, AttrValue::Flag(true));
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            false
        ); // Has focus and has no state 'a'
        component.perform(Cmd::Type('a'));
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            true
        ); // Has focus and has state 'a'
        component.attr(Attribute::Focus, AttrValue::Flag(false));
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            false
        ); // Has no focus and has state 'a'
    }

    #[test]
    fn clause_or_should_forward() {
        let mut component = MockFooInput::default();
        let clause = SubClause::or(
            SubClause::HasAttrValue(Attribute::Focus, AttrValue::Flag(true)),
            SubClause::HasState(State::One(StateValue::String(String::from("a")))),
        );
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            false
        ); // Has no focus and has no state 'a'
        component.attr(Attribute::Focus, AttrValue::Flag(true));
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            true
        ); // Has focus and has no state 'a'
        component.perform(Cmd::Type('a'));
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            true
        ); // Has focus and has state 'a'
        component.attr(Attribute::Focus, AttrValue::Flag(false));
        assert_eq!(
            clause.forward(|q| component.query(q), || component.state()),
            true
        ); // Has no focus and has state 'a'
    }

    #[test]
    fn should_create_a_sub() {
        let actual: Sub<MockEvent> = Sub::new(EventClause::Tick, SubClause::Always);
        let expected: Sub<MockEvent> = Sub(EventClause::Tick, SubClause::Always);
        assert_eq!(actual.0, expected.0);
        assert_eq!(actual.1, expected.1);
    }
}
