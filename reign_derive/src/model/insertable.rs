use crate::{
    model::model::{Model, ModelField},
    INTERNAL_ERR,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Ident};

impl Model {
    pub fn gen_insertable(&self) -> TokenStream {
        let gen_insertable_struct = self.gen_insertable_struct();
        let gen_insertable_setters = self.gen_insertable_setters();
        let gen_insertable_methods = self.gen_insertable_methods(&self.ident);
        let gen_insertable_action = self.gen_insertable_action(&self.ident, &self.fields);

        quote! {
            #gen_insertable_struct
            #gen_insertable_setters
            #gen_insertable_methods
            #gen_insertable_action
        }
    }

    pub fn gen_tag_insertable(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let gen_tag_insertable_methods = self.gen_insertable_methods(ident);
        let gen_tag_insertable_action = self.gen_insertable_action(ident, fields);

        quote! {
            #gen_tag_insertable_methods
            #gen_tag_insertable_action
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
            .filter(|x| !x.no_insert)
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

    // Generates individual column setters for `INSERT`
    fn gen_insertable_setters(&self) -> TokenStream {
        let insertable_ident = self.insertable_ident();
        // let schema = self.schema();
        // let backend = self.backend();

        // let (field_vis, field_ident) = self
        //     .fields
        //     .iter()
        //     .filter(|x| !x.no_insert)
        //     .map(|x| (&x.field.vis, x.field.ident.as_ref().expect(INTERNAL_ERR)))
        //     .unzip::<_, _, Vec<_>, Vec<_>>();

        quote! {
            impl<M> #insertable_ident<M> {
                // #(#field_vis fn #field_ident<E, X>(mut self, #field_ident: E) -> Self
                // {
                //     self.#field_ident = Some(#field_ident);
                //     self
                // })*
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
    fn gen_insertable_action(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let insertable_ident = self.insertable_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let db = self.db();
        let vis = &self.vis;

        let column_ident = fields.iter().map(|x| &x.column_ident).collect::<Vec<_>>();

        quote! {
            impl #insertable_ident<#ident> {
                #vis async fn save(self) -> Result<#ident, ::reign::model::tokio_diesel::AsyncError> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;
                    use ::reign::model::diesel::ExpressionMethods;

                    ::reign::model::diesel::insert_into(#schema::#table_ident::table)
                        .values((
                            #schema::#table_ident::id.eq(1),
                        ))
                        .returning((
                            #(#schema::#table_ident::#column_ident,)*
                        ))
                        .get_result_async::<#ident>(#db)
                        .await
                }
            }
        }
    }
}
