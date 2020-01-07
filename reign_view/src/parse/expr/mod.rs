use super::Tokenize;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    braced, bracketed,
    group::parse_group,
    parenthesized,
    parse::{discouraged::Speculative, Parse, ParseStream, Result},
    punctuated::{Pair, Punctuated},
    token::{self, Brace, Bracket, Paren},
    BinOp, ExprMacro, ExprPath, GenericMethodArgument, Ident, Lit, Macro, MacroDelimiter, Member,
    MethodTurbofish, Path, PathArguments, RangeLimits, Token, Type,
};

mod array;
mod binary;
mod call;
mod cast;
mod field;
mod field_value;
mod group;
mod index;
mod method_call;
mod paren;
mod range;
mod repeat;
mod struct_;
mod tuple;
mod type_;
mod unary;

use array::ExprArray;
use binary::ExprBinary;
use call::ExprCall;
use cast::ExprCast;
use field::ExprField;
use field_value::FieldValue;
use group::ExprGroup;
use index::ExprIndex;
use method_call::ExprMethodCall;
use paren::ExprParen;
use range::ExprRange;
use repeat::ExprRepeat;
use struct_::ExprStruct;
use tuple::ExprTuple;
use type_::ExprType;
use unary::ExprUnary;

// TODO:(view:expr) ExprIf
pub enum Expr {
    Array(ExprArray),
    Binary(ExprBinary),
    Call(ExprCall),
    Cast(ExprCast),
    Field(ExprField),
    Group(ExprGroup),
    Index(ExprIndex),
    MethodCall(ExprMethodCall),
    Paren(ExprParen),
    Path(ExprPath),
    Range(ExprRange),
    Repeat(ExprRepeat),
    Struct(ExprStruct),
    Tuple(ExprTuple),
    Type(ExprType),
    Unary(ExprUnary),
    // TODO:(view:expr) Only allow select macros?
    Macro(ExprMacro),
    Lit(Lit),
}

// The following code is copied and modified from syn

#[derive(Copy, Clone)]
struct AllowStruct(bool);

#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum Precedence {
    Any,
    Assign,
    Range,
    Or,
    And,
    Compare,
    BitOr,
    BitXor,
    BitAnd,
    Shift,
    Arithmetic,
    Term,
    Cast,
}

impl Precedence {
    fn of(op: &BinOp) -> Self {
        match *op {
            BinOp::Add(_) | BinOp::Sub(_) => Precedence::Arithmetic,
            BinOp::Mul(_) | BinOp::Div(_) | BinOp::Rem(_) => Precedence::Term,
            BinOp::And(_) => Precedence::And,
            BinOp::Or(_) => Precedence::Or,
            BinOp::BitXor(_) => Precedence::BitXor,
            BinOp::BitAnd(_) => Precedence::BitAnd,
            BinOp::BitOr(_) => Precedence::BitOr,
            BinOp::Shl(_) | BinOp::Shr(_) => Precedence::Shift,
            BinOp::Eq(_)
            | BinOp::Lt(_)
            | BinOp::Le(_)
            | BinOp::Ne(_)
            | BinOp::Ge(_)
            | BinOp::Gt(_) => Precedence::Compare,
            BinOp::AddEq(_)
            | BinOp::SubEq(_)
            | BinOp::MulEq(_)
            | BinOp::DivEq(_)
            | BinOp::RemEq(_)
            | BinOp::BitXorEq(_)
            | BinOp::BitAndEq(_)
            | BinOp::BitOrEq(_)
            | BinOp::ShlEq(_)
            | BinOp::ShrEq(_) => Precedence::Assign,
        }
    }
}

fn peek_precedence(input: ParseStream) -> Precedence {
    if let Ok(op) = input.fork().parse() {
        Precedence::of(&op)
    } else if input.peek(Token![=]) && !input.peek(Token![=>]) {
        Precedence::Assign
    } else if input.peek(Token![..]) {
        Precedence::Range
    } else if input.peek(Token![as]) || input.peek(Token![:]) && !input.peek(Token![::]) {
        Precedence::Cast
    } else {
        Precedence::Any
    }
}

impl Parse for Expr {
    fn parse(input: ParseStream) -> Result<Self> {
        ambiguous_expr(input, AllowStruct(true))
    }
}

