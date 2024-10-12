//! # tuirealm_derive
//!
//! [tuirealm_derive](https://github.com/veeso/tuirealm_derive) provides the derive macro
//! which automatically implements `MockComponent` for a [tui-realm](https://github.com/veeso/tui-realm) component.
//!
//! tuirealm_derive is a crate which implements the procedural macro `MockComponent` which can be used to automatically implement
//! the `MockComponent` trait for a tui-realm `Component`.
//! Indeed, as you already know if you're a tui-realm user, you've got two kind of component entities:
//!
//! - MockComponent: generic graphic component which is not bridged to the application and is "reusable"
//! - Component: which uses a MockComponent as "backend" and is bridged to the application using the **Event -> Msg** system.
//!
//! The Component wraps the MockComponent along with additional states.
//! Since `Component` **MUST** implement `MockComponent`, we need to implement the mock component trait too,
//! which in most of the cases it will just call the MockComponet methods on the inner `component` field.
//! This is obviously kinda annoying to do for each component.
//! That's why I implemented this procedural macro, which will automatically implement this logic on your component.
//!
//! So basically instead of implementing `MockComponent` for your components, you can just do as follows:
//!
//! ```rust
//! #[derive(MockComponent)]
//! pub struct IpAddressInput {
//!   component: Input,
//! }
//! ```
//!
//! With the directive `#[derive(MockComponent)]` we **don't have to** implement the mock component trait.
//!
//! > ❗ In order to work, the procedural macro requires you to name the "inner" mock component as `component` as I did in the example.
//!
//! If we give a deeper look at the macro, we'll see that what it does is:
//!
//! ```rust
//! impl MockComponent for IpAddressInput {
//!     fn view(&mut self, frame: &mut Frame, area: Rect) {
//!         self.component.view(frame, area);
//!     }
//!
//!     fn query(&self, attr: Attribute) -> Option<AttrValue> {
//!         self.component.query(attr)
//!     }
//!
//!     fn attr(&mut self, query: Attribute, attr: AttrValue) {
//!         self.component.attr(query, attr)
//!     }
//!
//!     fn state(&self) -> State {
//!         self.component.state()
//!     }
//!
//!     fn perform(&mut self, cmd: Cmd) -> CmdResult {
//!         self.component.perform(cmd)
//!     }
//! }
//! ```
//!
//! ## Get Started
//!
//! In order to get started with **tuirealm_derive** all you need to do is to add [tui-realm](https://github.com/veeso/tui-realm) to your dependencies and enable the `derive` feature if needed.
//!
//! If you're using the default features:
//!
//! ```toml
//! [dependencies]
//! tuirealm = "^2.0.0"
//! ```
//!
//! If you're not using the default features, be sure to enable the **derive** feature:
//!
//! ```toml
//! [dependencies]
//! tuirealm = { version = "^2.0.0", default-features = false, features = ["derive", "crossterm"] }
//! ```
//!
//! Then you need to include tuirealm in your project using the `macro use` directive:
//!
//! > src/lib.rs
//!
//! ```rust
//! #[macro_use]
//! extern crate tuirealm;
//! ```
//!
//! and finally derive `MockComponent` on your components:
//!
//! ```rust
//! #[derive(MockComponent)]
//! pub struct MyComponent {
//!   component: MyMockComponentImpl,
//! }
//! ```
//!
//! > ❗ In order to work, the procedural macro requires you to name the "inner" mock component as `component` as I did in the example.
//!
//! And ta-dah, you're ready to go 🎉
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://rawcdn.githack.com/veeso/tui-realm/39c38c3bd905f724403481514adb2cf2b4e69a7b/docs/images/cargo/tui-realm-128.png"
)]
#![doc(
    html_logo_url = "https://rawcdn.githack.com/veeso/tui-realm/39c38c3bd905f724403481514adb2cf2b4e69a7b/docs/images/cargo/tui-realm-512.png"
)]

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, FieldsNamed};

#[proc_macro_derive(MockComponent)]
pub fn mock_component(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    if let syn::Data::Struct(s) = data {
        // Check if "component" exists
        match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                if !named
                    .iter()
                    .any(|x| x.ident.as_ref().unwrap() == "component")
                {
                    panic!("`component` not found for struct '{}'", ident);
                }
            }
            _ => panic!("struct {} does not contain named fields", ident),
        }
        // Implement MockComponent for type
        let output = quote! {
            const _: () = {
                use ::tuirealm::command::{Cmd, CmdResult};
                use ::tuirealm::ratatui::layout::Rect;
                use ::tuirealm::{Attribute, AttrValue, Frame, MockComponent, State};
                impl MockComponent for #ident {
                    fn view(&mut self, frame: &mut Frame, area: Rect) {
                        self.component.view(frame, area);
                    }

                    fn query(&self, attr: Attribute) -> Option<AttrValue> {
                        self.component.query(attr)
                    }

                    fn attr(&mut self, query: Attribute, attr: AttrValue) {
                        self.component.attr(query, attr)
                    }

                    fn state(&self) -> State {
                        self.component.state()
                    }

                    fn perform(&mut self, cmd: Cmd) -> CmdResult {
                        self.component.perform(cmd)
                    }
                }
            };
        };

        output.into()
    } else {
        panic!("MockComponent must be derived by a `Struct`")
    }
}
