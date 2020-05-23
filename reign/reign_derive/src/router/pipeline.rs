use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::Comma,
    Expr, Ident,
};

pub struct Pipeline {
    name: Ident,
    middlewares: Punctuated<Expr, Comma>,
}

impl Parse for Pipeline {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Pipeline {
            name: input.parse()?,
            middlewares: {
                let content;

                input.parse::<Comma>()?;
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

pub fn pipeline(input: Pipeline) -> TokenStream {
    quote!(#input)
}
