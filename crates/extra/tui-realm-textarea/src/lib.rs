//! # tui-realm-textarea
//!
//! [tui-realm-textarea](https://github.com/veeso/tui-realm-textarea) is a
//! [tui-realm](https://github.com/veeso/tui-realm) implementation of a textarea component.
//! The tree engine is based on [Orange-trees](https://docs.rs/orange-trees/).
//!
//! ## Get Started
//!
//! ### Adding `tui-realm-textarea` as dependency
//!
//! ```toml
//! tui-realm-textarea = "^1.1.0"
//! ```
//!
//! Or if you don't use **Crossterm**, define the backend as you would do with tui-realm:
//!
//! ```toml
//! tui-realm-textarea = { version = "^1.1.0", default-features = false, features = [ "with-termion" ] }
//! ```
//!
//! #### Features ⚙️

//! These features can be enabled in tui-realm-textarea:
//!
//! - `clipboard` enables system clipboard support
//! - `search` enables the string search in the textarea
//!
//! ## Component API
//!
//! **Commands**:
//!
//! | Cmd                                            | Result         | Behaviour                               |
//! |------------------------------------------------|----------------|-----------------------------------------|
//! | `Custom($TEXTAREA_CMD_NEWLINE)`                | `None`         | Insert newline                          |
//! | `Custom($TEXTAREA_CMD_DEL_LINE_BY_END)`        | `None`         | Delete line by end to current position  |
//! | `Custom($TEXTAREA_CMD_DEL_LINE_BY_HEAD)`       | `None`         | Delete line by head to current position |
//! | `Custom($TEXTAREA_CMD_DEL_WORD)`               | `None`         | Delete the current word                 |
//! | `Custom($TEXTAREA_CMD_DEL_NEXT_WORD)`          | `None`         | Delete the next word                    |
//! | `Custom($TEXTAREA_CMD_MOVE_WORD_FORWARD)`      | `None`         | Move to the next word                   |
//! | `Custom($TEXTAREA_CMD_MOVE_WORD_BACK)`         | `None`         | Move to the previous word               |
//! | `Custom($TEXTAREA_CMD_MOVE_PARAGRAPH_BACK)`    | `None`         | Move to the previous paragraph          |
//! | `Custom($TEXTAREA_CMD_MOVE_PARAGRAPH_FORWARD)` | `None`         | Move to the next paragraph              |
//! | `Custom($TEXTAREA_CMD_MOVE_TOP)`               | `None`         | Move to the beginning of the file       |
//! | `Custom($TEXTAREA_CMD_MOVE_BOTTOM)`            | `None`         | Move to the end of the file             |
//! | `Custom($TEXTAREA_CMD_UNDO)`                   | `None`         | Undo last change                        |
//! | `Custom($TEXTAREA_CMD_REDO)`                   | `None`         | Redo last change                        |
//! | `Custom($TEXTAREA_CMD_PASTE)`                  | `None`         | Paste the current content of the buffer |
//! | `Custom($TEXTAREA_CMD_SEARCH_BACK)`            | `None`         | Go to the previous search match         |
//! | `Custom($TEXTAREA_CMD_SEARCH_FORWARD)`         | `None`         | Go to the next search match             |
//! | `Cancel`                                       | `None`         | Delete next char                        |
//! | `Delete`                                       | `None`         | Delete previous char                    |
//! | `GoTo(Begin)`                                  | `None`         | Go to the head of the line              |
//! | `GoTo(End)`                                    | `None`         | Go to the end of the line               |
//! | `Move(Down)`                                   | `None`         | Move to the line below                  |
//! | `Move(Up)`                                     | `None`         | Move to the line above                  |
//! | `Move(Left)`                                   | `None`         | Move cursor to the left                 |
//! | `Move(Right)`                                  | `None`         | Move cursor to the right                |
//! | `Scroll(Up)`                                   | `None`         | Move by scroll_step lines up            |
//! | `Scroll(Down)`                                 | `None`         | Move by scroll_step lines down          |
//! | `Type(ch)`                                     | `None`         | Type a char in the editor               |
//! | `Submit`                                       | `Submit`       | Get current lines                       |
//!
//! > ❗ Paste command is supported only if the `clipboard` feature is enabled
//!
//! **State**: the state returned is a `Vec(String)` containing the lines in the text area.
//!
//! **Properties**:
//!
//! - `Borders(Borders)`: set borders properties for component
//! - `Custom($TREE_IDENT_SIZE, Size)`: Set space to render for each each depth level
//! - `Custom($TEXTAREA_MAX_HISTORY, Payload(One(Usize)))`: Set the history steps to record
//! - `Custom($TEXTAREA_CURSOR_STYLE, Style)`: Set the cursor style
//! - `Custom($TEXTAREA_CURSOR_LINE_STYLE, Style)`: Set the current line style
//! - `Custom($TEXTAREA_FOOTER_FMT, Payload(Tup2(Str, Style)))`: Set the format and the style for the footer bar
//! - `Custom($TEXTAREA_LINE_NUMBER_STYLE, Style)`: set the style for the line number
//! - `Custom($TEXTAREA_STATUS_FMT, Payload(Tup2(Str, Style)))`: Set the format and the style for the status bar
//! - `Custom($TEXTAREA_SEARCH_PATTERN, String`: Set search pattern
//! - `Custom($TEXTAREA_SEARCH_STYLE, Style`: Set search style
//! - `Style(Style)`: Set the general style for the textarea
//! - `Custom($TEXTAREA_TAB_SIZE, Size)`: Set the tab size to display
//! - `FocusStyle(Style)`: inactive style
//! - `ScrollStep(Length)`: Defines the maximum amount of rows to scroll
//! - `Title(Title)`: Set box titleù
//!
//! ### Footer and status format
//!
//! The status and footer bars support a special syntax. The following keys can be inserted into the string:
//!
//! - `{ROW}`: current row
//! - `{COL}`: current column
//!
//! ## Example
//!
//! ```rust
//! use std::{fs, io::{self, BufRead}};
//! use tuirealm::{
//!     application::PollStrategy,
//!     command::{Cmd, CmdResult, Direction, Position},
//!     event::{Event, Key, KeyEvent, KeyModifiers},
//!     props::{Alignment, AttrValue, Attribute, BorderType, Borders, Color, Style, TextModifiers},
//!     terminal::TerminalBridge,
//!     Application, Component, EventListenerCfg, MockComponent, NoUserEvent, State, StateValue,
//!     Update,
//! };
//! use tui_realm_textarea::TextArea;
//!
//! let textarea = match fs::File::open("README.md") {
//!     Ok(reader) => TextArea::new(
//!         io::BufReader::new(reader)
//!             .lines()
//!             .map(|l| l.unwrap())
//!             .collect::<_>(),
//!     ),
//!     Err(_) => TextArea::default(),
//! };
//! let component = textarea
//!     .borders(
//!         Borders::default()
//!             .color(Color::LightYellow)
//!             .modifiers(BorderType::Double),
//!     )
//!     .cursor_line_style(Style::default())
//!     .cursor_style(Style::default().add_modifier(TextModifiers::REVERSED))
//!     .footer_bar("Press <ESC> to quit", Style::default())
//!     .line_number_style(
//!         Style::default()
//!             .fg(Color::LightBlue)
//!             .add_modifier(TextModifiers::ITALIC),
//!     )
//!     .max_histories(64)
//!     .scroll_step(4)
//!     .status_bar(
//!         "README.md Ln {ROW}, Col {COL}",
//!         Style::default().add_modifier(TextModifiers::REVERSED),
//!     )
//!     .tab_length(4)
//!     .title("Editing README.md", Alignment::Left);
//! ```
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]

