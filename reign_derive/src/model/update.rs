use crate::{model::model::Model, INTERNAL_ERR};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Ident};

impl Model {
    pub fn gen_update(&self) -> TokenStream {
        let gen_update_struct = self.gen_update_struct();
        let gen_update_methods = self.gen_update_methods(&self.ident);

        quote! {
            #gen_update_struct
            #gen_update_methods
        }
    }

    fn update_ident(&self) -> Ident {
        format_ident!("Update{}", self.ident)
    }

    // Generates struct & constructor for `UPDATE` statements
    fn gen_update_struct(&self) -> TokenStream {
        let update_ident = self.update_ident();
        let vis = &self.vis;

        let (for_struct, for_new) = self
            .fields
            .iter()
            .filter(|x| !x.no_update)
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
            #vis struct #update_ident<M> {
                _phantom: std::marker::PhantomData<(M)>,
                #(#for_struct,)*
            }

            impl<M> #update_ident<M> {
                fn new() -> Self {
                    Self {
                        _phantom: std::marker::PhantomData,
                        #(#for_new,)*
                    }
                }
            }
        }
    }

    // Generates starting methods for `UPDATE`
    fn gen_update_methods(&self, ident: &Ident) -> TokenStream {
        let update_ident = self.update_ident();
        let vis = &self.vis;

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #ident {
                #vis fn change() -> #update_ident<#ident> {
                    #update_ident::new()
                }
            }
        }
    }
}
