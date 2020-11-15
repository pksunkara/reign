use crate::{
    model::model::{Model, ModelField},
    INTERNAL_ERR,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Ident};

impl Model {
    pub fn gen_insert(&self) -> TokenStream {
        let gen_insert_struct = self.gen_insert_struct();
        let gen_insert_setters = self.gen_insert_setters();
        let gen_insert_methods = self.gen_insert_methods(&self.ident);
        let gen_insert_action = self.gen_insert_action(&self.ident, &self.fields);

        quote! {
            #gen_insert_struct
            #gen_insert_setters
            #gen_insert_methods
            #gen_insert_action
        }
    }

    pub fn gen_tag_insert(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let gen_tag_insert_methods = self.gen_insert_methods(ident);
        let gen_tag_insert_action = self.gen_insert_action(ident, fields);

        quote! {
            #gen_tag_insert_methods
            #gen_tag_insert_action
        }
    }

    fn insert_ident(&self) -> Ident {
        format_ident!("Insert{}", self.ident)
    }

    // Generates struct & constructor for `INSERT` statements
    fn gen_insert_struct(&self) -> TokenStream {
        let insert_ident = self.insert_ident();
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
            #vis struct #insert_ident<M> {
                _phantom: std::marker::PhantomData<(M)>,
                #(#for_struct,)*
            }

            impl<M> #insert_ident<M> {
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
    fn gen_insert_setters(&self) -> TokenStream {
        let insert_ident = self.insert_ident();
        // let schema = self.schema();
        // let backend = self.backend();

        // let (field_vis, field_ident) = self
        //     .fields
        //     .iter()
        //     .filter(|x| !x.no_insert)
        //     .map(|x| (&x.field.vis, x.field.ident.as_ref().expect(INTERNAL_ERR)))
        //     .unzip::<_, _, Vec<_>, Vec<_>>();

        quote! {
            impl<M> #insert_ident<M> {
                // #(#field_vis fn #field_ident<E, X>(mut self, #field_ident: E) -> Self
                // {
                //     self.#field_ident = Some(#field_ident);
                //     self
                // })*
            }
        }
    }

    // Generates starting methods for `INSERT`
    fn gen_insert_methods(&self, ident: &Ident) -> TokenStream {
        let insert_ident = self.insert_ident();
        let vis = &self.vis;

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #ident {
                #vis fn new() -> #insert_ident<#ident> {
                    #insert_ident::new()
                }
            }
        }
    }

    // Generates actual action for `INSERT`
    fn gen_insert_action(&self, ident: &Ident, fields: &[ModelField]) -> TokenStream {
        let insert_ident = self.insert_ident();
        let table_ident = &self.table_ident;
        let schema = self.schema();
        let db = self.db();
        let vis = &self.vis;

        let field_ident = fields
            .iter()
            .map(|x| x.field.ident.as_ref().expect(INTERNAL_ERR))
            .collect::<Vec<_>>();

        quote! {
            impl #insert_ident<#ident> {
                #vis async fn save(self) -> Result<#ident, ::reign::model::tokio_diesel::AsyncError> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;
                    use ::reign::model::diesel::ExpressionMethods;

                    ::reign::model::diesel::insert_into(#schema::#table_ident::table)
                        .values((
                            #schema::#table_ident::id.eq(1),
                        ))
                        .returning((
                            #(#schema::#table_ident::#field_ident,)*
                        ))
                        .get_result_async::<#ident>(#db)
                        .await
                }
            }
        }
    }
}
