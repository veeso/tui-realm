//! Extra extensions to handle Properties.

use tuirealm::props::{
    AttrValue, AttrValueRef, Attribute, Borders, LineStatic, QueryResult, Style, TextModifiers,
    Title,
};
use tuirealm::ratatui::text::Line;
use tuirealm::ratatui::widgets::Block;

use crate::utils::borrow_clone_line;

/// Prop Store for very common props.
///
/// This structure helps to have a common way to handle the "very common" properties, reducing boilerplate
/// and potential mis-matches.
///
/// Additionally, using this over [`Props`](tuirealm::props::Props), saves on indirection and heap-size.
/// On usage (usually on `view`), it also saves on `unwraps` or "defaulting".
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct CommonProps {
    /// The main and common style for a given widget. This is mostly used unless overwritten by more specific styles.
    pub style: Style,
    /// Set a different style for the `border` when the current component is unfocused. This will effectively be merged with `style`.
    pub border_unfocused_style: Style,

    /// A Border to apply, if set.
    pub border: Option<Borders>,
    /// The title for the Border.
    pub title: Option<Title>,

    /// Determines if the current component should be drawn at all.
    pub display: bool,
    /// Determines if the current component is focused or not.
    pub focused: bool,

    /// Determines if the current component should always use the "active" style, regardless if it is focused or not.
    pub always_active: bool,
}

impl Default for CommonProps {
    fn default() -> Self {
        Self {
            style: Style::default(),
            border_unfocused_style: Style::default(),
            border: Option::default(),
            title: Option::default(),
            display: true,
            focused: false,
            always_active: false,
        }
    }
}

impl CommonProps {
    /// Try to set a given [`Attribute`]. Returns `Some` if the value is unhandled. `None` if handled.
    pub fn set(&mut self, attr: Attribute, value: AttrValue) -> Option<AttrValue> {
        match (attr, value) {
            // handle style attributes
            (Attribute::Style, AttrValue::Style(val)) => self.style = val,
            (Attribute::Foreground, AttrValue::Color(val)) => self.style = self.style.fg(val),
            (Attribute::Background, AttrValue::Color(val)) => self.style = self.style.bg(val),
            (Attribute::UnfocusedBorderStyle, AttrValue::Style(val)) => {
                self.border_unfocused_style = val
            }
            (Attribute::TextProps, AttrValue::TextModifiers(val)) => {
                self.style = self.style.add_modifier(val)
            }

            // handle flags
            (Attribute::Display, AttrValue::Flag(val)) => self.display = val,
            (Attribute::Focus, AttrValue::Flag(val)) => self.focused = val,
            (Attribute::AlwaysActive, AttrValue::Flag(val)) => self.always_active = val,

            // handle borders & titles
            (Attribute::Borders, AttrValue::Borders(val)) => self.border = Some(val),
            (Attribute::Title, AttrValue::Title(val)) => self.title = Some(val),

            // other
            (_, value) => return Some(value),
        }

        None
    }

    /// Try to get a given [`Attribute`].
    pub fn get<'a>(&'a self, attr: Attribute) -> Option<AttrValueRef<'a>> {
        match attr {
            // handle style attributes
            Attribute::Style => Some(AttrValueRef::Style(self.style)),
            Attribute::Foreground => self.style.fg.map(AttrValueRef::Color),
            Attribute::Background => self.style.bg.map(AttrValueRef::Color),
            Attribute::UnfocusedBorderStyle => {
                Some(AttrValueRef::Style(self.border_unfocused_style))
            }
            Attribute::TextProps => Some(AttrValueRef::TextModifiers(self.style.add_modifier)),

            // handle flags
            Attribute::Display => Some(AttrValueRef::Flag(self.display)),
            Attribute::Focus => Some(AttrValueRef::Flag(self.focused)),
            Attribute::AlwaysActive => Some(AttrValueRef::Flag(self.always_active)),

            // handle borders & titles
            Attribute::Borders => self.border.map(AttrValueRef::Borders),
            Attribute::Title => self.title.as_ref().map(AttrValueRef::Title),

            // other
            _ => None,
        }
    }

    /// Try to get a given [`Attribute`] as a type compatible with [`Component::query`](tuirealm::component::Component::query).
    #[inline]
    pub fn get_for_query<'a>(&'a self, attr: Attribute) -> Option<QueryResult<'a>> {
        self.get(attr).map(QueryResult::Borrowed)
    }

    /// Get a [`Block`] with the configuration from the common props, if `borders` are defined.
    ///
    /// Uses [`get_block`](crate::utils::get_block).
    pub fn get_block(&self) -> Option<Block<'_>> {
        let borders = self.border?;
        let title = self.title.as_ref();

        let block = crate::utils::get_block(
            borders,
            title,
            self.is_active(),
            Some(self.border_unfocused_style),
        );

        Some(block)
    }

    /// Get if the current component is determined to be "active", either by having "Always active" active or being focused.
    pub fn is_active(&self) -> bool {
        self.always_active || self.focused
    }
}

