use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

pub fn redirect(input: Expr) -> TokenStream {
    if cfg!(feature = "views-actix") {
        quote! {
            ::reign::view::redirect_actix(#input)
        }
    } else if cfg!(feature = "views-gotham") {
        quote! {
            ::reign::view::redirect_gotham(#input)
        }
    } else if cfg!(feature = "views-tide") {
        quote! {
            ::reign::view::redirect_tide(#input)
        }
    } else if cfg!(feature = "views-warp") {
        quote! {
            ::reign::view::redirect_warp(#input)
        }
    } else {
        quote! {}
    }
}
