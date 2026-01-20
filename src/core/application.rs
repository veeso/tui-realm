//! ## Application
//!
//! This module exposes the Application, which is the core struct of tui-realm.

use std::hash::Hash;
use std::time::{Duration, Instant};

use ratatui::Frame;
use thiserror::Error;

use super::{Subscription, View, WrappedComponent};
use crate::listener::{EventListener, EventListenerCfg, ListenerError, PollError};
use crate::ratatui::layout::Rect;
use crate::{
    AttrValue, Attribute, Component, Event, Injector, State, Sub, SubEventClause, ViewError,
};

/// Result retuned by [`Application`].
/// Ok depends on method
/// Err is always [`ApplicationError`]
pub type ApplicationResult<T> = Result<T, ApplicationError>;

/// The application defines a tui-realm application.
/// It will handle events, subscriptions and the view too.
/// It provides functions to interact with the view (mount, umount, query, etc), but also
/// the main function: [`Application::tick`].
pub struct Application<ComponentId, Msg, UserEvent>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    Msg: PartialEq,
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    listener: EventListener<UserEvent>,
    subs: Vec<Subscription<ComponentId, UserEvent>>,
    /// If true, subs won't be processed. (Default: False)
    sub_lock: bool,
    view: View<ComponentId, Msg, UserEvent>,
}

