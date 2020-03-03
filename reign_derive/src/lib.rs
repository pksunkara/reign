#![cfg_attr(feature = "build-docs", feature(external_doc))]
#![doc(html_root_url = "https://docs.rs/reign_derive/0.1.2")]
#![cfg_attr(feature = "build-docs", doc(include = "../README.md"))]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, Expr};

#[cfg(feature = "router")]
mod router;
mod views;

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
#[proc_macro]
pub fn views(input: TokenStream) -> TokenStream {
    let input: views::Views = parse_macro_input!(input);

    views::views(input).into()
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
#[proc_macro]
#[proc_macro_error]
pub fn render(input: TokenStream) -> TokenStream {
    let input: views::Render = parse_macro_input!(input);

    views::render(input).into()
}

/// Shorthand notation for returning a redirect response.
///
/// # Examples
///
/// Redirect to the given url
///
/// ```ignore
/// use reign::prelude::*;
///
/// redirect!("/dashboard")
/// ```
#[proc_macro]
pub fn redirect(input: TokenStream) -> TokenStream {
    let input: Expr = parse_macro_input!(input);

    views::redirect(input).into()
}

#[cfg(feature = "router")]
#[proc_macro_attribute]
pub fn action(_: TokenStream, input: TokenStream) -> TokenStream {
    let input: syn::ItemFn = parse_macro_input!(input);

    router::action(input).into()
}

#[cfg(feature = "router")]
#[proc_macro]
pub fn pipelines(input: TokenStream) -> TokenStream {
    let input: router::Pipelines = parse_macro_input!(input);

    router::pipelines(input).into()
}

#[cfg(feature = "router")]
#[proc_macro]
pub fn scope(input: TokenStream) -> TokenStream {
    let input: router::Scope = parse_macro_input!(input);

    router::scope(input).into()
}

#[cfg(feature = "router")]
#[proc_macro]
pub fn get(input: TokenStream) -> TokenStream {
    let input: router::Method = parse_macro_input!(input);

    router::get(input).into()
}

#[cfg(feature = "router")]
#[proc_macro]
pub fn post(input: TokenStream) -> TokenStream {
    let input: router::Method = parse_macro_input!(input);

    router::post(input).into()
}
