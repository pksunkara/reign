use crate::utils::Options;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_str, Expr,
};

pub struct Json {
    expr: Expr,
    options: Options,
}

impl Parse for Json {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Json {
            expr: input.parse()?,
            options: input.parse()?,
        })
    }
}

pub fn json(mut input: Json) -> TokenStream {
    let expr = input.expr;
    let status: Expr = input
        .options
        .remove("status")
        .unwrap_or_else(|| parse_str("200").unwrap());

    if cfg!(feature = "views-actix") {
        quote! {
            ::reign::view::json_actix(#expr, #status)
        }
    } else if cfg!(feature = "views-gotham") {
        quote! {
            ::reign::view::json_gotham(#expr, #status)
        }
    } else if cfg!(feature = "views-tide") {
        quote! {
            ::reign::view::json_tide(#expr, #status)
        }
    } else if cfg!(feature = "views-warp") {
        quote! {
            ::reign::view::json_warp(#expr, #status)
        }
    } else {
        quote! {}
    }
}
