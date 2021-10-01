//! ## Title
//!
//! `title` is the module which defines properties for block title

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
use super::Alignment;

/// ## BlockTitle
///
/// Title properties for block containing the component
#[derive(Clone)]
pub struct BlockTitle {
    text: String,
    alignment: Alignment,
}

impl BlockTitle {
    pub fn new<S: AsRef<str>>(text: S, alignment: Alignment) -> Self {
        Self {
            text: text.as_ref().to_string(),
            alignment,
        }
    }

    /// ### text
    ///
    /// get text alignment
    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    /// ### alignment
    ///
    /// get block alignment
    pub fn alignment(&self) -> Alignment {
        self.alignment
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_props_title() {
        let title: BlockTitle = BlockTitle::new("Omar", Alignment::Center);
        assert_eq!(title.text(), "Omar");
        assert_eq!(title.alignment, Alignment::Center); // not dropped
        assert_eq!(title.alignment, Alignment::Center); // not dropped
    }
}
