//! ## Injector
//!
//! properties injector

use std::hash::Hash;

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2022 Christian Visintin
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
use super::props::{AttrValue, Attribute};

/// An injector is a trait object which can provide properties to inject to a certain component.
/// The injector is called each time a component is mounted, providing the id of the mounted
/// component and may return a list of (`Attribute`, `AttrValue`) to inject.
pub trait Injector<ComponentId>
where
    ComponentId: Eq + PartialEq + Clone + Hash,
{
    fn inject(&self, id: &ComponentId) -> Vec<(Attribute, AttrValue)>;
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::mock::{MockComponentId, MockInjector};

    #[test]
    fn should_create_a_trait_object_injector() {
        let injector = MockInjector::default();
        assert_eq!(
            injector.inject(&MockComponentId::InputBar),
            vec![(
                Attribute::Text,
                AttrValue::String(String::from("hello, world!")),
            )]
        );
        assert_eq!(injector.inject(&MockComponentId::InputFoo), vec![]);
    }
}
