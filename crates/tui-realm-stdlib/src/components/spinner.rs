//! ## Spinner
//!
//! A loading spinner. You can provide the "spinning sequence". At each `view()` call, the sequence step is increased

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{Alignment, AttrValue, Attribute, Color, Props, Style};
use tuirealm::tui::{
    layout::Rect,
    text::{Span as TuiSpan, Spans, Text},
    widgets::Paragraph,
};
use tuirealm::{Frame, MockComponent, State};

// -- states

#[derive(Default)]
pub struct SpinnerStates {
    pub sequence: Vec<char>,
    pub step: usize,
}

impl SpinnerStates {
    /// ### reset
    ///
    /// Re initialize sequence
    pub fn reset(&mut self, sequence: &str) {
        self.sequence = sequence.chars().collect();
        self.step = 0;
    }

    /// ### step
    ///
    /// Get current step char and increments step
    pub fn step(&mut self) -> char {
        let ch = self.sequence.get(self.step).cloned().unwrap_or(' ');
        // Incr step
        if self.step + 1 >= self.sequence.len() {
            self.step = 0;
        } else {
            self.step += 1;
        }
        ch
    }
}

// -- Component

/// ## Spinner
///
/// A textual spinner which step changes at each `view()` call
#[derive(Default)]
pub struct Spinner {
    props: Props,
    pub states: SpinnerStates,
}

impl Spinner {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn sequence<S: AsRef<str>>(mut self, s: S) -> Self {
        self.attr(Attribute::Text, AttrValue::String(s.as_ref().to_string()));
        self
    }
}

impl MockComponent for Spinner {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        // Make a Span
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Make text
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            // Get text
            let text: Text = Text::from(Spans::from(TuiSpan::from(self.states.step().to_string())));
            render.render_widget(
                Paragraph::new(text)
                    .alignment(Alignment::Left)
                    .style(Style::default().bg(background).fg(foreground)),
                area,
            );
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if matches!(attr, Attribute::Text) {
            // Update sequence
            self.states.reset(value.unwrap_string().as_str());
        } else {
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
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_span() {
        let component = Spinner::default()
            .background(Color::Blue)
            .foreground(Color::Red)
            .sequence("⣾⣽⣻⢿⡿⣟⣯⣷");
        // Get value
        assert_eq!(component.state(), State::None);
    }
}
