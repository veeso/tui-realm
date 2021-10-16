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
use crate::{AttrValue, Attribute, Event, Frame, State, Sub, Update, View, ViewError};

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

// TODO: can we render everything from here?

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
    pub fn mount(
        &mut self,
        id: &'a str,
        component: WrappedComponent<Msg, UserEvent>,
        subs: Vec<Sub<UserEvent>>,
    ) -> ApplicationResult<()> {
        // Mount
        self.view.mount(id, component)?;
        // Subscribe
        subs.into_iter()
            .for_each(|x| self.subs.push(Subscription::new(id, x)));
        Ok(())
    }

    /// ### umount
    ///
    /// Umount component associated to `id` and remove all its subscriptions.
    /// Returns Error if the component doesn't exist
    pub fn umount(&mut self, id: &'a str) -> ApplicationResult<()> {
        self.view.umount(id)?;
        self.unsubscribe(id);
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

    // -- private

    /// ### unsubscribe
    ///
    /// remove all subscriptions for component
    fn unsubscribe(&mut self, id: &'a str) {
        self.subs.retain(|x| x.target() != id)
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
    /// The poll() function will be called once, will update and then will return
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
    #[error("Listener error: {0}")]
    Listener(ListenerError),
    #[error("View error: {0}")]
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
