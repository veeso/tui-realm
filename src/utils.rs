//! ## Utils
//!
//! Utilities functions to work with components

// deps
extern crate textwrap;
extern crate unicode_width;
use std::borrow::Cow;

// local
use tuirealm::props::{Borders, Title};
// ext
use tuirealm::ratatui::style::{Color, Style};
use tuirealm::ratatui::text::{Line, Span, Text};
use tuirealm::ratatui::widgets::{Block, TitlePosition};
use unicode_width::UnicodeWidthStr;

/// ### wrap_spans
///
/// Given a vector of [`Span`]s, it creates a list of `Spans` which mustn't exceed the provided width parameter.
/// Each [`Line`] in the returned `Vec` is a line in the text.
#[must_use]
pub fn wrap_spans<'a, 'b: 'a>(spans: &[&'b Span<'a>], width: usize) -> Vec<Line<'a>> {
    // Prepare result (capacity will be at least spans.len)
    let mut res: Vec<Line> = Vec::with_capacity(spans.len());
    // Prepare environment
    let mut line_width: usize = 0; // Incremental line width; mustn't exceed `width`.
    let mut line_spans: Vec<Span> = Vec::new(); // Current line; when done, push to res and re-initialize
    for span in spans {
        // Check if width would exceed...
        if line_width + span.content.width() > width {
            // Check if entire line is wider than the area
            if span.content.width() > width {
                // Wrap
                let span_lines = textwrap::wrap(&span.content, width);
                // iter lines
                for span_line in span_lines {
                    // Check if width would exceed...
                    if line_width + span_line.width() > width {
                        // New line
                        res.push(Line::from(line_spans));
                        line_width = 0;
                        line_spans = Vec::new();
                    }
                    // Increment line width
                    line_width += span_line.width();
                    // Push to line
                    line_spans.push(Span::styled(span_line, span.style));
                }
                // Go to next iteration
                continue;
            }
            // Just initialize a new line
            res.push(Line::from(line_spans));
            line_width = 0;
            line_spans = Vec::new();
        }
        // Push span to line
        line_width += span.content.width();
        line_spans.push(Span::styled(span.content.to_string(), span.style));
    }
    // if there are still elements in spans, push to result
    if !line_spans.is_empty() {
        res.push(Line::from(line_spans));
    }
    // return res
    res
}

/// ### get_block
///
/// Construct a block for widget using block properties.
/// If focus is true the border color is applied, otherwise inactive_style
#[must_use]
pub fn get_block(
    props: Borders,
    title: Option<&Title>,
    focus: bool,
    inactive_style: Option<Style>,
) -> Block<'_> {
    let mut block = Block::default()
        .borders(props.sides)
        .border_style(if focus {
            props.style()
        } else {
            inactive_style.unwrap_or_else(|| Style::default().fg(Color::Reset).bg(Color::Reset))
        })
        .border_type(props.modifiers);

    if let Some(title) = title {
        block = match title.position {
            TitlePosition::Top => block.title_top(borrow_clone_line(&title.content)),
            TitlePosition::Bottom => block.title_bottom(borrow_clone_line(&title.content)),
        };
    }

    block
}

/// ### calc_utf8_cursor_position
///
/// Calculate the UTF8 compliant position for the cursor given the characters preceeding the cursor position.
/// Use this function to calculate cursor position whenever you want to handle UTF8 texts with cursors
#[must_use]
pub fn calc_utf8_cursor_position(chars: &[char]) -> u16 {
    chars.iter().collect::<String>().width() as u16
}

/// Convert a `&Span` to a `Span` by using [`Cow::Borrowed`].
///
/// Note that a normal `Span::clone` (and by extension `Cow::clone`) will preserve the `Cow` Variant.
pub fn borrow_clone_span<'a, 'b: 'a>(span: &'b Span<'a>) -> Span<'a> {
    Span {
        style: span.style,
        content: Cow::Borrowed(&*span.content),
    }
}

/// Convert a `&Line` to a `Line` by using [`Cow::Borrowed`].
pub fn borrow_clone_line<'a, 'b: 'a>(line: &'b Line<'a>) -> Line<'a> {
    Line {
        spans: line.spans.iter().map(borrow_clone_span).collect(),
        ..*line
    }
}

