use super::{Error, Parse, ParseStream, Tokenize};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct Expr {}

impl Parse for Expr {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        Ok(Expr {})
    }
}

impl Tokenize for Expr {
    fn tokenize(&self) -> TokenStream {
        quote! {}
    }
}
