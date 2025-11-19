use std::any::Any;
use std::fmt::Debug;

use dyn_clone::DynClone;

// PartialEq

// PartialEq impl should be recommended; this trait here is private and implemented for "Any + PartialEq" types
/// Compare `dyn Any + PartialEq` objects. The default compare implementation only compares objects of the same type.
/// Or in other words `String("Hello") == String("Hello")` but not `str("Hello") != String("Hello")`.
///
/// Implementation mainly from <https://quinedot.github.io/rust-learning/dyn-trait-eq.html>
trait DynCompare: Any {
    fn dyn_eq(&self, other: &dyn DynCompare) -> bool;
}

impl<T> DynCompare for T
where
    T: Any + PartialEq,
{
    fn dyn_eq(&self, other: &dyn DynCompare) -> bool {
        if let Some(other) = (other as &dyn Any).downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }
}

impl PartialEq<dyn DynCompare> for dyn DynCompare {
    fn eq(&self, other: &dyn DynCompare) -> bool {
        self.dyn_eq(other)
    }
}

// Public type

/// Trait for multiple supertraits, as we need to implement custom behavior (see `PartialEq` impl).
///
/// Note that equivalence ([`PartialEq`]) will only work if the types are the same (ex. `String` will compare with `String`, but not `str`).
#[allow(private_bounds)]
pub trait PropBound: Any + DynClone + DynCompare + Debug {
    /// Convert any [`PropBound`] value to a [`AnyPropBox`] value.
    fn to_any_prop(self) -> AnyPropBox;
}

impl<T> PropBound for T
where
    T: Any + DynClone + DynCompare + Debug + Sized,
{
    fn to_any_prop(self) -> AnyPropBox {
        Box::new(self)
    }
}

impl PartialEq<dyn PropBound> for dyn PropBound {
    fn eq(&self, other: &dyn PropBound) -> bool {
        self as &dyn DynCompare == other as &dyn DynCompare
    }
}

dyn_clone::clone_trait_object!(PropBound);

/// The outside type to use. It is also the type in [`PropPayload::Any`](super::PropPayload::Any).
pub type AnyPropBox = Box<dyn PropBound>;

/// Extra convenience functions for [`PropBound`].
///
/// This mainly exists because "negative trait implementations" are not stable / supported,
/// so if we would add this to [`PropBound`]'s `impl _ for T`, it would implement the following functions
/// for [`Box`] as well, causing `Box<_> as Any` instead of `Box<_>.deref() as Any`.
pub trait PropBoundExt {
    /// Convenience function to cast to [`Any`].
    fn as_any(&self) -> &dyn Any;
    /// Convenience function to cast to [`Any`] mutably.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl PropBoundExt for dyn PropBound {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::props::{PropBound, PropBoundExt};

    #[test]
    fn should_work_basic() {
        let value = String::from("Some string");
        let prop_boxed = value.to_any_prop();

        assert_eq!(
            prop_boxed.as_any().downcast_ref::<String>().unwrap(),
            &String::from("Some string")
        );

        let mut prop_boxed = prop_boxed;
        prop_boxed
            .as_any_mut()
            .downcast_mut::<String>()
            .unwrap()
            .push_str(" hello");

        assert_eq!(
            prop_boxed.as_any().downcast_ref::<String>().unwrap(),
            &String::from("Some string hello")
        );
    }
}
