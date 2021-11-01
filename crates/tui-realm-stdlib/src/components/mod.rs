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
// Modules
mod bar_chart;
mod canvas;
mod chart;
mod checkbox;
mod container;
mod input;
mod label;
mod line_gauge;
mod list;
mod paragraph;
mod phantom;
mod progress_bar;
mod radio;
mod select;
mod span;
mod sparkline;
mod spinner;
mod table;
mod textarea;

pub mod props;

// Exports
pub use bar_chart::BarChart;
pub use canvas::Canvas;
pub use chart::Chart;
pub use checkbox::Checkbox;
pub use container::Container;
pub use input::Input;
pub use label::Label;
pub use line_gauge::LineGauge;
pub use list::List;
pub use paragraph::Paragraph;
pub use phantom::Phantom;
pub use progress_bar::ProgressBar;
pub use radio::Radio;
pub use select::Select;
pub use span::Span;
pub use sparkline::Sparkline;
pub use spinner::Spinner;
pub use table::Table;
pub use textarea::Textarea;
