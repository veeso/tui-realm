//! ## Core
//!
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
pub use command::Cmd;
pub use component::{Component, MockComponent};
pub use state::{State, StateValue};
pub use view::{View, ViewError};

// -- internal
pub(crate) use subscription::Subscription;
pub(crate) use view::WrappedComponent;

// -- Update

/// The update trait defines the prototype of the function to be used to handle the events coming from the View.
pub trait Update<Msg>
where
    Msg: PartialEq,
{
    /// update the current state handling a message from the view.
    /// This function may return a Message,
    /// so this function has to be intended to be call recursively if necessary
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg>;
}
