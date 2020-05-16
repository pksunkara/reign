use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::{Comma, Eq},
    Expr, Ident,
};

pub struct Options {
    pub inner: HashMap<String, Expr>,
}

impl Options {
    pub fn remove(&mut self, key: &'static str) -> Option<Expr> {
        self.inner.remove(key)
    }
}

impl Parse for Options {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut inner = HashMap::new();

        while input.peek(Comma) {
            input.parse::<Comma>()?;
            let ident: Ident = input.parse()?;
            input.parse::<Eq>()?;

            inner.insert(ident.to_string(), input.parse()?);
        }

        Ok(Options { inner })
    }
}
