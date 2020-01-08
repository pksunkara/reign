use super::Tokenize;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream, Result},
    token::Ref,
    Ident,
};

pub struct PatIdent {
    pub by_ref: Option<Ref>,
    pub ident: Ident,
}

impl Parse for PatIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(PatIdent {
            by_ref: input.parse()?,
            ident: input.call(Ident::parse_any)?,
        })
    }
}

impl Tokenize for PatIdent {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>, _: &Vec<Ident>) {
        self.by_ref.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        idents.push(self.ident.clone());
    }
}
