/// A macro to generate a chain of [`crate::SubClause::And`] from a list of
/// Ids with the case [`crate::SubClause::IsMounted`] for every id.
///
/// ### example
///
/// ```rust
/// use tuirealm::{SubClause, subclause_and};
///
/// #[derive(Debug, Eq, PartialEq, Clone, Hash)]
/// pub enum Id {
///     InputBar,
///     InputFoo,
///     InputOmar,
/// }
///
/// let sub_clause = subclause_and!(
///    Id::InputBar,
///    Id::InputFoo,
///    Id::InputOmar
///    );
///
/// assert_eq!(
///     sub_clause,
///     SubClause::And(
///         Box::new(SubClause::IsMounted(Id::InputBar)),
///         Box::new(SubClause::And(
///             Box::new(SubClause::IsMounted(Id::InputFoo)),
///             Box::new(SubClause::IsMounted(Id::InputOmar))
///         ))
///     )
///  );
/// ```
///
#[macro_export]
macro_rules! subclause_and {
    ($id:expr) => {
        SubClause::IsMounted($id)
    };
    ($id:expr, $($rest:expr),+) => {
        SubClause::And(
            Box::new(SubClause::IsMounted($id)),
            Box::new(subclause_and!($($rest),+))
        )
    };
}

/// A macro to generate a chain of [`crate::SubClause::Or`] from a list of
/// Ids with the case [`crate::SubClause::IsMounted`] for every id.
///
/// ### example
///
/// ```rust
/// use tuirealm::{SubClause, subclause_or};
///
/// #[derive(Debug, Eq, PartialEq, Clone, Hash)]
/// pub enum Id {
///     InputBar,
///     InputFoo,
///     InputOmar,
/// }
///
/// let sub_clause = subclause_or!(
///    Id::InputBar,
///    Id::InputFoo,
///    Id::InputOmar
///    );
///
/// assert_eq!(
///     sub_clause,
///     SubClause::Or(
///         Box::new(SubClause::IsMounted(Id::InputBar)),
///         Box::new(SubClause::Or(
///             Box::new(SubClause::IsMounted(Id::InputFoo)),
///             Box::new(SubClause::IsMounted(Id::InputOmar))
///         ))
///     )
///  );
/// ```
///
#[macro_export]
macro_rules! subclause_or {
    ($id:expr) => {
        SubClause::IsMounted($id)
    };
    ($id:expr, $($rest:expr),+) => {
        SubClause::Or(
            Box::new(SubClause::IsMounted($id)),
            Box::new(subclause_or!($($rest),+))
        )
    };
}
