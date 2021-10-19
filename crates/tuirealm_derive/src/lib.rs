//! # tuirealm_derive
//!
//! [tuirealm_derive](https://github.com/veeso/tuirealm_derive) provides the derive macro
//! to automatically implement `MockComponent` for a [tui-realm](https://github.com/veeso/tui-realm) component.
//!
//! ## Get Started
//!
//! TODO:
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://rawcdn.githack.com/veeso/tui-realm/39c38c3bd905f724403481514adb2cf2b4e69a7b/docs/images/cargo/tui-realm-128.png"
)]
#![doc(
    html_logo_url = "https://rawcdn.githack.com/veeso/tui-realm/39c38c3bd905f724403481514adb2cf2b4e69a7b/docs/images/cargo/tui-realm-512.png"
)]

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
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
                if named
                    .iter()
                    .find(|x| x.ident.as_ref().unwrap() == "component")
                    .is_none()
                {
                    panic!("`component` not found for struct '{}'", ident);
                }
            }
            _ => panic!("struct {} does not contain named fields", ident),
        }
        // Implement MockComponent for type
        let output = quote! {
            impl tuirealm::MockComponent for #ident {
                fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
                    self.component.view(frame, area);
                }

                fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
                    self.component.query(attr)
                }

                fn attr(&mut self, query: tuirealm::Attribute, attr: tuirealm::AttrValue) {
                    self.component.attr(query, attr)
                }

                fn state(&self) -> tuirealm::State {
                    self.component.state()
                }

                fn perform(&mut self, cmd: tuirealm::Cmd) -> tuirealm::CmdResult {
                    self.component.perform(cmd)
                }
            }
        };

        output.into()
    } else {
        panic!("MockComponent must be derived by a `Struct`")
    }
}
