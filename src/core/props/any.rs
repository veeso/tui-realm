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
    /// Convert any [`PropBound`] value to a [`AnyProp`] value.
    fn to_any_prop(self) -> AnyPropBox;
    /// This function is necessary as rust as of 1.90 does not have stable cast to super-trait (here to `Any`)
    fn as_any_i(&self) -> &dyn Any;
}

impl<T> PropBound for T
where
    T: Any + DynClone + DynCompare + Debug + Sized,
{
    fn to_any_prop(self) -> AnyPropBox {
        Box::new(self)
    }

    /// This function should likely not be called directly, instead via [`PropBoundExt`], if the input type is Boxed.
    /// If the current type is *not* boxed, this is perfectly fine to call.
    fn as_any_i(&self) -> &dyn Any {
        self
    }
}

/// Extra helpers for [`PropBound`] and [`AnyProp`].
// This is mainly required due to not being able to specialize impl's yet or exempt specific types.
// This *might* not be necessary, if [`AnyProp`] would be a distinct struct instead of a type alias,
// but that still wouldnt solve [`PropBound::as_any_i`] to be potentially be callable on `Box` itself.
pub trait PropBoundExt {
    /// This function is necessary as rust as of 1.90 does not have stable cast to super-trait (here to `Any`)
    fn as_any(&self) -> &dyn Any;
    /// This function is necessary as rust as of 1.90 does not have stable cast to super-trait (here to `Any`)
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

impl PartialEq<dyn PropBound> for dyn PropBound {
    fn eq(&self, other: &dyn PropBound) -> bool {
        self as &dyn DynCompare == other as &dyn DynCompare
    }
}

dyn_clone::clone_trait_object!(PropBound);

/// The outside type to use. It is also the type in [`PropPayload::Any`](super::PropPayload::Any).
pub type AnyPropBox = Box<dyn PropBound>;
