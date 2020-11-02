use proc_macro_error::{abort, ResultExt};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Comma, Eq},
    Attribute, Ident,
};

#[derive(Clone)]
pub enum Attr {
    NoInsert(Ident),
    NoUpdate(Ident),
    Tag(Ident, Punctuated<Ident, Comma>),
    ColumnName(Ident, Ident),
    TableName(Ident, Ident),
    PrimaryKey(Ident, Punctuated<Ident, Comma>),
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        match name.to_string().as_str() {
            "no_insert" => Ok(Attr::NoInsert(name)),
            "no_update" => Ok(Attr::NoUpdate(name)),
            "tag" => Ok(Attr::Tag(name, parenthesized_list(input)?)),
            "column_name" => Ok(Attr::ColumnName(name, eq(input)?)),
            "table_name" => Ok(Attr::TableName(name, eq(input)?)),
            "primary_key" => Ok(Attr::PrimaryKey(name, parenthesized_list(input)?)),
            _ => abort!(name, "unexpected attribute: {}", name),
        }
    }
}

impl Attr {
    pub fn parse_attributes(attrs: &[Attribute], for_struct: bool) -> Vec<Self> {
        let attrs = attrs
            .iter()
            .filter(|attr| attr.path.is_ident("model"))
            .flat_map(|attr| {
                attr.parse_args_with(Punctuated::<Attr, Comma>::parse_terminated)
                    .unwrap_or_abort()
            })
            .collect::<Vec<_>>();

        for attr in &attrs {
            match attr {
                Attr::NoInsert(ident) if for_struct => {
                    abort!(ident, "`no_insert` is not allowed on struct")
                }
                Attr::NoUpdate(ident) if for_struct => {
                    abort!(ident, "`no_update` is not allowed on struct")
                }
                Attr::Tag(ident, _) if for_struct => {
                    abort!(ident, "`tag` is not allowed on struct")
                }
                Attr::ColumnName(ident, _) if for_struct => {
                    abort!(ident, "`column_name` is not allowed on struct")
                }
                Attr::TableName(ident, _) if !for_struct => {
                    abort!(ident, "`table_name` is not allowed on field")
                }
                Attr::PrimaryKey(ident, _) if !for_struct => {
                    abort!(ident, "`primary_key` is not allowed on field")
                }
                _ => {}
            }
        }

        attrs
    }
}

fn eq<T: Parse>(input: ParseStream) -> Result<T> {
    input.parse::<Eq>()?;
    input.parse()
}

fn parenthesized_list<T: Parse>(input: ParseStream) -> Result<Punctuated<T, Comma>> {
    let content;
    parenthesized!(content in input);

    content.parse_terminated(T::parse)
}
