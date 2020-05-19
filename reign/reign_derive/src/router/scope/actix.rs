use crate::router::Scope;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn actix(input: Scope) -> TokenStream {
    let Scope {
        path,
        pipe,
        block,
        prev,
    } = input;

    let pipes = if !pipe.is_empty() {
        pipe.into_iter().fold(quote!(scope), |tokens, i| {
            let name = Ident::new(&format!("{}_pipe", i), i.span());

            quote! {
                #name!(#tokens)
            }
        })
    } else {
        quote!(scope)
    };

    let (paths, params) = path.actix(true);

    let rest = block.stmts.into_iter().map(|stmt| stmt).collect::<Vec<_>>();

    paths
        .iter()
        .map(|path| {
            quote! {
                app = app.service({
                    let scope = ::actix_web::web::scope(#path);
                    let mut app = #pipes;

                    #(#rest)*

                    app
                })
            }
        })
        .collect()
}
