//! Compatibility layer for collections that works in both std and no_std environments

#[cfg(feature = "std")]
pub use std::collections::{HashMap, LinkedList};

#[cfg(not(feature = "std"))]
pub use alloc::collections::LinkedList;

#[cfg(not(feature = "std"))]
extern crate hashbrown;

#[cfg(not(feature = "std"))]
pub use hashbrown::HashMap;