// -- internal
mod fmt;
use fmt::LineFmt;

// deps

#[macro_use]
extern crate lazy_regex;

#[cfg(feature = "clipboard")]
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use tui_textarea::{CursorMove, TextArea as TextAreaWidget};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, PropPayload, PropValue, Props, Style, TextModifiers,
};
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout, Rect};
use tuirealm::tui::widgets::{Block, Paragraph};
use tuirealm::{Frame, MockComponent, State, StateValue};

// -- props
pub const TEXTAREA_CURSOR_LINE_STYLE: &str = "cursor-line-style";
pub const TEXTAREA_CURSOR_STYLE: &str = "cursor-style";
pub const TEXTAREA_FOOTER_FMT: &str = "footer-fmt";
pub const TEXTAREA_LINE_NUMBER_STYLE: &str = "line-number-style";
pub const TEXTAREA_MAX_HISTORY: &str = "max-history";
pub const TEXTAREA_STATUS_FMT: &str = "status-fmt";
pub const TEXTAREA_TAB_SIZE: &str = "tab-size";
pub const TEXTAREA_HARD_TAB: &str = "hard-tab";
#[cfg(feature = "search")]
pub const TEXTAREA_SEARCH_PATTERN: &str = "search-pattern";
#[cfg(feature = "search")]
pub const TEXTAREA_SEARCH_STYLE: &str = "search-style";

