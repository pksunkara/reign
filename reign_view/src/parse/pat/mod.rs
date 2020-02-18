use super::{expr::expr_no_struct, Expr, Tokenize, ViewFields};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Brace, Comma, Dot2, In, Paren},
    Ident, PatRest, PatWild, Path, Token,
};

mod field_pat;
mod ident;
mod reference;
mod struct_;
mod tuple;
mod tuple_struct;

use field_pat::FieldPat;
use ident::PatIdent;
use reference::PatReference;
use struct_::PatStruct;
use tuple::PatTuple;
use tuple_struct::PatTupleStruct;

// TODO: Slice, Range
pub enum Pat {
    Ident(PatIdent),
    Reference(PatReference),
    Rest(PatRest),
    Struct(PatStruct),
    Tuple(PatTuple),
    TupleStruct(PatTupleStruct),
    Wild(PatWild),
}

impl Parse for Pat {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident)
            && ({ input.peek2(Token![::]) || input.peek2(Brace) || input.peek2(Paren) })
            || input.peek(Token![self]) && input.peek2(Token![::])
            || lookahead.peek(Token![::])
            || lookahead.peek(Token![<])
            || input.peek(Token![Self])
            || input.peek(Token![super])
            || input.peek(Token![extern])
            || input.peek(Token![crate])
        {
            pat_struct_or_tuple_struct(input)
        } else if lookahead.peek(Token![_]) {
            Ok(Pat::Wild(PatWild {
                attrs: Vec::new(),
                underscore_token: input.parse()?,
            }))
        } else if lookahead.peek(Token![&]) {
            Ok(Pat::Reference(input.parse()?))
        } else if lookahead.peek(Paren) {
            Ok(Pat::Tuple(input.parse()?))
        } else if lookahead.peek(Token![ref]) || input.peek(Token![self]) || input.peek(Ident) {
            Ok(Pat::Ident(input.parse()?))
        } else if lookahead.peek(Dot2) && !input.peek(Token![...]) {
            Ok(Pat::Rest(PatRest {
                attrs: Vec::new(),
                dot2_token: input.parse()?,
            }))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Tokenize for Pat {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        match self {
            Pat::Ident(p) => p.tokenize(tokens, idents, scopes),
            Pat::Reference(p) => p.tokenize(tokens, idents, scopes),
            Pat::Rest(p) => p.to_tokens(tokens),
            Pat::Struct(p) => p.tokenize(tokens, idents, scopes),
            Pat::TupleStruct(p) => p.tokenize(tokens, idents, scopes),
            Pat::Tuple(p) => p.tokenize(tokens, idents, scopes),
            Pat::Wild(p) => p.to_tokens(tokens),
        };
    }
}

pub struct For {
    pub pat: Pat,
    pub in_token: In,
    pub expr: Box<Expr>,
}

impl For {
    pub fn declared(&self) -> ViewFields {
        let mut declared = ViewFields::new();
        let mut tokens = TokenStream::new();
        let scopes = ViewFields::new();

        self.pat.tokenize(&mut tokens, &mut declared, &scopes);
        declared
    }
}

impl Parse for For {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(For {
            pat: input.parse()?,
            in_token: input.parse()?,
            expr: Box::new(input.call(expr_no_struct)?),
        })
    }
}

impl Tokenize for For {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        let mut declared = ViewFields::new();

        self.pat.tokenize(tokens, &mut declared, scopes);
        self.in_token.to_tokens(tokens);
        self.expr.tokenize(tokens, idents, scopes);
    }
}

// The following code is copied and modified from syn

fn pat_struct_or_tuple_struct(input: ParseStream) -> Result<Pat> {
    let path: Path = input.parse()?;

    if input.peek(Brace) {
        pat_struct(input, path).map(Pat::Struct)
    } else if input.peek(Paren) {
        pat_tuple_struct(input, path).map(Pat::TupleStruct)
    } else {
        Err(input.error("expected struct or tuple struct"))
    }
}

fn pat_struct(input: ParseStream, path: Path) -> Result<PatStruct> {
    let content;
    let brace_token = braced!(content in input);
    let mut fields = Punctuated::new();

    while !content.is_empty() && !content.peek(Dot2) {
        fields.push_value(content.parse()?);

        if !content.peek(Comma) {
            break;
        }

        let punct: Comma = content.parse()?;
        fields.push_punct(punct);
    }

    let dot2_token = if fields.empty_or_trailing() && content.peek(Dot2) {
        Some(content.parse()?)
    } else {
        None
    };

    Ok(PatStruct {
        path,
        brace_token,
        fields,
        dot2_token,
    })
}

fn pat_tuple_struct(input: ParseStream, path: Path) -> Result<PatTupleStruct> {
    Ok(PatTupleStruct {
        path,
        pat: input.parse()?,
    })
}
