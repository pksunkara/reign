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

    quote! {
        route.get(#path).to(#action);
    }
}

pub fn post(input: Method) -> TokenStream {
    let Method { path, action } = input;

    quote! {
        route.post(#path).to(#action);
    }
}
