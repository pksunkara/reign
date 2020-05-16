use super::{FieldPat, Tokenize, ViewFields};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    punctuated::Punctuated,
    token::{Brace, Comma, Dot2},
    Path,
};

pub struct PatStruct {
    pub path: Path,
    pub brace_token: Brace,
    pub fields: Punctuated<FieldPat, Comma>,
    pub dot2_token: Option<Dot2>,
}

impl Tokenize for PatStruct {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.path.to_tokens(tokens);

        self.brace_token.surround(tokens, |tokens| {
            self.fields.tokenize(tokens, idents, scopes);
            self.dot2_token.to_tokens(tokens);
        })
    }
}
