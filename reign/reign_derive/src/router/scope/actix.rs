use crate::router::scope::Scope;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub fn actix(input: Scope) -> TokenStream {
    let Scope { path, pipe, rest } = input;

    let app = quote! {
        ::actix_web::web::scope(#path)
    };

    let pipes = if let Some(pipe) = pipe {
        pipe.into_iter().fold(app, |tokens, i| {
            let name = Ident::new(&format!("{}_pipe", i), Span::call_site());

            quote! {
                #name!(#tokens)
            }
        })
    } else {
        app
    };

    quote! {
        app = app.service({
            let mut app = #pipes;

            #rest

            app
        })
    }
}