// -- cmd
pub const TEXTAREA_CMD_NEWLINE: &str = "0";
pub const TEXTAREA_CMD_DEL_LINE_BY_END: &str = "1";
pub const TEXTAREA_CMD_DEL_LINE_BY_HEAD: &str = "2";
pub const TEXTAREA_CMD_DEL_WORD: &str = "3";
pub const TEXTAREA_CMD_DEL_NEXT_WORD: &str = "4";
pub const TEXTAREA_CMD_MOVE_WORD_FORWARD: &str = "5";
pub const TEXTAREA_CMD_MOVE_WORD_BACK: &str = "6";
pub const TEXTAREA_CMD_MOVE_PARAGRAPH_FORWARD: &str = "7";
pub const TEXTAREA_CMD_MOVE_PARAGRAPH_BACK: &str = "8";
pub const TEXTAREA_CMD_MOVE_TOP: &str = "9";
pub const TEXTAREA_CMD_MOVE_BOTTOM: &str = "a";
pub const TEXTAREA_CMD_UNDO: &str = "b";
pub const TEXTAREA_CMD_REDO: &str = "c";
#[cfg(feature = "clipboard")]
pub const TEXTAREA_CMD_PASTE: &str = "d";
#[cfg(feature = "search")]
pub const TEXTAREA_CMD_SEARCH_FORWARD: &str = "e";
#[cfg(feature = "search")]
pub const TEXTAREA_CMD_SEARCH_BACK: &str = "f";

/// textarea tui-realm component
pub struct TextArea<'a> {
    props: Props,
    widget: TextAreaWidget<'a>,
    /// Status fmt
    status_fmt: Option<LineFmt>,
    /// footer fmt
    footer_fmt: Option<LineFmt>,
}

impl<'a, I> From<I> for TextArea<'a>
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    fn from(i: I) -> Self {
        Self::new(i.into_iter().map(|s| s.into()).collect::<Vec<String>>())
    }
}

impl<'a> Default for TextArea<'a> {
    fn default() -> Self {
        Self::new(Vec::default())
    }
}

