use super::{Pat, Tokenize};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::And,
    Ident,
};

pub struct PatReference {
    pub and_token: And,
    pub pat: Box<Pat>,
}

impl Parse for PatReference {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(PatReference {
            and_token: input.parse()?,
            pat: input.parse()?,
        })
    }
}

impl Tokenize for PatReference {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>, scopes: &Vec<Ident>) {
        self.and_token.to_tokens(tokens);
        self.pat.tokenize(tokens, idents, scopes);
    }
}
