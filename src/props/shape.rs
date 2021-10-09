//! ## Shape
//!
//! This module exposes the shape attribute type

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
use super::Color;
use tui::widgets::canvas::{Line, Map, Rectangle};

/// ## Shape
///
/// Describes the shape to draw on the canvas
#[derive(Clone, Debug)]
pub enum Shape {
    Label((f64, f64, String, Color)),
    Layer,
    Line(Line),
    Map(Map),
    Points((Vec<(f64, f64)>, Color)),
    Rectangle(Rectangle),
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Shape::Label(a), Shape::Label(b)) => a == b,
            (Shape::Layer, Shape::Layer) => true,
            (Shape::Line(a), Shape::Line(b)) => {
                a.x1 == b.x1 && a.x2 == b.x2 && a.y1 == b.y1 && a.y2 == b.y2 && a.color == b.color
            }
            (Shape::Map(a), Shape::Map(b)) => a.color == b.color,
            (Shape::Points(a), Shape::Points(b)) => a == b,
            (Shape::Rectangle(a), Shape::Rectangle(b)) => {
                a.x == b.x
                    && a.y == b.y
                    && a.width == b.width
                    && a.height == b.height
                    && a.color == b.color
            }
            (_, _) => false,
        }
    }
}
