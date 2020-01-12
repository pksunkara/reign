use super::{PatTuple, Tokenize, ViewFields};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Path;

pub struct PatTupleStruct {
    pub path: Path,
    pub pat: PatTuple,
}

impl Tokenize for PatTupleStruct {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.path.to_tokens(tokens);
        self.pat.tokenize(tokens, idents, scopes);
    }
}
