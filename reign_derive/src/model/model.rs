use crate::{model::attr::Attr, INTERNAL_ERR};

use inflector::{cases::snakecase::to_snake_case, string::pluralize::to_plural};
use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
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
    // TODO:(model) generics
    let gen_query = model.gen_query();
    let gen_tags = model.gen_tags();

    quote! {
        #gen_query
        #(#gen_tags)*
    }
}

#[derive(Clone)]
pub struct ModelField {
    pub field: Field,
    pub attrs: Vec<Attr>,
    pub column_ident: Ident,
}

impl ModelField {
    fn new(field: &Field) -> Self {
        let attrs = Attr::parse_attributes(&field.attrs, false);
        let mut column_ident = field.ident.as_ref().expect(INTERNAL_ERR).clone();

        for attr in &attrs {
            match attr {
                Attr::ColumnName(_, value) => column_ident = value.clone(),
                _ => {}
            }
        }

        Self {
            field: field.to_owned(),
            attrs,
            column_ident,
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

        for attr in &attrs {
            match attr {
                Attr::TableName(_, value) => table_ident = value.clone(),
                _ => {}
            }
        }

        Self {
            vis: vis.clone(),
            ident: ident.clone(),
            attrs,
            fields: fields.iter().map(ModelField::new).collect(),
            table_ident,
        }
    }

    pub fn schema(&self) -> TokenStream {
        quote! {
            crate::schema
        }
    }

    pub fn db(&self) -> TokenStream {
        quote! {
            ::reign::model::DatabasePlugin::get()
        }
    }

    pub fn backend(&self) -> TokenStream {
        quote! {
            ::reign::model::diesel::pg::Pg
        }
    }
}
