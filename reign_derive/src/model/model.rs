use crate::{model::attr::Attr, INTERNAL_ERR};

use inflector::{cases::snakecase::to_snake_case, string::pluralize::to_plural};
use proc_macro2::{Span, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Data, DataStruct, DeriveInput, Field, Fields,
    Ident, Visibility,
};

pub fn model(input: DeriveInput) -> TokenStream {
    let DeriveInput {
        data,
        vis,
        ident,
        attrs,
        ..
    } = &input;

    match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => gen_for_struct(Model::new(vis, ident, &fields.named, attrs)),
        _ => abort_call_site!("`#[derive(Model)]` only supports non-unit and non-tuple structs"),
    }
}

fn gen_for_struct(model: Model) -> TokenStream {
    // TODO: model: generics
    let gen_id = model.gen_id();
    let gen_filterable = model.gen_filterable();
    let gen_selectable = model.gen_selectable();
    let gen_insertable = model.gen_insertable();
    let gen_updateable = model.gen_updateable();
    let gen_deleteable = model.gen_deleteable();
    let gen_tags = model.gen_tags();

    quote! {
        #gen_id
        #gen_filterable
        #gen_selectable
        #gen_insertable
        #gen_updateable
        #gen_deleteable
        #(#gen_tags)*
    }
}

#[derive(Clone)]
pub struct ModelField {
    pub field: Field,
    pub attrs: Vec<Attr>,
    pub column_ident: Ident,
    pub no_write: bool,
    pub primary_key: bool,
    pub tags: Vec<Ident>,
}

impl ModelField {
    fn new(field: &Field, primary_keys: &[Ident]) -> Self {
        let attrs = Attr::parse_attributes(&field.attrs, false);

        let mut column_ident = field.ident.as_ref().expect(INTERNAL_ERR).clone();
        let mut no_write = false;
        let mut tags = vec![];

        for attr in &attrs {
            match attr {
                Attr::ColumnName(_, value) => column_ident = value.clone(),
                Attr::NoWrite(_) => no_write = true,
                Attr::Tag(_, value) => value.iter().for_each(|i| tags.push(i.clone())),
                _ => {}
            }
        }

        let primary_key = (primary_keys.is_empty() && column_ident == "id")
            || primary_keys.iter().find(|x| **x == column_ident).is_some();

        Self {
            field: field.to_owned(),
            attrs,
            column_ident,
            no_write,
            primary_key,
            tags,
        }
    }
}

#[derive(Clone)]
pub struct Model {
    pub vis: Visibility,
    pub ident: Ident,
    pub attrs: Vec<Attr>,
    pub fields: Vec<ModelField>,
    pub table_ident: Ident,
    pub primary_keys_size: usize,
}

impl Model {
    fn new(
        vis: &Visibility,
        ident: &Ident,
        fields: &Punctuated<Field, Comma>,
        attrs: &[Attribute],
    ) -> Self {
        let attrs = Attr::parse_attributes(attrs, true);

        let mut table_ident =
            Ident::new(&to_plural(&to_snake_case(&ident.to_string())), ident.span());
        let mut primary_keys = vec![];

        for attr in &attrs {
            match attr {
                Attr::TableName(_, value) => table_ident = value.clone(),
                Attr::PrimaryKey(_, value) => primary_keys = value.iter().cloned().collect(),
                _ => {}
            }
        }

        let fields = fields
            .into_iter()
            .map(|f| ModelField::new(f, &primary_keys))
            .collect::<Vec<_>>();

        if primary_keys.is_empty() {
            primary_keys.push(Ident::new("id", Span::call_site()));
        }

        // Check if primary key ident isn't a column
        for key in &primary_keys {
            if fields.iter().find(|x| x.column_ident == *key).is_none() {
                abort!(
                    key,
                    format!("unable to find column `{}` used for primary key", key)
                );
            }
        }

        Self {
            vis: vis.clone(),
            ident: ident.clone(),
            attrs,
            fields,
            table_ident,
            primary_keys_size: primary_keys.len(),
        }
    }

    pub fn schema(&self) -> TokenStream {
        quote! {
            crate::schema
        }
    }

    pub fn db(&self) -> TokenStream {
        quote! {
            ::reign::model::Database::get()
        }
    }

    pub fn backend(&self) -> TokenStream {
        quote! {
            ::reign::model::diesel::pg::Pg
        }
    }
}
