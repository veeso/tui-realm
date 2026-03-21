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

/// Trait for [`PropPayload::Any`](crate::props::PropPayload::Any).
///
/// This trait is necessary because we need multiple supertraits, this is necessary as we need to implement custom behavior for [`Clone`] and [`PartialEq`].
///
/// Note that equivalence ([`PartialEq`]) will only work if the types are the same (ex. `String` will compare with `String`, but not `str`).
#[allow(private_bounds)]
pub trait PropBound: Any + DynClone + DynCompare + Debug + Send + Sync {
    /// Convert any [`PropBound`] value to a [`AnyPropBox`] value.
    fn to_any_prop(self) -> AnyPropBox;
}

impl<T> PropBound for T
where
    T: Any + DynClone + DynCompare + Debug + Sized + Send + Sync,
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

impl dyn PropBound {
    /// Convenience function to cast to [`Any`].
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    /// Convenience function to cast to [`Any`] mutably.
    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::props::PropBound;

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
