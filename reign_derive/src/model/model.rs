use crate::{model::model_attr::ModelAttr, INTERNAL_ERR};
use inflector::{
    cases::{pascalcase::to_pascal_case, snakecase::to_snake_case},
    string::pluralize::to_plural,
};
use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::quote;
use std::collections::HashMap as Map;
use syn::{
    punctuated::Punctuated, token::Comma, Attribute, Data, DataStruct, DeriveInput, Field, Fields,
    Ident, Visibility,
};

pub fn model(input: DeriveInput) -> TokenStream {
    quote! {
        #[derive(::reign::prelude::ModelHidden, ::diesel::Queryable)]
        #input
    }
}

pub fn model_hidden(input: DeriveInput) -> TokenStream {
    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => gen_for_struct(Model::new(
            &input.vis,
            &input.ident,
            &fields.named,
            &input.attrs,
        )),
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => gen_for_struct(Model::new(
            &input.vis,
            &input.ident,
            &Punctuated::<Field, Comma>::new(),
            &input.attrs,
        )),
        _ => abort_call_site!("`#[model]` only supports non-tuple structs"),
    }
}

fn gen_for_struct(model: Model) -> TokenStream {
    // TODO:(model) generics
    let fields = model.fields.iter().map(|x| &x.0).collect::<Vec<_>>();

    let gen_query = model.gen_query();
    let gen_query_new = model.gen_query_new();
    let gen_query_filters = model.gen_query_filters();
    let gen_query_limit_offset = model.gen_query_limit_offset();
    let gen_methods = model.gen_methods(None);
    let gen_loader = model.gen_loader(None, &fields);
    let gen_tags = model.gen_tags();

    quote! {
        #gen_query
        #gen_query_new
        #gen_query_limit_offset
        #gen_query_filters
        #gen_methods
        #gen_loader
        #(#gen_tags)*
    }
}

struct Model {
    vis: Visibility,
    ident: Ident,
    _attrs: Vec<ModelAttr>,
    fields: Vec<(Field, Vec<ModelAttr>)>,
}

impl Model {
    fn new(
        vis: &Visibility,
        ident: &Ident,
        fields: &Punctuated<Field, Comma>,
        attrs: &[Attribute],
    ) -> Self {
        Self {
            vis: vis.clone(),
            ident: ident.clone(),
            _attrs: ModelAttr::from_struct(attrs),
            fields: fields
                .iter()
                .map(|x| (x.to_owned(), ModelAttr::from_field(&x.attrs)))
                .collect(),
        }
    }

    fn query_ident(&self) -> Ident {
        Ident::new(&format!("Query{}", self.ident), self.ident.span())
    }

    fn table_ident(&self) -> Ident {
        Ident::new(
            &to_plural(&to_snake_case(&self.ident.to_string())),
            self.ident.span(),
        )
    }

    fn tag_ident(&self, tag: &str) -> Ident {
        Ident::new(&format!("{}{}", self.ident, tag), self.ident.span())
    }

