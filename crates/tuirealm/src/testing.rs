//! Utilities for snapshot and rendering tests.
//!
//! This module provides helpers to render a [`Component`] into a string
//! representation of the terminal buffer, which is useful for snapshot
//! testing with tools like [`insta`](https://docs.rs/insta).
//!
//! # Example
//!
//! ```no_run
//! # use tuirealm::component::Component;
//! # use tuirealm::testing::render_to_string;
//! # use tuirealm::ratatui::layout::Size;
//! #
//! fn assert_component_snapshot(component: &mut dyn Component) {
//!     let rendered = render_to_string(component, Size::new(40, 10));
//!     insta::assert_snapshot!(rendered);
//! }
//! ```

use crate::component::Component;
use crate::ratatui::buffer::Buffer;
use crate::ratatui::layout::Size;
use crate::terminal::{TerminalAdapter, TestTerminalAdapter};

/// Convert a ratatui [`Buffer`] to a string, one line per row,
/// with trailing whitespace trimmed from each line.
///
/// Each row of the buffer becomes a single line in the output,
/// terminated by a newline character.
///
/// # Arguments
///
/// * `buffer` — the ratatui [`Buffer`] to convert.
pub fn buffer_to_string(buffer: &Buffer) -> String {
    let area = *buffer.area();
    let mut result = String::new();
    for y in area.y..area.y + area.height {
        let mut line = String::new();
        for x in area.x..area.x + area.width {
            if let Some(cell) = buffer.cell((x, y)) {
                line.push_str(cell.symbol());
            }
        }
        result.push_str(line.trim_end());
        result.push('\n');
    }
    result
}

/// Render a [`Component`] into a [`TestTerminalAdapter`] and return
/// the buffer content as a string.
///
/// This creates an off-screen terminal of the given dimensions, calls
/// [`Component::view`] to render into it, and then converts the
/// resulting buffer to a string via [`buffer_to_string`].
///
/// # Arguments
///
/// * `component` — the component to render.
/// * `size` — width & height of the virtual terminal.
///
/// # Panics
///
/// Panics if the [`TestTerminalAdapter`] cannot be created or if the
/// draw call fails.
pub fn render_to_string(component: &mut dyn Component, size: Size) -> String {
    let mut adapter = TestTerminalAdapter::new(size).expect("failed to create TestTerminalAdapter");
    let completed = adapter
        .raw_mut()
        .draw(|f| component.view(f, f.area()))
        .expect("failed to draw component");
    buffer_to_string(completed.buffer)
}
