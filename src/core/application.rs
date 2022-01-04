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
use super::{Subscription, View, WrappedComponent};
use crate::listener::{EventListener, EventListenerCfg, ListenerError};
use crate::tui::layout::Rect;
use crate::{AttrValue, Attribute, Event, Frame, State, Sub, SubEventClause, ViewError};

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
    /// If true, subs won't be processed. (Default: False)
    sub_lock: bool,
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
            sub_lock: false,
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

    /// ### lock_ports
    ///
    /// Lock ports. As long as Ports are locked, ports won't be polled.
    /// Locking ports will also prevent Tick events from being generated.
    pub fn lock_ports(&mut self) -> ApplicationResult<()> {
        self.listener.pause().map_err(ApplicationError::from)
    }

    /// ### unlock_ports
    ///
    /// Unlock Ports. Once called, the event listener will resume polling Ports.
    pub fn unlock_ports(&mut self) -> ApplicationResult<()> {
        self.listener.unpause().map_err(ApplicationError::from)
    }

    /// ### tick
    ///
    /// The tick method makes the application to run once.
    /// The workflow of the tick method is the following one:
    ///
    /// 1. The event listener is fetched according to the provided `PollStrategy`
    /// 2. All the received events are sent to the current active component
    /// 3. All the received events are forwarded to the subscribed components which satisfy the received events and conditions.
    /// 4. Returns messages to process
    ///
    /// As soon as function returns, you should call the `view()` method.
    ///
    /// > You can also call `view` from the `update()` if you need it
    pub fn tick(&mut self, strategy: PollStrategy) -> ApplicationResult<Vec<Msg>> {
        // Poll event listener
        let events = self.poll(strategy)?;
        // Forward to active element
        let mut messages: Vec<Msg> = events
            .iter()
            .map(|x| self.forward_to_active_component(x.clone()))
            .flatten()
            .collect();
        // Forward to subscriptions and extend vector
        if !self.sub_lock {
            messages.extend(self.forward_to_subscriptions(events));
        }
        Ok(messages)
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
        subs: Vec<Sub<K, UserEvent>>,
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

    /// ### remount
    ///
    /// Remount provided component.
    /// Returns Err if failed to mount. It ignores whether the component already exists or not.
    /// If component had focus, focus is preserved
    pub fn remount(
        &mut self,
        id: K,
        component: WrappedComponent<Msg, UserEvent>,
        subs: Vec<Sub<K, UserEvent>>,
    ) -> ApplicationResult<()> {
        let had_focus = self.view.has_focus(&id);
        let _ = self.umount(&id);
        self.mount(id.clone(), component, subs)?;
        // Keep focus if necessary
        if had_focus {
            self.active(&id)
        } else {
            Ok(())
        }
    }

    /// ### umount_all
    ///
    /// Umount all components in the view and removed all associated subscriptions
    pub fn umount_all(&mut self) {
        self.view.umount_all();
        self.subs.clear();
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

    /// Get a reference to the id of the current active component in the view
    pub fn focus(&self) -> Option<&K> {
        self.view.focus()
    }

    // -- subs bridge

    /// ### subscribe
    ///
    /// Subscribe component to a certain event.
    /// Returns Error if the component doesn't exist or if the component is already subscribed to this event
    pub fn subscribe(&mut self, id: &K, sub: Sub<K, UserEvent>) -> ApplicationResult<()> {
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

    /// ### lock_subs
    ///
    /// Lock subscriptions. As long as the subscriptions are locked, events won't be propagated to
    /// subscriptions.
    pub fn lock_subs(&mut self) {
        self.sub_lock = true;
    }

    /// ### unlock_subs
    ///
    /// Unlock subscriptions. Application will now resume propagating events to subscriptions.
    pub fn unlock_subs(&mut self) {
        self.sub_lock = false;
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
            PollStrategy::TryFor(timeout) => self.poll_with_timeout(timeout),
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
                    |id, q| self.view.query(id, q).ok().flatten(),
                    |id| self.view.state(id).ok(),
                    |id| self.view.mounted(id),
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
}

/// ## PollStrategy
///
/// Poll strategy defines how to call `poll` on the event listener.
pub enum PollStrategy {
    /// The poll() function will be called once
    Once,
    /// The application will keep waiting for events for the provided duration
    TryFor(Duration),
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
    use crate::event::{Key, KeyEvent};
    use crate::mock::{MockBarInput, MockComponentId, MockEvent, MockFooInput, MockMsg, MockPoll};
    use crate::{StateValue, SubClause};

    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[test]
    fn should_initialize_application() {
        let application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config());
        assert!(application.subs.is_empty());
        assert_eq!(application.view.mounted(&MockComponentId::InputFoo), false);
        assert_eq!(application.sub_lock, false);
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
        // Remount with mount
        assert!(application
            .mount(
                MockComponentId::InputFoo,
                Box::new(MockFooInput::default()),
                vec![]
            )
            .is_err());
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(application.focus().unwrap(), &MockComponentId::InputFoo);
        // Remount
        assert!(application
            .remount(
                MockComponentId::InputFoo,
                Box::new(MockFooInput::default()),
                vec![]
            )
            .is_ok());
        assert!(application.view.has_focus(&MockComponentId::InputFoo));
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
                        SubClause::HasAttrValue(
                            MockComponentId::InputFoo,
                            Attribute::InputLength,
                            AttrValue::Length(8)
                        )
                    ), // NOTE: This event will be ignored
                    Sub::new(
                        SubEventClause::User(MockEvent::Bar),
                        SubClause::HasAttrValue(
                            MockComponentId::InputFoo,
                            Attribute::Focus,
                            AttrValue::Flag(true)
                        )
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
                    SubClause::HasAttrValue(
                        MockComponentId::InputFoo,
                        Attribute::Focus,
                        AttrValue::Flag(false)
                    )
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
                    SubClause::HasAttrValue(
                        MockComponentId::InputFoo,
                        Attribute::Focus,
                        AttrValue::Flag(false)
                    )
                )
            )
            .is_err());
        // Subscribe for unexisting component
        assert!(application
            .subscribe(
                &MockComponentId::InputBar,
                Sub::new(
                    SubEventClause::User(MockEvent::Foo),
                    SubClause::HasAttrValue(
                        MockComponentId::InputBar,
                        Attribute::Focus,
                        AttrValue::Flag(false)
                    )
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
    fn should_umount_all() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config());
        assert!(application
            .mount(
                MockComponentId::InputFoo,
                Box::new(MockFooInput::default()),
                vec![
                    Sub::new(SubEventClause::Tick, SubClause::Always),
                    Sub::new(
                        SubEventClause::User(MockEvent::Bar),
                        SubClause::HasAttrValue(
                            MockComponentId::InputFoo,
                            Attribute::Focus,
                            AttrValue::Flag(true)
                        )
                    )
                ]
            )
            .is_ok());
        assert!(application
            .mount(
                MockComponentId::InputBar,
                Box::new(MockFooInput::default()),
                vec![Sub::new(SubEventClause::Any, SubClause::Always)]
            )
            .is_ok());
        assert_eq!(application.subs.len(), 3);
        // Let's umount all
        application.umount_all();
        assert_eq!(application.mounted(&MockComponentId::InputFoo), false);
        assert_eq!(application.mounted(&MockComponentId::InputBar), false);
        assert!(application.subs.is_empty());
    }

    #[test]
    fn should_do_tick() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
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
                        SubClause::HasAttrValue(
                            MockComponentId::InputBar,
                            Attribute::Focus,
                            AttrValue::Flag(true)
                        )
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
                .tick(PollStrategy::UpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::from("")), MockMsg::BarTick]
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
                .tick(PollStrategy::Once)
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::BarSubmit(String::from(""))]
        );
        // Let's try TryFor strategy
        let events = application
            .tick(PollStrategy::TryFor(Duration::from_millis(300)))
            .ok()
            .unwrap();
        assert!(events.len() >= 2);
    }

    #[test]
    fn should_not_propagate_event_when_subs_are_locked() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
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
                        SubClause::HasAttrValue(
                            MockComponentId::InputBar,
                            Attribute::Focus,
                            AttrValue::Flag(true)
                        )
                    )
                ]
            )
            .is_ok());
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        // lock subs
        application.lock_subs();
        assert_eq!(application.sub_lock, true);
        assert_eq!(
            application
                .tick(PollStrategy::UpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::from(""))]
        );
        // unlock subs
        application.unlock_subs();
        assert_eq!(application.sub_lock, false);
    }

    #[test]
    fn should_not_propagate_events_if_has_attr_cond_is_not_satisfied() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
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
                vec![Sub::new(
                    // NOTE: won't be thrown, since requires focus
                    SubEventClause::Tick,
                    SubClause::HasAttrValue(
                        MockComponentId::InputBar,
                        Attribute::Focus,
                        AttrValue::Flag(true)
                    )
                )]
            )
            .is_ok());
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(
            application
                .tick(PollStrategy::UpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::from(""))]
        );
    }

    #[test]
    fn should_propagate_events_if_has_attr_cond_is_satisfied() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
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
                vec![Sub::new(
                    SubEventClause::Tick,
                    SubClause::HasAttrValue(
                        MockComponentId::InputFoo,
                        Attribute::Focus,
                        AttrValue::Flag(true)
                    )
                )]
            )
            .is_ok());
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(
            application
                .tick(PollStrategy::UpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::from("")), MockMsg::BarTick]
        );
    }

    #[test]
    fn should_not_propagate_events_if_has_state_cond_is_not_satisfied() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
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
                vec![Sub::new(
                    SubEventClause::Tick,
                    SubClause::HasState(MockComponentId::InputFoo, State::None)
                )]
            )
            .is_ok());
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(
            application
                .tick(PollStrategy::UpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::from(""))]
        );
    }

    #[test]
    fn should_propagate_events_if_has_state_cond_is_satisfied() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
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
                vec![Sub::new(
                    SubEventClause::Tick,
                    SubClause::HasState(
                        MockComponentId::InputFoo,
                        State::One(StateValue::String(String::new()))
                    )
                )]
            )
            .is_ok());
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        // No event should be generated
        assert_eq!(
            application
                .tick(PollStrategy::UpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::from("")), MockMsg::BarTick]
        );
    }

    #[test]
    fn should_not_propagate_events_if_is_mounted_cond_is_not_satisfied() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
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
                vec![Sub::new(
                    SubEventClause::Tick,
                    SubClause::IsMounted(MockComponentId::InputOmar)
                )]
            )
            .is_ok());
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(
            application
                .tick(PollStrategy::UpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::from(""))]
        );
    }

    #[test]
    fn should_propagate_events_if_is_mounted_cond_is_not_satisfied() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
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
                vec![Sub::new(
                    SubEventClause::Tick,
                    SubClause::IsMounted(MockComponentId::InputFoo)
                )]
            )
            .is_ok());
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(
            application
                .tick(PollStrategy::UpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::from("")), MockMsg::BarTick]
        );
    }

    #[test]
    fn should_lock_ports() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_millis(500)));
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
                vec![Sub::new(
                    SubEventClause::Tick,
                    SubClause::IsMounted(MockComponentId::InputFoo)
                )]
            )
            .is_ok());
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(
            application
                .tick(PollStrategy::UpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::from("")), MockMsg::BarTick]
        );
        // Lock ports
        assert!(application.lock_ports().is_ok());
        // Wait 1 sec
        std::thread::sleep(Duration::from_millis(1000));
        // Tick ( No tick event )
        assert_eq!(
            application
                .tick(PollStrategy::UpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[]
        );
        // Unlock ports
        assert!(application.unlock_ports().is_ok());
        // Wait 100 ms
        std::thread::sleep(Duration::from_millis(50));
        // Tick
        assert_eq!(
            application
                .tick(PollStrategy::UpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::from("")), MockMsg::BarTick]
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
}
