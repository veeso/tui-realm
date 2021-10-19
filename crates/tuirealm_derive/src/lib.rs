//! # tui-realm-derive
//!
//! [tui-realm](https://github.com/veeso/tui-realm-derive) provides the derive macro
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
use syn::{parse_macro_input, DeriveInput, MetaItem};

#[proc_macro_derive(MockComponent, attributes(component))]
pub fn mock_component(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);

    if let syn::Data::Struct(s) = data {
        // Let's get the name of the field with type `MockComponent`
        let field = attrs
            .iter()
            .filter(|attr| attr.path.is_ident("component"))
            .map(|attr| {
                attr.parse_meta()
                    .expect("Could not parse meta for component")
            })
            .map(|attr| {
                if let syn::Meta::List(syn::MetaList {
                    path: _,
                    paren_token: _,
                    nested,
                }) = attr
                {
                    panic!("Found our attribute with contents: {:?}", nested);
                }
                "ciccio"
            });

        let output = quote! {
            impl #ident {
                fn describe() {
                    println!("test", stringify!(#ident));
                }
            }
        };

        //let output = quote! {
        //    impl MockComponent for #field {
        //        fn describe() {
        //            println!("{} is {}.", stringify!(#ident), #description);
        //        }
        //    }
        //};

        output.into()
    } else {
        panic!("MockComponent must be derived by a `Struct`")
    }
}

#[cfg(test)]
mod tests {

    use crate::MockComponent;

    #[derive(MockComponent)]
    pub struct Dummy {
        #[component]
        pub foo: usize,
    }

    #[test]
    fn should_impl_mock() {
        let d = Dummy { foo: 5 };
        d.describe();
    }
}
