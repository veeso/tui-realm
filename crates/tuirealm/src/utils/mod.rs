//! This module exposes utilities

pub mod parser;
mod types;

use std::borrow::Cow;

use ratatui::text::{Line, Span, Text};
// export types
pub use types::{Email, PhoneNumber};

use crate::props::{LineStatic, SpanStatic, TextStatic};

/// Convert a `&Span` to a `Span` by using [`Cow::Owned`].
///
/// Note that a normal [`Span::clone`] (and by extension `Cow::clone`) will preserve the `Cow` Variant.
pub fn clone_span<'a, 'b: 'a>(span: &'b Span<'a>) -> SpanStatic {
    Span {
        content: Cow::Owned(span.content.to_string()),
        ..*span
    }
}

/// Convert a `&Line` to a `Line` by using [`Cow::Owned`].
pub fn clone_line<'a, 'b: 'a>(line: &'b Line<'a>) -> LineStatic {
    Line {
        spans: line.spans.iter().map(clone_span).collect(),
        ..*line
    }
}

/// Convert a `&Text` to a `Text` by using [`Cow::Owned`].
pub fn clone_text<'a, 'b: 'a>(text: &'b Text<'a>) -> TextStatic {
    Text {
        lines: text.lines.iter().map(clone_line).collect(),
        ..*text
    }
}
