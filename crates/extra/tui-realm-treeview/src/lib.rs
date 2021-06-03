//! # tui-realm-treeview
//!
//! [tui-realm-treeview](https://github.com/veeso/tui-realm-treeview) is a [tui-realm](https://github.com/veeso/tui-realm) implementation
//! of a treeview component
//!
//! ## Get Started
//!
//! ### Adding `tui-realm-treeview` as dependency
//!
//! ```toml
//! tui-realm-treeview = "0.1.0"
//! ```
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]

/**
 * MIT License
 *
 * tui-realm-treeview - Copyright (C) 2021 Christian Visintin
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
extern crate tui_tree_widget;
extern crate tuirealm;

use tuirealm::{Component, PropPayload, PropValue, Props};

// -- states

// -- props

// -- component

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
