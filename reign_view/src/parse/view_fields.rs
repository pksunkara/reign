use proc_macro2::TokenStream;
use std::collections::hash_map::IntoIter;
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
        self.fields.insert(ident, Some(TokenStream::new()));
    }

    pub fn append(&mut self, other: ViewFields) {
        for field in other.fields {
            self.fields.insert(field.0, field.1);
        }
    }

    pub fn contains(&self, ident: &Ident) -> bool {
        self.fields.get(ident).is_some()
    }
}

impl IntoIterator for ViewFields {
    type Item = (Ident, Option<TokenStream>);
    type IntoIter = IntoIter<Ident, Option<TokenStream>>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}
