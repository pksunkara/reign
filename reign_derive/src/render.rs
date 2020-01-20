use proc_macro2::TokenStream;
use quote::quote;
use syn::ExprStruct;

#[cfg(not(feature = "views-gotham"))]
fn final_render(input: TokenStream) -> TokenStream {
    quote! {
        format!("{}", #input)
    }
}

#[cfg(feature = "views-gotham")]
fn final_render(input: TokenStream) -> TokenStream {
    quote! {
        ::reign::view::render_gotham(state, #input)
    }
}

// TODO: Capture local variables unhygienically and send them to templates
pub(super) fn render(input: ExprStruct) -> TokenStream {
    final_render(quote! { #input })
}
