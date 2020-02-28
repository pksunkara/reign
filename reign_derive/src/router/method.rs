use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Comma,
    LitStr, Path,
};

pub struct Method {
    path: LitStr,
    action: Path,
}

impl Parse for Method {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Method {
            path: input.parse()?,
            action: {
                input.parse::<Comma>()?;
                input.parse()?
            },
        })
    }
}

pub fn get(input: Method) -> TokenStream {
    let Method { path, action } = input;

    if cfg!(feature = "router-actix") {
        quote! {
            app = app.route(#path, ::actix_web::web::get().to(#action))
        }
    } else if cfg!(feature = "router-gotham") {
        quote! {
            route.get(#path).to(#action)
        }
    } else if cfg!(feature = "router-tide") {
        quote! {
            app.at(#path).get(#action)
        }
    } else {
        quote! {}
    }
}

pub fn post(input: Method) -> TokenStream {
    let Method { path, action } = input;

    if cfg!(feature = "router-actix") {
        quote! {
            app = app.route(#path, ::actix_web::web::post().to(#action))
        }
    } else if cfg!(feature = "router-gotham") {
        quote! {
            route.post(#path).to(#action)
        }
    } else if cfg!(feature = "router-tide") {
        quote! {
            app.at(#path).post(#action)
        }
    } else {
        quote! {}
    }
}
