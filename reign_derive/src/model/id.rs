use crate::{
    model::model::{Model, ModelField},
    INTERNAL_ERR,
};

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

impl Model {
    pub fn gen_id(&self) -> TokenStream {
        let gen_identifiable = self.gen_identifiable(&self.ident, &self.fields);

        quote! {
            #gen_identifiable
        }
    }

    pub fn gen_tag_id(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let gen_tag_identifiable = self.gen_identifiable(ident, fields);

        quote! {
            #gen_tag_identifiable
        }
    }

    // Generates `id` method
    fn gen_identifiable(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let table_ident = &self.table_ident;
        let schema = self.schema();

        let keys = fields.iter().filter(|x| x.primary_key).collect::<Vec<_>>();

        if keys.len() != self.primary_keys_size {
            return quote! {};
        }

        let (field_ty, field_ident) = keys
            .iter()
            .map(|f| (&f.field.ty, f.field.ident.as_ref().expect(INTERNAL_ERR)))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        quote! {
            impl ::reign::model::diesel::associations::HasTable for #ident
            {
                type Table = #schema::#table_ident::table;

                fn table() -> Self::Table {
                    #schema::#table_ident::table
                }
            }

            impl<'ident> ::reign::model::diesel::associations::Identifiable for &'ident #ident
            {
                type Id = (#(&'ident #field_ty),*);

                fn id(self) -> Self::Id {
                    (#(&self.#field_ident),*)
                }
            }
        }
    }
}