impl<'a> TextArea<'a> {
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            props: Props::default(),
            widget: TextAreaWidget::new(lines),
            status_fmt: None,
            footer_fmt: None,
        }
    }

    /// Set another style from default to use when component is inactive
    pub fn inactive(mut self, s: Style) -> Self {
        self.attr(Attribute::FocusStyle, AttrValue::Style(s));
        self
    }

    /// Set widget border properties
    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    /// Set widget title
    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }

    /// Set scroll step for scrolling command
    pub fn scroll_step(mut self, step: usize) -> Self {
        self.attr(Attribute::ScrollStep, AttrValue::Length(step));
        self
    }

    /// Set how many modifications are remembered for undo/redo. Setting 0 disables undo/redo.
    pub fn max_histories(mut self, max: usize) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_MAX_HISTORY),
            AttrValue::Payload(PropPayload::One(PropValue::Usize(max))),
        );
        self
    }

    /// Set text editor cursor style
    pub fn cursor_style(mut self, s: Style) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_CURSOR_STYLE),
            AttrValue::Style(s),
        );
        self
    }

    /// Set text editor style for selected line
    pub fn cursor_line_style(mut self, s: Style) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_CURSOR_LINE_STYLE),
            AttrValue::Style(s),
        );
        self
    }

    /// Set footer bar fmt and style for the footer bar
    /// Default: no footer bar is displayed
    pub fn footer_bar(mut self, fmt: &str, style: Style) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_FOOTER_FMT),
            AttrValue::Payload(PropPayload::Tup2((
                PropValue::Str(fmt.to_string()),
                PropValue::Style(style),
            ))),
        );
        self
    }

    /// Set text editor style for line numbers
    pub fn line_number_style(mut self, s: Style) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_LINE_NUMBER_STYLE),
            AttrValue::Style(s),
        );
        self
    }

    /// Set status bar fmt and style for the status bar
    /// Default: no status bar is displayed
    pub fn status_bar(mut self, fmt: &str, style: Style) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_STATUS_FMT),
            AttrValue::Payload(PropPayload::Tup2((
                PropValue::Str(fmt.to_string()),
                PropValue::Style(style),
            ))),
        );
        self
    }

    /// Set text style for editor
    pub fn style(mut self, s: Style) -> Self {
        self.attr(Attribute::Style, AttrValue::Style(s));
        self
    }

    /// Set `<TAB>` size
    pub fn tab_length(mut self, l: u8) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_TAB_SIZE),
            AttrValue::Size(l as u16),
        );
        self
    }

    /// Set another style from default to use when component is inactive
    pub fn hard_tab(mut self, enabled: bool) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_HARD_TAB),
            AttrValue::Flag(enabled),
        );
        self
    }

    #[cfg(feature = "search")]
    /// Set search style
    pub fn search_style(mut self, s: Style) -> Self {
        self.attr(
            Attribute::Custom(TEXTAREA_SEARCH_STYLE),
            AttrValue::Style(s),
        );
        self
    }

    // -- private
    fn get_block(&self) -> Block<'a> {
        let mut block = Block::default();
        if let Some(AttrValue::Title((title, alignment))) = self.query(Attribute::Title) {
            block = block.title(title).title_alignment(alignment);
        }
        if let Some(AttrValue::Borders(borders)) = self.query(Attribute::Borders) {
            let inactive_style = self
                .query(Attribute::FocusStyle)
                .unwrap_or_else(|| AttrValue::Style(Style::default()))
                .unwrap_style();
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            block = block
                .border_style(match focus {
                    true => borders.style(),
                    false => inactive_style,
                })
                .border_type(borders.modifiers)
                .borders(borders.sides);
        }

        block
    }

    #[cfg(feature = "clipboard")]
    fn paste(&mut self) {
        // get content from context
        if let Ok(Ok(yank)) = ClipboardContext::new().map(|mut ctx| ctx.get_contents()) {
            self.widget.set_yank_text(yank);
            self.widget.paste();
            self.widget.set_yank_text(String::default());
        }
    }
}

