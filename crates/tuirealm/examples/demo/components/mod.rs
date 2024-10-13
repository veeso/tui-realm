//! ## Components
//!
//! demo example components

use tuirealm::props::{Alignment, Borders, Color, Style};
use tuirealm::ratatui::widgets::Block;

use super::Msg;

// -- modules
mod clock;
mod counter;
mod label;

// -- export
pub use clock::Clock;
pub use counter::{DigitCounter, LetterCounter};
pub use label::Label;

/// ### get_block
///
/// Get block
pub(crate) fn get_block<'a>(props: Borders, title: (String, Alignment), focus: bool) -> Block<'a> {
    Block::default()
        .borders(props.sides)
        .border_style(match focus {
            true => props.style(),
            false => Style::default().fg(Color::Reset).bg(Color::Reset),
        })
        .border_type(props.modifiers)
        .title(title.0)
        .title_alignment(title.1)
}