impl Tokenize for Expr {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>) {
        match self {
            Expr::Array(e) => e.tokenize(tokens, idents),
            Expr::Binary(e) => e.tokenize(tokens, idents),
            Expr::Call(e) => e.tokenize(tokens, idents),
            Expr::Cast(e) => e.tokenize(tokens, idents),
            Expr::Field(e) => e.tokenize(tokens, idents),
            Expr::Group(e) => e.tokenize(tokens, idents),
            Expr::Index(e) => e.tokenize(tokens, idents),
            Expr::MethodCall(e) => e.tokenize(tokens, idents),
            Expr::Paren(e) => e.tokenize(tokens, idents),
            Expr::Path(path) => {
                if let Some(ident) = path.path.get_ident() {
                    idents.push(ident.clone());
                    tokens.append_all(quote! {
                        self.#ident
                    });
                } else {
                    path.to_tokens(tokens);
                }
            }
            Expr::Range(e) => e.tokenize(tokens, idents),
            Expr::Repeat(e) => e.tokenize(tokens, idents),
            Expr::Struct(e) => e.tokenize(tokens, idents),
            Expr::Tuple(e) => e.tokenize(tokens, idents),
            Expr::Type(e) => e.tokenize(tokens, idents),
            Expr::Unary(e) => e.tokenize(tokens, idents),
            Expr::Macro(e) => e.to_tokens(tokens),
            Expr::Lit(e) => e.to_tokens(tokens),
        };
    }
}

impl<T, P> Tokenize for Punctuated<T, P>
where
    T: Tokenize,
    P: ToTokens,
{
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>) {
        let mut iter = self.pairs();

        loop {
            let item = iter.next();

            if item.is_none() {
                break;
            }

            match item.unwrap() {
                Pair::Punctuated(t, p) => {
                    t.tokenize(tokens, idents);
                    p.to_tokens(tokens);
                }
                Pair::End(t) => t.tokenize(tokens, idents),
            }
        }
    }
}

impl<T> Tokenize for Option<Box<T>>
where
    T: Tokenize,
{
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>) {
        if self.is_some() {
            self.as_ref().unwrap().tokenize(tokens, idents);
        }
    }
}

fn expr_group(input: ParseStream) -> Result<ExprGroup> {
    let group = parse_group(input)?;

    Ok(ExprGroup {
        group_token: group.token,
        expr: group.content.parse()?,
    })
}

pub(super) fn expr_no_struct(input: ParseStream) -> Result<Expr> {
    ambiguous_expr(input, AllowStruct(false))
}

fn ambiguous_expr(input: ParseStream, allow_struct: AllowStruct) -> Result<Expr> {
    let lhs = unary_expr(input, allow_struct)?;
    parse_expr(input, lhs, allow_struct, Precedence::Any)
}

fn unary_expr(input: ParseStream, allow_struct: AllowStruct) -> Result<Expr> {
    let ahead = input.fork();

    if ahead.peek(Token![*]) || ahead.peek(Token![!]) || ahead.peek(Token![-]) {
        input.advance_to(&ahead);

        Ok(Expr::Unary(ExprUnary {
            op: input.parse()?,
            expr: Box::new(unary_expr(input, allow_struct)?),
        }))
    } else {
        trailer_expr(input, allow_struct)
    }
}

fn parse_expr(
    input: ParseStream,
    mut lhs: Expr,
    allow_struct: AllowStruct,
    base: Precedence,
) -> Result<Expr> {
    loop {
        if input
            .fork()
            .parse::<BinOp>()
            .ok()
            .map_or(false, |op| Precedence::of(&op) >= base)
        {
            let op: BinOp = input.parse()?;
            let precedence = Precedence::of(&op);

            if precedence == Precedence::Assign {
                return Err(input.error("expected non-assignment expression"));
            }

            let mut rhs = unary_expr(input, allow_struct)?;

            loop {
                let next = peek_precedence(input);
                if next > precedence {
                    rhs = parse_expr(input, rhs, allow_struct, next)?;
                } else {
                    break;
                }
            }

            lhs = Expr::Binary(ExprBinary {
                left: Box::new(lhs),
                op,
                right: Box::new(rhs),
            });
        } else if Precedence::Assign >= base
            && input.peek(Token![=])
            && !input.peek(Token![==])
            && !input.peek(Token![=>])
        {
            return Err(input.error("expected non-assignment expression"));
        } else if Precedence::Range >= base && input.peek(Token![..]) {
            let limits: RangeLimits = input.parse()?;

            let rhs = if input.is_empty()
                || input.peek(Token![,])
                || input.peek(Token![;])
                || !allow_struct.0 && input.peek(token::Brace)
            {
                None
            } else {
                let mut rhs = unary_expr(input, allow_struct)?;
                loop {
                    let next = peek_precedence(input);
                    if next > Precedence::Range {
                        rhs = parse_expr(input, rhs, allow_struct, next)?;
                    } else {
                        break;
                    }
                }
                Some(rhs)
            };

            lhs = Expr::Range(ExprRange {
                from: Some(Box::new(lhs)),
                limits,
                to: rhs.map(Box::new),
            });
        } else if Precedence::Cast >= base && input.peek(Token![as]) {
            let as_token: Token![as] = input.parse()?;
            let ty = input.call(Type::without_plus)?;

            lhs = Expr::Cast(ExprCast {
                expr: Box::new(lhs),
                as_token,
                ty: Box::new(ty),
            });
        } else if Precedence::Cast >= base && input.peek(Token![:]) && !input.peek(Token![::]) {
            let colon_token: Token![:] = input.parse()?;
            let ty = input.call(Type::without_plus)?;

            lhs = Expr::Type(ExprType {
                expr: Box::new(lhs),
                colon_token,
                ty: Box::new(ty),
            });
        } else {
            break;
        }
    }

    Ok(lhs)
}

