//! Utilities functions to work with components

use std::borrow::Cow;

use tuirealm::props::{Borders, Title};
use tuirealm::ratatui::style::Style;
use tuirealm::ratatui::text::{Line, Span, Text};
use tuirealm::ratatui::widgets::{Block, TitlePosition};
use unicode_width::UnicodeWidthStr;

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

    res
}

/// Make a new empty [`Line`], but with the original style applied.
#[inline]
fn make_new_line<'a>(orig: &Line<'a>) -> Line<'a> {
    Line::default().style(orig.style)
}

/// Commit the current `newline` and create a new one in its place
#[inline]
fn commit_line<'a>(newline: &mut Line<'a>, newlines: &mut Vec<Line<'a>>, orig: &Line<'a>) {
    let mut final_line = make_new_line(orig);
    std::mem::swap(newline, &mut final_line);
    newlines.push(final_line);
}

/// Wrap a single [`Span`] into multiple [`Line`]s.
///
/// Returns the amount of consumed width in the last line.
fn wrap_single_span<'a>(
    span: &'a Span<'a>,
    newlines: &mut Vec<Line<'a>>,
    newline: &mut Line<'a>,
    orig_line: &Line<'a>,
    width: usize,
    consumed_width: &mut usize,
) -> usize {
    let mut remainder_width = width - *consumed_width;

    // textwrap seemingly adds at least *one* character if the given width is 0
    // so lets commit the line here so that we dont run into that case.
    if remainder_width == 0 && newline.width() != 0 {
        commit_line(newline, newlines, orig_line);
        remainder_width = width;
    }

    // Use textwrap for the actual splitting.
    // We know here that wrapping *is* necessary.
    let words = textwrap::WordSeparator::AsciiSpace.find_words(&span.content);
    let split_words =
        textwrap::word_splitters::split_words(words, &textwrap::WordSplitter::HyphenSplitter);
    let broken_words = textwrap::core::break_words(split_words, remainder_width);

    let line_widths = [remainder_width, width];
    let wrapped_words = textwrap::WrapAlgorithm::FirstFit.wrap(&broken_words, &line_widths);

    // The index into "span.content" which is already consumed
    let mut consumed_idx = 0;
    let last_idx = wrapped_words.len().saturating_sub(1);
    let mut final_consumed_width = 0;
    // Each "words" loop represents a final line, except for the last iteration.
    for (idx, words) in wrapped_words.iter().enumerate() {
        if words.is_empty() {
            continue;
        }

        // The following is disabled as this can (in its current form) only catch some whitespaces to trim
        // but it would then not fully align with the fast-path in the other function.
        // so for our purposes, it is not worth it.
        // // only trim the last whitespace *if* we know we commit the line here
        // // as otherwise, we dont know if something *might* follow
        // let minus_whitespace = if idx != last_idx {
        //     words.last().map_or(0, |word| word.whitespace.len())
        // } else {
        //     0
        // };
        let minus_whitespace = 0;

        // length in bytes of the current words line
        let len = words
            .iter()
            .map(|word| word.len() + word.whitespace.len())
            .sum::<usize>()
            - minus_whitespace;

        let split_text = &span.content[consumed_idx..consumed_idx + len];
        consumed_idx += len + minus_whitespace;

        let newspan = Span::styled(split_text, span.style);
        newline.push_span(newspan);

        // unless this is the last loop, there are more lines to come
        if idx != last_idx {
            commit_line(newline, newlines, orig_line);
        } else {
            final_consumed_width = newline.width();
        }
    }

    final_consumed_width
}

/// Wrap the given lines to fit within `width`.
pub fn wrap_lines<'a, 'b: 'a>(lines: &[&'b Line<'a>], width: usize) -> Vec<Line<'a>> {
    // Prepare result (capacity will be at least lines.len)
    let mut new_lines: Vec<Line> = Vec::with_capacity(lines.len());

    for line in lines {
        // fast path for when no wrapping is necessary
        if line.width() <= width {
            new_lines.push(borrow_clone_line(line));
            continue;
        }

        // Width that already has been consumed with the current line iteration
        let mut consumed_width: usize = 0;
        let mut newline = make_new_line(line);

        for span in line.iter() {
            // fast path for when no wrapping is necessary on a span-level
            let span_width = span.content.width();
            if span_width <= width - consumed_width {
                newline.push_span(borrow_clone_span(span));
                consumed_width += span_width;
                continue;
            }

            let new_consumed = wrap_single_span(
                span,
                &mut new_lines,
                &mut newline,
                line,
                width,
                &mut consumed_width,
            );
            consumed_width = new_consumed;
        }

        // commit the final newline, if it is not empty
        if !newline.spans.is_empty() {
            new_lines.push(newline);
        }
    }

    new_lines
}

