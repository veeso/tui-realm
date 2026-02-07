//! ## Spinner
//!
//! A loading spinner. You can provide the "spinning sequence". At each `view()` call, the sequence step is increased

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{AttrValue, Attribute, Color, HorizontalAlignment, Props, Style};
use tuirealm::ratatui::text::Line as Spans;
use tuirealm::ratatui::{
    layout::Rect,
    text::{Span as TuiSpan, Text},
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
        let ch = self.sequence.get(self.step).copied().unwrap_or(' ');
        // Incr step
        if self.step + 1 >= self.sequence.len() {
            self.step = 0;
        } else {
            self.step += 1;
        }
        ch
    }

    /// Get the current char to display
    ///
    /// Unlike [`step`](Self::step), this function does not increment the step.
    pub fn current_step(&self) -> char {
        self.sequence.get(self.step).copied().unwrap_or(' ')
    }
}

// -- Component

/// ## Spinner
///
/// A textual spinner which step changes at each `view()` call
#[must_use]
pub struct Spinner {
    props: Props,
    pub states: SpinnerStates,
    /// Automatically call [`SpinnerStates::step`] in [`view`](Spinner::view).
    ///
    /// This option might be removed in a future major version
    pub view_auto_step: bool,
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            props: Default::default(),
            states: Default::default(),
            view_auto_step: true,
        }
    }
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

    pub fn sequence<S: Into<String>>(mut self, s: S) -> Self {
        self.attr(Attribute::Text, AttrValue::String(s.into()));
        self
    }

    /// Dont automatically step the sequence in a [`view`](Self::view) call
    pub fn manual_step(mut self) -> Self {
        self.view_auto_step = false;
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
            let seq_char = if self.view_auto_step {
                self.states.step()
            } else {
                self.states.current_step()
            };
            let text: Text = Text::from(Spans::from(TuiSpan::from(seq_char.to_string())));
            render.render_widget(
                Paragraph::new(text)
                    .alignment(HorizontalAlignment::Left)
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
    use tuirealm::ratatui::{self};

    #[test]
    fn test_components_span() {
        let component = Spinner::default()
            .background(Color::Blue)
            .foreground(Color::Red)
            .sequence("⣾⣽⣻⢿⡿⣟⣯⣷");
        // Get value
        assert_eq!(component.state(), State::None);
    }

    #[test]
    fn should_step_in_view() {
        let mut component = Spinner::default().sequence("123");

        assert_eq!(component.states.step, 0);

        let mut terminal =
            ratatui::Terminal::new(ratatui::backend::TestBackend::new(16, 16)).unwrap();

        terminal
            .draw(|f| {
                component.view(f, f.area());
                assert_eq!(component.states.step, 1);
            })
            .unwrap();

        terminal
            .draw(|f| {
                component.view(f, f.area());
                assert_eq!(component.states.step, 2);
            })
            .unwrap();

        terminal
            .draw(|f| {
                component.view(f, f.area());
                assert_eq!(component.states.step, 0);
            })
            .unwrap();
    }

    #[test]
    fn should_not_step_in_view() {
        let mut component = Spinner::default().sequence("123").manual_step();

        assert_eq!(component.states.step, 0);

        let mut terminal =
            ratatui::Terminal::new(ratatui::backend::TestBackend::new(16, 16)).unwrap();

        terminal
            .draw(|f| {
                component.view(f, f.area());
                assert_eq!(component.states.step, 0);
            })
            .unwrap();

        component.states.step();

        terminal
            .draw(|f| {
                component.view(f, f.area());
                assert_eq!(component.states.step, 1);
            })
            .unwrap();

        component.states.step();

        terminal
            .draw(|f| {
                component.view(f, f.area());
                assert_eq!(component.states.step, 2);
            })
            .unwrap();

        component.states.step();

        terminal
            .draw(|f| {
                component.view(f, f.area());
                assert_eq!(component.states.step, 0);
            })
            .unwrap();
    }
}
