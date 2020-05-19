use crate::router::Scope;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn actix(input: Scope) -> TokenStream {
    let Scope { path, pipe, rest } = input;

    let pipes = if let Some(pipe) = pipe {
        pipe.into_iter().fold(quote!(scope), |tokens, i| {
            let name = Ident::new(&format!("{}_pipe", i), i.span());

            quote! {
                #name!(#tokens)
            }
        })
    } else {
        quote!(scope)
    };

    path.actix(true)
        .iter()
        .map(|path| {
            quote! {
                app = app.service({
                    let scope = ::actix_web::web::scope(#path);
                    let mut app = #pipes;

                    #rest

                    app
                })
            }
        })
        .collect()
}
