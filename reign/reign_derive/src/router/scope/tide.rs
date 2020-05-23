use crate::router::Scope;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn tide(input: Scope) -> TokenStream {
    let Scope {
        path,
        pipe,
        block,
        prev: _prev,
    } = input;

    let pipes = pipe
        .into_iter()
        .map(|i| {
            let name = Ident::new(&format!("{}_pipe", i), i.span());

            quote! {
                #name(&mut app);
            }
        })
        .collect::<Vec<_>>();

    let (paths, _params) = path.tide(true);

    let rest = block.stmts.into_iter().map(|stmt| stmt).collect::<Vec<_>>();

    paths
        .iter()
        .map(|path| {
            quote! {
                app.at(#path).nest({
                    let mut app = ::tide::new();

                    #(#pipes)*
                    #(#rest)*

                    app
                })
            }
        })
        .collect()
}
