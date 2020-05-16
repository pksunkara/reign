use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Colon, Comma},
    Expr, Ident,
};

struct Pipeline {
    name: Ident,
    middlewares: Punctuated<Expr, Comma>,
}

impl Parse for Pipeline {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Pipeline {
            name: input.parse()?,
            middlewares: {
                let content;

                input.parse::<Colon>()?;
                bracketed!(content in input);

                content.parse_terminated(|i| i.parse::<Expr>())?
            },
        })
    }
}

impl ToTokens for Pipeline {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = Ident::new(&format!("{}_pipe", self.name), Span::call_site());
        let middlewares = self.middlewares.iter().map(|i| i);

        if cfg!(feature = "router-actix") {
            tokens.append_all(quote! {
                macro_rules! #name {
                    ($app:expr) => {
                        $app
                            #(.wrap(#middlewares))*
                    };
                }
            });
        } else if cfg!(feature = "router-gotham") {
            tokens.append_all(quote! {
                let (pipelines, #name) = pipelines.add(
                    ::gotham::pipeline::new_pipeline()
                        #(.add(#middlewares))*
                        .build()
                );
            });
        } else if cfg!(feature = "router-tide") {
            tokens.append_all(quote! {
                fn #name<S>(app: &mut ::tide::Server<S>)
                where
                    S: Send + Sync + 'static,
                {
                    #(app.middleware(#middlewares);)*
                }
            });
        }
    }
}

pub struct Pipelines {
    items: Punctuated<Pipeline, Comma>,
}

impl Parse for Pipelines {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Pipelines {
            items: input.parse_terminated(|i| i.parse::<Pipeline>())?,
        })
    }
}

pub fn pipelines(input: Pipelines) -> TokenStream {
    let items = input.items.into_iter().map(|i| i);

    if cfg!(feature = "router-actix") {
        quote! {
            #(#items)*
        }
    } else if cfg!(feature = "router-gotham") {
        quote! {
            let pipelines = ::gotham::pipeline::set::new_pipeline_set();

            #(#items)*

            let pipeline_set = ::gotham::pipeline::set::finalize_pipeline_set(pipelines);
        }
    } else if cfg!(feature = "router-tide") {
        quote! {
            #(#items)*
        }
    } else {
        quote! {}
    }
}
