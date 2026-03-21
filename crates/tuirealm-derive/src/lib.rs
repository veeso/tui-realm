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
//! # use tuirealm_derive::MockComponent;
//! # use tui_realm_stdlib::Input;
//! #
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
//! # use tuirealm::command::{Cmd, CmdResult};
//! # use tuirealm::ratatui::layout::Rect;
//! # use tuirealm::{Attribute, AttrValue, Frame, MockComponent, State};
//! # use tui_realm_stdlib::Input;
//! #
//! # pub struct IpAddressInput {
//! #   component: Input,
//! # }
//! #
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
//! ```rust ignore
//! #[macro_use]
//! extern crate tuirealm;
//! ```
//!
//! and finally derive `MockComponent` on your components:
//!
//! ```rust
//! # use tuirealm_derive::MockComponent;
//! # use tui_realm_stdlib::Radio;
//!
//! #[derive(MockComponent)]
//! pub struct MyComponent {
//!   component: Radio,
//! }
//! ```
//!
//! > ❗ In order to work, the procedural macro requires you to name the "inner" mock component as `component` as I did in the example.
//!
//! And ta-dah, you're ready to go 🎉
//!
//! ### Custom field names
//!
//! By default a field of name `component` will be used as can be seen in the earlier examples, but this can be customized.
//!
//! First option is to use a container-level attribute:
//!
//! ```rust
//! # use tuirealm_derive::MockComponent;
//! # use tui_realm_stdlib::Radio;
//! #
//! #[derive(MockComponent)]
//! #[component("radio")]
//! pub struct MyComponent1 {
//!   radio: Radio,
//! }
//!
//! #[derive(MockComponent)]
//! #[component = "radio"]
//! pub struct MyComponent2 {
//!   radio: Radio,
//! }
//! ```
//!
//! Or field-level attribute:
//!
//! ```rust
//! # use tuirealm_derive::MockComponent;
//! # use tui_realm_stdlib::Radio;
//! #
//! #[derive(MockComponent)]
//! pub struct MyComponent {
//!   #[component]
//!   radio: Radio,
//! }
//! ```
//!
//! > ❗ Only one field can be the component and container- & field-level attributes cannot be used together.
//!
//! Tuple Structs are also supported:
//!
//! ```rust
//! # use tuirealm_derive::MockComponent;
//! # use tui_realm_stdlib::Radio;
//! #
//! # pub struct SomeOtherType;
//! #
//! #[derive(MockComponent)]
//! pub struct MyComponent1(Radio, SomeOtherType);
//!
//! #[derive(MockComponent)]
//! pub struct MyComponent2(SomeOtherType, #[component] Radio);
//! ```
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://rawcdn.githack.com/veeso/tui-realm/39c38c3bd905f724403481514adb2cf2b4e69a7b/docs/images/cargo/tui-realm-128.png"
)]
#![doc(
    html_logo_url = "https://rawcdn.githack.com/veeso/tui-realm/39c38c3bd905f724403481514adb2cf2b4e69a7b/docs/images/cargo/tui-realm-512.png"
)]

use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Attribute, DeriveInput, Field, FieldsNamed, FieldsUnnamed, Ident, Meta, parse_macro_input,
};

/// Try to find if in the given attributes there is a `#[component]` with ident to the field
fn get_container_attr_value(attrs: &[Attribute]) -> Option<Ident> {
    let mut component_name = None;

    for attr in attrs {
        // get value from attribute
        if attr.path().is_ident("component") {
            // The follow "if" is the best way i had found to parse "#[component = \"field\"]"
            if let Meta::NameValue(val) = &attr.meta {
                if let Ok(val) = syn::parse2::<syn::LitStr>(val.value.clone().into_token_stream()) {
                    component_name = Some(syn::Ident::new(&val.value(), Span::call_site()));
                    break;
                }
            }

            // The following handles "#[component(\"field\)]"
            let name: syn::LitStr = attr.parse_args()
                .unwrap_or_else(|_| {
                    panic!("`component` attribute must be a string literal, e.g. `#[component(\"my_component\")]`!");
                });

            component_name = Some(syn::Ident::new(&name.value(), Span::call_site()));
            break;
        }
    }

    component_name
}

