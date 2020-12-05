use crate::{
    model::model::{Model, ModelField},
    INTERNAL_ERR,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Ident};

// TODO: model: Allow batch inserting using `Vec`
impl Model {
    pub fn gen_insertable(&self) -> TokenStream {
        let gen_insertable_struct = self.gen_insertable_struct();
        let gen_insertable_trait = self.gen_insertable_trait();
        let gen_insertable_setters = self.gen_insertable_setters();
        let gen_insertable_methods = self.gen_insertable_methods(&self.ident);
        let gen_insertable_actions = self.gen_insertable_actions(&self.ident, &self.fields);

        quote! {
            #gen_insertable_struct
            #gen_insertable_trait
            #gen_insertable_setters
            #gen_insertable_methods
            #gen_insertable_actions
        }
    }

    pub fn gen_tag_insertable(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let gen_tag_insertable_methods = self.gen_insertable_methods(ident);
        let gen_tag_insertable_actions = self.gen_insertable_actions(ident, fields);

        quote! {
            #gen_tag_insertable_methods
            #gen_tag_insertable_actions
        }
    }

    fn insertable_ident(&self) -> Ident {
        format_ident!("Insertable{}", self.ident)
    }

    // Generates struct & constructor for `INSERT` statements
    fn gen_insertable_struct(&self) -> TokenStream {
        let insertable_ident = self.insertable_ident();
        let vis = &self.vis;

        let (for_struct, for_new) = self
            .fields
            .iter()
            .filter(|x| !x.no_write)
            .map(|f| {
                let Field { ty, ident, .. } = &f.field;
                let ident = ident.as_ref().expect(INTERNAL_ERR);

                (
                    quote! {
                        #ident: Option<#ty>
                    },
                    quote! {
                        #ident: None
                    },
                )
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        quote! {
            #vis struct #insertable_ident<M> {
                _phantom: std::marker::PhantomData<(M)>,
                #(#for_struct,)*
            }

            impl<M> #insertable_ident<M> {
                fn new() -> Self {
                    Self {
                        _phantom: std::marker::PhantomData,
                        #(#for_new,)*
                    }
                }
            }
        }
    }

    fn gen_insertable_trait(&self) -> TokenStream {
        let insertable_ident = self.insertable_ident();
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
            impl<M> ::reign::model::diesel::Insertable<#schema::#table_ident::table> for #insertable_ident<M>
            {
                type Values = <(#(#val_ty,)*) as ::reign::model::diesel::Insertable<#schema::#table_ident::table>>::Values;

                fn values(self) -> Self::Values {
                    use ::reign::model::diesel::ExpressionMethods;

                    (#(#val,)*).values()
                }
            }
        }
    }

    // Generates individual column setters for `INSERT`
    // TODO:(model) Allow `AsExpression<SqlTypeOf>` so that we can take any value
    fn gen_insertable_setters(&self) -> TokenStream {
        let insertable_ident = self.insertable_ident();

        let setters = self
            .fields
            .iter()
            .filter(|x| !x.no_write)
            .map(|f| {
                let Field { vis, ident, ty, .. } = &f.field;
                let ident = ident.as_ref().expect(INTERNAL_ERR);

                quote! {
                    #vis fn #ident(mut self, #ident: #ty) -> Self {
                        self.#ident = Some(#ident);
                        self
                    }
                }
            })
            .collect::<Vec<_>>();

        quote! {
            impl<M> #insertable_ident<M> {
                #(#setters)*
            }
        }
    }

    // Generates starting methods for `INSERT`
    fn gen_insertable_methods(&self, ident: &Ident) -> TokenStream {
        let insertable_ident = self.insertable_ident();
        let vis = &self.vis;

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #ident {
                #vis fn new() -> #insertable_ident<#ident> {
                    #insertable_ident::new()
                }
            }
        }
    }

    // Generates actual action for `INSERT`
    fn gen_insertable_actions(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let insertable_ident = self.insertable_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let db = self.db();
        let vis = &self.vis;

        let column_ident = fields.iter().map(|x| &x.column_ident).collect::<Vec<_>>();

        quote! {
            impl #insertable_ident<#ident> {
                #vis async fn save(self) -> Result<#ident, ::reign::model::Error> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;
                    use ::reign::model::diesel::ExpressionMethods;

                    Ok(::reign::model::diesel::insert_into(#schema::#table_ident::table)
                        .values(self)
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
