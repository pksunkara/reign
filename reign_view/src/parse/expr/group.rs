use super::{Expr, Tokenize, ViewFields};
use proc_macro2::TokenStream;
use syn::token::Group;

pub struct ExprGroup {
    pub group_token: Group,
    pub expr: Box<Expr>,
}

impl Tokenize for ExprGroup {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.group_token.surround(tokens, |tokens| {
            self.expr.tokenize(tokens, idents, scopes);
        });
    }
}
