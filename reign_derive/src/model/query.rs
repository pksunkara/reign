use crate::{
    model::model::{Model, ModelField},
    INTERNAL_ERR,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Index};

impl Model {
    pub fn gen_query(&self) -> TokenStream {
        let gen_queryable = self.gen_queryable(&self.ident, &self.fields);
        let gen_query_struct = self.gen_query_struct();
        let gen_query_filters = self.gen_query_filters();
        let gen_query_limit_offset = self.gen_query_limit_offset();
        let gen_query_methods = self.gen_query_methods(&self.ident);
        let gen_query_action = self.gen_query_action(&self.ident, &self.fields);

        quote! {
            #gen_queryable
            #gen_query_struct
            #gen_query_filters
            #gen_query_limit_offset
            #gen_query_methods
            #gen_query_action
        }
    }

    pub fn gen_tag_query(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let gen_tag_queryable = self.gen_queryable(ident, fields);
        let gen_tag_query_methods = self.gen_query_methods(ident);
        let gen_tag_query_action = self.gen_query_action(ident, fields);

        quote! {
            #gen_tag_queryable
            #gen_tag_query_methods
            #gen_tag_query_action
        }
    }

    fn query_ident(&self) -> Ident {
        format_ident!("Query{}", self.ident)
    }

    // Generates Queryable
    fn gen_queryable(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
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

    // Generates struct & constructor for `SELECT` statements
    fn gen_query_struct(&self) -> TokenStream {
        let query_ident = self.query_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let backend = self.backend();
        let vis = &self.vis;

        quote! {
            #vis struct #query_ident<T, M> {
                _phantom: std::marker::PhantomData<(T, M)>,
                limit: Option<i64>,
                offset: Option<i64>,
                statement: #schema::#table_ident::BoxedQuery<'static, #backend>,
            }

            impl<T, M> #query_ident<T, M> {
                fn new() -> Self {
                    Self {
                        _phantom: std::marker::PhantomData,
                        limit: None,
                        offset: None,
                        statement: #schema::#table_ident::table.into_boxed(),
                    }
                }
            }
        }
    }

    // Generates individual column filters for `SELECT`
    // TODO:(model) support other operations, maybe custom filter by just forwarding to `filter`
    fn gen_query_filters(&self) -> TokenStream {
        let table_ident = &self.table_ident;
        let query_ident = self.query_ident();
        let schema = self.schema();
        let backend = self.backend();

        let (field_vis, field_ident) = self
            .fields
            .iter()
            .map(|x| (&x.field.vis, x.field.ident.as_ref().expect(INTERNAL_ERR)))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        // TODO: external: Use dummy mod once https://github.com/rust-analyzer/rust-analyzer/issues/1559
        quote! {
            impl<T, M> #query_ident<T, M> {
                #(#field_vis fn #field_ident<E, X>(mut self, #field_ident: E) -> Self
                where
                    E: ::reign::model::diesel::expression::AsExpression<
                        ::reign::model::diesel::dsl::SqlTypeOf<#schema::#table_ident::#field_ident>,
                        Expression = X,
                    >,
                    X: ::reign::model::diesel::expression::BoxableExpression<
                            #schema::#table_ident::table,
                            #backend,
                            SqlType = ::reign::model::diesel::dsl::SqlTypeOf<#schema::#table_ident::#field_ident>
                        >
                        + ::reign::model::diesel::expression::ValidGrouping<
                            (),
                            IsAggregate = ::reign::model::diesel::expression::is_aggregate::Never
                        >
                        + Send
                        + 'static,
                {
                    self.statement = self.statement.filter(#schema::#table_ident::#field_ident.eq(#field_ident));
                    self
                })*
            }
        }
    }

    // Generates limit & offset setters for `SELECT` (only `All`)
    fn gen_query_limit_offset(&self) -> TokenStream {
        let query_ident = self.query_ident();
        let vis = &self.vis;

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl<M> #query_ident<::reign::model::query::All, M> {
                #vis fn limit(mut self, limit: i64) -> Self {
                    self.limit = Some(limit);
                    self
                }

                #vis fn offset(mut self, offset: i64) -> Self {
                    self.offset = Some(offset);
                    self
                }
            }
        }
    }

    // Generates starting methods for `SELECT`
    fn gen_query_methods(&self, ident: &Ident) -> TokenStream {
        let query_ident = self.query_ident();
        let vis = &self.vis;

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #ident {
                #vis fn all() -> #query_ident<::reign::model::query::All, #ident> {
                    #query_ident::new()
                }

                #vis fn one() -> #query_ident<::reign::model::query::One, #ident> {
                    #query_ident::new()
                }
            }
        }
    }

    // Generates actual action for `SELECT`
    fn gen_query_action(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let query_ident = self.query_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let db = self.db();
        let vis = &self.vis;

        let field_ident = fields
            .iter()
            .map(|x| x.field.ident.as_ref().expect(INTERNAL_ERR))
            .collect::<Vec<_>>();

        quote! {
            impl #query_ident<::reign::model::query::All, #ident> {
                #vis async fn load(self) -> Result<Vec<#ident>, ::reign::model::tokio_diesel::AsyncError> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;

                    let select = self.statement.select((
                        #(#schema::#table_ident::#field_ident,)*
                    ));

                    if let Some(limit) = self.limit {
                        if let Some(offset) = self.offset {
                            select
                                .limit(limit)
                                .offset(offset)
                                .load_async::<#ident>(#db)
                                .await
                        } else {
                            select.limit(limit).load_async::<#ident>(#db).await
                        }
                    } else {
                        select.load_async::<#ident>(#db).await
                    }
                }
            }

            impl #query_ident<::reign::model::query::One, #ident> {
                #vis async fn load(self) -> Result<Option<#ident>, ::reign::model::tokio_diesel::AsyncError> {
                    use ::reign::model::tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};

                    let select = self.statement.select((
                        #(#schema::#table_ident::#field_ident,)*
                    ));

                    select
                        .limit(1)
                        .get_result_async::<#ident>(#db)
                        .await
                        .optional()
                }
            }
        }
    }
}
