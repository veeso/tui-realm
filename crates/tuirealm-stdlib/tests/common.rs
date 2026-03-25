use tuirealm::component::Component;
use tuirealm::ratatui::buffer::Buffer;
use tuirealm::ratatui::layout::Rect;
use tuirealm::terminal::{TerminalAdapter, TestTerminalAdapter};

/// Convert a ratatui Buffer to a string, one line per row, trailing whitespace trimmed.
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

/// Render a component into a TestTerminalAdapter and return the buffer as a string.
pub fn render_to_string(component: &mut dyn Component, width: u16, height: u16) -> String {
    let mut adapter = TestTerminalAdapter::new(width, height).unwrap();
    let area = Rect::new(0, 0, width, height);
    let completed = adapter.raw_mut().draw(|f| component.view(f, area)).unwrap();
    buffer_to_string(completed.buffer)
}
