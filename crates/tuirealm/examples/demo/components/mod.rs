//! ## Components
//!
//! demo example components

use tuirealm::props::{Borders, Color, Style, Title};
use tuirealm::ratatui::widgets::{Block, TitlePosition};

use super::Msg;

// -- modules
mod clock;
mod counter;
mod label;

// -- export
pub use clock::Clock;
pub use counter::{DigitCounter, LetterCounter};
pub use label::Label;

/// ### `get_block`
///
/// Get block
pub(crate) fn get_block<'a>(props: Borders, title: Title, focus: bool) -> Block<'a> {
    let block = Block::default()
        .borders(props.sides)
        .border_style(if focus {
            props.style()
        } else {
            Style::default().fg(Color::Reset).bg(Color::Reset)
        })
        .border_type(props.modifiers);

    match title.position {
        TitlePosition::Top => block.title_top(title.content),
        TitlePosition::Bottom => block.title_bottom(title.content),
    }
}
