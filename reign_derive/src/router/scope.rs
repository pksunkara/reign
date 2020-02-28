use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Bracket, Comma},
    Ident, LitStr,
};

pub struct Scope {
    path: LitStr,
    pipe: Option<Punctuated<Ident, Comma>>,
    rest: TokenStream,
}

impl Parse for Scope {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut content;

        Ok(Scope {
            path: input.parse()?,
            pipe: {
                if input.peek2(Bracket) {
                    input.parse::<Comma>()?;
                    bracketed!(content in input);
                    Some(content.parse_terminated(|i| i.parse::<Ident>())?)
                } else {
                    None
                }
            },
            rest: {
                input.parse::<Comma>()?;
                braced!(content in input);
                content.parse()?
            },
        })
    }
}

fn actix(input: Option<Punctuated<Ident, Comma>>, path: LitStr) -> TokenStream {
    let app = quote! {
        ::actix_web::web::scope(#path)
    };

    if input.is_none() {
        return app;
    }

    input.unwrap().into_iter().fold(app, |tokens, i| {
        let name = Ident::new(&format!("{}_pipe", i), Span::call_site());

        quote! {
            #name!(#tokens)
        }
    })
}

fn gotham(input: Punctuated<Ident, Comma>) -> TokenStream {
    let mut chains = vec![];
    let mut iter = input.into_iter().map(|i| i);
    let mut prev = None;

    while let Some(i) = iter.next() {
        let name = Ident::new(&format!("{}_pipe", i), Span::call_site());
        let chain = Ident::new(&format!("{}_chain", i), Span::call_site());

        if prev.is_none() {
            chains.push(quote! {
                let #chain = (#name, ());
            });
        } else {
            let prev_chain = Ident::new(&format!("{}_chain", prev.unwrap()), Span::call_site());

            chains.push(quote! {
                let #chain = (#name, #prev_chain);
            });
        }

        prev = Some(i);
    }

    let chain = if prev.is_some() {
        let prev_chain = Ident::new(&format!("{}_chain", prev.unwrap()), Span::call_site());

        quote! {
            #prev_chain
        }
    } else {
        quote! {
            ()
        }
    };

    quote! {
        {
            #(#chains)*
            #chain
        }
    }
}

fn tide(input: Punctuated<Ident, Comma>) -> Vec<TokenStream> {
    input
        .into_iter()
        .map(|i| {
            let name = Ident::new(&format!("{}_pipe", i), Span::call_site());

            quote! {
                #name(&mut app);
            }
        })
        .collect()
}

pub fn scope(input: Scope) -> TokenStream {
    let Scope { path, pipe, rest } = input;

    if cfg!(feature = "router-actix") {
        let pipe_tokens = actix(pipe, path);

        quote! {
            app = app.service({
                let mut app = #pipe_tokens;

                #rest

                app
            })
        }
    } else if cfg!(feature = "router-gotham") {
        let pipe_tokens = if pipe.is_none() {
            quote! {
                ()
            }
        } else {
            gotham(pipe.unwrap())
        };

        quote! {
            route
                .delegate(#path)
                .to_router(::gotham::router::builder::build_router(
                    #pipe_tokens,
                    pipeline_set.clone(),
                    |route| {
                        #rest
                    }
                ))
        }
    } else if cfg!(feature = "router-tide") {
        let pipe_tokens = if pipe.is_none() {
            vec![]
        } else {
            tide(pipe.unwrap())
        };

        quote! {
            app.at(#path).nest({
                let mut app = ::tide::new();

                #(#pipe_tokens)*
                #rest

                app
            })
        }
    } else {
        quote! {}
    }
}
