use crate::{
    model::model::{Model, ModelField},
    INTERNAL_ERR,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

impl Model {
    pub fn gen_deleteable(&self) -> TokenStream {
        let gen_deleteable_struct = self.gen_deleteable_struct();
        let gen_deleteable_filters = self.gen_deleteable_filters();
        let gen_deleteable_methods = self.gen_deleteable_methods(&self.ident, &self.fields);
        let gen_deleteable_actions = self.gen_deleteable_actions(&self.ident, &self.fields);

        quote! {
            #gen_deleteable_struct
            #gen_deleteable_filters
            #gen_deleteable_methods
            #gen_deleteable_actions
        }
    }

    pub fn gen_tag_deleteable(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let gen_deleteable_methods = self.gen_deleteable_methods(ident, fields);
        let gen_deleteable_actions = self.gen_deleteable_actions(ident, fields);

        quote! {
            #gen_deleteable_methods
            #gen_deleteable_actions
        }
    }

    fn deleteable_ident(&self) -> Ident {
        format_ident!("Deleteable{}", self.ident)
    }

    // Generates struct & constructor for `DELETE` statements
    fn gen_deleteable_struct(&self) -> TokenStream {
        let deleteable_ident = self.deleteable_ident();
        let vis = &self.vis;
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let backend = self.backend();

        quote! {
            #vis struct #deleteable_ident<M> {
                _phantom: std::marker::PhantomData<(M)>,
                statement: ::reign::model::diesel::query_builder::BoxedDeleteStatement<
                    'static,
                    #backend,
                    #schema::#table_ident::table
                >,
            }

            impl<M> #deleteable_ident<M> {
                fn new() -> Self {
                    Self {
                        _phantom: std::marker::PhantomData,
                        statement: ::reign::model::diesel::delete(#schema::#table_ident::table).into_boxed(),
                    }
                }
            }
        }
    }

    // Generates individual column filters for `DELETE`
    fn gen_deleteable_filters(&self) -> TokenStream {
        let deleteable_ident = self.deleteable_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let backend = self.backend();

        let (field_vis, column_ident) = self
            .fields
            .iter()
            .map(|x| (&x.field.vis, &x.column_ident))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl<M> #deleteable_ident<M> {
                #(#field_vis fn #column_ident<E, X>(mut self, #column_ident: E) -> Self
                where
                    E: ::reign::model::diesel::expression::AsExpression<
                        ::reign::model::diesel::dsl::SqlTypeOf<#schema::#table_ident::#column_ident>,
                        Expression = X,
                    >,
                    X: ::reign::model::diesel::expression::BoxableExpression<
                            #schema::#table_ident::table,
                            #backend,
                            SqlType = ::reign::model::diesel::dsl::SqlTypeOf<#schema::#table_ident::#column_ident>
                        >
                        + ::reign::model::diesel::expression::ValidGrouping<
                            (),
                            IsAggregate = ::reign::model::diesel::expression::is_aggregate::Never
                        >
                        + Send
                        + 'static,
                {
                    use ::reign::model::diesel::{ExpressionMethods, QueryDsl};

                    self.statement = self.statement.filter(#schema::#table_ident::#column_ident.eq(#column_ident));
                    self
                })*
            }
        }
    }

    // Generates starting methods for `UPDATE`
    fn gen_deleteable_methods(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let deleteable_ident = self.deleteable_ident();
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
                #vis fn delete() -> #deleteable_ident<Vec<#ident>> {
                    #deleteable_ident::new()
                }

                #vis async fn drop(&self) -> Result<#ident, ::reign::model::Error> {
                    #deleteable_ident::<#ident>::new()
                        #(.#column_ident(self.#field_ident.clone()))*
                        .drop()
                        .await
                }
            }
        }
    }

    // Generates actual action for `DELETE`
    fn gen_deleteable_actions(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let deleteable_ident = self.deleteable_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let db = self.db();
        let vis = &self.vis;

        let column_ident = fields.iter().map(|x| &x.column_ident).collect::<Vec<_>>();

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #deleteable_ident<Vec<#ident>> {
                #vis async fn drop(self) -> Result<Vec<#ident>, ::reign::model::Error> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;
                    use ::reign::model::diesel::QueryDsl;

                    Ok(self.statement
                        .returning((
                            #(#schema::#table_ident::#column_ident,)*
                        ))
                        .get_results_async::<#ident>(#db)
                        .await?)
                }
            }

            #[allow(dead_code, unreachable_code)]
            impl #deleteable_ident<#ident> {
                #vis async fn drop(self) -> Result<#ident, ::reign::model::Error> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;
                    use ::reign::model::diesel::QueryDsl;

                    Ok(self.statement
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
