//! ## Direction
//!
//! Describes a generic direction

/// ## Direction
///
/// Defines the 4 directions
#[derive(Debug, Eq, PartialEq, Copy, Clone, PartialOrd, Hash)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}
