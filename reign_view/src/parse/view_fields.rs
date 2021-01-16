use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::Ident;

#[derive(Clone, Default)]
pub struct ViewFields {
    pub fields: HashMap<Ident, Option<TokenStream>>,
}

impl ViewFields {
    pub fn new() -> Self {
        ViewFields {
            fields: HashMap::new(),
        }
    }

    pub fn push(&mut self, ident: Ident) {
        self.insert(ident, None);
    }

    pub fn insert(&mut self, ident: Ident, tokens: Option<TokenStream>) {
        if let Some(ots) = self.fields.get(&ident) {
            if ots.is_some() && tokens.is_some() {
                // TODO: Unable to compare the syn::Type or TokenStream here
                // TODO:(view:err) Show the error position
                panic!("identifier `{}` has multiple type ascription hints", ident);
            } else if ots.is_none() {
                self.fields.insert(ident, tokens);
            }
        } else {
            self.fields.insert(ident, tokens);
        }
    }

    pub fn append(&mut self, other: ViewFields) {
        for field in other.fields {
            self.insert(field.0, field.1);
        }
    }

    pub fn contains(&self, ident: &Ident) -> bool {
        self.fields.get(ident).is_some()
    }

    pub fn keys(&self) -> Vec<(Ident, bool)> {
        self.fields
            .iter()
            .map(|(k, v)| (k.clone(), v.is_some()))
            .collect()
    }

    pub fn values(&self) -> Vec<TokenStream> {
        self.fields
            .values()
            .cloned()
            .map(|x| {
                if let Some(ts) = x {
                    ts
                } else {
                    quote! {
                        &'a str
                    }
                }
            })
            .collect()
    }
}
