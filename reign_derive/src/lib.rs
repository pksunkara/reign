#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_derive/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::parse_macro_input;

#[cfg(feature = "framework")]
mod framework;
#[cfg(feature = "model-postgres")]
mod model;
#[cfg(feature = "router")]
mod router;
#[cfg(feature = "view")]
mod view;

#[cfg(feature = "view")]
mod utils;

pub(crate) const INTERNAL_ERR: &str =
    "Internal error on reign_derive. Please create an issue on https://github.com/pksunkara/reign";

/// Auto load the views from the given directory.
///
/// Folder names should start with an alphabet and end with alphanumeric
/// with underscores being allowed in the middle.
///
/// File names should start with an alphabet and end with alphanumeric
/// with underscores being allowed in the middle. The only allowed
/// extension is `.html`.
///
/// Ignores the other files and folders which do not adhere the above rules.
///
/// Both the folder and file names are converted to lower case before
/// building the template.
///
/// # Examples
///
/// ```ignore
/// use reign::prelude::*;
///
/// views!("src", "views");
/// ```
#[cfg(feature = "view")]
#[proc_macro]
pub fn views(input: TokenStream) -> TokenStream {
    let input: view::render::Views = parse_macro_input!(input);

    view::render::views(input).into()
}

/// Shorthand notation for rendering a view.
///
/// # Examples
///
/// Render the given view
///
/// ```ignore
/// use reign::prelude::*;
///
/// render!(pages::home)
/// ```
///
/// You can also specify a status code
///
/// ```ignore
/// use reign::prelude::*;
///
/// render!(pages::home, status = 201)
/// ```
#[cfg(feature = "view")]
#[proc_macro]
#[proc_macro_error]
pub fn render(input: TokenStream) -> TokenStream {
    let input: view::render::Render = parse_macro_input!(input);

    view::render::render(input).into()
}

/// Helper for using path params in a [reign_router] handle.
///
/// # Examples
///
/// ```
/// use reign::prelude::*;
///
/// #[params]
/// async fn name(req: &mut Request, id: String) -> Result<impl Response, Error> {
///     Ok(id)
/// }
/// ```
#[cfg(feature = "router")]
#[proc_macro_attribute]
#[proc_macro_error]
pub fn params(_: TokenStream, input: TokenStream) -> TokenStream {
    let input: syn::ItemFn = parse_macro_input!(input);

    router::params::params(input).into()
}

/// Helper for defining a [reign_router] Path.
///
/// # Examples
///
/// ```
/// use reign::{
///     prelude::*,
///     router::{Router}
/// };
/// #
/// # async fn foobar(req: &mut Request) -> Result<impl Response, Error> { Ok("foobar") }
/// #
/// # async fn number(req: &mut Request) -> Result<impl Response, Error> { Ok("number") }
/// #
/// # async fn tree(req: &mut Request) -> Result<impl Response, Error> { Ok("tree") }
///
/// fn router(r: &mut Router) {
///     // Required param
///     r.get(p!("foo" / id / "bar"), foobar);
///
///     // Optional param
///     r.get(p!("foo" / id?), foobar);
///
///     // Regex param
///     r.get(p!("number" / id @ "[0-9]+"), number);
///
///     // Optional Regex param
///     r.get(p!("number" / id? @ "[0-9]+"), number);
///
///     // Glob param
///     r.get(p!("tree" / id*), tree);
///
///     // Optional Glob param
///     r.get(p!("tree" / id*?), tree);
/// }
/// ```
// TODO: derive: Maybe we don't need a proc macro here and use `macro_rules`
#[cfg(feature = "router")]
#[proc_macro]
#[proc_macro_error]
pub fn p(input: TokenStream) -> TokenStream {
    let input: router::path::Path = parse_macro_input!(input);

    router::path::path(input).into()
}

#[cfg(feature = "framework")]
#[proc_macro_derive(Config)]
#[proc_macro_error]
pub fn config(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = parse_macro_input!(input);

    framework::config::config(input).into()
}

#[cfg(feature = "model-postgres")]
#[proc_macro_derive(Model, attributes(model))]
#[proc_macro_error]
pub fn model(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = parse_macro_input!(input);

    model::model::model(input).into()
}
