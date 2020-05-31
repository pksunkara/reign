use super::super::consts::*;
use super::{Code, Error, Parse, ParseStream, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::LitStr;

#[derive(Debug)]
pub struct DynamicAttribute {
    pub symbol: String,
    pub prefix: String,
    pub name: Code,
    pub suffix: String,
    pub value: Code,
}

impl Parse for DynamicAttribute {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        Ok(DynamicAttribute {
            symbol: input.step(":")?,
            prefix: input.matched(DY_ATTR_NAME_PART)?,
            name: {
                let name = input.capture(DY_ATTR_EXPR, 1)?;
                Code::parse_expr_from_str(input, &name)?
            },
            suffix: input.matched(DY_ATTR_NAME_PART)?,
            value: Code::parse_expr(input)?,
        })
    }
}

impl Tokenize for DynamicAttribute {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        let prefix = LitStr::new(&self.prefix, Span::call_site());
        let suffix = LitStr::new(&self.suffix, Span::call_site());
        let mut name = TokenStream::new();
        let mut value = TokenStream::new();

        self.name.tokenize(&mut name, idents, scopes);
        self.value.tokenize(&mut value, idents, scopes);

        // TODO:(view:html-escape) value
        tokens.append_all(quote! {
            write!(f, " {}{}{}=\"{}\"", #prefix, #name, #suffix, #value)?;
        });
    }
}
