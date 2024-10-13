//! ## Injector
//!
//! properties injector

use std::hash::Hash;

use super::props::{AttrValue, Attribute};

/// An injector is a trait object which can provide properties to inject to a certain component.
/// The injector is called each time a component is mounted, providing the id of the mounted
/// component and may return a list of ([`Attribute`], [`AttrValue`]) to inject.
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