/// Check that the ident `field_ident` actually exists in the given fields.
///
/// Mostly to check [`get_container_attr_value`].
fn check_fields_for_path<'a>(field_ident: &Ident, mut fields: impl Iterator<Item = &'a Field>) {
    if !fields.any(|x| x.ident.as_ref().unwrap() == field_ident) {
        panic!("`{field_ident}` not found on struct!",);
    }
}

/// Find a field with the `#[component]` attribute. Also checks for duplicate usage.
fn find_field_with_attr<'a>(fields: impl Iterator<Item = &'a Field>) -> Option<Ident> {
    let mut found_ident = None;

    for field in fields {
        if let Some(_attr) = field.attrs.iter().find(|v| v.path().is_ident("component")) {
            let attr_ident = field.ident.as_ref().unwrap().clone();
            if let Some(found_ident) = found_ident {
                panic!(
                    "Found attribute `#[component]` more than once! (first: `{found_ident}`, second: `{attr_ident}`)"
                )
            }

            found_ident = Some(attr_ident);
        }
    }

    found_ident
}

/// Find a field with the `#[component]` attribute. Also checks for duplicate usage.
///
/// Similar to [`find_field_with_attr`], only that instead of the ident of the field, the index of the field is returned.
fn find_field_with_attr_unnamed<'a>(fields: impl Iterator<Item = &'a Field>) -> Option<usize> {
    let mut found_ident = None;

    for (idx, field) in fields.enumerate() {
        if let Some(_attr) = field.attrs.iter().find(|v| v.path().is_ident("component")) {
            if let Some(found_idx) = found_ident {
                panic!(
                    "Found attribute `#[component]` more than once! (first: `{found_idx}`, second: `{idx}`)"
                )
            }

            found_ident = Some(idx);
        }
    }

    found_ident
}

/// Find default field `component` in the given `fields`.
fn find_default_field<'a>(fields: impl Iterator<Item = &'a Field>) -> Option<Ident> {
    for field in fields {
        if field.ident.as_ref().is_some_and(|v| v == "component") {
            return Some(field.ident.as_ref().unwrap().clone());
        }
    }

    None
}

#[proc_macro_derive(MockComponent, attributes(component))]
pub fn mock_component(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        attrs,
        ..
    } = parse_macro_input!(input);

    if let syn::Data::Struct(s) = data {
        // Check if `component`` exists
        let component_field = match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                // get field name from attributes
                let component_field = get_container_attr_value(&attrs);

                if let Some(component_field) = component_field {
                    check_fields_for_path(&component_field, named.iter());
                    if find_field_with_attr(named.iter()).is_some() {
                        panic!("Cannot mix container level and field level `#[component]` usage!");
                    }
                    quote! { #component_field }
                } else {
                    let component_field = find_field_with_attr(named.iter())
                        .or_else(|| find_default_field(named.iter()))
                        .expect("Expected struct to have field \"component\" or a field with attribute \"#[component]\"");

                    quote! { #component_field }
                }
            }
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                if unnamed.is_empty() {
                    panic!("Expected at least one unnamed field!");
                }

                let field_idx = find_field_with_attr_unnamed(unnamed.iter()).unwrap_or(0);
                let field_idx = syn::Index::from(field_idx);

                quote! { #field_idx }
            }
            _ => panic!("Only Named And Tuple structs are supported"),
        };

        // Implement MockComponent for type
        let output = quote! {
            const _: () = {
                use ::tuirealm::command::{Cmd, CmdResult};
                use ::tuirealm::ratatui::layout::Rect;
                use ::tuirealm::{Attribute, AttrValue, Frame, MockComponent, State};
                #[automatically_derived]
                impl #generics MockComponent for #ident #generics {
                    fn view(&mut self, frame: &mut Frame, area: Rect) {
                        self.#component_field.view(frame, area);
                    }

                    fn query(&self, attr: Attribute) -> Option<AttrValue> {
                        self.#component_field.query(attr)
                    }

                    fn attr(&mut self, query: Attribute, attr: AttrValue) {
                        self.#component_field.attr(query, attr)
                    }

                    fn state(&self) -> State {
                        self.#component_field.state()
                    }

                    fn perform(&mut self, cmd: Cmd) -> CmdResult {
                        self.#component_field.perform(cmd)
                    }
                }
            };
        };

        output.into()
    } else {
        panic!("MockComponent must be derived by a `Struct`")
    }
}
