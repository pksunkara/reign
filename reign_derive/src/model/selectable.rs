use crate::model::model::{Model, ModelField};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Index};

impl Model {
    pub fn gen_selectable(&self) -> TokenStream {
        let gen_queryable_trait = self.gen_queryable_trait(&self.ident, &self.fields);
        let gen_selectable_methods = self.gen_selectable_methods(&self.ident);
        let gen_selectable_actions = self.gen_selectable_actions(&self.ident, &self.fields);

        quote! {
            #gen_queryable_trait
            #gen_selectable_methods
            #gen_selectable_actions
        }
    }

    pub fn gen_tag_selectable(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let gen_tag_queryable_trait = self.gen_queryable_trait(ident, fields);
        let gen_tag_selectable_methods = self.gen_selectable_methods(ident);
        let gen_tag_selectable_actions = self.gen_selectable_actions(ident, fields);

        quote! {
            #gen_tag_queryable_trait
            #gen_tag_selectable_methods
            #gen_tag_selectable_actions
        }
    }

    // Generates Queryable
    fn gen_queryable_trait(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let table_ident = &self.table_ident;
        let schema = self.schema();

        let field_ty = fields.iter().map(|f| &f.field.ty).collect::<Vec<_>>();

        let field_sql_ty = fields
            .iter()
            .map(|f| {
                let column = &f.column_ident;

                quote! {
                    ::reign::model::diesel::dsl::SqlTypeOf<#schema::#table_ident::#column>
                }
            })
            .collect::<Vec<_>>();

        let build_expr = fields.iter().enumerate().map(|(i, f)| {
            let i = Index::from(i);
            let ident = &f.field.ident;

            quote! {
                #ident: row.#i
            }
        });

        quote! {
            impl<B> ::reign::model::diesel::deserialize::Queryable<(#(#field_sql_ty,)*), B> for #ident
            where
                B: ::reign::model::diesel::backend::Backend,
                (#(#field_ty,)*): ::reign::model::diesel::deserialize::FromStaticSqlRow<(#(#field_sql_ty,)*), B>,
            {
                type Row = (#(#field_ty,)*);

                fn build(row: Self::Row) -> Self {
                    Self {
                        #(#build_expr,)*
                    }
                }
            }
        }
    }

    // Generates starting methods for `SELECT`
    fn gen_selectable_methods(&self, ident: &Ident) -> TokenStream {
        let vis = &self.vis;

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #ident {
                #vis async fn all() -> Result<Vec<#ident>, ::reign::model::Error> {
                    #ident::filter().all().await
                }

                #vis async fn one() -> Result<Option<#ident>, ::reign::model::Error> {
                    #ident::filter().one().await
                }
            }
        }
    }

    // Generates actual action for `SELECT`
    fn gen_selectable_actions(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let filterable_ident = self.filterable_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let db = self.db();
        let vis = &self.vis;

        let column_ident = fields.iter().map(|x| &x.column_ident).collect::<Vec<_>>();

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #filterable_ident<#ident> {
                #vis async fn all(self) -> Result<Vec<#ident>, ::reign::model::Error> {
                    self.all_from(None, None).await
                }

                #vis async fn all_from(self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<#ident>, ::reign::model::Error> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;
                    use ::reign::model::diesel::QueryDsl;

                    let mut select = #schema::#table_ident::table.filter(self.statement)
                        .select((
                            #(#schema::#table_ident::#column_ident,)*
                        ))
                        .into_boxed();

                    if let Some(offset) = offset {
                        select = select.offset(offset);
                    }

                    if let Some(limit) = limit {
                        select = select.limit(limit);
                    }

                    Ok(select
                        .load_async::<#ident>(#db)
                        .await?)
                }

                #vis async fn one(self) -> Result<Option<#ident>, ::reign::model::Error> {
                    self.one_from(None).await
                }

                #vis async fn one_from(self, offset: Option<i64>) -> Result<Option<#ident>, ::reign::model::Error> {
                    use ::reign::model::tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};
                    use ::reign::model::diesel::QueryDsl;

                    let mut select = #schema::#table_ident::table.filter(self.statement)
                        .select((
                            #(#schema::#table_ident::#column_ident,)*
                        ))
                        .limit(1)
                        .into_boxed();

                    if let Some(offset) = offset {
                        select = select.offset(offset);
                    }

                    Ok(select
                        .get_result_async::<#ident>(#db)
                        .await
                        .optional()?)
                }
            }
        }
    }
}
