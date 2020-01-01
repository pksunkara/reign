use proc_macro2::{Span, TokenStream};
use quote::quote;

use syn::{
    parse::{Parse, ParseStream, Result},
    token::Comma,
    ExprStruct, Ident,
};

pub(super) struct Render {
    template: ExprStruct,
    layout: Option<Ident>,
}

impl Parse for Render {
    fn parse(input: ParseStream) -> Result<Self> {
        let template: ExprStruct = input.parse()?;
        let comma: Option<Comma> = input.parse()?;

        let layout: Option<Ident> = if comma.is_some() {
            input.parse()?
        } else {
            None
        };

        Ok(Render { template, layout })
    }
}

// TODO: Capture local variables unhygienically and send them to templates
pub(super) fn render(input: Render) -> TokenStream {
    let template = input.template;
    let mut layout = input.layout;

    if layout.is_none() {
        layout = Some(Ident::new("Application", Span::call_site()));
    }

    quote! {
        ::reign::view::render(state, #template, crate::layouts::#layout::default())
    }
}
