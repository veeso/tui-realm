//! ## Utils
//!
//! `Utils` provides structures useful to implement gui with tui-rs

#![allow(unused)] // clippy / rust do not see shared files in examples as being used

mod data_gen;
mod loader;

pub use data_gen::DataGen;
pub use loader::Loader;
