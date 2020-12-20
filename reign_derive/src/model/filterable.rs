use crate::model::model::{Model, ModelField};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

impl Model {
    pub fn gen_filterable(&self) -> TokenStream {
        let gen_filterable_struct = self.gen_filterable_struct();
        let gen_filterable_filters = self.gen_filterable_filters();
        let gen_filterable_methods = self.gen_filterable_methods(&self.ident);

        quote! {
            #gen_filterable_struct
            #gen_filterable_filters
            #gen_filterable_methods
        }
    }

    pub fn gen_tag_filterable(&self, ident: &Ident, _fields: &[ModelField]) -> TokenStream {
        let gen_filterable_methods = self.gen_filterable_methods(ident);

        quote! {
            #gen_filterable_methods
        }
    }

    pub fn filterable_ident(&self) -> Ident {
        format_ident!("Filterable{}", self.ident)
    }

    fn gen_filterable_struct(&self) -> TokenStream {
        let filterable_ident = self.filterable_ident();
        let vis = &self.vis;
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let backend = self.backend();

        // TODO: external: Use dummy mod once https://github.com/rust-analyzer/rust-analyzer/issues/1559
        quote! {
            #vis struct #filterable_ident<M> {
                _phantom: std::marker::PhantomData<(M)>,
                statement: Box<
                    dyn ::reign::model::diesel::expression::BoxableExpression<
                        #schema::#table_ident::table,
                        #backend,
                        SqlType = ::reign::model::diesel::sql_types::Bool,
                    >,
                >,
            }

            impl<M> #filterable_ident<M> {
                fn new() -> Self {
                    use ::reign::model::diesel::{
                        sql_types::{Bool, Nullable},
                        PgExpressionMethods, IntoSql,
                    };

                    let none: Option<bool> = None;

                    Self {
                        _phantom: std::marker::PhantomData,
                        statement: Box::new(none.into_sql::<Nullable<Bool>>().is_not_distinct_from(none)),
                    }
                }
            }
        }
    }

    // TODO: model: More filters when both `Bool` and `Nullable<Bool>` can be stored at once
    // Generates individual column filters
    fn gen_filterable_filters(&self) -> TokenStream {
        let filterable_ident = self.filterable_ident();
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
            impl<M> #filterable_ident<M> {
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
                    use ::reign::model::diesel::{BoolExpressionMethods, PgExpressionMethods, QueryDsl};

                    self.statement = Box::new(self.statement.and(#schema::#table_ident::#column_ident.is_not_distinct_from::<E>(#column_ident)));
                    self
                })*
            }
        }
    }

    fn gen_filterable_methods(&self, ident: &Ident) -> TokenStream {
        let filterable_ident = self.filterable_ident();
        let vis = &self.vis;

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #ident {
                #vis fn filter() -> #filterable_ident<#ident> {
                    #filterable_ident::new()
                }
            }
        }
    }
}
