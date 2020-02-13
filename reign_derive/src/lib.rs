extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, LitStr};

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
/// #![feature(type_ascription)]
/// use reign::prelude::*;
///
/// views!("src", "views");
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
/// ```ignore
/// use reign::prelude::*;
///
/// render!("pages:home")
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn render(input: TokenStream) -> TokenStream {
    let lit: LitStr = parse_macro_input!(input);

    views::render(lit).into()
}
