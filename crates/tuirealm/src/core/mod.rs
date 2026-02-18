//! Core implements the core functionalities and types for tui-realm

pub mod application;
pub mod command;
mod component;
pub mod event;
pub mod injector;
pub mod props;
mod state;
pub mod subscription;
mod view;

// -- export
pub use component::{Component, MockComponent};
pub use state::{State, StateValue};
// -- internal
pub(crate) use subscription::Subscription;
pub(crate) use view::WrappedComponent;
pub use view::{View, ViewError};