/// Prop Store for very common highlight props.
///
/// This structure helps to have a common way to handle the "very common" highlight properties, reducing boilerplate
/// and potential mis-matches.
///
/// Additionally, using this over [`Props`](tuirealm::props::Props), saves on indirection and heap-size.
/// On usage (usually on `view`), it also saves on `unwraps` or "defaulting"
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct CommonHighlight {
    /// The main style to patch [`CommonProps::style`] with for the currently active element.
    pub style: Style,
    /// A Style to patch on-top of [`style`](Self::style) when unfocused.
    pub style_unfocused: Style,
    /// The symbol to use to indicate the currently selected element.
    pub symbol: LineStatic,
}

impl Default for CommonHighlight {
    fn default() -> Self {
        Self {
            style: Style::default().add_modifier(TextModifiers::REVERSED),
            style_unfocused: Style::default(),
            symbol: LineStatic::default(),
        }
    }
}

impl CommonHighlight {
    /// Try to set a given [`Attribute`]. Returns `Some` if the value is unhandled. `None` if handled.
    pub fn set(&mut self, attr: Attribute, value: AttrValue) -> Option<AttrValue> {
        match (attr, value) {
            (Attribute::HighlightStyle, AttrValue::Style(val)) => self.style = val,
            (Attribute::HighlightStyleUnfocused, AttrValue::Style(val)) => {
                self.style_unfocused = val
            }
            (Attribute::HighlightedStr, AttrValue::TextLine(val)) => self.symbol = val,

            // other
            (_, value) => return Some(value),
        }

        None
    }

