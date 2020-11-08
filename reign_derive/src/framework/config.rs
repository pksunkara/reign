use inflector::cases::screamingsnakecase::to_screaming_snake_case;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

pub fn config(input: DeriveInput) -> TokenStream {
    let DeriveInput { ident, .. } = input;
    let scream = Ident::new(&to_screaming_snake_case(&ident.to_string()), ident.span());

    // TODO: (external) Use dummy mod once https://github.com/rust-analyzer/rust-analyzer/issues/1559
    quote! {
        static #scream: ::reign::once_cell::sync::OnceCell<#ident> = ::reign::once_cell::sync::OnceCell::new();

        impl ::reign::Config for #ident {
            fn get() -> &'static Self {
                #scream.get().expect("Config must be loaded before using it")
            }

            fn cell() -> &'static ::reign::once_cell::sync::OnceCell<Self> {
                &#scream
            }
        }
    }
}
