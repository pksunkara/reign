use crate::{
    model::model::{Model, ModelField},
    INTERNAL_ERR,
};

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

impl Model {
    pub fn gen_deleteable(&self) -> TokenStream {
        let gen_deleteable_methods = self.gen_deleteable_methods(&self.ident, &self.fields);
        let gen_deleteable_actions = self.gen_deleteable_actions(&self.ident, &self.fields);

        quote! {
            #gen_deleteable_methods
            #gen_deleteable_actions
        }
    }

    pub fn gen_tag_deleteable(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let gen_tag_deleteable_methods = self.gen_deleteable_methods(ident, fields);
        let gen_tag_deleteable_actions = self.gen_deleteable_actions(ident, fields);

        quote! {
            #gen_tag_deleteable_methods
            #gen_tag_deleteable_actions
        }
    }

    // Generates starting methods for `DELETE`
    fn gen_deleteable_methods(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let vis = &self.vis;

        let keys = fields.iter().filter(|x| x.primary_key).collect::<Vec<_>>();

        if keys.len() != self.primary_keys_size {
            return quote! {};
        }

        let (column_ident, field_ident) = keys
            .iter()
            .map(|f| (&f.column_ident, f.field.ident.as_ref().expect(INTERNAL_ERR)))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #ident {
                #vis async fn drop(&self) -> Result<#ident, ::reign::model::Error> {
                    #ident::filter()
                        #(.#column_ident(self.#field_ident.clone()))*
                        .drop_one()
                        .await
                }
            }
        }
    }

    // Generates actual action for `DELETE`
    fn gen_deleteable_actions(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let filterable_ident = self.filterable_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let db = self.db();
        let vis = &self.vis;

        let column_ident = fields.iter().map(|x| &x.column_ident).collect::<Vec<_>>();

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #filterable_ident<#ident> {
                #vis async fn drop(self) -> Result<Vec<#ident>, ::reign::model::Error> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;
                    use ::reign::model::diesel::QueryDsl;

                    Ok(::reign::model::diesel::delete(
                            #schema::#table_ident::table.filter(self.statement),
                        )
                        .returning((
                            #(#schema::#table_ident::#column_ident,)*
                        ))
                        .get_results_async::<#ident>(#db)
                        .await?)
                }

                async fn drop_one(self) -> Result<#ident, ::reign::model::Error> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;
                    use ::reign::model::diesel::QueryDsl;

                    Ok(::reign::model::diesel::delete(
                            #schema::#table_ident::table.filter(self.statement),
                        )
                        .returning((
                            #(#schema::#table_ident::#column_ident,)*
                        ))
                        .get_result_async::<#ident>(#db)
                        .await?)
                }
            }
        }
    }
}
