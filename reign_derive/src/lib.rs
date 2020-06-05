#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_derive/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::parse_macro_input;

#[cfg(feature = "router-backend")]
mod router;
#[cfg(feature = "view")]
mod views;

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
    let input: views::render::Views = parse_macro_input!(input);

    views::render::views(input).into()
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
    let input: views::render::Render = parse_macro_input!(input);

    views::render::render(input).into()
}

/// Shorthand notation for returning a json response.
///
/// # Examples
///
/// Serialize into JSON and send the given value
///
/// ```ignore
/// use reign::prelude::*;
///
/// // User implements serde::Serialize
/// let user = User {
///     name: "John"
/// };
///
/// json!(user)
/// ```
///
/// You can also specify a status code
///
/// ```ignore
/// use reign::prelude::*;
///
/// json!(user, status = 201)
/// ```
#[cfg(feature = "view-backend")]
#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    let input: views::json::Json = parse_macro_input!(input);

    views::json::json(input).into()
}

/// Helper for defining a [reign_router](https://docs.rs/reign_router) handler.
///
/// # Examples
///
/// ```
/// use reign::prelude::*;
///
/// #[action]
/// async fn name(req: &mut Request, id: String) -> Result<impl Response, Error> {
///     Ok(id)
/// }
/// ```
#[cfg(feature = "router-backend")]
#[proc_macro_attribute]
#[proc_macro_error]
pub fn action(_: TokenStream, input: TokenStream) -> TokenStream {
    let input: syn::ItemFn = parse_macro_input!(input);

    router::action::action(input).into()
}

/// Helper for defining a [reign_router](https://docs.rs/reign_router) Path.
///
/// # Examples
///
/// ```
/// use reign::{
///     prelude::*,
///     router::{Router}
/// };
/// #
/// # #[action]
/// # async fn foobar(req: &mut Request) -> Result<impl Response, Error> { Ok("foobar") }
/// #
/// # #[action]
/// # async fn number(req: &mut Request) -> Result<impl Response, Error> { Ok("number") }
/// #
/// # #[action]
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
#[cfg(feature = "router-backend")]
#[proc_macro]
#[proc_macro_error]
pub fn p(input: TokenStream) -> TokenStream {
    let input: router::path::Path = parse_macro_input!(input);

    router::path::path(input).into()
}
