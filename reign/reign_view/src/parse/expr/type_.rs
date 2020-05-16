use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Colon,
    Error, Type,
};

pub struct ExprType {
    pub expr: Box<Expr>,
    pub colon_token: Colon,
    pub ty: Box<Type>,
}

impl Parse for ExprType {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Type(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        "expected type ascription expression",
                    ))
                }
            }
        }
    }
}

impl Tokenize for ExprType {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        let mut ty_tokens = TokenStream::new();

        self.ty.to_tokens(&mut ty_tokens);

        if let Expr::Path(path) = &*self.expr {
            if let Some(ident) = path.path.get_ident() {
                if !scopes.contains(&ident) {
                    idents.insert(ident.clone(), Some(ty_tokens));

                    tokens.append_all(quote! {
                        self.#ident
                    });
                    return;
                }
            }
        }

        self.expr.tokenize(tokens, idents, scopes);
    }
}