impl<ComponentId, Msg, UserEvent> Application<ComponentId, Msg, UserEvent>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
    Msg: PartialEq + 'static,
    UserEvent: Eq + PartialEq + Clone + Send + 'static,
{
    /// Initialize a new [`Application`].
    /// The event listener is immediately created and started.
    pub fn init(listener_cfg: EventListenerCfg<UserEvent>) -> Self {
        // TODO: maybe consider bubbling this up?
        let listener = listener_cfg
            .start()
            .expect("EventListenerCfg to be configured correctly");

        Self {
            listener,
            subs: Vec::new(),
            sub_lock: false,
            view: View::default(),
        }
    }

    /// Restart listener in case the previous listener has died or if you want to start a new one with a new configuration.
    ///
    /// > The listener has died if you received a [`ApplicationError::Listener(ListenerError::ListenerDied))`](ApplicationError::Listener).
    pub fn restart_listener(
        &mut self,
        listener_cfg: EventListenerCfg<UserEvent>,
    ) -> ApplicationResult<()> {
        self.listener.stop()?;
        self.listener = listener_cfg.start()?;
        Ok(())
    }

    /// Lock ports. As long as Ports are locked, ports won't be polled.
    /// Locking ports will also prevent Tick events from being generated.
    pub fn lock_ports(&mut self) -> ApplicationResult<()> {
        self.listener.pause().map_err(ApplicationError::from)
    }

    /// Unlock Ports. Once called, the event listener will resume polling Ports.
    pub fn unlock_ports(&mut self) -> ApplicationResult<()> {
        self.listener.unpause().map_err(ApplicationError::from)
    }

    /// The tick method makes the application to run once.
    /// The workflow of the tick method is the following one:
    ///
    /// 1. The event listener is fetched according to the provided [`PollStrategy`]
    /// 2. All the received events are sent to the current active component
    /// 3. All the received events are forwarded to the subscribed components which satisfy the received events and conditions.
    /// 4. Returns messages to process
    ///
    /// As soon as function returns, you should call the [`Application::view`] method.
    ///
    /// > You can also call [`Application::view`] from the [`crate::Update`] if you need it
    pub fn tick(&mut self, strategy: PollStrategy) -> ApplicationResult<Vec<Msg>> {
        // Poll event listener
        let events = self.poll(strategy)?;
        // Forward to active element
        let mut messages: Vec<Msg> = events
            .iter()
            .filter_map(|x| self.forward_to_active_component(x))
            .collect();
        // Forward to subscriptions and extend vector
        if !self.sub_lock {
            self.forward_to_subscriptions(&events, &mut messages);
        }
        Ok(messages)
    }

    // -- view bridge

    /// Add an injector to the view
    pub fn add_injector(&mut self, injector: Box<dyn Injector<ComponentId>>) {
        self.view.add_injector(injector);
    }

    /// Mount component to view and associate subscriptions for it.
    /// Returns error if component is already mounted
    /// NOTE: if subs vector contains duplicated, these will be discarded
    pub fn mount(
        &mut self,
        id: ComponentId,
        component: WrappedComponent<Msg, UserEvent>,
        subs: Vec<Sub<ComponentId, UserEvent>>,
    ) -> ApplicationResult<()> {
        // Mount
        self.view.mount(&id, component)?;
        // Subscribe
        self.insert_subscriptions(&id, subs);
        Ok(())
    }

    /// Umount component associated to `id` and remove ALL its SUBSCRIPTIONS.
    /// Returns Error if the component doesn't exist
    pub fn umount(&mut self, id: &ComponentId) -> ApplicationResult<()> {
        self.view.umount(id)?;
        self.unsubscribe_component(id);
        Ok(())
    }

    /// Remount provided component.
    /// Returns Err if failed to mount. It ignores whether the component already exists or not.
    /// If component had focus, focus is preserved
    pub fn remount(
        &mut self,
        id: ComponentId,
        component: WrappedComponent<Msg, UserEvent>,
        subs: Vec<Sub<ComponentId, UserEvent>>,
    ) -> ApplicationResult<()> {
        // remove subs
        self.unsubscribe_component(&id);
        // remount into view
        self.view.remount(&id, component)?;
        // re-add subs
        self.insert_subscriptions(&id, subs);
        Ok(())
    }

    /// Umount all components in the view and removed all associated subscriptions
    pub fn umount_all(&mut self) {
        self.view.umount_all();
        self.subs.clear();
    }

    /// Returns whether component `id` is mounted
    pub fn mounted(&self, id: &ComponentId) -> bool {
        self.view.mounted(id)
    }

    /// Render component called `id`
    pub fn view(&mut self, id: &ComponentId, f: &mut Frame, area: Rect) {
        self.view.view(id, f, area);
    }

    /// Query view component for a certain `AttrValue`
    /// Returns error if the component doesn't exist
    /// Returns None if the attribute doesn't exist.
    pub fn query(
        &self,
        id: &ComponentId,
        query: Attribute,
    ) -> ApplicationResult<Option<AttrValue>> {
        self.view.query(id, query).map_err(ApplicationError::from)
    }

    /// Set attribute for component `id`
    /// Returns error if the component doesn't exist
    pub fn attr(
        &mut self,
        id: &ComponentId,
        attr: Attribute,
        value: AttrValue,
    ) -> ApplicationResult<()> {
        self.view
            .attr(id, attr, value)
            .map_err(ApplicationError::from)
    }

    /// Get state for component `id`.
    /// Returns `Err` if component doesn't exist
    pub fn state(&self, id: &ComponentId) -> ApplicationResult<State> {
        self.view.state(id).map_err(ApplicationError::from)
    }

    /// Shorthand for `attr(id, Attribute::Focus(AttrValue::Flag(true)))`.
    /// It also sets the component as the current one having focus.
    /// Previous active component, if any, GETS PUSHED to the STACK
    /// Returns error: if component doesn't exist. Use `mounted()` to check if component exists
    ///
    /// > NOTE: users should always use this function to give focus to components.
    pub fn active(&mut self, id: &ComponentId) -> ApplicationResult<()> {
        self.view.active(id).map_err(ApplicationError::from)
    }

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
    pub fn focus(&self) -> Option<&ComponentId> {
        self.view.focus()
    }

    /// Get a reference to the registered component for the given `id`, if there is one.
    pub fn get_component(&self, id: &ComponentId) -> Option<&dyn Component<Msg, UserEvent>> {
        self.view.get_component(id)
    }

    /// Get a mutable reference to the registered component for the given `id`, if there is one.
    pub fn get_component_mut(
        &mut self,
        id: &ComponentId,
    ) -> Option<&mut dyn Component<Msg, UserEvent>> {
        self.view.get_component_mut(id)
    }

    // -- subs bridge

    /// Subscribe component to a certain event.
    /// Returns Error if the component doesn't exist or if the component is already subscribed to this event
    pub fn subscribe(
        &mut self,
        id: &ComponentId,
        sub: Sub<ComponentId, UserEvent>,
    ) -> ApplicationResult<()> {
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

    /// Unsubscribe a component from a certain event.
    /// Returns error if the component doesn't exist or if the component is not subscribed to this event
    pub fn unsubscribe(
        &mut self,
        id: &ComponentId,
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

    /// Lock subscriptions. As long as the subscriptions are locked, events won't be propagated to
    /// subscriptions.
    pub fn lock_subs(&mut self) {
        self.sub_lock = true;
    }

    /// Unlock subscriptions. Application will now resume propagating events to subscriptions.
    pub fn unlock_subs(&mut self) {
        self.sub_lock = false;
    }

    // -- private

    /// remove all subscriptions for component
    fn unsubscribe_component(&mut self, id: &ComponentId) {
        self.subs.retain(|x| x.target() != id);
    }

    /// Returns whether component `id` is subscribed to event described by `clause`
    fn subscribed(&self, id: &ComponentId, clause: &SubEventClause<UserEvent>) -> bool {
        self.subs
            .iter()
            .any(|s| s.target() == id && s.event() == clause)
    }

    /// Insert subscriptions
    fn insert_subscriptions(&mut self, id: &ComponentId, subs: Vec<Sub<ComponentId, UserEvent>>) {
        for sub in subs {
            // Push only if not already subscribed
            let subscription = Subscription::new(id.clone(), sub);
            if !self.subscribed(id, subscription.event()) {
                self.subs.push(subscription);
            }
        }
    }

    /// Poll listener according to provided strategy
    fn poll(&mut self, strategy: PollStrategy) -> ApplicationResult<Vec<Event<UserEvent>>> {
        match strategy {
            PollStrategy::Once => self
                .poll_listener_timeout()
                .map(|x| x.map(|x| vec![x]).unwrap_or_default()),
            PollStrategy::TryFor(timeout) => self.poll_try_for(timeout),
            PollStrategy::UpTo(times) => self.poll_upto(times),
            PollStrategy::UpToNoWait(times) => self.poll_upto_nowait(times),
            PollStrategy::BlockCollectUpTo(times) => self.poll_blocking_upto(times),
        }
    }

    /// Poll event listener up to `t` times
    fn poll_upto(&mut self, t: usize) -> ApplicationResult<Vec<Event<UserEvent>>> {
        let mut evs: Vec<Event<UserEvent>> = Vec::with_capacity(t);
        for _ in 0..t {
            match self.poll_listener_timeout() {
                Err(err) => return Err(err),
                Ok(None) => break,
                Ok(Some(ev)) => evs.push(ev),
            }
        }
        Ok(evs)
    }

    /// Poll event listener up to `t` times, without waiting to return if there are events.
    fn poll_upto_nowait(&mut self, t: usize) -> ApplicationResult<Vec<Event<UserEvent>>> {
        if t == 0 {
            return Ok(Vec::new());
        }

        let mut evs: Vec<Event<UserEvent>> = Vec::with_capacity(t);

        match self.poll_listener_timeout() {
            Err(err) => return Err(err),
            Ok(None) => (),
            Ok(Some(ev)) => evs.push(ev),
        }

        let t = t.saturating_sub(1);

        for _ in 0..t {
            match self.try_poll_listener() {
                Err(err) => return Err(err),
                Ok(None) => break,
                Ok(Some(ev)) => evs.push(ev),
            }
        }
        Ok(evs)
    }

    /// Poll event listener up to `t` times, without waiting to return if there are events in a blocking fashion.
    fn poll_blocking_upto(&mut self, t: usize) -> ApplicationResult<Vec<Event<UserEvent>>> {
        if t == 0 {
            return Ok(Vec::new());
        }

        let mut evs: Vec<Event<UserEvent>> = Vec::with_capacity(t);

        match self.poll_listener_blocking() {
            Err(err) => return Err(err),
            Ok(ev) => evs.push(ev),
        }

        let t = t.saturating_sub(1);

        for _ in 0..t {
            match self.try_poll_listener() {
                Err(err) => return Err(err),
                Ok(None) => break,
                Ok(Some(ev)) => evs.push(ev),
            }
        }
        Ok(evs)
    }

    /// Poll event listener until `timeout` is elapsed
    fn poll_try_for(&mut self, timeout: Duration) -> ApplicationResult<Vec<Event<UserEvent>>> {
        let started = Instant::now();
        let mut evs: Vec<Event<UserEvent>> = Vec::new();
        while started.elapsed() < timeout {
            match self.poll_listener_timeout() {
                Err(err) => return Err(err),
                Ok(None) => continue,
                Ok(Some(ev)) => evs.push(ev),
            }
        }
        Ok(evs)
    }

    /// Poll event listener once with timeout
    fn poll_listener_timeout(&mut self) -> ApplicationResult<Option<Event<UserEvent>>> {
        self.listener.poll_timeout().map_err(ApplicationError::from)
    }

    /// Poll event listener once in a blocking fashion
    fn poll_listener_blocking(&mut self) -> ApplicationResult<Event<UserEvent>> {
        self.listener
            .poll_blocking()
            .map_err(ApplicationError::from)
    }

    /// Try to Poll event listener once, without blocking whatsoever
    fn try_poll_listener(&mut self) -> ApplicationResult<Option<Event<UserEvent>>> {
        self.listener.try_poll().map_err(ApplicationError::from)
    }

    /// Forward event to current active component, if any.
    fn forward_to_active_component(&mut self, ev: &Event<UserEvent>) -> Option<Msg> {
        self.view
            .focus()
            .cloned()
            .and_then(|x| self.view.forward(&x, ev).ok().unwrap())
    }

    /// Forward events to subscriptions listening to the incoming event.
    fn forward_to_subscriptions(&mut self, events: &[Event<UserEvent>], messages: &mut Vec<Msg>) {
        // NOTE: don't touch this code again and don't try to use iterators, cause it's not gonna work :)
        for ev in events {
            for sub in &self.subs {
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
                if let Some(msg) = self.view.forward(sub.target(), ev).ok().unwrap() {
                    messages.push(msg);
                }
            }
        }
    }
}

/// Poll strategy defines how to call `Application::poll` on the event listener.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PollStrategy {
    /// `Application::poll` function will be called once, with the timeout specified in `EventListenerCfg`.
    Once,
    /// The application will keep waiting for events for the provided duration.
    ///
    /// This will block until the full duration is over, regardless if there is a event or not.
    /// Will collect all events during that time.
    TryFor(Duration),
    /// `Application::poll` function will be called up to `n` times, until it will return [`Option::None`].
    ///
    /// This will block if there are not `n` events for up to the timeout specified in `EventListenerCfg`.
    UpTo(usize),

    /// Practially the same as [`UpTo`](PollStrategy::UpTo), only that it does not wait the timeout specified in `EventListenerCfg` if there is at least one event.
    ///
    /// This will because of that wait for the first event to be come available for `timeout`, but collect all remaining event without waiting.
    /// If there are enough event already, never blocks.
    ///
    /// This *might* become the default [`UpTo`](PollStrategy::UpTo) behavior in the future.
    UpToNoWait(usize),

    /// Block until there is at least one event available, and then collect `n-1` additional events, if available.
    ///
    /// This ingores the timeout set in `EventListenerCfg`.
    BlockCollectUpTo(usize),
}

// -- error

/// Error variants returned by [`Application`]
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("already subscribed")]
    AlreadySubscribed,
    #[error("listener error: {0}")]
    Listener(#[from] ListenerError),
    #[error("poll(): {0}")]
    Poll(#[from] PollError),
    #[error("no such subscription")]
    NoSuchSubscription,
    #[error("view error: {0}")]
    View(#[from] ViewError),
}

#[cfg(test)]
mod test {

    use std::time::Duration;

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::event::{Key, KeyEvent};
    use crate::mock::{
        MockBarInput, MockComponentId, MockEvent, MockFooInput, MockInjector, MockMsg, MockPoll,
    };
    use crate::{StateValue, SubClause};

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
        assert!(
            application
                .mount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_ok()
        );
        // Remount with mount
        assert!(
            application
                .mount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_err()
        );
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(application.focus().unwrap(), &MockComponentId::InputFoo);
        // Remount
        assert!(
            application
                .remount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_ok()
        );
        assert!(application.view.has_focus(&MockComponentId::InputFoo));
        // Mount bar
        assert!(
            application
                .mount(
                    MockComponentId::InputBar,
                    Box::new(MockBarInput::default()),
                    vec![]
                )
                .is_ok()
        );
        // Mounted
        assert!(application.mounted(&MockComponentId::InputFoo));
        assert!(application.mounted(&MockComponentId::InputBar));
        assert_eq!(application.mounted(&MockComponentId::InputOmar), false);
        // Attribute and Query
        assert!(
            application
                .query(&MockComponentId::InputFoo, Attribute::InputLength)
                .ok()
                .unwrap()
                .is_none()
        );
        assert!(
            application
                .attr(
                    &MockComponentId::InputFoo,
                    Attribute::InputLength,
                    AttrValue::Length(8)
                )
                .is_ok()
        );
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
            State::Single(StateValue::String(String::default()))
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
        assert!(
            application
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
                .is_ok()
        );
        assert_eq!(application.subs.len(), 2);
        // Subscribe for another event
        assert!(
            application
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
                .is_ok()
        );
        assert_eq!(application.subs.len(), 3);
        // Try to re-subscribe
        assert!(
            application
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
                .is_err()
        );
        // Subscribe for unexisting component
        assert!(
            application
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
                .is_err()
        );
        // Unsubscribe element
        assert!(
            application
                .unsubscribe(
                    &MockComponentId::InputFoo,
                    SubEventClause::User(MockEvent::Foo)
                )
                .is_ok()
        );
        // Unsubcribe twice
        assert!(
            application
                .unsubscribe(
                    &MockComponentId::InputFoo,
                    SubEventClause::User(MockEvent::Foo)
                )
                .is_err()
        );
    }

    #[test]
    fn should_umount_all() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config());
        assert!(
            application
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
                .is_ok()
        );
        assert!(
            application
                .mount(
                    MockComponentId::InputBar,
                    Box::new(MockFooInput::default()),
                    vec![Sub::new(SubEventClause::Any, SubClause::Always)]
                )
                .is_ok()
        );
        assert_eq!(application.subs.len(), 3);
        // Let's umount all
        application.umount_all();
        assert_eq!(application.mounted(&MockComponentId::InputFoo), false);
        assert_eq!(application.mounted(&MockComponentId::InputBar), false);
        assert!(application.subs.is_empty());
    }

    #[test]
    fn should_do_tick() {
        let mut listener = listener_config_with_tick(Duration::from_secs(60));
        let barrier_rx = listener.with_test_barrier();
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener);
        // Mount foo and bar
        assert!(
            application
                .mount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            application
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
                .is_ok()
        );
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());

        barrier_rx.recieve_cycle();

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
            &[MockMsg::FooSubmit(String::new()), MockMsg::BarTick]
        );
        // Active BAR
        assert!(application.active(&MockComponentId::InputBar).is_ok());

        barrier_rx.recieve_cycle();

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
            &[MockMsg::BarSubmit(String::new())]
        );

        barrier_rx.recieve_cycle();
        barrier_rx.recieve_cycle();

        let before = Instant::now();
        // Let's try TryFor strategy
        let events = application
            .tick(PollStrategy::TryFor(Duration::from_millis(400)))
            .ok()
            .unwrap();
        assert!(events.len() >= 2);
        assert!(before.elapsed() > Duration::from_millis(400));
        assert!(before.elapsed() < Duration::from_millis(500));
    }

    #[test]
    fn strategy_upto_nowait_should_work() {
        let mut listener =
            listener_config_with_tick(Duration::from_secs(60)).poll_timeout(Duration::from_secs(5));
        let barrier_rx = listener.with_test_barrier();
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener);

        // Mount foo and bar
        assert!(
            application
                .mount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            application
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
                .is_ok()
        );
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());

        barrier_rx.recieve_cycle();

        let before = Instant::now();

        assert_eq!(
            application
                .tick(PollStrategy::UpToNoWait(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::new()), MockMsg::BarTick]
        );

        // messages should be available, so "UpToNoWait" should not block again after the first event
        assert!(before.elapsed() < Duration::from_millis(100));
    }

    #[test]
    fn strategy_blocking_upto_should_work() {
        let mut listener = listener_config_with_tick(Duration::from_secs(60))
            .poll_timeout(Duration::from_secs(60));
        let barrier_rx = listener.with_test_barrier();
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener);

        // Mount foo and bar
        assert!(
            application
                .mount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            application
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
                .is_ok()
        );
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());

        barrier_rx.recieve_cycle();

        let before = Instant::now();

        assert_eq!(
            application
                .tick(PollStrategy::BlockCollectUpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::new()), MockMsg::BarTick]
        );

        // messages should be available, so "BlockingCollectUpTo" should not block again after the first event
        assert!(before.elapsed() < Duration::from_millis(100));
    }

    #[test]
    fn should_not_propagate_event_when_subs_are_locked() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
        // Mount foo and bar
        assert!(
            application
                .mount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            application
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
                .is_ok()
        );
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
            &[MockMsg::FooSubmit(String::new())]
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
        assert!(
            application
                .mount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            application
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
                .is_ok()
        );
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(
            application
                .tick(PollStrategy::BlockCollectUpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::new())]
        );
    }

    #[test]
    fn should_propagate_events_if_has_attr_cond_is_satisfied() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
        // Mount foo and bar
        assert!(
            application
                .mount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            application
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
                .is_ok()
        );
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(
            application
                .tick(PollStrategy::BlockCollectUpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::new()), MockMsg::BarTick]
        );
    }

    #[test]
    fn should_not_propagate_events_if_has_state_cond_is_not_satisfied() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
        // Mount foo and bar
        assert!(
            application
                .mount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            application
                .mount(
                    MockComponentId::InputBar,
                    Box::new(MockBarInput::default()),
                    vec![Sub::new(
                        SubEventClause::Tick,
                        SubClause::HasState(MockComponentId::InputFoo, State::None)
                    )]
                )
                .is_ok()
        );
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(
            application
                .tick(PollStrategy::BlockCollectUpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::new())]
        );
    }

    #[test]
    fn should_propagate_events_if_has_state_cond_is_satisfied() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
        // Mount foo and bar
        assert!(
            application
                .mount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            application
                .mount(
                    MockComponentId::InputBar,
                    Box::new(MockBarInput::default()),
                    vec![Sub::new(
                        SubEventClause::Tick,
                        SubClause::HasState(
                            MockComponentId::InputFoo,
                            State::Single(StateValue::String(String::new()))
                        )
                    )]
                )
                .is_ok()
        );
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        // No event should be generated
        assert_eq!(
            application
                .tick(PollStrategy::BlockCollectUpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::new()), MockMsg::BarTick]
        );
    }

    #[test]
    fn should_not_propagate_events_if_is_mounted_cond_is_not_satisfied() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
        // Mount foo and bar
        assert!(
            application
                .mount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            application
                .mount(
                    MockComponentId::InputBar,
                    Box::new(MockBarInput::default()),
                    vec![Sub::new(
                        SubEventClause::Tick,
                        SubClause::IsMounted(MockComponentId::InputOmar)
                    )]
                )
                .is_ok()
        );
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(
            application
                .tick(PollStrategy::BlockCollectUpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::new())]
        );
    }

    #[test]
    fn should_propagate_events_if_is_mounted_cond_is_not_satisfied() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_secs(60)));
        // Mount foo and bar
        assert!(
            application
                .mount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            application
                .mount(
                    MockComponentId::InputBar,
                    Box::new(MockBarInput::default()),
                    vec![Sub::new(
                        SubEventClause::Tick,
                        SubClause::IsMounted(MockComponentId::InputFoo)
                    )]
                )
                .is_ok()
        );
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());
        assert_eq!(
            application
                .tick(PollStrategy::BlockCollectUpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::new()), MockMsg::BarTick]
        );
    }

    #[test]
    fn should_lock_ports() {
        let mut listener = listener_config_with_tick(Duration::from_millis(100));
        let barrier_rx = listener.with_test_barrier();
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener);
        // Mount foo and bar
        assert!(
            application
                .mount(
                    MockComponentId::InputFoo,
                    Box::new(MockFooInput::default()),
                    vec![]
                )
                .is_ok()
        );
        assert!(
            application
                .mount(
                    MockComponentId::InputBar,
                    Box::new(MockBarInput::default()),
                    vec![Sub::new(
                        SubEventClause::Tick,
                        SubClause::IsMounted(MockComponentId::InputFoo)
                    )]
                )
                .is_ok()
        );
        // Active FOO
        assert!(application.active(&MockComponentId::InputFoo).is_ok());

        // verify it start unpaused
        barrier_rx.recieve_cycle();

        assert_eq!(
            application
                .tick(PollStrategy::BlockCollectUpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::new()), MockMsg::BarTick]
        );

        // Lock ports
        assert!(application.lock_ports().is_ok());

        // wait for multiple cycles to verify that no events are generated
        barrier_rx.recieve_start();
        barrier_rx.recieve_start();
        barrier_rx.recieve_start();

        // Tick ( No tick event )
        assert_eq!(
            application
                .tick(PollStrategy::Once)
                .ok()
                .unwrap()
                .as_slice(),
            &[]
        );
        // Unlock ports
        assert!(application.unlock_ports().is_ok());

        // wait for the tick time to definitely be over
        std::thread::sleep(Duration::from_millis(100));
        // only then run the loop which will trigger the tick
        barrier_rx.recieve_cycle();

        // Tick
        assert_eq!(
            application
                .tick(PollStrategy::BlockCollectUpTo(5))
                .ok()
                .unwrap()
                .as_slice(),
            &[MockMsg::FooSubmit(String::new()), MockMsg::BarTick]
        );
    }

    #[test]
    fn application_should_add_injectors() {
        let mut application: Application<MockComponentId, MockMsg, MockEvent> =
            Application::init(listener_config_with_tick(Duration::from_millis(500)));
        application.add_injector(Box::new(MockInjector));
    }

    fn listener_config() -> EventListenerCfg<MockEvent> {
        EventListenerCfg::default().add_port(
            Box::new(MockPoll::<MockEvent>::default()),
            Duration::from_millis(100),
            1,
        )
    }

    fn listener_config_with_tick(tick: Duration) -> EventListenerCfg<MockEvent> {
        listener_config().tick_interval(tick)
    }
}
