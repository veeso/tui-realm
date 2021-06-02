//! ## Components
//!
//! `Components` provides a "standard" library of components.

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
// deps
extern crate textwrap;
extern crate unicode_width;
// local
use crate::props::{BordersProps, TextSpan};
use crate::Props;
// ext
use crate::tui::style::{Color, Modifier, Style};
use crate::tui::text::{Span, Spans};
use crate::tui::widgets::Block;
use unicode_width::UnicodeWidthStr;

/// ### wrap_spans
///
/// Given a vector of `TextSpans`, it creates a list of `Spans` which mustn't exceed the provided width parameter.
/// Each `Spans` in the returned `Vec` is a line in the text.
pub fn wrap_spans<'a>(spans: &[TextSpan], width: usize, props: &Props) -> Vec<Spans<'a>> {
    // Prepare result (capacity will be at least spans.len)
    let mut res: Vec<Spans> = Vec::with_capacity(spans.len());
    // Prepare environment
    let mut line_width: usize = 0; // Incremental line width; mustn't exceed `width`.
    let mut line_spans: Vec<Span> = Vec::new(); // Current line; when done, push to res and re-initialize
    for span in spans.iter() {
        // Get styles
        let (fg, bg, tmod) = use_or_default_styles(props, span);
        // Check if width would exceed...
        if line_width + span.content.width() > width {
            // Check if entire line is wider than the area
            if span.content.width() > width {
                // Wrap
                let span_lines = textwrap::wrap(span.content.as_str(), width);
                // iter lines
                for span_line in span_lines.iter() {
                    // Check if width would exceed...
                    if line_width + span_line.width() > width {
                        // New line
                        res.push(Spans::from(line_spans));
                        line_width = 0;
                        line_spans = Vec::new();
                    }
                    // Increment line width
                    line_width += span_line.width();
                    // Push to line
                    line_spans.push(Span::styled(
                        span_line.to_string(),
                        Style::default().fg(fg).bg(bg).add_modifier(tmod),
                    ));
                }
                // Go to next iteration
                continue;
            } else {
                // Just initialize a new line
                res.push(Spans::from(line_spans));
                line_width = 0;
                line_spans = Vec::new();
            }
        }
        // Push span to line
        line_width += span.content.width();
        line_spans.push(Span::styled(
            span.content.to_string(),
            Style::default().fg(fg).bg(bg).add_modifier(tmod),
        ));
    }
    // if there are still elements in spans, push to result
    if !line_spans.is_empty() {
        res.push(Spans::from(line_spans));
    }
    // return res
    res
}

/// ### use_or_default_styles
///
/// Returns the styles to be used; in case in span are default, use props'
pub fn use_or_default_styles(props: &Props, span: &TextSpan) -> (Color, Color, Modifier) {
    (
        match span.fg {
            Color::Reset => props.foreground,
            _ => span.fg,
        },
        match span.bg {
            Color::Reset => props.background,
            _ => span.bg,
        },
        match span.modifiers.is_empty() {
            true => props.modifiers,
            false => span.modifiers,
        },
    )
}

/// ### get_block
///
/// Get block
pub fn get_block<'a>(props: &BordersProps, title: &Option<String>, focus: bool) -> Block<'a> {
    let div: Block = Block::default()
        .borders(props.borders)
        .border_style(match focus {
            true => props.style(),
            false => Style::default(),
        })
        .border_type(props.variant);
    // Set title
    match title.as_ref() {
        Some(t) => div.title(t.to_string()),
        None => div,
    }
}

/// ### calc_utf8_cursor_position
///
/// Calculate the UTF8 compliant position for the cursor given the characters preceeding the cursor position.
/// Use this function to calculate cursor position whenever you want to handle UTF8 texts with cursors
pub fn calc_utf8_cursor_position(chars: &[char]) -> u16 {
    chars.iter().collect::<String>().width() as u16
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::props::builder::PropsBuilder;
    use crate::props::TextSpanBuilder;
    use crate::tui::widgets::{BorderType, Borders};
    use crate::GenericPropsBuilder;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_components_utils_wrap_spans() {
        let props: Props = GenericPropsBuilder::default()
            .bold()
            .with_foreground(Color::Red)
            .with_background(Color::White)
            .build();
        // Prepare spans; let's start with two simple spans, which fits the line
        let spans: Vec<TextSpan> = vec![TextSpan::from("hello, "), TextSpan::from("world!")];
        assert_eq!(wrap_spans(&spans, 64, &props).len(), 1);
        // Let's make a sentence, which would require two lines
        let spans: Vec<TextSpan> = vec![
            TextSpan::from("Hello, everybody, I'm Uncle Camel!"),
            TextSpan::from("How's it going today?"),
        ];
        assert_eq!(wrap_spans(&spans, 32, &props).len(), 2);
        // Let's make a sentence, which requires 3 lines, but with only one span
        let spans: Vec<TextSpan> = vec![TextSpan::from(
            "Hello everybody! My name is Uncle Camel. How's it going today?",
        )];
        // makes Hello everybody, my name is uncle, camel. how's it, goind today
        assert_eq!(wrap_spans(&spans, 16, &props).len(), 4);
        // Combine
        let spans: Vec<TextSpan> = vec![
            TextSpan::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit."),
            TextSpan::from("Canem!"),
            TextSpan::from("In posuere sollicitudin vulputate"),
            TextSpan::from("Sed vitae rutrum quam."),
        ];
        // "Lorem ipsum dolor sit amet,", "consectetur adipiscing elit. Canem!", "In posuere sollicitudin vulputate", "Sed vitae rutrum quam."
        assert_eq!(wrap_spans(&spans, 36, &props).len(), 4);
    }

    #[test]
    fn test_components_utils_use_or_default_styles() {
        let props: Props = GenericPropsBuilder::default()
            .bold()
            .with_foreground(Color::Red)
            .with_background(Color::White)
            .build();
        let span: TextSpan = TextSpanBuilder::new("test")
            .underlined()
            .with_foreground(Color::Yellow)
            .with_background(Color::Cyan)
            .build();
        // Not-default
        let (fg, bg, modifiers) = use_or_default_styles(&props, &span);
        assert_eq!(fg, Color::Yellow);
        assert_eq!(bg, Color::Cyan);
        assert!(modifiers.intersects(Modifier::UNDERLINED));
        // Default
        let span: TextSpan = TextSpan::from("test");
        let (fg, bg, modifiers) = use_or_default_styles(&props, &span);
        assert_eq!(fg, Color::Red);
        assert_eq!(bg, Color::White);
        assert!(modifiers.intersects(Modifier::BOLD));
    }

    #[test]
    fn test_components_utils_get_block() {
        let props: BordersProps = BordersProps {
            borders: Borders::ALL,
            variant: BorderType::Rounded,
            color: Color::Red,
        };
        get_block(&props, &Some(String::from("title")), true);
        get_block(&props, &None, false);
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