    fn gen_query(&self) -> TokenStream {
        let query_ident = self.query_ident();
        let vis = &self.vis;

        quote! {
            #vis struct #query_ident<T, M, SS> {
                _phantom: std::marker::PhantomData<(T, M)>,
                limit: Option<i64>,
                offset: Option<i64>,
                statement: SS,
            }
        }
    }

    fn gen_query_new(&self) -> TokenStream {
        let table_ident = self.table_ident();
        let query_ident = self.query_ident();

        quote! {
            impl<T, M> #query_ident<T, M, ::diesel::query_builder::SelectStatement<schema::#table_ident::table>> {
                fn new() -> Self {
                    use ::diesel::associations::HasTable;
                    use ::diesel::query_builder::AsQuery;

                    Self {
                        _phantom: std::marker::PhantomData,
                        limit: None,
                        offset: None,
                        statement: schema::#table_ident::table::table().as_query(),
                    }
                }
            }
        }
    }

    // TODO:(model) custom filter by just forwarding to `filter`
    fn gen_query_filters(&self) -> TokenStream {
        let table_ident = self.table_ident();
        let query_ident = self.query_ident();

        let (field_vis, field_idents) = self
            .fields
            .iter()
            .map(|x| (&x.0.vis, x.0.ident.as_ref().expect(INTERNAL_ERR)))
            .unzip::<_, _, Vec<_>, Vec<_>>();

        quote! {
            impl<T, M, SS> #query_ident<T, M, SS> {
                #(#field_vis fn #field_idents<E, X, O>(self, #field_idents: E) -> #query_ident<T, M, O>
                where
                    SS: ::diesel::query_dsl::filter_dsl::FilterDsl<
                        ::diesel::expression::operators::Eq<schema::#table_ident::#field_idents, X>,
                        Output = O,
                    >,
                    E: ::diesel::expression::AsExpression<
                        ::diesel::dsl::SqlTypeOf<schema::#table_ident::#field_idents>,
                        Expression = X,
                    >,
                    X: ::diesel::expression::Expression<SqlType = ::diesel::dsl::SqlTypeOf<schema::#table_ident::#field_idents>>,
                {
                    #query_ident {
                        _phantom: self._phantom,
                        limit: self.limit,
                        offset: self.offset,
                        statement: self.statement.filter(schema::#table_ident::#field_idents.eq(#field_idents)),
                    }
                })*
            }
        }
    }

    fn gen_query_limit_offset(&self) -> TokenStream {
        let query_ident = self.query_ident();
        let vis = &self.vis;

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl<M, SS> #query_ident<::reign::model::query::All, M, SS> {
                #vis fn limit(mut self, limit: i64) -> Self {
                    self.limit = Some(limit);
                    self
                }

                #vis fn offset(mut self, offset: i64) -> Self {
                    self.offset = Some(offset);
                    self
                }
            }
        }
    }

    fn gen_struct(&self, tag: &str, fields: &[&Field]) -> TokenStream {
        let ident = self.tag_ident(tag);
        let vis = &self.vis;

        // TODO:(model) Forward attrs for fields and derives/attrs for struct
        let fields = fields
            .iter()
            .map(|f| {
                let Field { vis, ident, ty, .. } = f;

                quote! {
                    #vis #ident: #ty
                }
            })
            .collect::<Vec<_>>();

        quote! {
            #[derive(::diesel::Queryable)]
            #vis struct #ident {
                #(#fields),*
            }
        }
    }

    fn gen_methods(&self, tag: Option<&str>) -> TokenStream {
        let query_ident = self.query_ident();
        let table_ident = self.table_ident();
        let vis = &self.vis;

        let ident = if let Some(tag) = tag {
            self.tag_ident(tag)
        } else {
            self.ident.clone()
        };

        quote! {
            #[allow(dead_code, unreachable_code)]
            impl #ident {
                #vis fn all() -> #query_ident<
                    ::reign::model::query::All,
                    #ident,
                    ::diesel::query_builder::SelectStatement<schema::#table_ident::table>
                > {
                    #query_ident::new()
                }

                #vis fn one() -> #query_ident<
                    ::reign::model::query::One,
                    #ident,
                    ::diesel::query_builder::SelectStatement<schema::#table_ident::table>
                > {
                    #query_ident::new()
                }
            }
        }
    }

    fn gen_loader(&self, tag: Option<&str>, fields: &[&Field]) -> TokenStream {
        let query_ident = self.query_ident();
        let table_ident = self.table_ident();
        let vis = &self.vis;

        let ident = if let Some(tag) = tag {
            self.tag_ident(tag)
        } else {
            self.ident.clone()
        };

        let field_idents = fields
            .iter()
            .map(|x| x.ident.as_ref().expect(INTERNAL_ERR))
            .collect::<Vec<_>>();

        quote! {
            impl<S, W>
                #query_ident<
                    ::reign::model::query::All,
                    #ident,
                    ::diesel::query_builder::SelectStatement<
                        schema::#table_ident::table,
                        S,
                        ::diesel::query_builder::distinct_clause::NoDistinctClause,
                        W,
                    >,
                >
            where
                W: ::diesel::query_builder::QueryId
                    + ::diesel::query_builder::QueryFragment<::diesel::pg::Pg>
                    + ::diesel::query_builder::where_clause::ValidWhereClause<schema::#table_ident::table>
                    + Send
                    + 'static,
            {
                #vis async fn load(self) -> Result<Vec<#ident>, ::reign::model::tokio_diesel::AsyncError> {
                    use ::reign::model::tokio_diesel::AsyncRunQueryDsl;

                    let select = self.statement.select((
                        #(schema::#table_ident::#field_idents,)*
                    ));

                    if let Some(limit) = self.limit {
                        if let Some(offset) = self.offset {
                            select
                                .limit(limit)
                                .offset(offset)
                                .load_async::<#ident>(&DB)
                                .await
                        } else {
                            select.limit(limit).load_async::<#ident>(&DB).await
                        }
                    } else {
                        select.load_async::<#ident>(&DB).await
                    }
                }
            }

            impl<S, W>
                #query_ident<
                    ::reign::model::query::One,
                    #ident,
                    ::diesel::query_builder::SelectStatement<
                        schema::#table_ident::table,
                        S,
                        ::diesel::query_builder::distinct_clause::NoDistinctClause,
                        W,
                    >,
                >
            where
                W: ::diesel::query_builder::QueryId
                    + ::diesel::query_builder::QueryFragment<::diesel::pg::Pg>
                    + ::diesel::query_builder::where_clause::ValidWhereClause<schema::#table_ident::table>
                    + Send
                    + 'static,
            {
                #vis async fn load(self) -> Result<Option<#ident>, ::reign::model::tokio_diesel::AsyncError> {
                    use ::reign::model::tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};

                    let select = self.statement.select((
                        #(schema::#table_ident::#field_idents,)*
                    ));

                    select
                        .limit(1)
                        .get_result_async::<#ident>(&DB)
                        .await
                        .optional()
                }
            }
        }
    }

    fn gen_tags(&self) -> Vec<TokenStream> {
        let mut map = Map::<String, Vec<&Field>>::new();
        let mut ret = vec![];

        for (field, attrs) in &self.fields {
            for attr in attrs {
                if let ModelAttr::Tag(_, tags) = attr {
                    for tag in tags {
                        let tag = to_pascal_case(&tag.to_string());

                        if !map.contains_key(&tag) {
                            map.insert(tag.clone(), vec![]);
                        }

                        map.get_mut(&tag).expect(INTERNAL_ERR).push(field);
                    }
                }
            }
        }

        for (tag, fields) in map.iter() {
            let gen_struct = self.gen_struct(tag, fields);
            let gen_methods = self.gen_methods(Some(tag));
            let gen_loader = self.gen_loader(Some(tag), fields);

            ret.push(quote! {
                #gen_struct
                #gen_methods
                #gen_loader
            });
        }

        ret
    }
}
