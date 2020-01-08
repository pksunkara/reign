use super::{Pat, PatIdent, Tokenize};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::{Colon, Ref},
    Ident, Member,
};

pub struct FieldPat {
    pub member: Member,
    pub colon_token: Option<Colon>,
    pub pat: Box<Pat>,
}

impl Parse for FieldPat {
    fn parse(input: ParseStream) -> Result<Self> {
        let by_ref: Option<Ref> = input.parse()?;
        let member: Member = input.parse()?;

        if by_ref.is_none() && input.peek(Colon) || !member.is_named() {
            return Ok(FieldPat {
                member,
                colon_token: input.parse()?,
                pat: input.parse()?,
            });
        }

        let ident = match member {
            Member::Named(ident) => ident,
            Member::Unnamed(_) => unreachable!(),
        };

        let pat = Pat::Ident(PatIdent {
            by_ref,
            ident: ident.clone(),
        });

        Ok(FieldPat {
            member: Member::Named(ident),
            colon_token: None,
            pat: Box::new(pat),
        })
    }
}

impl Tokenize for FieldPat {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>, scopes: &Vec<Ident>) {
        self.member.to_tokens(tokens);

        if let Some(colon_token) = &self.colon_token {
            colon_token.to_tokens(tokens);
            self.pat.tokenize(tokens, idents, scopes);
        } else {
            // Member is always named
            if let Member::Named(ident) = &self.member {
                idents.push(ident.clone());
            }
        }
    }
}
