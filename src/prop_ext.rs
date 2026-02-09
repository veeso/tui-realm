use tuirealm::{
    AttrValue, Attribute,
    props::{Borders, Style, Title},
    ratatui::widgets::Block,
};

/// Prop Store for very common props.
///
/// This structure helps to have a common way to handle the "very common" properties, reducing boilerplate
/// and potential mis-matches.
///
/// Additionally, using this over [`Props`](tuirealm::Props), saves on indirection and heap-size.
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
            (Attribute::FocusStyle, AttrValue::Style(val)) => self.border_unfocused_style = val,
            (Attribute::TextProps, AttrValue::TextModifiers(val)) => {
                self.style = self.style.add_modifier(val)
            }

            // handle flags
            (Attribute::Display, AttrValue::Flag(val)) => self.display = val,
            (Attribute::Focus, AttrValue::Flag(val)) => self.focused = val,

            // handle borders & titles
            (Attribute::Borders, AttrValue::Borders(val)) => self.border = Some(val),
            (Attribute::Title, AttrValue::Title(val)) => self.title = Some(val),

            // other
            (_, value) => return Some(value),
        }

        None
    }

    /// Try to get a given [`Attribute`].
    pub fn get(&self, attr: Attribute) -> Option<AttrValue> {
        match attr {
            // handle style attributes
            Attribute::Style => Some(AttrValue::Style(self.style)),
            Attribute::Foreground => self.style.fg.map(AttrValue::Color),
            Attribute::Background => self.style.bg.map(AttrValue::Color),
            Attribute::FocusStyle => Some(AttrValue::Style(self.border_unfocused_style)),
            Attribute::TextProps => Some(AttrValue::TextModifiers(self.style.add_modifier)),

            // handle flags
            Attribute::Display => Some(AttrValue::Flag(self.display)),
            Attribute::Focus => Some(AttrValue::Flag(self.focused)),

            // handle borders & titles
            Attribute::Borders => self.border.map(AttrValue::Borders),
            Attribute::Title => self.title.clone().map(AttrValue::Title),

            // other
            _ => None,
        }
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
            self.focused,
            Some(self.border_unfocused_style),
        );

        Some(block)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use tuirealm::{
        AttrValue, Attribute,
        props::{
            BorderSides, BorderType, Borders, Color, HorizontalAlignment, LineStatic, Style,
            TextModifiers, Title,
        },
        ratatui::widgets::{Block, TitlePosition},
    };

    use crate::prop_ext::CommonProps;

    #[test]
    fn common_should_have_expected_defaults() {
        let props = CommonProps::default();

        // test defaults
        assert_eq!(
            props.get(Attribute::Style).unwrap().unwrap_style(),
            Style::new()
        );
        assert_eq!(
            props.get(Attribute::FocusStyle).unwrap().unwrap_style(),
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
            Attribute::FocusStyle,
            AttrValue::Style(Style::new().add_modifier(TextModifiers::REVERSED)),
        );

        assert_eq!(
            props.get(Attribute::FocusStyle).unwrap().unwrap_style(),
            Style::new().add_modifier(TextModifiers::REVERSED)
        );

        // flags

        props.set(Attribute::Display, AttrValue::Flag(false));
        props.set(Attribute::Focus, AttrValue::Flag(true));

        assert!(!props.get(Attribute::Display).unwrap().unwrap_flag());
        assert!(props.get(Attribute::Focus).unwrap().unwrap_flag());

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
            Title::default()
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

        let block = props.get_block().unwrap();

        assert_eq!(
            block,
            Block::new()
                .title_bottom(LineStatic::from("Hello").centered())
                .borders(BorderSides::TOP)
                .border_type(BorderType::Double)
        );
    }
}
