use super::{expr::expr_no_struct, Expr, Tokenize};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::In,
    Ident, Pat,
};

pub struct For {
    pub pat: Pat,
    pub in_token: In,
    pub expr: Box<Expr>,
}

impl Parse for For {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(For {
            pat: input.parse()?,
            in_token: input.parse()?,
            expr: Box::new(input.call(expr_no_struct)?),
        })
    }
}

impl Tokenize for For {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>) {
        self.pat.to_tokens(tokens);
        self.in_token.to_tokens(tokens);
        self.expr.tokenize(tokens, idents);
    }
}
