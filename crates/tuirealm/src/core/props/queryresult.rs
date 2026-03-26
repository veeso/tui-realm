use std::fmt::Debug;

use crate::props::{AttrValue, AttrValueRef};

/// The Resulting value of a [`Component::query`](crate::component::Component::query) call.
///
/// Where possible the [`Borrowed`](Self::Borrowed) variant should be used over the [`Owned`](Self::Owned) variant.
///
/// For handling both cases *readonly*, [`as_ref`](Self::as_ref) can be used.
/// For handling both cases *mutably*, [`into_attr`](Self::into_attr) can be used.
///
/// Practically, this means only [`PropPayload::Any`](crate::props::PropPayload::Any) should require use of [`Owned`](Self::Owned).
///
/// This enum practically mirrors [`Cow`](std::borrow::Cow), only that no [`ToOwned`] or [`Borrow`](std::borrow::Borrow) trait implementations are required,
/// which cannot be implemented on types with lifetimes.
#[derive(Debug, Clone)]
pub enum QueryResult<'a> {
    Owned(AttrValue),
    Borrowed(AttrValueRef<'a>),
}

impl<'a> PartialEq for QueryResult<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Owned(l0), Self::Owned(r0)) => l0 == r0,
            (Self::Borrowed(l0), Self::Borrowed(r0)) => l0 == r0,
            (Self::Owned(l0), Self::Borrowed(r0)) | (Self::Borrowed(r0), Self::Owned(l0)) => {
                l0 == r0
            }
        }
    }
}

impl<'a> PartialEq<AttrValue> for QueryResult<'a> {
    fn eq(&self, other: &AttrValue) -> bool {
        match self {
            QueryResult::Owned(attr_value) => attr_value == other,
            QueryResult::Borrowed(attr_value_ref) => attr_value_ref == other,
        }
    }
}

impl<'a> PartialEq<QueryResult<'a>> for AttrValue {
    fn eq(&self, other: &QueryResult<'a>) -> bool {
        other == self
    }
}

impl<'a> PartialEq<AttrValueRef<'a>> for QueryResult<'a> {
    fn eq(&self, other: &AttrValueRef<'a>) -> bool {
        match self {
            QueryResult::Owned(attr_value) => attr_value == other,
            QueryResult::Borrowed(attr_value_ref) => attr_value_ref == other,
        }
    }
}

impl<'a> PartialEq<QueryResult<'a>> for AttrValueRef<'a> {
    fn eq(&self, other: &QueryResult<'a>) -> bool {
        other == self
    }
}

impl<'a> QueryResult<'a> {
    /// Convert the result to be of variant [`Owned`](Self::Owned)
    pub fn to_attr(self) -> Self {
        match self {
            QueryResult::Owned(v) => Self::Owned(v),
            QueryResult::Borrowed(v) => Self::Owned(v.into()),
        }
    }

    /// Convert the current result into a [`AttrValue`].
    pub fn into_attr(self) -> AttrValue {
        match self {
            QueryResult::Owned(v) => v,
            QueryResult::Borrowed(v) => v.into(),
        }
    }

    /// Convert the current result into a [`AttrValue`].
    ///
    /// Alias of [`into_attr`](Self::into_attr).
    #[inline]
    pub fn into_owned(self) -> AttrValue {
        self.into_attr()
    }

    /// Get the current data as a [`AttrValueRef`].
    pub fn as_ref(&'a self) -> AttrValueRef<'a> {
        match self {
            QueryResult::Owned(attr_value) => attr_value.as_attr_ref(),
            QueryResult::Borrowed(attr_value_ref) => *attr_value_ref,
        }
    }
}

impl<'a> From<AttrValue> for QueryResult<'a> {
    fn from(value: AttrValue) -> Self {
        Self::Owned(value)
    }
}

impl<'a> From<&'a AttrValue> for QueryResult<'a> {
    fn from(value: &'a AttrValue) -> Self {
        Self::Borrowed(value.as_attr_ref())
    }
}

