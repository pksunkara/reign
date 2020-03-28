use inflector::cases::screamingsnakecase::to_screaming_snake_case;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::Comma,
    Ident, LitStr, Path,
};

pub struct Methods {
    methods: Punctuated<Ident, Comma>,
    path: LitStr,
    action: Path,
}

impl Parse for Methods {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;

        Ok(Methods {
            methods: {
                bracketed!(content in input);
                content.parse_terminated(|i| i.parse::<Ident>())?
                //TODO:(router) Unallowed methods
            },
            path: {
                input.parse::<Comma>()?;
                input.parse()?
            },
            action: {
                input.parse::<Comma>()?;
                input.parse()?
            },
        })
    }
}

pub fn methods(input: Methods) -> TokenStream {
    let Methods {
        methods,
        path,
        action,
    } = input;

    if cfg!(feature = "router-actix") {
        let methods = methods.iter().map(|i| i);

        quote! {
            app = app
                #(.route(#path, ::actix_web::web::#methods().to(#action)))*
        }
    } else if cfg!(feature = "router-gotham") {
        let methods = methods
            .iter()
            .map(|i| Ident::new(&to_screaming_snake_case(&i.to_string()), i.span()));

        quote! {
            route.request(vec![#(::gotham::hyper::Method::#methods),*], #path).to(#action)
        }
    } else if cfg!(feature = "router-tide") {
        let methods = methods.iter().map(|i| i);

        quote! {
            app.at(#path)
                #(.#methods(#action))*
        }
    } else {
        quote! {}
    }
}

pub fn get(input: TokenStream) -> TokenStream {
    quote! {
        methods!([get], #input)
    }
}

pub fn post(input: TokenStream) -> TokenStream {
    quote! {
        methods!([post], #input)
    }
}

pub fn put(input: TokenStream) -> TokenStream {
    quote! {
        methods!([put], #input)
    }
}

pub fn patch(input: TokenStream) -> TokenStream {
    quote! {
        methods!([patch], #input)
    }
}

pub fn delete(input: TokenStream) -> TokenStream {
    quote! {
        methods!([delete], #input)
    }
}
