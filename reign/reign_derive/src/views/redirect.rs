use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

pub fn redirect(input: Expr) -> TokenStream {
    if cfg!(feature = "view-actix") {
        quote! {
            ::reign::view::redirect_actix(#input)
        }
    } else if cfg!(feature = "view-gotham") {
        quote! {
            ::reign::view::redirect_gotham(#input)
        }
    } else if cfg!(feature = "view-tide") {
        quote! {
            ::reign::view::redirect_tide(#input)
        }
    } else if cfg!(feature = "view-warp") {
        quote! {
            ::reign::view::redirect_warp(#input)
        }
    } else {
        quote! {}
    }
}
