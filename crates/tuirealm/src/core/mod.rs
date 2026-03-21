//! Core implements the core functionalities and types for tui-realm

pub mod application;
pub mod command;
pub mod component;
pub mod event;
pub mod injector;
pub mod props;
pub mod state;
pub mod subscription;
pub mod view;

// -- internal
pub(crate) use subscription::Subscription;
pub(crate) use view::WrappedComponent;
