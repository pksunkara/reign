extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod layouts;
mod render;
mod templates;
mod views;

#[proc_macro]
pub fn templates(_: TokenStream) -> TokenStream {
    templates::templates().into()
}

/// Auto load the layouts (askama templates) from `src/views/_layouts` directory.
///
/// Ignores the files whose name do not start with an alphabet.
///
/// # Examples
///
/// ```
/// use reign::prelude::*;
///
/// layouts!();
/// ```
#[proc_macro]
pub fn layouts(_: TokenStream) -> TokenStream {
    layouts::layouts().into()
}

/// Denote an askama template as a layout.
///
/// This template should have a field called `content`.
///
/// # Examples
///
/// ```
/// use reign::prelude::*;
///
/// #[derive(Debug, Layout, Template)]
/// #[template(path = "different_layouts_dir/plain.html")]
/// pub struct Plain {
///     pub content: String,
///     pub title: String,
/// }
/// ```
#[proc_macro_derive(Layout)]
pub fn derive_layout(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    layouts::layout_derive(input).into()
}

/// Auto load the views (askama templates) from `src/views/[input]` directory.
///
/// Ignores the files whose name do not start with an alphabet.
///
/// # Examples
///
/// ```
/// use reign::prelude::*;
///
/// views!(pages);
/// ```
#[proc_macro]
pub fn views(input: TokenStream) -> TokenStream {
    let input: views::Views = parse_macro_input!(input);

    views::views(input).into()
}

/// Shorthand notation for rendering a template in a controller action.
///
/// # Examples
///
/// Render the given template using the default application layout (`src/views/_layouts/application.html`)
///
/// ```
/// use reign::prelude::*;
///
/// pub fn handler(mut state: State) -> (State, Response<Body>) {
///     render!(ViewIndex {})
/// }
/// ```
///
/// Render the given template using a different layout (`src/views/_layouts/different.html`)
///
/// ```
/// use reign::prelude::*;
///
/// pub fn handler(mut state: State) -> (State, Response<Body>) {
///     render!(ViewIndex {}, Different)
/// }
/// ```
#[proc_macro]
pub fn render(input: TokenStream) -> TokenStream {
    let input: render::Render = parse_macro_input!(input);

    render::render(input).into()
}
