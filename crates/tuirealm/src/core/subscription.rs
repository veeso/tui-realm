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
use crate::{AttrSelector, Attribute, Event, MockComponent, State};
use std::fmt;

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
pub struct Subscription<UserEvent>
where
    UserEvent: fmt::Debug + Eq + PartialEq + Clone + PartialOrd,
{
    target: String,
    ev: Event<UserEvent>,
    when: SubClause,
}

impl<U> Subscription<U>
where
    U: fmt::Debug + Eq + PartialEq + Clone + PartialOrd,
{
    /// ### new
    ///
    /// Instantiates a new `Subscription`
    pub fn new<S: AsRef<str>>(target: S, ev: Event<U>, when: SubClause) -> Self {
        Self {
            target: target.as_ref().to_string(),
            ev,
            when,
        }
    }

    /// ### forward
    ///
    /// Returns whether to forward event to component
    pub fn forward(&self, id: &str, ev: &Event<U>, component: &Box<dyn MockComponent>) -> bool {
        self.target.as_str() == id && &self.ev == ev && self.when.forward(component)
    }
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
pub enum SubClause {
    /// Always forward event to component
    Always,
    /// Forward event if target component has provided attribute
    HasAttribute(AttrSelector, Attribute),
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
    /// ### forward
    ///
    /// Returns whether the subscription clause is satisfied
    pub(crate) fn forward(&self, target: &Box<dyn MockComponent>) -> bool {
        match self {
            Self::Always => true,
            Self::HasAttribute(query, value) => Self::has_attribute(query, value, target),
            Self::HasState(state) => Self::has_state(state, target),
            Self::Not(clause) => !(clause.forward(target)),
            Self::And(a, b) => (a.forward(target)) && (b.forward(target)),
            Self::Or(a, b) => (a.forward(target)) || (b.forward(target)),
        }
    }

    // -- privates

    fn has_attribute(
        query: &AttrSelector,
        value: &Attribute,
        target: &Box<dyn MockComponent>,
    ) -> bool {
        match target.query(*query) {
            None => false,
            Some(v) => *value == v,
        }
    }

    fn has_state(state: &State, target: &Box<dyn MockComponent>) -> bool {
        target.state() == *state
    }
}
