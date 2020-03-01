use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn action(input: ItemFn) -> TokenStream {
    let ItemFn {
        attrs, sig, block, ..
    } = input;
    let name = sig.ident;

    if cfg!(feature = "router-actix") {
        quote! {}
    } else if cfg!(feature = "router-gotham") {
        quote! {
            #(#attrs)*
            pub fn #name(
                #[allow(unused_mut)] mut state: ::gotham::state::State,
            ) -> std::pin::Pin<Box<::gotham::handler::HandlerFuture>> {
                use ::futures::prelude::*;
                async move #block.boxed()
            }
        }
    } else if cfg!(feature = "router-tide") {
        quote! {}
    } else {
        quote! {}
    }
}
