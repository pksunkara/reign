use super::{Pat, Tokenize, ViewFields};
use proc_macro2::TokenStream;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Comma, Paren},
};

pub struct PatTuple {
    pub paren_token: Paren,
    pub elems: Punctuated<Pat, Comma>,
}

impl Parse for PatTuple {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);

        let mut elems = Punctuated::new();

        while !content.is_empty() {
            let value: Pat = content.parse()?;
            elems.push_value(value);

            if content.is_empty() {
                break;
            }

            let punct = content.parse()?;
            elems.push_punct(punct);
        }

        Ok(PatTuple { paren_token, elems })
    }
}

impl Tokenize for PatTuple {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.paren_token.surround(tokens, |tokens| {
            self.elems.tokenize(tokens, idents, scopes);
        });
    }
}