fn trailer_expr(input: ParseStream, allow_struct: AllowStruct) -> Result<Expr> {
    if input.peek(token::Group) {
        return input.call(expr_group).map(Expr::Group);
    }

    let atom = atom_expr(input, allow_struct)?;
    trailer_helper(input, atom)
}

fn generic_method_argument(input: ParseStream) -> Result<GenericMethodArgument> {
    input.parse().map(GenericMethodArgument::Type)
}

fn trailer_helper(input: ParseStream, mut e: Expr) -> Result<Expr> {
    loop {
        if input.peek(token::Paren) {
            let content;
            e = Expr::Call(ExprCall {
                func: Box::new(e),
                paren_token: parenthesized!(content in input),
                args: content.parse_terminated(Expr::parse)?,
            });
        } else if input.peek(Token![.]) && !input.peek(Token![..]) {
            let dot_token: Token![.] = input.parse()?;
            let member: Member = input.parse()?;

            let turbofish = if member.is_named() && input.peek(Token![::]) {
                Some(MethodTurbofish {
                    colon2_token: input.parse()?,
                    lt_token: input.parse()?,
                    args: {
                        let mut args = Punctuated::new();
                        loop {
                            if input.peek(Token![>]) {
                                break;
                            }

                            let value = input.call(generic_method_argument)?;
                            args.push_value(value);

                            if input.peek(Token![>]) {
                                break;
                            }

                            let punct = input.parse()?;
                            args.push_punct(punct);
                        }
                        args
                    },
                    gt_token: input.parse()?,
                })
            } else {
                None
            };

            if turbofish.is_some() || input.peek(token::Paren) {
                if let Member::Named(method) = member {
                    let content;

                    e = Expr::MethodCall(ExprMethodCall {
                        receiver: Box::new(e),
                        dot_token,
                        method,
                        turbofish,
                        paren_token: parenthesized!(content in input),
                        args: content.parse_terminated(Expr::parse)?,
                    });

                    continue;
                }
            }

            e = Expr::Field(ExprField {
                base: Box::new(e),
                dot_token,
                member,
            });
        } else if input.peek(token::Bracket) {
            let content;

            e = Expr::Index(ExprIndex {
                expr: Box::new(e),
                bracket_token: bracketed!(content in input),
                index: content.parse()?,
            });
        } else {
            break;
        }
    }

    Ok(e)
}

fn atom_expr(input: ParseStream, allow_struct: AllowStruct) -> Result<Expr> {
    if input.peek(token::Group) {
        input.call(expr_group).map(Expr::Group)
    } else if input.peek(Lit) {
        input.parse().map(Expr::Lit)
    } else if input.peek(Ident)
        || input.peek(Token![::])
        || input.peek(Token![<])
        || input.peek(Token![self])
        || input.peek(Token![Self])
        || input.peek(Token![super])
        || input.peek(Token![extern])
        || input.peek(Token![crate])
    {
        path_or_macro_or_struct(input, allow_struct)
    } else if input.peek(token::Paren) {
        paren_or_tuple(input)
    } else if input.peek(token::Bracket) {
        array_or_repeat(input)
    } else if input.peek(Token![..]) {
        expr_range(input, allow_struct).map(Expr::Range)
    } else {
        Err(input.error("expected expression"))
    }
}

pub fn parse_delimiter(input: ParseStream) -> Result<(MacroDelimiter, TokenStream)> {
    input.step(|cursor| {
        if let Some((TokenTree::Group(g), rest)) = cursor.token_tree() {
            let span = g.span();
            let delimiter = match g.delimiter() {
                Delimiter::Parenthesis => MacroDelimiter::Paren(Paren(span)),
                Delimiter::Brace => MacroDelimiter::Brace(Brace(span)),
                Delimiter::Bracket => MacroDelimiter::Bracket(Bracket(span)),
                Delimiter::None => {
                    return Err(cursor.error("expected delimiter"));
                }
            };
            Ok(((delimiter, g.stream()), rest))
        } else {
            Err(cursor.error("expected delimiter"))
        }
    })
}

