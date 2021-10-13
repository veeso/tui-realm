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
use crate::listener::{EventListener, EventListenerCfg};
use crate::subscription::Subscription;
use crate::{Update, View};

use std::fmt;

/// ## Application
///
/// TODO:
pub struct Application<'a, Msg, UserEvent>
where
    Msg: PartialEq,
    UserEvent: fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send,
{
    listener: EventListener<UserEvent>,
    subs: Vec<Subscription<UserEvent>>,
    view: View<'a, Msg, UserEvent>,
}

// TODO: can we render everything from here?

impl<'a, Msg, UserEvent> Application<'a, Msg, UserEvent>
where
    Msg: PartialEq,
    UserEvent: fmt::Debug + Eq + PartialEq + Clone + PartialOrd + Send,
{
    // TODO: new takes listenerCfg

    // TODO: pub fn tick(&mut self, model: &mut dyn Update, strategy: PollStrategy) {}

    // TODO: view bridge
}

/// ## PollStrategy
///
/// TODO:
pub enum PollStrategy {
    One,
    UpTo(usize),
}