/// Convert a `&Text` to a `Text` by using [`Cow::Borrowed`].
pub fn borrow_clone_text<'a, 'b: 'a>(text: &'b Text<'a>) -> Text<'a> {
    Text {
        lines: text.lines.iter().map(borrow_clone_line).collect(),
        ..*text
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use tuirealm::props::{Alignment, BorderSides, BorderType};

    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_utils_wrap_spans() {
        // Prepare spans; let's start with two simple spans, which fits the line
        let spans: Vec<Span> = vec![Span::from("hello, "), Span::from("world!")];
        let spans: Vec<&Span> = spans.iter().collect();
        assert_eq!(wrap_spans(&spans, 64).len(), 1);
        // Let's make a sentence, which would require two lines
        let spans: Vec<Span> = vec![
            Span::from("Hello, everybody, I'm Uncle Camel!"),
            Span::from("How's it going today?"),
        ];
        let spans: Vec<&Span> = spans.iter().collect();
        assert_eq!(wrap_spans(&spans, 32).len(), 2);
        // Let's make a sentence, which requires 3 lines, but with only one span
        let spans: Vec<Span> = vec![Span::from(
            "Hello everybody! My name is Uncle Camel. How's it going today?",
        )];
        let spans: Vec<&Span> = spans.iter().collect();
        // makes Hello everybody, my name is uncle, camel. how's it, goind today
        assert_eq!(wrap_spans(&spans, 16).len(), 4);
        // Combine
        let spans: Vec<Span> = vec![
            Span::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit."),
            Span::from("Canem!"),
            Span::from("In posuere sollicitudin vulputate"),
            Span::from("Sed vitae rutrum quam."),
        ];
        let spans: Vec<&Span> = spans.iter().collect();
        // "Lorem ipsum dolor sit amet,", "consectetur adipiscing elit. Canem!", "In posuere sollicitudin vulputate", "Sed vitae rutrum quam."
        assert_eq!(wrap_spans(&spans, 36).len(), 4);
    }

    #[test]
    fn wrap_spans_should_preserve_style_if_wrapped() {
        let input = [
            Span::styled("hello there", Style::new().fg(Color::Black)),
            Span::raw("test"),
        ];
        let input = input.iter().collect::<Vec<_>>();
        let res = wrap_spans(&input, 5);
        assert_eq!(res.len(), 3);
        assert_eq!(
            res[0],
            Line::from(Span::styled("hello", Style::new().fg(Color::Black)))
        );
        assert_eq!(
            res[1],
            Line::from(Span::styled("there", Style::new().fg(Color::Black)))
        );
        assert_eq!(res[2], Line::from(Span::raw("test")));
    }

    #[test]
    fn test_components_utils_get_block() {
        let borders = Borders::default()
            .sides(BorderSides::ALL)
            .color(Color::Red)
            .modifiers(BorderType::Rounded);
        let _ = get_block(
            borders,
            Some(&Title::from("title").alignment(Alignment::Center)),
            true,
            None,
        );
        let _ = get_block(borders, None, false, None);
    }

    #[test]
    fn test_components_utils_calc_utf8_cursor_position() {
        let chars: Vec<char> = vec!['v', 'e', 'e', 's', 'o'];
        // Entire
        assert_eq!(calc_utf8_cursor_position(chars.as_slice()), 5);
        assert_eq!(calc_utf8_cursor_position(&chars[0..3]), 3);
        // With special characters
        let chars: Vec<char> = vec!['я', ' ', 'х', 'о', 'ч', 'у', ' ', 'с', 'п', 'а', 'т', 'ь'];
        assert_eq!(calc_utf8_cursor_position(&chars[0..6]), 6);
        let chars: Vec<char> = vec!['H', 'i', '😄'];
        assert_eq!(calc_utf8_cursor_position(chars.as_slice()), 4);
        let chars: Vec<char> = vec!['我', '之', '😄'];
        assert_eq!(calc_utf8_cursor_position(chars.as_slice()), 6);
    }
}
