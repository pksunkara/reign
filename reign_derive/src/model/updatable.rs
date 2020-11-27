use crate::{model::model::Model, INTERNAL_ERR};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Ident};

impl Model {
    pub fn gen_updatable(&self) -> TokenStream {
        let gen_updatable_struct = self.gen_updatable_struct();
        let gen_updatable_methods = self.gen_updatable_methods(&self.ident);

        quote! {
            #gen_updatable_struct
            #gen_updatable_methods
        }
    }

    fn updatable_ident(&self) -> Ident {
        format_ident!("Updatable{}", self.ident)
    }

    // Generates struct & constructor for `UPDATE` statements
    fn gen_updatable_struct(&self) -> TokenStream {
        let updatable_ident = self.updatable_ident();
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
            #vis struct #updatable_ident<M> {
                _phantom: std::marker::PhantomData<(M)>,
                #(#for_struct,)*
            }

            impl<M> #updatable_ident<M> {
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
    fn gen_updatable_methods(&self, ident: &Ident) -> TokenStream {
        let updatable_ident = self.updatable_ident();
        let vis = &self.vis;

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #ident {
                #vis fn change() -> #updatable_ident<#ident> {
                    #updatable_ident::new()
                }
            }
        }
    }
}
