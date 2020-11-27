use crate::{
    model::model::{Model, ModelField},
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
            for tag in &mf.tags {
                let tag = to_pascal_case(&tag.to_string());

                if !map.contains_key(&tag) {
                    map.insert(tag.clone(), vec![]);
                }

                map.get_mut(&tag).expect(INTERNAL_ERR).push(mf.clone());
            }
        }

        map.iter()
            .map(|(tag, fields)| {
                let ident = self.tag_ident(tag);

                let gen_tag_struct = self.gen_tag_struct(&ident, fields);
                let gen_tag_id = self.gen_tag_id(&ident, fields);
                let gen_tag_selectable = self.gen_tag_selectable(&ident, fields);
                let gen_tag_insert = self.gen_tag_insertable(&ident, fields);

                quote! {
                    #gen_tag_struct
                    #gen_tag_id
                    #gen_tag_selectable
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
        let fields = fields
            .iter()
            .map(|mf| {
                let Field { vis, ident, ty, .. } = &mf.field;

                quote! {
                    #vis #ident: #ty
                }
            })
            .collect::<Vec<_>>();

        let vis = &self.vis;

        quote! {
            #[allow(dead_code, unreachable_code)]
            #vis struct #ident {
                #(#fields),*
            }
        }
    }
}
