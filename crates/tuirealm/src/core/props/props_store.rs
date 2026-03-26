use std::collections::HashMap;

use crate::props::{AttrValue, Attribute, QueryResult};

/// The props struct holds all the attributes associated to the component.
/// Properties have been designed to be versatile for all kind of components, but without introducing
/// too many attributes at the same time.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Props {
    attrs: HashMap<Attribute, AttrValue>,
}

impl Props {
    /// Get, if any, the attribute associated to the selector by reference.
    pub fn get(&self, query: Attribute) -> Option<&AttrValue> {
        self.attrs.get(&query)
    }

    /// Get, if any, the attribute associated to the selector by mutable reference.
    pub fn get_mut(&mut self, query: Attribute) -> Option<&mut AttrValue> {
        self.attrs.get_mut(&query)
    }

    /// Get, if any, the attribute associated to the selector by reference and return as a type compatible with [`Component::query`](crate::component::Component::query).
    pub fn get_for_query<'a>(&'a self, query: Attribute) -> Option<QueryResult<'a>> {
        self.get(query).map(QueryResult::from)
    }

    /// Set a new attribute into Properties
    pub fn set(&mut self, query: Attribute, value: AttrValue) {
        self.attrs.insert(query, value);
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use ratatui::layout::HorizontalAlignment;

    use super::*;

    #[test]
    fn should_set_get_props() {
        let mut props = Props::default();
        assert_eq!(props.get(Attribute::AlignmentHorizontal), None);
        assert_eq!(props.get(Attribute::AlignmentHorizontal), None);

        props.set(
            Attribute::AlignmentHorizontal,
            AttrValue::AlignmentHorizontal(HorizontalAlignment::Left),
        );
        assert_eq!(
            props.get(Attribute::AlignmentHorizontal),
            Some(&AttrValue::AlignmentHorizontal(HorizontalAlignment::Left))
        );
        assert_eq!(
            props.get(Attribute::AlignmentHorizontal),
            Some(&AttrValue::AlignmentHorizontal(HorizontalAlignment::Left))
        );

        let val = props.get_mut(Attribute::AlignmentHorizontal).unwrap();
        assert_eq!(
            val,
            &AttrValue::AlignmentHorizontal(HorizontalAlignment::Left)
        );
        let v = val.as_alignment_horizontal_mut().unwrap();
        *v = HorizontalAlignment::Center;

        assert_eq!(
            props.get(Attribute::AlignmentHorizontal).unwrap(),
            &AttrValue::AlignmentHorizontal(HorizontalAlignment::Center)
        );
    }
}
