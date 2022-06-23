//! ## Direction
//!
//! Describes a generic direction

/// Defines the 4 directions
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}
