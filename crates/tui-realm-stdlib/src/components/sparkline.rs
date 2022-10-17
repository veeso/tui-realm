//! ## Sparkline
//!
//! A sparkline over more lines

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, PropPayload, PropValue, Props, Style,
};
use tuirealm::tui::{layout::Rect, widgets::Sparkline as TuiSparkline};
use tuirealm::{Frame, MockComponent, State};

// -- component

/// ## Sparkline
///
/// A sparkline over more lines
#[derive(Default)]
pub struct Sparkline {
    props: Props,
}

impl Sparkline {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }

    pub fn max_entries(mut self, max: usize) -> Self {
        self.attr(Attribute::Width, AttrValue::Length(max));
        self
    }

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
            .map(|x| x.unwrap_payload().unwrap_vec().len())
            .unwrap_or(0)
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

impl MockComponent for Sparkline {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let title = self
                .props
                .get_or(
                    Attribute::Title,
                    AttrValue::Title((String::default(), Alignment::Center)),
                )
                .unwrap_title();
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let max_entries = self
                .props
                .get_or(Attribute::Width, AttrValue::Length(self.data_len()))
                .unwrap_length();
            // Get data
            let data: Vec<u64> = self.get_data(max_entries);
            // Create widget
            let widget: TuiSparkline = TuiSparkline::default()
                .block(crate::utils::get_block(borders, Some(title), false, None))
                .data(data.as_slice())
                .max(max_entries as u64)
                .style(Style::default().fg(foreground).bg(background));
            // Render
            render.render_widget(widget, area);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value)
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

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_sparkline() {
        let component = Sparkline::default()
            .background(Color::White)
            .foreground(Color::Black)
            .title("bandwidth", Alignment::Center)
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
