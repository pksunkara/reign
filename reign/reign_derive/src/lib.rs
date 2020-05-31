#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_root_url = "https://docs.rs/reign_derive/0.1.2")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::parse_macro_input;

#[cfg(feature = "router")]
mod router;
#[cfg(feature = "view")]
mod views;

mod utils;

pub(crate) const INTERNAL_ERR: &'static str =
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
/// #![feature(proc_macro_hygiene)]
///
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
#[cfg(feature = "view-router")]
#[proc_macro]
pub fn json(input: TokenStream) -> TokenStream {
    let input: views::json::Json = parse_macro_input!(input);

    views::json::json(input).into()
}

#[cfg(feature = "router")]
#[proc_macro_attribute]
#[proc_macro_error]
pub fn action(_: TokenStream, input: TokenStream) -> TokenStream {
    let input: syn::ItemFn = parse_macro_input!(input);

    router::action(input).into()
}
