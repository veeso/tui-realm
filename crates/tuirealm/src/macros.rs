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
        SubClause::and(
            SubClause::IsMounted($id),
            subclause_and!($($rest),+)
        )
    };
}

/// A macro to generate a chain of [`crate::SubClause::And`] from a list of
/// Ids with the case  [`crate::SubClause::Not`] containing [`crate::SubClause::IsMounted`] for every id.
///
/// Why is this useful?
/// Well, it happens quite often at least in my application to require a subclause for a "Global Listener" item
/// to have no "Popup" mounted in the application.
///
/// ### example
///
/// ```rust
/// use tuirealm::{SubClause, subclause_and_not};
///
/// #[derive(Debug, Eq, PartialEq, Clone, Hash)]
/// pub enum Id {
///     InputBar,
///     InputFoo,
///     InputOmar,
/// }
///
/// let sub_clause = subclause_and_not!(
///    Id::InputBar,
///    Id::InputFoo,
///    Id::InputOmar
///    );
///
/// assert_eq!(
///     sub_clause,
///     SubClause::And(
///         Box::new(SubClause::Not(Box::new(SubClause::IsMounted(Id::InputBar)))),
///         Box::new(SubClause::And(
///             Box::new(SubClause::Not(Box::new(SubClause::IsMounted(Id::InputFoo)))),
///             Box::new(SubClause::Not(Box::new(SubClause::IsMounted(
///                 Id::InputOmar
///             ))))
///         ))
///     )
///  );
/// ```
///
#[macro_export]
macro_rules! subclause_and_not {
    ($id:expr) => {
        SubClause::not(SubClause::IsMounted($id))
    };
    ($id:expr, $($rest:expr),+) => {
        SubClause::and(
            SubClause::not(SubClause::IsMounted($id)),
            subclause_and_not!($($rest),+)
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
///     SubClause::or(
///         SubClause::IsMounted(Id::InputBar),
///         SubClause::or(
///             SubClause::IsMounted(Id::InputFoo),
///             SubClause::IsMounted(Id::InputOmar)
///         )
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
        SubClause::or(
            SubClause::IsMounted($id),
            subclause_or!($($rest),+)
        )
    };
}
