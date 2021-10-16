//! ## Application
//!
//! This module exposes the Application, which is the core struct of tui-realm.

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
use super::{Subscription, WrappedComponent};
use crate::listener::{EventListener, EventListenerCfg, ListenerError};
use crate::tui::layout::Rect;
use crate::{
    AttrValue, Attribute, Event, Frame, State, Sub, SubEventClause, Update, View, ViewError,
};

use std::fmt;
use thiserror::Error;

/// ## ApplicationResult
///
/// Result retuned by `Application`.
/// Ok depends on method
/// Err is always `ApplicationError`
pub type ApplicationResult<T> = Result<T, ApplicationError>;

/// ## Application
///
/// The application defines a tui-realm application.
/// It will handle events, subscriptions and the view too.
/// It provides functions to interact with the view (mount, umount, query, etc), but also
/// the main function: `tick()`. See [tick](#tick)
pub struct Application<'a, Msg, UserEvent>
where
    Msg: PartialEq,
    UserEvent: fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    listener: EventListener<UserEvent>,
    subs: Vec<Subscription<'a, UserEvent>>,
    view: View<'a, Msg, UserEvent>,
}

impl<'a, Msg, UserEvent> Application<'a, Msg, UserEvent>
where
    Msg: PartialEq,
    UserEvent: fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    /// ### init
    ///
    /// Initialize a new `Application`.
    /// The event listener is immediately created and started.
    pub fn init(listener_cfg: EventListenerCfg<UserEvent>) -> Self {
        Self {
            listener: listener_cfg.start(),
            subs: Vec::new(),
            view: View::default(),
        }
    }

    /// ### restart_listener
    ///
    /// Restart listener in case the previous listener has died or if you want to start a new one with a new configuration.
    ///
    /// > The listener has died if you received a `ApplicationError::Listener(ListenerError::ListenerDied))`
    pub fn restart_listener(
        &mut self,
        listener_cfg: EventListenerCfg<UserEvent>,
    ) -> ApplicationResult<()> {
        self.listener.stop()?;
        self.listener = listener_cfg.start();
        Ok(())
    }

    /// ### tick
    ///
    /// The tick method makes the application to run once.
    /// The workflow of the tick method is the following one:
    ///
    /// 1. The event listener is fetched according to the provided `PollStrategy`
    /// 2. All the received events are sent to the current active component
    /// 3. All the received events are forwarded to the subscribed components which satisfy the received events and conditions.
    /// 4. Returns the messages. Once messages are returned call `Application::update()`
    ///
    /// As soon as function returns, you should call the `view()` method.
    ///
    /// > You can also call `view` from the `update()` if you need it
    pub fn tick(&'a mut self, strategy: PollStrategy) -> ApplicationResult<Vec<Msg>> {
        // Poll event listener
        let events = self.poll(strategy)?;
        // Forward to active element
        let mut messages: Vec<Msg> = events
            .iter()
            .map(|x| self.forward_to_active_component(x.clone()))
            .flatten()
            .collect();
        // Forward to subscriptions and extend vector
        // NOTE: don't change this code ever and never. Putting the line below into an iterator won't build :)
        messages.extend(self.forward_to_subscriptions(events).into_iter());
        // NOTE: from now on, since lifetime 'a is borrowed into forward, we cannot call self.
        // Return messages
        Ok(messages)
    }

    /// ### update
    ///
    /// Call update on model for each message returned by `tick()`
    pub fn update(&mut self, model: &mut dyn Update<Msg, UserEvent>, messages: Vec<Msg>) {
        messages.into_iter().for_each(|msg| {
            assert!(self.recurse_update(model, Some(msg)).is_none());
        });
    }

    // -- view bridge

    /// ### mount
    ///
    /// Mount component to view and associate subscriptions for it.
    /// Returns error if component is already mounted
    /// NOTE: if subs vector contains duplicated, these will be discarded
    pub fn mount(
        &mut self,
        id: &'a str,
        component: WrappedComponent<Msg, UserEvent>,
        subs: Vec<Sub<UserEvent>>,
    ) -> ApplicationResult<()> {
        // Mount
        self.view.mount(id, component)?;
        // Subscribe
        subs.into_iter().for_each(|x| {
            // Push only if not already subscribed
            let subscription = Subscription::new(id, x);
            if !self.subscribed(id, subscription.event()) {
                self.subs.push(subscription);
            }
        });
        Ok(())
    }

    /// ### umount
    ///
    /// Umount component associated to `id` and remove ALL its SUBSCRIPTIONS.
    /// Returns Error if the component doesn't exist
    pub fn umount(&mut self, id: &'a str) -> ApplicationResult<()> {
        self.view.umount(id)?;
        self.unsubscribe_component(id);
        Ok(())
    }

    /// ### mounted
    ///
    /// Returns whether component `id` is mounted
    pub fn mounted(&self, id: &'a str) -> bool {
        self.view.mounted(id)
    }

    /// ### view
    ///
    /// Render component called `id`
    pub fn view(&mut self, id: &'a str, f: &mut Frame, area: Rect) {
        self.view.view(id, f, area);
    }

    /// ### query
    ///
    /// Query view component for a certain `AttrValue`
    /// Returns error if the component doesn't exist
    /// Returns None if the attribute doesn't exist.
    pub fn query(&self, id: &'a str, query: Attribute) -> ApplicationResult<Option<AttrValue>> {
        self.view.query(id, query).map_err(ApplicationError::from)
    }

    /// ### attr
    ///
    /// Set attribute for component `id`
    /// Returns error if the component doesn't exist
    pub fn attr(
        &mut self,
        id: &'a str,
        attr: Attribute,
        value: AttrValue,
    ) -> ApplicationResult<()> {
        self.view
            .attr(id, attr, value)
            .map_err(ApplicationError::from)
    }

    /// ### state
    ///
    /// Get state for component `id`.
    /// Returns `Err` if component doesn't exist
    pub fn state(&self, id: &'a str) -> ApplicationResult<State> {
        self.view.state(id).map_err(ApplicationError::from)
    }

    /// ### active
    ///
    /// Shorthand for `attr(id, Attribute::Focus(AttrValue::Flag(true)))`.
    /// It also sets the component as the current one having focus.
    /// Previous active component, if any, GETS PUSHED to the STACK
    /// Returns error: if component doesn't exist. Use `mounted()` to check if component exists
    ///
    /// > NOTE: users should always use this function to give focus to components.
    pub fn active(&mut self, id: &'a str) -> ApplicationResult<()> {
        self.view.active(id).map_err(ApplicationError::from)
    }

    /// ### blur
    ///
    /// Blur selected element AND DON'T PUSH CURRENT ACTIVE ELEMENT INTO THE STACK
    /// Shorthand for `attr(id, Attribute::Focus(AttrValue::Flag(false)))`.
    /// It also unset the current focus and give it to the first element in stack.
    /// Returns error: if no component has focus
    ///
    /// > NOTE: users should always use this function to remove focus to components.
    pub fn blur(&mut self) -> ApplicationResult<()> {
        self.view.blur().map_err(ApplicationError::from)
    }

    // -- subs bridge

    /// ### subscribe
    ///
    /// Subscribe component to a certain event.
    /// Returns Error if the component doesn't exist or if the component is already subscribed to this event
    pub fn subscribe(&mut self, id: &'a str, sub: Sub<UserEvent>) -> ApplicationResult<()> {
        if !self.view.mounted(id) {
            return Err(ViewError::ComponentNotFound.into());
        }
        let subscription = Subscription::new(id, sub);
        if self.subscribed(id, subscription.event()) {
            return Err(ApplicationError::AlreadySubscribed);
        }
        self.subs.push(subscription);
        Ok(())
    }

    /// ### unsubscribe
    ///
    /// Unsubscribe a component from a certain event.
    /// Returns error if the component doesn't exist or if the component is not subscribed to this event
    pub fn unsubscribe(
        &mut self,
        id: &'a str,
        ev: SubEventClause<UserEvent>,
    ) -> ApplicationResult<()> {
        if !self.view.mounted(id) {
            return Err(ViewError::ComponentNotFound.into());
        }
        if !self.subscribed(id, &ev) {
            return Err(ApplicationError::NoSuchSubscription);
        }
        self.subs.retain(|s| s.target() != id && s.event() != &ev);
        Ok(())
    }

    // -- private

    /// ### unsubscribe_component
    ///
    /// remove all subscriptions for component
    fn unsubscribe_component(&mut self, id: &'a str) {
        self.subs.retain(|x| x.target() != id)
    }

    /// ### subscribed
    ///
    /// Returns whether component `id` is subscribed to event described by `clause`
    fn subscribed(&self, id: &'a str, clause: &SubEventClause<UserEvent>) -> bool {
        self.subs
            .iter()
            .find(|s| s.target() == id && s.event() == clause)
            .is_some()
    }

    /// ### poll
    ///
    /// Poll listener according to provided strategy
    fn poll(&mut self, strategy: PollStrategy) -> ApplicationResult<Vec<Event<UserEvent>>> {
        match strategy {
            PollStrategy::Once => self
                .poll_listener()
                .map(|x| x.map(|x| vec![x]).unwrap_or_default()),
            PollStrategy::UpTo(times) => self.poll_times(times),
        }
    }

    /// ### poll_times
    ///
    /// Poll event listener up to `t` times
    fn poll_times(&mut self, t: usize) -> ApplicationResult<Vec<Event<UserEvent>>> {
        let mut evs: Vec<Event<UserEvent>> = Vec::with_capacity(t);
        for _ in 0..t {
            match self.poll_listener() {
                Err(err) => return Err(err),
                Ok(None) => break,
                Ok(Some(ev)) => evs.push(ev),
            }
        }
        Ok(evs)
    }

    /// ### poll_listener
    ///
    /// Poll event listener once
    fn poll_listener(&mut self) -> ApplicationResult<Option<Event<UserEvent>>> {
        self.listener.poll().map_err(ApplicationError::from)
    }

    /// ### forward_to_active_component
    ///
    /// Forward event to current active component, if any.
    fn forward_to_active_component(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        self.view
            .focus()
            .map(|x| self.view.forward(x, ev).ok().unwrap())
            .flatten()
    }

    /// ### forward_to_subscriptions
    ///
    /// Forward events to subscriptions listening to the incoming event.
    fn forward_to_subscriptions(&'a mut self, events: Vec<Event<UserEvent>>) -> Vec<Msg> {
        let mut messages: Vec<Msg> = Vec::new();
        // NOTE: don't touch this code again and don't try to use iterators, cause it's not gonna work :)
        for ev in events.iter() {
            for sub in self.subs.iter() {
                if !sub.forward(
                    ev,
                    |q| self.view.component(sub.target()).unwrap().query(q),
                    || self.view.component(sub.target()).unwrap().state(),
                ) {
                    continue;
                }
                if let Some(msg) = self.view.forward(sub.target(), ev.clone()).ok().unwrap() {
                    messages.push(msg);
                }
            }
        }
        messages
    }

    /// ### update
    ///
    /// Calls update on model passing the view as a mutable reference.
    /// This method will keep calling `update` on model until `None` is returned.
    /// This function ALWAYS return `None` to the caller.
    fn recurse_update(
        &mut self,
        model: &mut dyn Update<Msg, UserEvent>,
        msg: Option<Msg>,
    ) -> Option<Msg> {
        if let Some(msg) = model.update(&mut self.view, msg) {
            self.recurse_update(model, Some(msg))
        } else {
            None
        }
    }
}

