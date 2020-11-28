use crate::{
    model::model::{Model, ModelField},
    INTERNAL_ERR,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Ident};

impl Model {
    pub fn gen_updateable(&self) -> TokenStream {
        let gen_updateable_struct = self.gen_updateable_struct();
        let gen_as_changeset_trait = self.gen_as_changeset_trait();
        let gen_updateable_setters = self.gen_updateable_setters();
        let gen_updateable_filter_struct = self.gen_updateable_filter_struct();
        let gen_updateable_filters = self.gen_updateable_filters();
        let gen_updateable_filter_methods = self.gen_updateable_filter_methods(&self.ident);
        let gen_updateable_methods = self.gen_updateable_methods(&self.ident, &self.fields);
        let gen_updateable_actions = self.gen_updateable_actions(&self.ident, &self.fields);

        quote! {
            #gen_updateable_struct
            #gen_as_changeset_trait
            #gen_updateable_setters
            #gen_updateable_filter_struct
            #gen_updateable_filters
            #gen_updateable_filter_methods
            #gen_updateable_methods
            #gen_updateable_actions
        }
    }

    pub fn gen_tag_updateable(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let gen_updateable_filter_methods = self.gen_updateable_filter_methods(ident);
        let gen_updateable_methods = self.gen_updateable_methods(ident, fields);
        let gen_updateable_actions = self.gen_updateable_actions(ident, fields);

        quote! {
            #gen_updateable_filter_methods
            #gen_updateable_methods
            #gen_updateable_actions
        }
    }

    fn updateable_ident(&self) -> Ident {
        format_ident!("Updateable{}", self.ident)
    }

    fn updateable_inner_ident(&self) -> Ident {
        format_ident!("{}Inner", self.updateable_ident())
    }

    fn updateable_filter_ident(&self) -> Ident {
        format_ident!("{}Filter", self.updateable_ident())
    }

