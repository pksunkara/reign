use crate::{
    model::{
        attr::Attr,
        model::{Model, ModelField},
    },
    INTERNAL_ERR,
};

use inflector::cases::pascalcase::to_pascal_case;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Ident};

use std::collections::HashMap as Map;

impl Model {
    pub fn gen_tags(&self) -> Vec<TokenStream> {
        let mut map = Map::<String, Vec<ModelField>>::new();

        for mf in &self.fields {
            for attr in &mf.attrs {
                if let Attr::Tag(_, tags) = attr {
                    for tag in tags {
                        let tag = to_pascal_case(&tag.to_string());

                        if !map.contains_key(&tag) {
                            map.insert(tag.clone(), vec![]);
                        }

                        map.get_mut(&tag).expect(INTERNAL_ERR).push(mf.clone());
                    }
                }
            }
        }

        map.iter()
            .map(|(tag, fields)| {
                let ident = self.tag_ident(tag);

                let gen_tag_struct = self.gen_tag_struct(&ident, fields);
                let gen_tag_query = self.gen_tag_query(&ident, fields);
                let gen_tag_insert = self.gen_tag_insert(&ident, fields);

                quote! {
                    #gen_tag_struct
                    #gen_tag_query
                    #gen_tag_insert
                }
            })
            .collect()
    }

    fn tag_ident(&self, tag: &str) -> Ident {
        format_ident!("{}{}", self.ident, tag)
    }

    // Generates tagged structs
    fn gen_tag_struct(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let vis = &self.vis;

        let fields = fields
            .iter()
            .map(|mf| {
                let Field { vis, ident, ty, .. } = &mf.field;

                quote! {
                    #vis #ident: #ty
                }
            })
            .collect::<Vec<_>>();

        quote! {
            #vis struct #ident {
                #(#fields),*
            }
        }
    }
}
