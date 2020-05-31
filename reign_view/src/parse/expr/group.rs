use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{TokenStream, TokenTree};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse2,
    token::Group,
};

pub struct ExprGroup {
    pub group_token: Group,
    pub expr: Box<Expr>,
}

impl Parse for ExprGroup {
    fn parse(input: ParseStream) -> Result<Self> {
        let group = match input.parse::<TokenTree>()? {
            TokenTree::Group(group) => group,
            _ => return Err(input.error("expected invisible group")),
        };

        let group_token = Group(group.span());
        let expr = parse2(group.stream())?;

        Ok(ExprGroup { group_token, expr })
    }
}

impl Tokenize for ExprGroup {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.group_token.surround(tokens, |tokens| {
            self.expr.tokenize(tokens, idents, scopes);
        });
    }
}