    // Generates struct & constructor for `UPDATE` statements
    fn gen_updateable_struct(&self) -> TokenStream {
        let updateable_ident = self.updateable_ident();
        let updateable_inner_ident = self.updateable_inner_ident();
        let vis = &self.vis;
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let backend = self.backend();

        let (for_struct, for_new) = self
            .fields
            .iter()
            .filter(|x| !x.no_write)
            .map(|x| {
                let Field { vis, ty, ident, .. } = &x.field;
                let ident = ident.as_ref().expect(INTERNAL_ERR);

                (
                    quote! {
                        #vis #ident: Option<#ty>
                    },
                    quote! {
                        #ident: None
                    },
                )
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        quote! {
            #vis struct #updateable_ident<M> {
                _phantom: std::marker::PhantomData<(M)>,
                statement: ::reign::model::diesel::query_builder::BoxedUpdateStatement<
                    'static,
                    #backend,
                    #schema::#table_ident::table
                >,
                inner: #updateable_inner_ident,
            }

            #vis struct #updateable_inner_ident {
                #(#for_struct,)*
            }

            impl<M> #updateable_ident<M> {
                fn new(statement: ::reign::model::diesel::query_builder::BoxedUpdateStatement<'static, #backend, #schema::#table_ident::table>) -> Self {
                    Self {
                        _phantom: std::marker::PhantomData,
                        statement,
                        inner: #updateable_inner_ident::new(),
                    }
                }
            }

            impl #updateable_inner_ident {
                fn new() -> Self {
                    Self {
                        #(#for_new,)*
                    }
                }
            }
        }
    }

    fn gen_as_changeset_trait(&self) -> TokenStream {
        let updateable_ident = self.updateable_ident();
        let updateable_inner_ident = self.updateable_inner_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();

        let (val_ty, val) = self
            .fields
            .iter()
            .filter(|x| !x.no_write)
            .map(|f| {
                let Field { ident, ty, .. } = &f.field;
                let ident = ident.as_ref().expect(INTERNAL_ERR);
                let column_ident = &f.column_ident;

                (
                    quote! {
                        Option<::reign::model::diesel::dsl::Eq<#schema::#table_ident::#column_ident, #ty>>
                    },
                    quote! {
                        self.#ident.map(|x| #schema::#table_ident::#column_ident.eq(x))
                    },
                )
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        quote! {
            impl<M> ::reign::model::diesel::AsChangeset for #updateable_ident<M>
            {
                type Target = #schema::#table_ident::table;
                type Changeset = <#updateable_inner_ident as ::reign::model::diesel::AsChangeset>::Changeset;

                fn as_changeset(self) -> Self::Changeset {
                    use ::reign::model::diesel::ExpressionMethods;

                    self.inner.as_changeset()
                }
            }

            impl ::reign::model::diesel::AsChangeset for #updateable_inner_ident
            {
                type Target = #schema::#table_ident::table;
                type Changeset = <(#(#val_ty,)*) as ::reign::model::diesel::AsChangeset>::Changeset;

                fn as_changeset(self) -> Self::Changeset {
                    use ::reign::model::diesel::ExpressionMethods;

                    (#(#val,)*).as_changeset()
                }
            }
        }
    }

    // Generates individual column setters for `UPDATE`
    fn gen_updateable_setters(&self) -> TokenStream {
        let updateable_ident = self.updateable_ident();

        let setters = self
            .fields
            .iter()
            .filter(|x| !x.no_write)
            .map(|f| {
                let Field { vis, ident, ty, .. } = &f.field;
                let ident = ident.as_ref().expect(INTERNAL_ERR);

                quote! {
                    #vis fn #ident(mut self, #ident: #ty) -> Self {
                        self.inner.#ident = Some(#ident);
                        self
                    }
                }
            })
            .collect::<Vec<_>>();

        quote! {
            impl<M> #updateable_ident<M> {
                #(#setters)*
            }
        }
    }

    fn gen_updateable_filter_struct(&self) -> TokenStream {
        let updateable_ident = self.updateable_ident();
        let updateable_filter_ident = self.updateable_filter_ident();
        let vis = &self.vis;
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let backend = self.backend();

        quote! {
            #vis struct #updateable_filter_ident<M> {
                _phantom: std::marker::PhantomData<(M)>,
                statement: ::reign::model::diesel::query_builder::BoxedUpdateStatement<
                    'static,
                    #backend,
                    #schema::#table_ident::table
                >,
            }

            impl<M> #updateable_filter_ident<M> {
                fn new() -> Self {
                    Self {
                        _phantom: std::marker::PhantomData,
                        statement: ::reign::model::diesel::update(#schema::#table_ident::table).into_boxed(),
                    }
                }

                fn set(self) -> #updateable_ident<M> {
                    #updateable_ident::new(self.statement)
                }
            }
        }
    }

    // Generates individual column filters for `UPDATE`
    fn gen_updateable_filters(&self) -> TokenStream {
        let updateable_filter_ident = self.updateable_filter_ident();
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
            impl<M> #updateable_filter_ident<M> {
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

    fn gen_updateable_filter_methods(&self, ident: &Ident) -> TokenStream {
        let updateable_filter_ident = self.updateable_filter_ident();
        let vis = &self.vis;

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #ident {
                #vis fn change() -> #updateable_filter_ident<Vec<#ident>> {
                    #updateable_filter_ident::new()
                }
            }
        }
    }

    // Generates starting methods for `UPDATE`
    fn gen_updateable_methods(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let updateable_ident = self.updateable_ident();
        let updateable_filter_ident = self.updateable_filter_ident();
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
                #vis fn set(&self) -> #updateable_ident<#ident> {
                    #updateable_filter_ident::new()
                        #(.#column_ident(self.#field_ident.clone()))*
                        .set()
                }
            }
        }
    }

    // Generates actual action for `SELECT`
    fn gen_updateable_actions(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let updateable_ident = self.updateable_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let db = self.db();
        let vis = &self.vis;

        let column_ident = fields.iter().map(|x| &x.column_ident).collect::<Vec<_>>();

        quote! {
            impl #updateable_ident<Vec<#ident>> {
                #vis async fn save(self) -> Result<Vec<#ident>, ::reign::model::Error> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;
                    use ::reign::model::diesel::QueryDsl;

                    Ok(self.statement
                        .set(self.inner)
                        .returning((
                            #(#schema::#table_ident::#column_ident,)*
                        ))
                        .get_results_async::<#ident>(#db)
                        .await?)
                }
            }

            impl #updateable_ident<#ident> {
                #vis async fn save(self) -> Result<#ident, ::reign::model::Error> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;
                    use ::reign::model::diesel::QueryDsl;

                    Ok(self.statement
                        .set(self.inner)
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
