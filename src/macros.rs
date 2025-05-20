/// A macro to generate a chain of [`crate::SubClause::AndMany`] from a list of
/// Ids with the case [`crate::SubClause::IsMounted`] for every id.
///
/// ### Example
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
/// );
///
/// assert_eq!(
///     sub_clause,
///     SubClause::AndMany(vec![
///         SubClause::IsMounted(Id::InputBar),
///         SubClause::IsMounted(Id::InputFoo),
///         SubClause::IsMounted(Id::InputOmar),
///     ])
///  );
/// ```
#[macro_export]
macro_rules! subclause_and {
    ($id:expr) => {
        SubClause::IsMounted($id)
    };
    ($($rest:expr),+ $(,)?) => {
        SubClause::AndMany(vec![
            $(SubClause::IsMounted($rest)),*
        ])
    };
}

/// A macro to generate a chain of [`crate::SubClause::And`] from a list of
/// Ids with the case  [`crate::SubClause::Not`] containing [`crate::SubClause::IsMounted`] for every id.
///
/// Why is this useful?
/// Well, it happens quite often at least in my application to require a subclause for a "Global Listener" item
/// to have no "Popup" mounted in the application.
///
/// ### Example
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
/// );
///
/// assert_eq!(
///     sub_clause,
///     SubClause::not(SubClause::AndMany(vec![
///         SubClause::IsMounted(Id::InputBar),
///         SubClause::IsMounted(Id::InputFoo),
///         SubClause::IsMounted(Id::InputOmar),
///     ]))
///  );
/// ```
#[macro_export]
macro_rules! subclause_and_not {
    ($id:expr) => {
        SubClause::not(SubClause::IsMounted($id))
    };
    ($($rest:expr),+ $(,)?) => {
        SubClause::not(
            SubClause::AndMany(vec![
                $(SubClause::IsMounted($rest)),*
            ])
        )
    };
}

/// A macro to generate a chain of [`crate::SubClause::OrMany`] from a list of
/// Ids with the case [`crate::SubClause::IsMounted`] for every id.
///
/// ### Example
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
/// );
///
/// assert_eq!(
///     sub_clause,
///     SubClause::OrMany(vec![
///         SubClause::IsMounted(Id::InputBar),
///         SubClause::IsMounted(Id::InputFoo),
///         SubClause::IsMounted(Id::InputOmar),
///     ])
///  );
/// ```
#[macro_export]
macro_rules! subclause_or {
    ($id:expr) => {
        SubClause::IsMounted($id)
    };
    ($($rest:expr),+ $(,)?) => {
        SubClause::OrMany(vec![
            $(SubClause::IsMounted($rest)),*
        ])
    };
}

#[cfg(test)]
mod tests {
    use crate::SubClause;
    use crate::mock::MockComponentId;

    #[test]
    fn subclause_and() {
        // single
        assert_eq!(
            subclause_and!(MockComponentId::InputBar),
            SubClause::IsMounted(MockComponentId::InputBar),
        );

        // multiple with no ending comma
        assert_eq!(
            subclause_and!(
                MockComponentId::InputBar,
                MockComponentId::InputFoo,
                MockComponentId::InputOmar
            ),
            SubClause::AndMany(vec![
                SubClause::IsMounted(MockComponentId::InputBar),
                SubClause::IsMounted(MockComponentId::InputFoo),
                SubClause::IsMounted(MockComponentId::InputOmar),
            ])
        );

        // multiple with ending comma
        assert_eq!(
            subclause_and!(
                MockComponentId::InputBar,
                MockComponentId::InputFoo,
                MockComponentId::InputOmar,
            ),
            SubClause::AndMany(vec![
                SubClause::IsMounted(MockComponentId::InputBar),
                SubClause::IsMounted(MockComponentId::InputFoo),
                SubClause::IsMounted(MockComponentId::InputOmar),
            ])
        );
    }

    #[test]
    fn subclause_and_not() {
        // single
        assert_eq!(
            subclause_and_not!(MockComponentId::InputBar),
            SubClause::not(SubClause::IsMounted(MockComponentId::InputBar)),
        );

        // multiple with no ending comma
        assert_eq!(
            subclause_and_not!(
                MockComponentId::InputBar,
                MockComponentId::InputFoo,
                MockComponentId::InputOmar
            ),
            SubClause::not(SubClause::AndMany(vec![
                SubClause::IsMounted(MockComponentId::InputBar),
                SubClause::IsMounted(MockComponentId::InputFoo),
                SubClause::IsMounted(MockComponentId::InputOmar),
            ]))
        );

        // multiple with ending comma
        assert_eq!(
            subclause_and_not!(
                MockComponentId::InputBar,
                MockComponentId::InputFoo,
                MockComponentId::InputOmar,
            ),
            SubClause::not(SubClause::AndMany(vec![
                SubClause::IsMounted(MockComponentId::InputBar),
                SubClause::IsMounted(MockComponentId::InputFoo),
                SubClause::IsMounted(MockComponentId::InputOmar),
            ]))
        );
    }

    #[test]
    fn subclause_or() {
        // single
        assert_eq!(
            subclause_or!(MockComponentId::InputBar),
            SubClause::IsMounted(MockComponentId::InputBar),
        );

        // multiple with no ending comma
        assert_eq!(
            subclause_or!(
                MockComponentId::InputBar,
                MockComponentId::InputFoo,
                MockComponentId::InputOmar
            ),
            SubClause::OrMany(vec![
                SubClause::IsMounted(MockComponentId::InputBar),
                SubClause::IsMounted(MockComponentId::InputFoo),
                SubClause::IsMounted(MockComponentId::InputOmar),
            ])
        );

        // multiple with ending comma
        assert_eq!(
            subclause_or!(
                MockComponentId::InputBar,
                MockComponentId::InputFoo,
                MockComponentId::InputOmar,
            ),
            SubClause::OrMany(vec![
                SubClause::IsMounted(MockComponentId::InputBar),
                SubClause::IsMounted(MockComponentId::InputFoo),
                SubClause::IsMounted(MockComponentId::InputOmar),
            ])
        );
    }
}
