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
        let gen_updateable_methods = self.gen_updateable_methods(&self.ident, &self.fields);
        let gen_updateable_actions = self.gen_updateable_actions(&self.ident, &self.fields);

        quote! {
            #gen_updateable_struct
            #gen_as_changeset_trait
            #gen_updateable_setters
            #gen_updateable_methods
            #gen_updateable_actions
        }
    }

    pub fn gen_tag_updateable(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let gen_updateable_methods = self.gen_updateable_methods(ident, fields);
        let gen_updateable_actions = self.gen_updateable_actions(ident, fields);

        quote! {
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

    // Generates struct & constructor for `UPDATE` statements
    fn gen_updateable_struct(&self) -> TokenStream {
        let filterable_ident = self.filterable_ident();
        let updateable_ident = self.updateable_ident();
        let updateable_inner_ident = self.updateable_inner_ident();
        let vis = &self.vis;

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
            #vis struct #updateable_ident<M, R> {
                _phantom: std::marker::PhantomData<(R)>,
                statement: #filterable_ident<M>,
                inner: #updateable_inner_ident,
            }

            #vis struct #updateable_inner_ident {
                #(#for_struct,)*
            }

            impl<M, R> #updateable_ident<M, R> {
                fn new(statement: #filterable_ident<M>) -> Self {
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
            impl<M, R> ::reign::model::diesel::AsChangeset for #updateable_ident<M, R>
            {
                type Target = #schema::#table_ident::table;
                type Changeset = <#updateable_inner_ident as ::reign::model::diesel::AsChangeset>::Changeset;

                fn as_changeset(self) -> Self::Changeset {
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
            impl<M, R> #updateable_ident<M, R> {
                #(#setters)*
            }
        }
    }

    // Generates starting methods for `UPDATE`
    fn gen_updateable_methods(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let updateable_ident = self.updateable_ident();
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
                #vis fn set(&self) -> #updateable_ident<#ident, #ident> {
                    #ident::filter()
                        #(.#column_ident(self.#field_ident.clone()))*
                        .set_one()
                }
            }
        }
    }

    // Generates actual action for `UPDATE`
    fn gen_updateable_actions(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let filterable_ident = self.filterable_ident();
        let updateable_ident = self.updateable_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let db = self.db();
        let vis = &self.vis;

        let column_ident = fields.iter().map(|x| &x.column_ident).collect::<Vec<_>>();

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #filterable_ident<#ident> {
                #vis fn set(self) -> #updateable_ident<#ident, Vec<#ident>> {
                    #updateable_ident::new(self)
                }

                fn set_one(self) -> #updateable_ident<#ident, #ident> {
                    #updateable_ident::new(self)
                }
            }

            #[allow(dead_code, unreachable_code)]
            impl #updateable_ident<#ident, Vec<#ident>> {
                #vis async fn save(self) -> Result<Vec<#ident>, ::reign::model::Error> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;
                    use ::reign::model::diesel::QueryDsl;

                    Ok(::reign::model::diesel::update(
                            #schema::#table_ident::table.filter(self.statement.statement),
                        )
                        .set(self.inner)
                        .returning((
                            #(#schema::#table_ident::#column_ident,)*
                        ))
                        .get_results_async::<#ident>(#db)
                        .await?)
                }
            }

            #[allow(dead_code, unreachable_code)]
            impl #updateable_ident<#ident, #ident> {
                #vis async fn save(self) -> Result<#ident, ::reign::model::Error> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;
                    use ::reign::model::diesel::QueryDsl;

                    Ok(::reign::model::diesel::update(
                            #schema::#table_ident::table.filter(self.statement.statement),
                        )
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
