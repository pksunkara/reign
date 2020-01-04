use proc_macro2::TokenStream;
use quote::quote;
use syn::ExprStruct;

// TODO: Capture local variables unhygienically and send them to templates
pub(super) fn render(input: ExprStruct) -> TokenStream {
    quote! {
        ::reign::view::render(state, crate::views::#input)
    }
}
