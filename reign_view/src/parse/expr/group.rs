use super::{Expr, Tokenize};
use proc_macro2::TokenStream;
use syn::{token::Group, Ident};

pub struct ExprGroup {
    pub group_token: Group,
    pub expr: Box<Expr>,
}

impl Tokenize for ExprGroup {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>, scopes: &Vec<Ident>) {
        self.group_token.surround(tokens, |tokens| {
            self.expr.tokenize(tokens, idents, scopes);
        });
    }
}
