use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn templates() -> TokenStream {
    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    quote! {}
}
