use proc_macro_error::{abort, ResultExt};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Comma, Eq},
    Attribute, Ident, LitStr,
};

pub enum ModelAttr {
    TableName(Ident, LitStr),
    PrimaryKey(Ident, Punctuated<Ident, Comma>),
    ColumnName(Ident, LitStr),
    Tag(Ident, Punctuated<Ident, Comma>),
}

impl Parse for ModelAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        match name_str.as_str() {
            "table_name" => {
                input.parse::<Eq>()?;
                Ok(ModelAttr::TableName(name, input.parse()?))
            }
            "primary_key" => {
                let content;
                parenthesized!(content in input);

                Ok(ModelAttr::PrimaryKey(
                    name,
                    content.parse_terminated(Ident::parse)?,
                ))
            }
            "column_name" => {
                input.parse::<Eq>()?;
                Ok(ModelAttr::ColumnName(name, input.parse()?))
            }
            "tag" => {
                let content;
                parenthesized!(content in input);

                Ok(ModelAttr::Tag(
                    name,
                    content.parse_terminated(Ident::parse)?,
                ))
            }
            _ => abort!(name, "unexpected attribute: {}", name_str),
        }
    }
}

impl ModelAttr {
    pub fn from_struct(attrs: &[Attribute]) -> Vec<Self> {
        let attrs = Self::parse_model_attributes(attrs);

        for attr in &attrs {
            match attr {
                ModelAttr::ColumnName(ident, _) => {
                    abort!(ident, "`column_name` is not allowed on struct")
                }
                ModelAttr::Tag(ident, _) => abort!(ident, "`tag` is not allowed on struct"),
                _ => {}
            }
        }

        attrs
    }

    pub fn from_field(attrs: &[Attribute]) -> Vec<Self> {
        let attrs = Self::parse_model_attributes(attrs);

        for attr in &attrs {
            match attr {
                ModelAttr::TableName(ident, _) => {
                    abort!(ident, "`table_name` is not allowed on fields")
                }
                ModelAttr::PrimaryKey(ident, _) => {
                    abort!(ident, "`primary_key` is not allowed on fields")
                }
                _ => {}
            }
        }

        attrs
    }

    fn parse_model_attributes(attrs: &[Attribute]) -> Vec<Self> {
        attrs
            .iter()
            .filter(|attr| attr.path.is_ident("model"))
            .flat_map(|attr| {
                attr.parse_args_with(Punctuated::<ModelAttr, Comma>::parse_terminated)
                    .unwrap_or_abort()
            })
            .collect()
    }
}
