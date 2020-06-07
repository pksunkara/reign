use inflector::cases::{screamingsnakecase::to_screaming_snake_case, snakecase::to_snake_case};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

pub fn config(input: DeriveInput) -> TokenStream {
    let DeriveInput { ident, .. } = input;

    let scream = Ident::new(&to_screaming_snake_case(&ident.to_string()), ident.span());
    let snake = Ident::new(&to_snake_case(&ident.to_string()), ident.span());

    quote! {
        pub static #scream: ::reign::once_cell::sync::OnceCell<#ident> = ::reign::once_cell::sync::OnceCell::new();

        pub fn #snake() -> &'static #ident {
            #scream.get().unwrap()
        }
    }
}
