use proc_macro2::TokenStream;
use quote::quote;
use syn::ExprStruct;

// TODO: Capture local variables unhygienically and send them to templates
pub(super) fn render(input: ExprStruct) -> TokenStream {
    if cfg!(feature = "views-gotham") {
        quote! {
            ::reign::view::render_gotham(state, #input)
        }
    } else if cfg!(feature = "views-warp") {
        quote! {
            ::reign::view::render_warp(#input)
        }
    } else if cfg!(feature = "views-tide") {
        quote! {
            ::reign::view::render_tide(#input)
        }
    } else if cfg!(feature = "views-actix") {
        quote! {
            ::reign::view::render_actix(#input)
        }
    } else {
        quote! {
            format!("{}", #input)
        }
    }
}
