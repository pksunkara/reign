use super::{Code, Error, ParseStream, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use regex::Regex;
use syn::LitStr;

#[derive(Debug)]
pub enum StringPart {
    Normal(String),
    Expr(Code),
}

impl StringPart {
    pub fn parse(input: &mut ParseStream, data: &str, in_attr: bool) -> Result<Vec<Self>, Error> {
        let mut parts = vec![];
        let start_regex = Regex::new(r"\\\{\{|\{\{|<").unwrap();
        let mut cursor = if !in_attr { input.cursor } else { 0 };

        loop {
            let remaining = data.get(cursor..).unwrap();

            if remaining.is_empty() {
                break;
            }

            let matches = start_regex.find(remaining);

            if matches.is_none() {
                parts.push(StringPart::Normal(remaining.to_string()));
                cursor += remaining.len();
                break;
            }

            let until = cursor + matches.unwrap().start();
            let sub_string = data.get(cursor..until).unwrap();

            if !sub_string.is_empty() {
                parts.push(StringPart::Normal(sub_string.to_string()));
                cursor = until;
            }

            match data.get(cursor..=cursor).unwrap() {
                "\\" => {
                    parts.push(StringPart::Normal("\\{{".to_string()));
                    cursor += 3;
                }
                "<" => {
                    if in_attr {
                        parts.push(StringPart::Normal("<".to_string()));
                        cursor += 1;
                    } else {
                        break;
                    }
                }
                "{" => {
                    cursor += 2;
                    let end_remaining = data.get(cursor..).unwrap();
                    let end_matches = end_remaining.find("}}");

                    if end_matches.is_none() {
                        if !in_attr {
                            input.cursor = cursor;
                        }

                        return Err(input.error("expression incomplete"));
                    }

                    let expr_until = cursor + end_matches.unwrap();
                    let expr_string = data.get(cursor..expr_until).unwrap();

                    parts.push(StringPart::Expr(Code::parse_expr_from_str(
                        input,
                        expr_string,
                    )?));
                    cursor = expr_until + 2;
                }
                _ => unreachable!(),
            }
        }

        if !in_attr {
            input.cursor = cursor;
        }

        Ok(parts)
    }
}

impl Tokenize for StringPart {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        match self {
            StringPart::Normal(n) => {
                let lit = LitStr::new(&n, Span::call_site());
                lit.to_tokens(tokens);
            }
            // TODO:(view:html-escape) expression
            StringPart::Expr(e) => e.tokenize(tokens, idents, scopes),
        }
    }
}

impl Tokenize for Vec<StringPart> {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        let format_arg_str = "{}".repeat(self.len());
        let format_arg_lit = LitStr::new(&format_arg_str, Span::call_site());

        let content: Vec<TokenStream> = self
            .iter()
            .map(|x| {
                let mut ts = TokenStream::new();

                x.tokenize(&mut ts, idents, scopes);
                ts
            })
            .collect();

        tokens.append_all(quote! {
            #format_arg_lit, #(#content),*
        });
    }
}
