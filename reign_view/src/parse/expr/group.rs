use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::token::Group;

#[derive(Debug)]
pub struct ExprGroup {
    pub group_token: Group,
    pub expr: Box<Expr>,
}

impl ToTokens for ExprGroup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.group_token.surround(tokens, |tokens| {
            self.expr.to_tokens(tokens);
        });
    }
}
