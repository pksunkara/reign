use crate::router::{
    path::{combine_params, PathSegment},
    To, ROUTE_NUM, STRUCT_NUM,
};
use inflector::cases::screamingsnakecase::to_screaming_snake_case;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub fn gotham(input: To) -> TokenStream {
    let To {
        methods,
        path,
        action,
        prev,
    } = input;

    let (paths, params) = path.gotham(false);
    let methods = methods
        .iter()
        .map(|i| Ident::new(&to_screaming_snake_case(&i.to_string()), i.span()))
        .collect::<Vec<_>>();

    let (fields, args) =
        combine_params(prev, params)
            .into_iter()
            .fold((quote!(), quote!()), |acc, s| match s {
                PathSegment::Static(_) => acc,
                PathSegment::Dynamic(d) => {
                    let fields = acc.0;
                    let args = acc.1;
                    let ty = d.ty();
                    let ident = d.ident;

                    (
                        quote! {
                            #fields
                            #ident: #ty,
                        },
                        quote!(
                            #args _path_data.#ident,
                        ),
                    )
                }
            });

    let (path_struct, path_data, path_extractor) = if !args.is_empty() {
        let struct_name = Ident::new(
            &format!("PathDataStruct{}", STRUCT_NUM.lock().unwrap().get()),
            Span::call_site(),
        );

        (
            quote! {
                #[derive(
                    ::serde::Deserialize,
                    ::gotham_derive::StateData,
                    ::gotham_derive::StaticResponseExtender
                )]
                struct #struct_name {
                    #fields
                }
            },
            quote! {
                let _path_data = #struct_name::take_from(&mut state);
            },
            quote! {
                .with_path_extractor::<#struct_name>()
            },
        )
    } else {
        (quote!(), quote!(), quote!())
    };

    let routes: TokenStream = paths
        .iter()
        .map(|path| {
            let name = Ident::new(
                &format!("_route_fn_{}", ROUTE_NUM.lock().unwrap().get()),
                Span::call_site(),
            );

            quote! {
                fn #name(
                    mut state: ::gotham::state::State,
                ) -> std::pin::Pin<Box<::gotham::handler::HandlerFuture>> {
                    use ::gotham::{state::FromState, handler::IntoResponse};
                    use ::futures::prelude::*;

                    async move {
                        #path_data
                        let _called = #action(&mut state, #args).await;

                        match _called {
                            Ok(r) => {
                                let r = r.into_response(&state);
                                Ok((state, r))
                            },
                            Err(e) => {
                                ::reign::log::error!("{}", e);
                                Ok((state, e.respond()))
                            },
                        }
                    }.boxed()
                }

                route
                    .request(vec![#(::gotham::hyper::Method::#methods),*], #path)
                    #path_extractor
                    .to(#name);
            }
        })
        .collect();

    quote! {
        #path_struct
        #routes
    }
}
