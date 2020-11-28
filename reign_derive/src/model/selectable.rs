use crate::model::model::{Model, ModelField};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Index};

impl Model {
    pub fn gen_selectable(&self) -> TokenStream {
        let gen_queryable_trait = self.gen_queryable_trait(&self.ident, &self.fields);
        let gen_selectable_struct = self.gen_selectable_struct();
        let gen_selectable_filters = self.gen_selectable_filters();
        let gen_selectable_limit_offset = self.gen_selectable_limit_offset();
        let gen_selectable_methods = self.gen_selectable_methods(&self.ident);
        let gen_selectable_actions = self.gen_selectable_actions(&self.ident, &self.fields);

        quote! {
            #gen_queryable_trait
            #gen_selectable_struct
            #gen_selectable_filters
            #gen_selectable_limit_offset
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

    fn selectable_ident(&self) -> Ident {
        format_ident!("Selectable{}", self.ident)
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

    // Generates struct & constructor for `SELECT` statements
    fn gen_selectable_struct(&self) -> TokenStream {
        let selectable_ident = self.selectable_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let backend = self.backend();
        let vis = &self.vis;

        quote! {
            #vis struct #selectable_ident<T> {
                _phantom: std::marker::PhantomData<T>,
                statement: #schema::#table_ident::BoxedQuery<'static, #backend>,
            }

            impl<T> #selectable_ident<T> {
                fn new() -> Self {
                    use ::reign::model::diesel::QueryDsl;

                    Self {
                        _phantom: std::marker::PhantomData,
                        statement: #schema::#table_ident::table.into_boxed(),
                    }
                }
            }
        }
    }

    // Generates individual column filters for `SELECT`
    // TODO:(model) support other operations, maybe custom filter by just forwarding to `filter`
    fn gen_selectable_filters(&self) -> TokenStream {
        let table_ident = &self.table_ident;
        let selectable_ident = self.selectable_ident();
        let schema = self.schema();
        let backend = self.backend();

        let (field_vis, column_ident) = self
            .fields
            .iter()
            .map(|x| (&x.field.vis, &x.column_ident))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        // TODO: external: Use dummy mod once https://github.com/rust-analyzer/rust-analyzer/issues/1559
        quote! {
            #[allow(dead_code, unreachable_code)]
            impl<T> #selectable_ident<T> {
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

    // Generates limit & offset setters for `SELECT`
    fn gen_selectable_limit_offset(&self) -> TokenStream {
        let selectable_ident = self.selectable_ident();
        let vis = &self.vis;

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl<T> #selectable_ident<T> {
                #vis fn limit(mut self, limit: i64) -> Self {
                    use ::reign::model::diesel::QueryDsl;

                    self.statement = self.statement.limit(limit);
                    self
                }

                #vis fn offset(mut self, offset: i64) -> Self {
                    use ::reign::model::diesel::QueryDsl;

                    self.statement = self.statement.offset(offset);
                    self
                }
            }
        }
    }

    // Generates starting methods for `SELECT`
    fn gen_selectable_methods(&self, ident: &Ident) -> TokenStream {
        let selectable_ident = self.selectable_ident();
        let vis = &self.vis;

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #ident {
                #vis fn all() -> #selectable_ident<Vec<#ident>> {
                    #selectable_ident::new()
                }

                #vis fn one() -> #selectable_ident<#ident> {
                    #selectable_ident::new().limit(1)
                }
            }
        }
    }

    // Generates actual action for `SELECT`
    fn gen_selectable_actions(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let selectable_ident = self.selectable_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let db = self.db();
        let vis = &self.vis;

        let column_ident = fields.iter().map(|x| &x.column_ident).collect::<Vec<_>>();

        quote! {
            impl #selectable_ident<Vec<#ident>> {
                #vis async fn load(self) -> Result<Vec<#ident>, ::reign::model::Error> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;
                    use ::reign::model::diesel::QueryDsl;

                    let select = self.statement.select((
                        #(#schema::#table_ident::#column_ident,)*
                    ));

                    Ok(select
                        .load_async::<#ident>(#db)
                        .await?)
                }
            }

            impl #selectable_ident<#ident> {
                #vis async fn load(self) -> Result<Option<#ident>, ::reign::model::Error> {
                    use ::reign::model::tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};
                    use ::reign::model::diesel::QueryDsl;

                    let select = self.statement.select((
                        #(#schema::#table_ident::#column_ident,)*
                    ));

                    Ok(select
                        .get_result_async::<#ident>(#db)
                        .await
                        .optional()?)
                }
            }
        }
    }
}
