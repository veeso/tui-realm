//! ## View
//!
//! TODO: complete

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
use crate::{Component, Update};
// -- ext
use std::collections::HashMap;

type WrappedComponent<Msg, UserEvent> = Box<dyn Component<Msg, UserEvent>>;

/// ## View
///
/// View is the wrapper and manager for all the components.
/// A View is a container for all the components in a certain layout.
/// Each View can have only one focused component at the time. At least one component must be always focused
pub struct View<Msg, UserEvent> {
    /// Components Mounted onto View
    components: HashMap<String, WrappedComponent<Msg, UserEvent>>,
    // TODO: add publisher
    // TODO: add subs
    /// Current active component
    focus: Option<String>, // TODO: change
    /// Focus stack; used to determine which component should hold focus in case the current element is blurred
    focus_stack: Vec<String>,
}

impl<Msg, UserEvent> View<Msg, UserEvent> {
    /// ### init
    ///
    /// Initialize a new `View`
    pub fn init() -> Self {
        Self {
            components: HashMap::new(),
            focus: None,
            focus_stack: Vec::new(),
        }
    }

    // TODO: pub fn update(&mut self, model: Box<&mut dyn Update>, strategy: Strategy) -> Option<Msg> {}

    // TODO: subscriptions

    // TODO: query(); attr(); poll();

    // TODO: strategy (One, UpTo(usize), All, Process)
}

// TODO: typedef sub