impl<'a> MockComponent for TextArea<'a> {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // set block
            self.widget.set_block(self.get_block());
            // make chunks
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Min(1),
                        Constraint::Length(if self.status_fmt.is_some() { 1 } else { 0 }),
                        Constraint::Length(if self.footer_fmt.is_some() { 1 } else { 0 }),
                    ]
                    .as_ref(),
                )
                .split(area);
            
            // Remove cursor if not in focus
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            if !focus {
                self.widget.set_cursor_style(Style::reset());
            } else {
                let style = self
                    .props
                    .get_or(
                        Attribute::Custom(TEXTAREA_CURSOR_STYLE),
                        AttrValue::Style(Style::default().add_modifier(TextModifiers::REVERSED)),
                    )
                    .unwrap_style();
                self.widget.set_cursor_style(style);
            }

            // render widget
            frame.render_widget(self.widget.widget(), chunks[0]);
            if let Some(fmt) = self.status_fmt.as_ref() {
                frame.render_widget(
                    Paragraph::new(fmt.fmt(&self.widget)).style(fmt.style()),
                    chunks[1],
                );
            }
            if let Some(fmt) = self.footer_fmt.as_ref() {
                frame.render_widget(
                    Paragraph::new(fmt.fmt(&self.widget)).style(fmt.style()),
                    chunks[2],
                );
            }
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value.clone());
        match (attr, value) {
            (Attribute::Custom(TEXTAREA_CURSOR_STYLE), AttrValue::Style(s)) => {
                self.widget.set_cursor_style(s);
            }
            (Attribute::Custom(TEXTAREA_CURSOR_LINE_STYLE), AttrValue::Style(s)) => {
                self.widget.set_cursor_line_style(s);
            }
            (
                Attribute::Custom(TEXTAREA_FOOTER_FMT),
                AttrValue::Payload(PropPayload::Tup2((
                    PropValue::Str(fmt),
                    PropValue::Style(style),
                ))),
            ) => {
                self.footer_fmt = Some(LineFmt::new(&fmt, style));
            }
            (
                Attribute::Custom(TEXTAREA_MAX_HISTORY),
                AttrValue::Payload(PropPayload::One(PropValue::Usize(max))),
            ) => {
                self.widget.set_max_histories(max);
            }
            (
                Attribute::Custom(TEXTAREA_STATUS_FMT),
                AttrValue::Payload(PropPayload::Tup2((
                    PropValue::Str(fmt),
                    PropValue::Style(style),
                ))),
            ) => {
                self.status_fmt = Some(LineFmt::new(&fmt, style));
            }
            (Attribute::Custom(TEXTAREA_LINE_NUMBER_STYLE), AttrValue::Style(s)) => {
                self.widget.set_line_number_style(s);
            }
            (Attribute::Custom(TEXTAREA_TAB_SIZE), AttrValue::Size(size)) => {
                self.widget.set_tab_length(size as u8);
            }
            (Attribute::Custom(TEXTAREA_HARD_TAB), AttrValue::Flag(enabled)) => {
                self.widget.set_hard_tab_indent(enabled);
            }
            #[cfg(feature = "search")]
            (Attribute::Custom(TEXTAREA_SEARCH_PATTERN), AttrValue::String(pattern)) => {
                let _ = self.widget.set_search_pattern(pattern);
            }
            #[cfg(feature = "search")]
            (Attribute::Custom(TEXTAREA_SEARCH_STYLE), AttrValue::Style(s)) => {
                self.widget.set_search_style(s);
            }
            (Attribute::Style, AttrValue::Style(s)) => {
                self.widget.set_style(s);
            }
            (_, _) => {
                self.widget.set_block(self.get_block());
            }
        }
    }

    fn state(&self) -> State {
        State::Vec(
            self.widget
                .lines()
                .iter()
                .map(|x| StateValue::String(x.to_string()))
                .collect(),
        )
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Cancel => {
                self.widget.delete_next_char();
                CmdResult::None
            }
            Cmd::Custom(TEXTAREA_CMD_DEL_LINE_BY_END) => {
                self.widget.delete_line_by_end();
                CmdResult::None
            }
            Cmd::Custom(TEXTAREA_CMD_DEL_LINE_BY_HEAD) => {
                self.widget.delete_line_by_head();
                CmdResult::None
            }
            Cmd::Custom(TEXTAREA_CMD_DEL_NEXT_WORD) => {
                self.widget.delete_next_word();
                CmdResult::None
            }
            Cmd::Custom(TEXTAREA_CMD_DEL_WORD) => {
                self.widget.delete_word();
                CmdResult::None
            }
            Cmd::Custom(TEXTAREA_CMD_MOVE_PARAGRAPH_BACK) => {
                self.widget.move_cursor(CursorMove::ParagraphBack);
                CmdResult::None
            }
            Cmd::Custom(TEXTAREA_CMD_MOVE_PARAGRAPH_FORWARD) => {
                self.widget.move_cursor(CursorMove::ParagraphForward);
                CmdResult::None
            }
            Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_BACK) => {
                self.widget.move_cursor(CursorMove::WordBack);
                CmdResult::None
            }
            Cmd::Custom(TEXTAREA_CMD_MOVE_WORD_FORWARD) => {
                self.widget.move_cursor(CursorMove::WordForward);
                CmdResult::None
            }
            Cmd::Custom(TEXTAREA_CMD_MOVE_BOTTOM) => {
                self.widget.move_cursor(CursorMove::Bottom);
                CmdResult::None
            }
            Cmd::Custom(TEXTAREA_CMD_MOVE_TOP) => {
                self.widget.move_cursor(CursorMove::Top);
                CmdResult::None
            }
            #[cfg(feature = "clipboard")]
            Cmd::Custom(TEXTAREA_CMD_PASTE) => {
                self.paste();
                CmdResult::None
            }
            Cmd::Custom(TEXTAREA_CMD_REDO) => {
                self.widget.redo();
                CmdResult::None
            }
            #[cfg(feature = "search")]
            Cmd::Custom(TEXTAREA_CMD_SEARCH_BACK) => {
                self.widget.search_back(true);
                CmdResult::None
            }
            #[cfg(feature = "search")]
            Cmd::Custom(TEXTAREA_CMD_SEARCH_FORWARD) => {
                self.widget.search_forward(true);
                CmdResult::None
            }
            Cmd::Custom(TEXTAREA_CMD_UNDO) => {
                self.widget.undo();
                CmdResult::None
            }
            Cmd::Delete => {
                self.widget.delete_char();
                CmdResult::None
            }
            Cmd::GoTo(Position::Begin) => {
                self.widget.move_cursor(CursorMove::Head);
                CmdResult::None
            }
            Cmd::GoTo(Position::End) => {
                self.widget.move_cursor(CursorMove::End);
                CmdResult::None
            }
            Cmd::Move(Direction::Down) => {
                self.widget.move_cursor(CursorMove::Down);
                CmdResult::None
            }
            Cmd::Move(Direction::Left) => {
                self.widget.move_cursor(CursorMove::Back);
                CmdResult::None
            }
            Cmd::Move(Direction::Right) => {
                self.widget.move_cursor(CursorMove::Forward);
                CmdResult::None
            }
            Cmd::Move(Direction::Up) => {
                self.widget.move_cursor(CursorMove::Up);
                CmdResult::None
            }
            Cmd::Scroll(Direction::Down) => {
                let step = self
                    .props
                    .get_or(Attribute::ScrollStep, AttrValue::Length(8))
                    .unwrap_length();
                (0..step).for_each(|_| self.widget.move_cursor(CursorMove::Down));
                CmdResult::None
            }
            Cmd::Scroll(Direction::Up) => {
                let step = self
                    .props
                    .get_or(Attribute::ScrollStep, AttrValue::Length(8))
                    .unwrap_length();
                (0..step).for_each(|_| self.widget.move_cursor(CursorMove::Up));
                CmdResult::None
            }
            Cmd::Type('\t') => {
                self.widget.insert_tab();
                CmdResult::None
            }
            Cmd::Type('\n') | Cmd::Custom(TEXTAREA_CMD_NEWLINE) => {
                self.widget.insert_newline();
                CmdResult::None
            }
            Cmd::Type(ch) => {
                self.widget.insert_char(ch);
                CmdResult::None
            }
            Cmd::Submit => CmdResult::Submit(self.state()),
            _ => CmdResult::None,
        }
    }
}