/// ## PollStrategy
///
/// Poll strategy defines how to call `poll` on the event listener.
pub enum PollStrategy {
    /// The poll() function will be called once
    Once,
    /// The poll() function will be called up to `n` times, until it will return `None`.
    UpTo(usize),
}

// -- error

/// ## ApplicationError
///
/// Error variants returned by `Application`
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("already subscribed")]
    AlreadySubscribed,
    #[error("listener error: {0}")]
    Listener(ListenerError),
    #[error("no such subscription")]
    NoSuchSubscription,
    #[error("view error: {0}")]
    View(ViewError),
}

impl From<ListenerError> for ApplicationError {
    fn from(e: ListenerError) -> Self {
        Self::Listener(e)
    }
}

impl From<ViewError> for ApplicationError {
    fn from(e: ViewError) -> Self {
        Self::View(e)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mock::{MockBarInput, MockEvent, MockFooInput, MockModel, MockMsg, MockPoll};
    use crate::{StateValue, SubClause};

    use pretty_assertions::assert_eq;
    use std::time::Duration;

    const INPUT_FOO: &'static str = "INPUT_FOO";
    const INPUT_BAR: &'static str = "INPUT_BAR";

    #[test]
    fn should_initialize_application() {
        let application: Application<MockMsg, MockEvent> = Application::init(listener_config());
        assert!(application.subs.is_empty());
        assert_eq!(application.view.mounted(INPUT_FOO), false);
    }

    #[test]
    fn should_restart_listener() {
        let mut application: Application<MockMsg, MockEvent> = Application::init(listener_config());
        assert!(application.restart_listener(listener_config()).is_ok());
    }

    #[test]
    fn should_manipulate_components() {
        let mut application: Application<MockMsg, MockEvent> = Application::init(listener_config());
        // Mount
        assert!(application
            .mount(INPUT_FOO, Box::new(MockFooInput::default()), vec![])
            .is_ok());
        // Remount
        assert!(application
            .mount(INPUT_FOO, Box::new(MockFooInput::default()), vec![])
            .is_err());
        // Mount bar
        assert!(application
            .mount(INPUT_BAR, Box::new(MockBarInput::default()), vec![])
            .is_ok());
        // Mounted
        assert!(application.mounted(INPUT_FOO));
        assert!(application.mounted(INPUT_BAR));
        assert_eq!(application.mounted("CICCIO"), false);
        // Attribute and Query
        assert!(application
            .query(INPUT_FOO, Attribute::InputLength)
            .ok()
            .unwrap()
            .is_none());
        assert!(application
            .attr(INPUT_FOO, Attribute::InputLength, AttrValue::Length(8))
            .is_ok());
        assert_eq!(
            application
                .query(INPUT_FOO, Attribute::InputLength)
                .ok()
                .unwrap()
                .unwrap(),
            AttrValue::Length(8)
        );
        // State
        assert_eq!(
            application.state(INPUT_FOO).ok().unwrap(),
            State::One(StateValue::String(String::default()))
        );
        // Active / blur
        assert!(application.active(INPUT_FOO).is_ok());
        assert!(application.active(INPUT_BAR).is_ok());
        assert!(application.active("CICCIO").is_err());
        assert!(application.blur().is_ok());
        assert!(application.blur().is_ok());
        // no focus
        assert!(application.blur().is_err());
        // Umount
        assert!(application.umount(INPUT_FOO).is_ok());
        assert!(application.umount(INPUT_FOO).is_err());
        assert!(application.umount(INPUT_BAR).is_ok());
    }

    #[test]
    fn should_subscribe_components() {
        let mut application: Application<MockMsg, MockEvent> = Application::init(listener_config());
        assert!(application
            .mount(
                INPUT_FOO,
                Box::new(MockFooInput::default()),
                vec![
                    Sub::new(SubEventClause::Tick, SubClause::Always),
                    Sub::new(
                        SubEventClause::Tick,
                        SubClause::HasAttrValue(Attribute::InputLength, AttrValue::Length(8))
                    ), // NOTE: This event will be ignored
                    Sub::new(
                        SubEventClause::User(MockEvent::Bar),
                        SubClause::HasAttrValue(Attribute::Focus, AttrValue::Flag(true))
                    )
                ]
            )
            .is_ok());
        assert_eq!(application.subs.len(), 2);
        // Subscribe for another event
        assert!(application
            .subscribe(
                INPUT_FOO,
                Sub::new(
                    SubEventClause::User(MockEvent::Foo),
                    SubClause::HasAttrValue(Attribute::Focus, AttrValue::Flag(false))
                )
            )
            .is_ok());
        assert_eq!(application.subs.len(), 3);
        // Try to re-subscribe
        assert!(application
            .subscribe(
                INPUT_FOO,
                Sub::new(
                    SubEventClause::User(MockEvent::Foo),
                    SubClause::HasAttrValue(Attribute::Focus, AttrValue::Flag(false))
                )
            )
            .is_err());
        // Subscribe for unexisting component
        assert!(application
            .subscribe(
                INPUT_BAR,
                Sub::new(
                    SubEventClause::User(MockEvent::Foo),
                    SubClause::HasAttrValue(Attribute::Focus, AttrValue::Flag(false))
                )
            )
            .is_err());
        // Unsubscribe element
        assert!(application
            .unsubscribe(INPUT_FOO, SubEventClause::User(MockEvent::Foo))
            .is_ok());
        // Unsubcribe twice
        assert!(application
            .unsubscribe(INPUT_FOO, SubEventClause::User(MockEvent::Foo))
            .is_err());
    }

    #[test]
    fn should_do_tick() {
        let mut application: Application<MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
        let mut model = MockModel::new(validate_should_do_tick);
        // Mount foo and bar
        assert!(application
            .mount(INPUT_FOO, Box::new(MockFooInput::default()), vec![])
            .is_ok());
        assert!(application
            .mount(
                INPUT_BAR,
                Box::new(MockFooInput::default()),
                vec![Sub::new(SubEventClause::Tick, SubClause::Always)]
            )
            .is_ok());
        // Active FOO
        assert!(application.active(INPUT_FOO).is_ok());
        /*
         * Here we should:
         *
         * - receive an Enter from MockPoll, sent to FOO and will return a `FooSubmit`
         * - receive a Tick from MockPoll, sent to FOO, but won't return a msg
         * - the Tick will be sent also to BAR since is subscribed and will return a `BarTick`
         */
        let msg = application.tick(PollStrategy::UpTo(5)).ok().unwrap();
        assert_eq!(msg.len(), 2);
        // Update
        application.update(&mut model, msg);
        // Active BAR
        assert!(application.active(INPUT_BAR).is_ok());
        // Tick
        /*
         * Here we should:
         *
         * - receive an Enter from MockPoll, sent to BAR and will return a `BarSubmit`
         */
        let msg = application.tick(PollStrategy::Once).ok().unwrap();
        assert_eq!(msg.len(), 1);
        application.update(&mut model, msg);
    }

    fn listener_config() -> EventListenerCfg<MockEvent> {
        EventListenerCfg::default().port(
            Box::new(MockPoll::<MockEvent>::default()),
            Duration::from_millis(100),
        )
    }

    fn listener_config_with_tick(tick: Duration) -> EventListenerCfg<MockEvent> {
        listener_config().tick_interval(tick)
    }

    fn validate_should_do_tick(msg: Option<MockMsg>) -> Option<MockMsg> {
        /*
        Allowed messages:
            - FooSubmit
            - BarTick
        */
        assert!(matches!(
            msg.unwrap(),
            MockMsg::FooSubmit(_) | MockMsg::BarTick | MockMsg::BarSubmit(_)
        ));
        None
    }
}