fn path_or_macro_or_struct(input: ParseStream, allow_struct: AllowStruct) -> Result<Expr> {
    let expr: ExprPath = input.parse()?;

    if expr.qself.is_some() {
        return Ok(Expr::Path(expr));
    }

    if input.peek(Token![!]) && !input.peek(Token![!=]) {
        let mut contains_arguments = false;

        for segment in &expr.path.segments {
            match segment.arguments {
                PathArguments::None => {}
                PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                    contains_arguments = true;
                }
            }
        }

        if !contains_arguments {
            let bang_token: Token![!] = input.parse()?;
            let (delimiter, tokens) = parse_delimiter(input)?;

            return Ok(Expr::Macro(ExprMacro {
                attrs: Vec::new(),
                mac: Macro {
                    path: expr.path,
                    bang_token,
                    delimiter,
                    tokens,
                },
            }));
        }
    }

    if allow_struct.0 && input.peek(token::Brace) {
        expr_struct_helper(input, expr.path).map(Expr::Struct)
    } else {
        Ok(Expr::Path(expr))
    }
}

fn expr_struct_helper(input: ParseStream, path: Path) -> Result<ExprStruct> {
    let content;
    let brace_token = braced!(content in input);
    let mut fields = Punctuated::new();

    while !content.is_empty() {
        if content.peek(Token![..]) {
            return Ok(ExprStruct {
                brace_token,
                path,
                fields,
                dot2_token: Some(content.parse()?),
                rest: Some(Box::new(content.parse()?)),
            });
        }

        fields.push(content.parse()?);

        if !content.peek(Token![,]) {
            break;
        }

        let punct: Token![,] = content.parse()?;
        fields.push_punct(punct);
    }

    Ok(ExprStruct {
        brace_token,
        path,
        fields,
        dot2_token: None,
        rest: None,
    })
}

fn array_or_repeat(input: ParseStream) -> Result<Expr> {
    let content;
    let bracket_token = bracketed!(content in input);

    if content.is_empty() {
        return Ok(Expr::Array(ExprArray {
            bracket_token,
            elems: Punctuated::new(),
        }));
    }

    let first: Expr = content.parse()?;

    if content.is_empty() || content.peek(Token![,]) {
        let mut elems = Punctuated::new();
        elems.push_value(first);

        while !content.is_empty() {
            let punct = content.parse()?;
            elems.push_punct(punct);
            if content.is_empty() {
                break;
            }
            let value = content.parse()?;
            elems.push_value(value);
        }

        Ok(Expr::Array(ExprArray {
            bracket_token,
            elems,
        }))
    } else if content.peek(Token![;]) {
        let semi_token: Token![;] = content.parse()?;
        let len: Expr = content.parse()?;

        Ok(Expr::Repeat(ExprRepeat {
            bracket_token,
            expr: Box::new(first),
            semi_token,
            len: Box::new(len),
        }))
    } else {
        Err(content.error("expected `,` or `;`"))
    }
}

fn paren_or_tuple(input: ParseStream) -> Result<Expr> {
    let content;
    let paren_token = parenthesized!(content in input);

    if content.is_empty() {
        return Ok(Expr::Tuple(ExprTuple {
            paren_token,
            elems: Punctuated::new(),
        }));
    }

    let first: Expr = content.parse()?;

    if content.is_empty() {
        return Ok(Expr::Paren(ExprParen {
            paren_token,
            expr: Box::new(first),
        }));
    }

    let mut elems = Punctuated::new();
    elems.push_value(first);

    while !content.is_empty() {
        let punct = content.parse()?;
        elems.push_punct(punct);
        if content.is_empty() {
            break;
        }
        let value = content.parse()?;
        elems.push_value(value);
    }

    Ok(Expr::Tuple(ExprTuple { paren_token, elems }))
}

fn expr_range(input: ParseStream, allow_struct: AllowStruct) -> Result<ExprRange> {
    Ok(ExprRange {
        from: None,
        limits: input.parse()?,
        to: {
            if input.is_empty()
                || input.peek(Token![,])
                || input.peek(Token![;])
                || !allow_struct.0 && input.peek(token::Brace)
            {
                None
            } else {
                let to = ambiguous_expr(input, allow_struct)?;
                Some(Box::new(to))
            }
        },
    })
}
