//! ## Builder
//!
//! `Builder` is the module which defines the prop builder trait.

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
use super::{AttrSelector, Attribute, Props};

// -- Props builder

/// ## PropsBuilder
///
/// The PropsBuilder trait just defines the method build, which all the builders must implement.
/// This method must return the Props hold by the ProspBuilder.
/// If you're looking on how to implement a Props Builder, check out the project examples
/// and the stdlib repository at <https://github.com/veeso/tui-realm-stdlib>
pub trait PropsBuilder {
    /// ### build
    ///
    /// Build Props from builder
    /// You shouldn't allow this method to be called twice.
    /// Panic is ok.
    fn build(&mut self) -> Props;

    /// ### hidden
    ///
    /// Initialize props with visible set to False
    fn hidden(&mut self) -> &mut Self {
        self.props_mut()
            .set(AttrSelector::Display, Attribute::Flag(false));
        self
    }

    /// ### visible
    ///
    /// Initialize props with visible set to True
    fn visible(&mut self) -> &mut Self {
        self.props_mut()
            .set(AttrSelector::Display, Attribute::Flag(true));
        self
    }

    fn props_mut(&mut self) -> &mut Props;
}