/// Construct a [`Block`] widget from the given properties.
///
/// If `focus` is `true`, [`Borders::style`] is applied as the Border style, if `false` `inactive_style` is applied, if `Some`.
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
            inactive_style.unwrap_or_default()
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

/// Calculate the actual amount of terminal space taken up, taking into account UTF things like multi-width, undrawn and combinatory characters.
///
/// Use this function to calculate cursor position whenever you want to handle UTF8 texts with cursors
#[must_use]
pub fn calc_utf8_cursor_position(chars: &[char]) -> u16 {
    chars.iter().collect::<String>().width() as u16
}

/// Convert a `&Span` to a `Span` by using [`Cow::Borrowed`].
///
/// Note that a normal [`Span::clone`] (and by extension `Cow::clone`) will preserve the `Cow` Variant.
pub fn borrow_clone_span<'a, 'b: 'a>(span: &'b Span<'a>) -> Span<'a> {
    Span {
        content: Cow::Borrowed(&*span.content),
        ..*span
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

    use pretty_assertions::assert_eq;
    use tuirealm::props::{BorderSides, BorderType, Color, HorizontalAlignment};

    use super::*;

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
            Some(&Title::from("title").alignment(HorizontalAlignment::Center)),
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

    mod lines_wrap {
        use pretty_assertions::assert_eq;
        use tuirealm::props::{LineStatic, SpanStatic, Style};
        use tuirealm::ratatui::text::Span;

        use crate::utils::wrap_lines;

        #[test]
        fn should_not_do_any_wrapping() {
            // empty
            assert_eq!(
                wrap_lines(&[&LineStatic::default()], 10),
                [LineStatic::default()]
            );

            // single span, fits within width
            assert_eq!(
                wrap_lines(&[&LineStatic::raw("test")], 10),
                [LineStatic::raw("test")]
            );

            // multi span, fits within width
            assert_eq!(
                wrap_lines(
                    &[&LineStatic::from_iter([
                        SpanStatic::raw("hello"),
                        SpanStatic::raw("there")
                    ])],
                    10
                ),
                [LineStatic::from_iter([
                    SpanStatic::raw("hello"),
                    SpanStatic::raw("there")
                ])]
            );
        }

        #[test]
        fn should_wrap_single_span() {
            assert_eq!(
                wrap_lines(&[&LineStatic::raw("something really long")], 10),
                [
                    LineStatic::raw("something "),
                    LineStatic::raw("really "),
                    LineStatic::raw("long")
                ]
            );

            // should preserve styles
            assert_eq!(
                wrap_lines(
                    &[&LineStatic::from(Span::styled(
                        "something really long",
                        Style::default().crossed_out()
                    ))
                    .style(Style::default().italic())],
                    10
                ),
                [
                    LineStatic::from(Span::styled("something ", Style::default().crossed_out()))
                        .style(Style::default().italic()),
                    LineStatic::from(Span::styled("really ", Style::default().crossed_out()))
                        .style(Style::default().italic()),
                    LineStatic::from(Span::styled("long", Style::default().crossed_out()))
                        .style(Style::default().italic())
                ]
            );
        }

        #[test]
        fn should_wrap_multi_span() {
            assert_eq!(
                wrap_lines(
                    &[&LineStatic::from_iter([
                        SpanStatic::raw("something "),
                        SpanStatic::raw("really "),
                        SpanStatic::raw("long")
                    ])],
                    10
                ),
                [
                    LineStatic::raw("something "),
                    LineStatic::from_iter([Span::raw("really "), Span::raw("lon")]),
                    LineStatic::raw("g")
                ]
            );

            // should preserve styles
            assert_eq!(
                wrap_lines(
                    &[&LineStatic::from_iter([
                        SpanStatic::styled("something ", Style::default().crossed_out()),
                        SpanStatic::raw("really "),
                        SpanStatic::styled("long", Style::default().italic())
                    ])],
                    10
                ),
                [
                    LineStatic::from(Span::styled("something ", Style::default().crossed_out())),
                    LineStatic::from_iter([
                        Span::raw("really "),
                        Span::styled("lon", Style::default().italic())
                    ]),
                    LineStatic::from(Span::styled("g", Style::default().italic()))
                ]
            );
        }
    }
}
