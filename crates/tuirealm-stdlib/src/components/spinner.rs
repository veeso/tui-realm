//! A loading spinner. You can provide the "spinning sequence". At each `view()` call, the sequence step is increased

use tuirealm::command::{Cmd, CmdResult};
use tuirealm::component::Component;
use tuirealm::props::{
    AttrValue, Attribute, Color, HorizontalAlignment, Props, QueryResult, Style,
};
use tuirealm::ratatui::Frame;
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Paragraph;
use tuirealm::state::State;

use crate::prop_ext::CommonProps;

// -- states

/// The state that has to be kept for the [`Spinner`] component.
#[derive(Default)]
pub struct SpinnerStates {
    pub sequence: Vec<char>,
    pub step: usize,
}

impl SpinnerStates {
    /// Set a new sequence of characters and reset the stepping to 0.
    pub fn reset(&mut self, sequence: &str) {
        self.sequence = sequence.chars().collect();
        self.step = 0;
    }

    /// Get the current step's characters in the sequence, and increment the stepping.
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

    /// Get the current character to display.
    ///
    /// Unlike [`step`](Self::step), this function does not increment the step.
    pub fn current_step(&self) -> char {
        self.sequence.get(self.step).copied().unwrap_or(' ')
    }
}

// -- Component

/// A textual spinner which step changes at each `view()` call (if `manual_step` is disabled).
#[must_use]
pub struct Spinner {
    common: CommonProps,
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
            common: CommonProps::default(),
            props: Props::default(),
            states: SpinnerStates::default(),
            view_auto_step: true,
        }
    }
}

impl Spinner {
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

    /// Set the sequence of characters to step through.
    pub fn sequence<S: Into<String>>(mut self, s: S) -> Self {
        self.attr(Attribute::Text, AttrValue::String(s.into()));
        self
    }

    /// Disable automatically stepping the sequence in a [`view`](Self::view) call.
    pub fn manual_step(mut self) -> Self {
        self.view_auto_step = false;
        self
    }
}

impl Component for Spinner {
    fn view(&mut self, render: &mut Frame, area: Rect) {
        if !self.common.display {
            return;
        }

        // Get text
        let seq_char = if self.view_auto_step {
            self.states.step()
        } else {
            self.states.current_step()
        };
        render.render_widget(
            Paragraph::new(seq_char.to_string())
                .alignment(HorizontalAlignment::Left)
                .style(self.common.style),
            area,
        );
    }

    fn query<'a>(&'a self, attr: Attribute) -> Option<QueryResult<'a>> {
        if let Some(value) = self.common.get_for_query(attr) {
            return Some(value);
        }

        self.props.get_for_query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if let Some(value) = self.common.set(attr, value) {
            if matches!(attr, Attribute::Text) {
                // Update sequence
                self.states.reset(value.unwrap_string().as_str());
            } else {
                self.props.set(attr, value);
            }
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

    use pretty_assertions::assert_eq;
    use tuirealm::ratatui::{self};

    use super::*;

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
