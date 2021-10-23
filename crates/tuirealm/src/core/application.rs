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

use std::hash::Hash;
use std::time::{Duration, Instant};
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
pub struct Application<ComponentId, Msg, UserEvent>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
{
    listener: EventListener<UserEvent>,
    subs: Vec<Subscription<ComponentId, UserEvent>>,
    view: View<ComponentId, Msg, UserEvent>,
}

impl<K, Msg, UserEvent> Application<K, Msg, UserEvent>
where
    K: Eq + PartialEq + Clone + Hash,
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone + PartialOrd + Send + 'static,
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
    /// 4. Call update() on model passing each message
    /// 5. Returns the amount of processed messages
    ///
    /// As soon as function returns, you should call the `view()` method.
    ///
    /// > You can also call `view` from the `update()` if you need it
    pub fn tick(
        &mut self,
        model: &mut dyn Update<K, Msg, UserEvent>,
        strategy: PollStrategy,
    ) -> ApplicationResult<usize> {
        // Poll event listener
        let events = self.poll(strategy)?;
        // Forward to active element
        let mut messages: Vec<Msg> = events
            .iter()
            .map(|x| self.forward_to_active_component(x.clone()))
            .flatten()
            .collect();
        // Forward to subscriptions and extend vector
        messages.extend(self.forward_to_subscriptions(events));
        // Update
        let msg_len = messages.len();
        messages.into_iter().for_each(|msg| {
            assert!(self.recurse_update(model, Some(msg)).is_none());
        });
        Ok(msg_len)
    }

    // -- view bridge

    /// ### mount
    ///
    /// Mount component to view and associate subscriptions for it.
    /// Returns error if component is already mounted
    /// NOTE: if subs vector contains duplicated, these will be discarded
    pub fn mount(
        &mut self,
        id: K,
        component: WrappedComponent<Msg, UserEvent>,
        subs: Vec<Sub<UserEvent>>,
    ) -> ApplicationResult<()> {
        // Mount
        self.view.mount(id.clone(), component)?;
        // Subscribe
        subs.into_iter().for_each(|x| {
            // Push only if not already subscribed
            let subscription = Subscription::new(id.clone(), x);
            if !self.subscribed(&id, subscription.event()) {
                self.subs.push(subscription);
            }
        });
        Ok(())
    }

    /// ### umount
    ///
    /// Umount component associated to `id` and remove ALL its SUBSCRIPTIONS.
    /// Returns Error if the component doesn't exist
    pub fn umount(&mut self, id: &K) -> ApplicationResult<()> {
        self.view.umount(id)?;
        self.unsubscribe_component(id);
        Ok(())
    }

    /// ### mounted
    ///
    /// Returns whether component `id` is mounted
    pub fn mounted(&self, id: &K) -> bool {
        self.view.mounted(id)
    }

    /// ### view
    ///
    /// Render component called `id`
    pub fn view(&mut self, id: &K, f: &mut Frame, area: Rect) {
        self.view.view(id, f, area);
    }

    /// ### query
    ///
    /// Query view component for a certain `AttrValue`
    /// Returns error if the component doesn't exist
    /// Returns None if the attribute doesn't exist.
    pub fn query(&self, id: &K, query: Attribute) -> ApplicationResult<Option<AttrValue>> {
        self.view.query(id, query).map_err(ApplicationError::from)
    }

    /// ### attr
    ///
    /// Set attribute for component `id`
    /// Returns error if the component doesn't exist
    pub fn attr(&mut self, id: &K, attr: Attribute, value: AttrValue) -> ApplicationResult<()> {
        self.view
            .attr(id, attr, value)
            .map_err(ApplicationError::from)
    }

    /// ### state
    ///
    /// Get state for component `id`.
    /// Returns `Err` if component doesn't exist
    pub fn state(&self, id: &K) -> ApplicationResult<State> {
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
    pub fn active(&mut self, id: &K) -> ApplicationResult<()> {
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
    pub fn subscribe(&mut self, id: &K, sub: Sub<UserEvent>) -> ApplicationResult<()> {
        if !self.view.mounted(id) {
            return Err(ViewError::ComponentNotFound.into());
        }
        let subscription = Subscription::new(id.clone(), sub);
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
    pub fn unsubscribe(&mut self, id: &K, ev: SubEventClause<UserEvent>) -> ApplicationResult<()> {
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
    fn unsubscribe_component(&mut self, id: &K) {
        self.subs.retain(|x| x.target() != id)
    }

    /// ### subscribed
    ///
    /// Returns whether component `id` is subscribed to event described by `clause`
    fn subscribed(&self, id: &K, clause: &SubEventClause<UserEvent>) -> bool {
        self.subs
            .iter()
            .any(|s| s.target() == id && s.event() == clause)
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
            PollStrategy::TryFor(timeout) => self.poll_with_timeout(timeout),
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

    /// ### poll_with_timeout
    ///
    /// Poll event listener until `timeout` is elapsed
    fn poll_with_timeout(&mut self, timeout: Duration) -> ApplicationResult<Vec<Event<UserEvent>>> {
        let started = Instant::now();
        let mut evs: Vec<Event<UserEvent>> = Vec::new();
        while started.elapsed() < timeout {
            match self.poll_listener() {
                Err(err) => return Err(err),
                Ok(None) => continue,
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
            .cloned()
            .map(|x| self.view.forward(&x, ev).ok().unwrap())
            .flatten()
    }

    /// ### forward_to_subscriptions
    ///
    /// Forward events to subscriptions listening to the incoming event.
    fn forward_to_subscriptions(&mut self, events: Vec<Event<UserEvent>>) -> Vec<Msg> {
        let mut messages: Vec<Msg> = Vec::new();
        // NOTE: don't touch this code again and don't try to use iterators, cause it's not gonna work :)
        for ev in events.iter() {
            for sub in self.subs.iter() {
                // ! Active component must be different from sub !
                if self.view.has_focus(sub.target()) {
                    continue;
                }
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
        model: &mut dyn Update<K, Msg, UserEvent>,
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
    /// The application will keep waiting for events for the provided duration
    TryFor(Duration),
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
    use crate::event::{Key, KeyEvent};
    use crate::mock::{
        MockBarInput, MockComponentId, MockEvent, MockFooInput, MockModel, MockMsg, MockPoll,
    };
    use crate::{StateValue, SubClause};

    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[test]
    fn should_initialize_application() {
        let application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config());
        assert!(application.subs.is_empty());
        assert_eq!(application.view.mounted(&MockComponentId::InputFoo), false);
    }

    #[test]
    fn should_restart_listener() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config());
        assert!(application.restart_listener(listener_config()).is_ok());
    }

    #[test]
    fn should_manipulate_components() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config());
        // Mount
        assert!(application
            .mount(
                MockComponentId::InputFoo,
                Box::new(MockFooInput::default()),
                vec![]
            )
            .is_ok());
        // Remount
        assert!(application
            .mount(
                MockComponentId::InputFoo,
                Box::new(MockFooInput::default()),
                vec![]
            )
            .is_err());
        // Mount bar
        assert!(application
            .mount(
                MockComponentId::InputBar,
                Box::new(MockBarInput::default()),
                vec![]
            )
            .is_ok());
        // Mounted
        assert!(application.mounted(&MockComponentId::InputFoo));
        assert!(application.mounted(&MockComponentId::InputBar));
        assert_eq!(application.mounted(&MockComponentId::InputOmar), false);
        // Attribute and Query
        assert!(application
            .query(&MockComponentId::InputFoo, Attribute::InputLength)
            .ok()
            .unwrap()
            .is_none());
        assert!(application
            .attr(
                &MockComponentId::InputFoo,
                Attribute::InputLength,
                AttrValue::Length(8)
            )
            .is_ok());
        assert_eq!(
            application
                .query(&MockComponentId::InputFoo, Attribute::InputLength)
                .ok()
                .unwrap()
                .unwrap(),
            AttrValue::Length(8)
        );
        // State
        assert_eq!(
            application.state(&MockComponentId::InputFoo).ok().unwrap(),
            State::One(StateValue::String(String::default()))
        );
        // Active / blur
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert!(application.active(&MockComponentId::InputBar).is_ok());
        assert!(application.active(&MockComponentId::InputOmar).is_err());
        assert!(application.blur().is_ok());
        assert!(application.blur().is_ok());
        // no focus
        assert!(application.blur().is_err());
        // Umount
        assert!(application.umount(&MockComponentId::InputFoo).is_ok());
        assert!(application.umount(&MockComponentId::InputFoo).is_err());
        assert!(application.umount(&MockComponentId::InputBar).is_ok());
    }

    #[test]
    fn should_subscribe_components() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config());
        assert!(application
            .mount(
                MockComponentId::InputFoo,
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
                &MockComponentId::InputFoo,
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
                &MockComponentId::InputFoo,
                Sub::new(
                    SubEventClause::User(MockEvent::Foo),
                    SubClause::HasAttrValue(Attribute::Focus, AttrValue::Flag(false))
                )
            )
            .is_err());
        // Subscribe for unexisting component
        assert!(application
            .subscribe(
                &MockComponentId::InputBar,
                Sub::new(
                    SubEventClause::User(MockEvent::Foo),
                    SubClause::HasAttrValue(Attribute::Focus, AttrValue::Flag(false))
                )
            )
            .is_err());
        // Unsubscribe element
        assert!(application
            .unsubscribe(
                &MockComponentId::InputFoo,
                SubEventClause::User(MockEvent::Foo)
            )
            .is_ok());
        // Unsubcribe twice
        assert!(application
            .unsubscribe(
                &MockComponentId::InputFoo,
                SubEventClause::User(MockEvent::Foo)
            )
            .is_err());
    }

    #[test]
    fn should_do_tick() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
        let mut model = MockModel::new(validate_should_do_tick);
        // Mount foo and bar
        assert!(application
            .mount(
                MockComponentId::InputFoo,
                Box::new(MockFooInput::default()),
                vec![]
            )
            .is_ok());
        assert!(application
            .mount(
                MockComponentId::InputBar,
                Box::new(MockBarInput::default()),
                vec![
                    Sub::new(SubEventClause::Tick, SubClause::Always),
                    Sub::new(
                        // NOTE: won't be thrown, since requires focus
                        SubEventClause::Keyboard(KeyEvent::from(Key::Enter)),
                        SubClause::HasAttrValue(Attribute::Focus, AttrValue::Flag(true))
                    )
                ]
            )
            .is_ok());
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        /*
         * Here we should:
         *
         * - receive an Enter from MockPoll, sent to FOO and will return a `FooSubmit`
         * - receive a Tick from MockPoll, sent to FOO, but won't return a msg
         * - the Tick will be sent also to BAR since is subscribed and will return a `BarTick`
         */
        assert_eq!(
            application
                .tick(&mut model, PollStrategy::UpTo(5))
                .ok()
                .unwrap(),
            2
        );
        // Active BAR
        assert!(application.active(&MockComponentId::InputBar).is_ok());
        // Wait 200ms (wait for poll)
        std::thread::sleep(Duration::from_millis(100));
        /*
         * Here we should:
         *
         * - receive an Enter from MockPoll, sent to BAR and will return a `BarSubmit`
         */
        assert_eq!(
            application
                .tick(&mut model, PollStrategy::Once)
                .ok()
                .unwrap(),
            1
        );
        // Let's try TryFor strategy
        assert!(
            application
                .tick(&mut model, PollStrategy::TryFor(Duration::from_millis(300)))
                .ok()
                .unwrap()
                >= 3
        );
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