impl<'a> From<AttrValueRef<'a>> for QueryResult<'a> {
    fn from(value: AttrValueRef<'a>) -> Self {
        Self::Borrowed(value)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_allow_from_either() {
        assert_eq!(
            QueryResult::from(AttrValue::Size(0)),
            QueryResult::Owned(AttrValue::Size(0))
        );

        assert_eq!(
            QueryResult::from(AttrValueRef::Size(0)),
            QueryResult::Borrowed(AttrValueRef::Size(0))
        );
    }

    #[test]
    fn should_allow_to_owned() {
        assert_eq!(
            QueryResult::Owned(AttrValue::Size(0)).to_attr(),
            QueryResult::Owned(AttrValue::Size(0))
        );

        assert_eq!(
            QueryResult::Borrowed(AttrValueRef::Size(0)).to_attr(),
            QueryResult::Owned(AttrValue::Size(0))
        );
    }

    #[test]
    fn should_allow_into_owned() {
        assert_eq!(
            QueryResult::Owned(AttrValue::Size(0)).into_attr(),
            AttrValue::Size(0)
        );

        assert_eq!(
            QueryResult::Borrowed(AttrValueRef::Size(0)).into_attr(),
            AttrValue::Size(0)
        );

        assert_eq!(
            QueryResult::Owned(AttrValue::Size(0)).into_owned(),
            AttrValue::Size(0)
        );

        assert_eq!(
            QueryResult::Borrowed(AttrValueRef::Size(0)).into_owned(),
            AttrValue::Size(0)
        );
    }

    #[test]
    fn should_allow_asref() {
        assert_eq!(
            QueryResult::Owned(AttrValue::Size(0)).as_ref(),
            AttrValueRef::Size(0)
        );

        assert_eq!(
            QueryResult::Borrowed(AttrValueRef::Size(0)).as_ref(),
            AttrValueRef::Size(0)
        );
    }

    #[test]
    fn should_compare_itself() {
        assert_eq!(
            QueryResult::Owned(AttrValue::Size(0)),
            QueryResult::Owned(AttrValue::Size(0))
        );
        assert_ne!(
            QueryResult::Owned(AttrValue::Size(1)),
            QueryResult::Owned(AttrValue::Size(0))
        );

        assert_eq!(
            QueryResult::Borrowed(AttrValueRef::Size(0)),
            QueryResult::Borrowed(AttrValueRef::Size(0))
        );
        assert_ne!(
            QueryResult::Borrowed(AttrValueRef::Size(1)),
            QueryResult::Borrowed(AttrValueRef::Size(0))
        );

        assert_eq!(
            QueryResult::Borrowed(AttrValueRef::Size(0)),
            QueryResult::Owned(AttrValue::Size(0))
        );
        assert_eq!(
            QueryResult::Owned(AttrValue::Size(0)),
            QueryResult::Borrowed(AttrValueRef::Size(0))
        );

        assert_ne!(
            QueryResult::Borrowed(AttrValueRef::Size(1)),
            QueryResult::Owned(AttrValue::Size(0))
        );
        assert_ne!(
            QueryResult::Owned(AttrValue::Size(1)),
            QueryResult::Borrowed(AttrValueRef::Size(0))
        );
    }

    #[test]
    fn should_compare_attrvalue() {
        assert_eq!(QueryResult::Owned(AttrValue::Size(0)), AttrValue::Size(0));
        assert_ne!(QueryResult::Owned(AttrValue::Size(1)), AttrValue::Size(0));

        assert_eq!(
            QueryResult::Borrowed(AttrValueRef::Size(0)),
            AttrValue::Size(0)
        );
        assert_ne!(
            QueryResult::Borrowed(AttrValueRef::Size(1)),
            AttrValue::Size(0)
        );
    }

    #[test]
    fn should_compare_attrvalueref() {
        assert_eq!(
            QueryResult::Owned(AttrValue::Size(0)),
            AttrValueRef::Size(0)
        );
        assert_ne!(
            QueryResult::Owned(AttrValue::Size(1)),
            AttrValueRef::Size(0)
        );

        assert_eq!(
            QueryResult::Borrowed(AttrValueRef::Size(0)),
            AttrValueRef::Size(0)
        );
        assert_ne!(
            QueryResult::Borrowed(AttrValueRef::Size(1)),
            AttrValueRef::Size(0)
        );
    }
}
