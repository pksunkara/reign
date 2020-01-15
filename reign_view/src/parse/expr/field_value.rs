use super::{is_member_named, Expr, Tokenize, ViewFields};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Colon,
    ExprPath, Member, Path, Token,
};

pub struct FieldValue {
    pub member: Member,
    pub colon_token: Option<Colon>,
    pub expr: Expr,
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

impl Tokenize for FieldValue {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        if let Some(colon_token) = &self.colon_token {
            self.member.to_tokens(tokens);
            colon_token.to_tokens(tokens);
            self.expr.tokenize(tokens, idents, scopes);
        } else {
            // Member is always named
            if let Member::Named(ident) = &self.member {
                idents.push(ident.clone());
                tokens.append_all(quote! {
                    #ident: self.#ident
                });
            }
        }
    }
}
