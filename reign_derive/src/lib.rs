extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ExprStruct};

mod render;
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
/// ```
/// use reign;
///
/// reign::prelude::views!("src", "views");
/// ```
#[proc_macro]
pub fn views(input: TokenStream) -> TokenStream {
    let input: views::Views = parse_macro_input!(input);

    views::views(input).into()
}

/// Shorthand notation for rendering a view in a controller action.
///
/// # Examples
///
/// Render the given view
///
/// ```
/// use reign::prelude::*;
///
/// pub fn handler(mut state: State) -> (State, Response<Body>) {
///     render!(pages::Home {})
/// }
/// ```
#[proc_macro]
pub fn render(input: TokenStream) -> TokenStream {
    let input: ExprStruct = parse_macro_input!(input);

    render::render(input).into()
}
