use super::{is_member_named, Expr};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Colon,
    ExprPath, Member, Path, Token,
};

#[derive(Debug)]
pub struct FieldValue {
    member: Member,
    colon_token: Option<Colon>,
    expr: Expr,
}

impl Parse for FieldValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let member: Member = input.parse()?;

        let (colon_token, value) = if input.peek(Token![:]) || !is_member_named(&member) {
            let colon_token: Token![:] = input.parse()?;
            let value: Expr = input.parse()?;

            (Some(colon_token), value)
        } else if let Member::Named(ident) = &member {
            let value = Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: Path::from(ident.clone()),
            });

            (None, value)
        } else {
            unreachable!()
        };

        Ok(FieldValue {
            member,
            colon_token,
            expr: value,
        })
    }
}

impl ToTokens for FieldValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.member.to_tokens(tokens);

        if let Some(colon_token) = &self.colon_token {
            colon_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
        }
    }
}
