//! This module exposes the shape attribute type
use alloc::string::String;
use alloc::vec::Vec;

use super::Color;
use crate::ratatui::widgets::canvas::{Line, Map, Rectangle};

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
