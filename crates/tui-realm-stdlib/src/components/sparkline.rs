//! A sparkline over more lines.

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, Style, Title,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Sparkline as TuiSparkline;
use tuirealm::state::State;

use crate::prop_ext::CommonProps;

// -- component

/// A sparkline over more lines.
///
/// A sparkline can be interpreted as a dense Vertical Bar Chart, without labels for each line.
///
/// This can be used for audio-level visualization or a type of history graph (like bandwidth, cpu usage, etc).
#[derive(Default)]
#[must_use]
pub struct Sparkline {
    common: CommonProps,
    props: Props,
}

impl Sparkline {
    /// Set the main foreground color. This may get overwritten by individual text styles.
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    /// Set the main background color. This may get overwritten by individual text styles.
    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    /// Set the main style. This may get overwritten by individual text styles.
    ///
    /// This option will overwrite any previous [`foreground`](Self::foreground), [`background`](Self::background)!
    pub fn style(mut self, style: Style) -> Self {
        self.attr(Attribute::Style, AttrValue::Style(style));
        self
    }

    /// Set a custom style for the border when the component is unfocused.
    pub fn inactive(mut self, s: Style) -> Self {
        self.attr(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    /// Add a border to the component.
    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    /// Add a title to the component.
    pub fn title<T: Into<Title>>(mut self, title: T) -> Self {
        self.attr(Attribute::Title, AttrValue::Title(title.into()));
        self
    }

    /// Set the max value of the bar.
    pub fn max_entries(mut self, max: usize) -> Self {
        self.attr(Attribute::Width, AttrValue::Length(max));
        self
    }

    /// Set the initial data.
    pub fn data(mut self, data: &[u64]) -> Self {
        self.attr(
            Attribute::Dataset,
            AttrValue::Payload(PropPayload::Vec(
                data.iter().map(|x| PropValue::U64(*x)).collect(),
            )),
        );
        self
    }

    /// ### data_len
    ///
    /// Retrieve current data len from properties
    fn data_len(&self) -> usize {
        self.props
            .get(Attribute::Dataset)
            .map_or(0, |x| x.unwrap_payload().unwrap_vec().len())
    }

    /// ### data
    ///
    /// Get data to be displayed, starting from provided index at `start` with a max length of `len`
    fn get_data(&self, max: usize) -> Vec<u64> {
        match self
            .props
            .get(Attribute::Dataset)
            .map(|x| x.unwrap_payload())
        {
            Some(PropPayload::Vec(list)) => {
                let mut data: Vec<u64> = Vec::with_capacity(max);
                list.iter()
                    .take(max)
                    .cloned()
                    .map(|x| x.unwrap_u64())
                    .for_each(|x| data.push(x));
                data
            }
            _ => Vec::new(),
        }
    }
}

impl Component for Sparkline {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if !self.common.display {
            return;
        }

        let max_entries = self
            .props
            .get_or(Attribute::Width, AttrValue::Length(self.data_len()))
            .unwrap_length();
        // Get data
        let data: Vec<u64> = self.get_data(max_entries);
        // Create widget
        let mut widget = TuiSparkline::default()
            .data(data.as_slice())
            .max(max_entries as u64)
            .style(self.common.style);

        if let Some(block) = self.common.get_block() {
            widget = widget.block(block);
        }

        // Render
        render.render_widget(widget, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        if let Some(value) = self.common.get(attr) {
            return Some(value);
        }

        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if let Some(value) = self.common.set(attr, value) {
            self.props.set(attr, value);
        }
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;
    use tuirealm::props::HorizontalAlignment;

    use super::*;

    #[test]
    fn test_components_sparkline() {
        let component = Sparkline::default()
            .background(Color::White)
            .foreground(Color::Black)
            .title(Title::from("bandwidth").alignment(HorizontalAlignment::Center))
            .borders(Borders::default())
            .max_entries(8)
            .data(&[
                60, 80, 90, 88, 76, 101, 98, 93, 96, 102, 110, 99, 88, 75, 34, 45, 67, 102,
            ]);
        // Commands
        assert_eq!(component.state(), State::None);
        // component funcs
        assert_eq!(component.data_len(), 18);
        assert_eq!(component.get_data(4), vec![60, 80, 90, 88]);
    }
}
