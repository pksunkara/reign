#[cfg(feature = "views-gotham")]
use gotham::state::State;
#[cfg(feature = "views-gotham")]
use hyper::{Body, Response, StatusCode};

use std::fmt::{self, write, Write};

pub use maplit;

pub mod parse;
mod slots;

pub use slots::{SlotRender, Slots};

pub trait View {
    fn render(&self, f: &mut dyn Write) -> fmt::Result;
}

#[cfg(not(feature = "views-gotham"))]
pub fn render<D: fmt::Display>(view: D) -> Result<String, fmt::Error> {
    let mut content = String::new();

    match write(&mut content, format_args!("{}", view)) {
        Ok(()) => Ok(content),
        Err(e) => Err(e),
    }
}

/// Renders a view for gotham handler.
///
/// # Examples
///
/// ```
/// use reign::view::{render, View};
/// use std::fmt::{Formatter, Result, Display}
///
/// struct CustomView {}
///
/// impl Display for CustomView {
///     fn fmt(&self, f: &mut Formatter) -> Result {
///         write!(f, "custom view")
///     }
/// }
///
/// pub fn handler(mut state: State) -> (State, Response<Body>) {
///     render(state, CustomView {})
/// }
/// ```
#[cfg(feature = "views-gotham")]
pub fn render<D: fmt::Display>(state: State, view: D) -> (State, Response<Body>) {
    use gotham::helpers::http::response::{create_empty_response, create_response};

    let mut content = String::new();

    let response = match write(&mut content, format_args!("{}", view)) {
        Ok(()) => create_response(
            &state,
            StatusCode::OK,
            mime::TEXT_HTML_UTF_8,
            content.into_bytes(),
        ),
        Err(_) => create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR),
    };

    (state, response)
}
