use proc_macro2::Span;
use std::collections::BTreeMap as Map;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::{Comma, Eq},
    Expr, Ident,
};

pub struct Options {
    pub inner: Map<Ident, Expr>,
}

impl Options {
    #[allow(unused)]
    pub fn remove(&mut self, key: &'static str) -> Option<Expr> {
        self.inner.remove(&Ident::new(key, Span::call_site()))
    }

    #[allow(unused)]
    pub fn find(&self, key: &str) -> Option<(&Ident, &Expr)> {
        self.inner.iter().find(|x| *x.0 == key)
    }
}

impl Parse for Options {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut inner = Map::new();

        while !input.is_empty() {
            input.parse::<Comma>()?;
            let ident: Ident = input.parse()?;
            input.parse::<Eq>()?;

            inner.insert(ident, input.parse()?);
        }

        Ok(Options { inner })
    }
}