    /// Try to get a given [`Attribute`].
    pub fn get<'a>(&'a self, attr: Attribute) -> Option<AttrValueRef<'a>> {
        match attr {
            Attribute::HighlightStyle => Some(AttrValueRef::Style(self.style)),
            Attribute::HighlightStyleUnfocused => Some(AttrValueRef::Style(self.style_unfocused)),
            Attribute::HighlightedStr => Some(AttrValueRef::TextLine(&self.symbol)),

            // other
            _ => None,
        }
    }

    /// Try to get a given [`Attribute`] as a type compatible with [`Component::query`](tuirealm::component::Component::query).
    #[inline]
    pub fn get_for_query<'a>(&'a self, attr: Attribute) -> Option<QueryResult<'a>> {
        self.get(attr).map(QueryResult::Borrowed)
    }

    /// Get the set highlight symbol as its own `Line` instance, but referencing the existing data.
    pub fn get_symbol(&self) -> Option<Line<'_>> {
        if self.symbol.spans.is_empty() {
            None
        } else {
            Some(borrow_clone_line(&self.symbol))
        }
    }

    /// Get the patched highlight style.
    #[inline]
    pub fn get_style(&self, normal_style: Style) -> Style {
        normal_style.patch(self.style)
    }

    /// Get the patched highlight style with focused or unfocused style.
    #[inline]
    pub fn get_style_focus(&self, normal_style: Style, focus: bool) -> Style {
        let style = normal_style.patch(self.style);

        if focus {
            style
        } else {
            style.patch(self.style_unfocused)
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use tuirealm::props::{
        AttrValue, Attribute, BorderSides, BorderType, Borders, Color, HorizontalAlignment,
        LineStatic, Style, TextModifiers, Title,
    };
    use tuirealm::ratatui::widgets::{Block, TitlePosition};

    use crate::prop_ext::{CommonHighlight, CommonProps};

    #[test]
    fn common_should_have_expected_defaults() {
        let props = CommonProps::default();

        // test defaults
        assert_eq!(
            props.get(Attribute::Style).unwrap().unwrap_style(),
            Style::new()
        );
        assert_eq!(
            props
                .get(Attribute::UnfocusedBorderStyle)
                .unwrap()
                .unwrap_style(),
            Style::new()
        );
        assert!(props.get(Attribute::Foreground).is_none());
        assert!(props.get(Attribute::Background).is_none());
        assert_eq!(
            props
                .get(Attribute::TextProps)
                .unwrap()
                .unwrap_text_modifiers(),
            TextModifiers::default()
        );

        assert!(props.get(Attribute::Display).unwrap().unwrap_flag());
        assert!(!props.get(Attribute::Focus).unwrap().unwrap_flag());
        assert!(!props.get(Attribute::AlwaysActive).unwrap().unwrap_flag());

        assert!(props.get(Attribute::Borders).is_none());
        assert!(props.get(Attribute::Title).is_none());
    }

    #[test]
    fn common_should_get_set() {
        let mut props = CommonProps::default();

        // style via individual attributes
        props.set(Attribute::Foreground, AttrValue::Color(Color::Black));
        props.set(Attribute::Background, AttrValue::Color(Color::Gray));
        props.set(
            Attribute::TextProps,
            AttrValue::TextModifiers(TextModifiers::BOLD),
        );

        assert_eq!(
            props.get(Attribute::Style).unwrap().unwrap_style(),
            Style::new()
                .fg(Color::Black)
                .bg(Color::Gray)
                .add_modifier(TextModifiers::BOLD)
        );
        assert_eq!(
            props.get(Attribute::Foreground).unwrap().unwrap_color(),
            Color::Black
        );
        assert_eq!(
            props.get(Attribute::Background).unwrap().unwrap_color(),
            Color::Gray
        );
        assert_eq!(
            props
                .get(Attribute::TextProps)
                .unwrap()
                .unwrap_text_modifiers(),
            TextModifiers::BOLD
        );

        // style via style attribute
        props.set(
            Attribute::Style,
            AttrValue::Style(
                Style::new()
                    .fg(Color::Blue)
                    .bg(Color::DarkGray)
                    .add_modifier(TextModifiers::DIM),
            ),
        );

        assert_eq!(
            props.get(Attribute::Style).unwrap().unwrap_style(),
            Style::new()
                .fg(Color::Blue)
                .bg(Color::DarkGray)
                .add_modifier(TextModifiers::DIM)
        );
        assert_eq!(
            props.get(Attribute::Foreground).unwrap().unwrap_color(),
            Color::Blue
        );
        assert_eq!(
            props.get(Attribute::Background).unwrap().unwrap_color(),
            Color::DarkGray
        );
        assert_eq!(
            props
                .get(Attribute::TextProps)
                .unwrap()
                .unwrap_text_modifiers(),
            TextModifiers::DIM
        );

        // focus style
        props.set(
            Attribute::UnfocusedBorderStyle,
            AttrValue::Style(Style::new().add_modifier(TextModifiers::REVERSED)),
        );

        assert_eq!(
            props
                .get(Attribute::UnfocusedBorderStyle)
                .unwrap()
                .unwrap_style(),
            Style::new().add_modifier(TextModifiers::REVERSED)
        );

        // flags

        props.set(Attribute::Display, AttrValue::Flag(false));
        props.set(Attribute::Focus, AttrValue::Flag(true));
        props.set(Attribute::AlwaysActive, AttrValue::Flag(true));

        assert!(!props.get(Attribute::Display).unwrap().unwrap_flag());
        assert!(props.get(Attribute::Focus).unwrap().unwrap_flag());
        assert!(props.get(Attribute::AlwaysActive).unwrap().unwrap_flag());

        // border & title

        props.set(
            Attribute::Borders,
            AttrValue::Borders(
                Borders::default()
                    .color(Color::Black)
                    .modifiers(BorderType::Double)
                    .sides(BorderSides::TOP),
            ),
        );
        props.set(
            Attribute::Title,
            AttrValue::Title(
                Title::default()
                    .content("Hello".into())
                    .alignment(HorizontalAlignment::Center)
                    .position(TitlePosition::Bottom),
            ),
        );

        assert_eq!(
            props.get(Attribute::Borders).unwrap().unwrap_borders(),
            Borders::default()
                .color(Color::Black)
                .modifiers(BorderType::Double)
                .sides(BorderSides::TOP)
        );
        assert_eq!(
            props.get(Attribute::Title).unwrap().unwrap_title(),
            &Title::default()
                .content("Hello".into())
                .alignment(HorizontalAlignment::Center)
                .position(TitlePosition::Bottom)
        );
    }

    #[test]
    fn common_should_get_block() {
        let mut props = CommonProps::default();

        assert!(props.get_block().is_none());

        props.set(
            Attribute::Borders,
            AttrValue::Borders(
                Borders::default()
                    .color(Color::Black)
                    .modifiers(BorderType::Double)
                    .sides(BorderSides::TOP),
            ),
        );
        props.set(
            Attribute::Title,
            AttrValue::Title(
                Title::default()
                    .content("Hello".into())
                    .alignment(HorizontalAlignment::Center)
                    .position(TitlePosition::Bottom),
            ),
        );

        // unfocused, no set inactive style
        let block = props.get_block().unwrap();
        assert_eq!(
            block,
            Block::new()
                .title_bottom(LineStatic::from("Hello").centered())
                .borders(BorderSides::TOP)
                .border_type(BorderType::Double)
        );

        // focused, no set inactive style
        props.set(Attribute::Focus, AttrValue::Flag(true));

        let block = props.get_block().unwrap();
        assert_eq!(
            block,
            Block::new()
                .border_style(Style::new().black())
                .title_bottom(LineStatic::from("Hello").centered())
                .borders(BorderSides::TOP)
                .border_type(BorderType::Double)
        );
    }

    #[test]
    fn block_should_be_active_with_alwaysactive() {
        let mut props = CommonProps::default();
        props.set(
            Attribute::Borders,
            AttrValue::Borders(
                Borders::default()
                    .color(Color::Black)
                    .modifiers(BorderType::Double)
                    .sides(BorderSides::TOP),
            ),
        );
        props.set(
            Attribute::Title,
            AttrValue::Title(
                Title::default()
                    .content("Hello".into())
                    .alignment(HorizontalAlignment::Center)
                    .position(TitlePosition::Bottom),
            ),
        );
        props.set(
            Attribute::UnfocusedBorderStyle,
            AttrValue::Style(Style::new().gray()),
        );
        props.set(Attribute::AlwaysActive, AttrValue::Flag(true));

        let block = props.get_block().unwrap();

        assert_eq!(
            block,
            Block::new()
                .border_style(Style::new().black())
                .title_bottom(LineStatic::from("Hello").centered())
                .borders(BorderSides::TOP)
                .border_type(BorderType::Double)
        );
    }

    #[test]
    fn common_highlight_should_have_expected_defaults() {
        let props = CommonHighlight::default();

        // test defaults
        assert_eq!(
            props.get(Attribute::HighlightStyle).unwrap().unwrap_style(),
            Style::new().add_modifier(TextModifiers::REVERSED)
        );
        assert_eq!(
            props
                .get(Attribute::HighlightStyleUnfocused)
                .unwrap()
                .unwrap_style(),
            Style::new()
        );
        assert_eq!(
            props
                .get(Attribute::HighlightedStr)
                .unwrap()
                .unwrap_textline(),
            &LineStatic::default()
        );
    }

    #[test]
    fn common_highlight_should_get_set() {
        let mut props = CommonHighlight::default();

        // style via highlight style attribute
        props.set(
            Attribute::HighlightStyle,
            AttrValue::Style(
                Style::new()
                    .fg(Color::Blue)
                    .add_modifier(TextModifiers::DIM),
            ),
        );

        assert_eq!(
            props.get(Attribute::HighlightStyle).unwrap().unwrap_style(),
            Style::new()
                .fg(Color::Blue)
                .add_modifier(TextModifiers::DIM)
        );

        // style unfocused via highlight style attribute
        props.set(
            Attribute::HighlightStyleUnfocused,
            AttrValue::Style(Style::new().remove_modifier(TextModifiers::DIM)),
        );

        assert_eq!(
            props
                .get(Attribute::HighlightStyleUnfocused)
                .unwrap()
                .unwrap_style(),
            Style::new().remove_modifier(TextModifiers::DIM)
        );

        // symbol via highlight symbol attribute
        props.set(
            Attribute::HighlightedStr,
            AttrValue::TextLine(LineStatic::raw(">>")),
        );

        assert_eq!(
            props
                .get(Attribute::HighlightedStr)
                .unwrap()
                .unwrap_textline(),
            &LineStatic::raw(">>")
        );
    }
}
