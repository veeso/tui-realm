//! # fmt
//!
//! Module which provides the Editor fmt, which is used to format the status lines of the textarea

use super::TextAreaWidget;

use lazy_regex::{Lazy, Regex};
use tuirealm::props::Style;

/// FmtCallback: LineFmt, widget, wrkstr, prepend
type FmtCallback = fn(&LineFmt, &TextAreaWidget, &str, &str) -> String;

// Keys
const FMT_KEY_ROW: &str = "ROW";
const FMT_KEY_COLUMN: &str = "COL";

/**
 * Regex matches:
 *  - group 1: KEY NAME
 */
static FMT_KEY_REGEX: Lazy<Regex> = lazy_regex!(r"\{(.*?)\}");

pub struct LineFmt {
    call_chain: CallChainBlock,
    style: Style,
}

impl LineFmt {
    /// Instantiates a new `LineFmt`
    pub fn new(fmt_str: &str, style: Style) -> Self {
        LineFmt {
            call_chain: Self::make_callchain(fmt_str),
            style,
        }
    }

    /// Format fsentry
    pub fn fmt(&self, widget: &TextAreaWidget) -> String {
        // Execute callchain blocks
        self.call_chain.fmt(self, widget, "")
    }

    /// get style
    pub fn style(&self) -> Style {
        self.style
    }

    fn fmt_col(&self, widget: &TextAreaWidget, wrkstr: &str, prepend: &str) -> String {
        format!("{}{}{}", wrkstr, prepend, widget.cursor().1 + 1)
    }

    fn fmt_row(&self, widget: &TextAreaWidget, wrkstr: &str, prepend: &str) -> String {
        format!("{}{}{}", wrkstr, prepend, widget.cursor().0 + 1)
    }

    fn fmt_none(&self, _: &TextAreaWidget, wrkstr: &str, prepend: &str) -> String {
        format!("{}{}", wrkstr, prepend)
    }

    /// Make a callchain starting from the fmt str
    fn make_callchain(fmt_str: &str) -> CallChainBlock {
        // Init chain block
        let mut callchain: Option<CallChainBlock> = None;
        // Track index of the last match found, to get the prefix for each token
        let mut last_index: usize = 0;
        // Match fmt str against regex
        for regex_match in FMT_KEY_REGEX.captures_iter(fmt_str) {
            // Get match index (unwrap is safe, since always exists)
            let index: usize = fmt_str.find(&regex_match[0]).unwrap();
            // Get prefix
            let prepend: String = String::from(&fmt_str[last_index..index]);
            // Increment last index (sum prefix lenght and the length of the key)
            last_index += prepend.len() + regex_match[0].len();
            // Match attributes
            let callback = match regex_match.get(1).map(|x| x.as_str()) {
                Some(FMT_KEY_COLUMN) => Self::fmt_col,
                Some(FMT_KEY_ROW) => Self::fmt_row,
                Some(_) | None => Self::fmt_none,
            };
            // Create a callchain or push new element to its back
            match callchain.as_mut() {
                None => callchain = Some(CallChainBlock::new(callback, prepend)),
                Some(chain_block) => chain_block.push(callback, prepend),
            }
        }
        // Push remaining str
        if last_index + 1 != fmt_str.len() {
            let prepend = String::from(&fmt_str[last_index..]);
            match callchain.as_mut() {
                None => callchain = Some(CallChainBlock::new(Self::fmt_none, prepend)),
                Some(chain_block) => chain_block.push(Self::fmt_none, prepend),
            }
        }
        // Finalize and return
        callchain.unwrap_or_else(|| CallChainBlock::new(Self::fmt_none, String::new()))
    }
}

/// Call Chain block is a block in a chain of functions which are called in order to format the File.
/// A callChain is instantiated starting from the Formatter syntax and the regex, once the groups are found
/// a chain of function is made using the Formatters method.
/// This method provides an extremely fast way to format fs entries
struct CallChainBlock {
    /// The function to call to format current item
    func: FmtCallback,
    /// All the content which is between two `{KEY}` items
    prepend: String,
    /// The next block to format
    next_block: Option<Box<CallChainBlock>>,
}

impl CallChainBlock {
    /// Create a new `CallChainBlock`
    pub fn new(func: FmtCallback, prepend: String) -> Self {
        CallChainBlock {
            func,
            prepend,
            next_block: None,
        }
    }

    /// Call next callback in the CallChain
    pub fn fmt(&self, fmt: &LineFmt, widget: &TextAreaWidget, wrkstr: &str) -> String {
        // Call func
        let new_str: String = (self.func)(fmt, widget, wrkstr, self.prepend.as_str());
        // If next is some, call next fmt, otherwise (END OF CHAIN) return new_str
        match &self.next_block {
            Some(block) => block.fmt(fmt, widget, new_str.as_str()),
            None => new_str,
        }
    }

    /// Push func to the last element in the Call chain
    pub fn push(&mut self, func: FmtCallback, prepend: String) {
        // Call recursively until an element with next_block equal to None is found
        match &mut self.next_block {
            None => self.next_block = Some(Box::new(CallChainBlock::new(func, prepend))),
            Some(block) => block.push(func, prepend),
        }
    }
}
