use super::{PatTuple, Tokenize};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Ident, Path};

pub struct PatTupleStruct {
    pub path: Path,
    pub pat: PatTuple,
}

impl Tokenize for PatTupleStruct {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>, scopes: &Vec<Ident>) {
        self.path.to_tokens(tokens);
        self.pat.tokenize(tokens, idents, scopes);
    }
}
