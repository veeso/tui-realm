//! Compatibility layer for collections that works in both std and no_std environments

#[cfg(feature = "std")]
pub use std::collections::{HashMap, LinkedList};

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::collections::LinkedList;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate hashbrown;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use hashbrown::HashMap;
