extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, Ident, Item};

mod layouts;
mod views;

#[proc_macro_attribute]
pub fn layouts(_: TokenStream, input: TokenStream) -> TokenStream {
    let item: Item = syn::parse_macro_input!(input);

    layouts::layouts_attribute(item).into()
}

#[proc_macro_derive(Layout)]
pub fn derive_layout(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(input);

    layouts::layout_derive(input).into()
}

#[proc_macro_attribute]
pub fn views(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ident: Ident = syn::parse_macro_input!(attr);
    let item: Item = syn::parse_macro_input!(input);

    views::views_attribute(&ident.to_string(), item).into()
}
