use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
pub use maplit;
use mime;
use std::fmt::{write, Display, Result, Write};

pub mod parse;
mod slots;

pub use slots::{SlotRender, Slots};

pub trait View: Display {
    fn render(&self, f: &mut dyn Write) -> Result;
}

// TODO: Can I do impl Display for View

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
pub fn render<D: Display>(state: State, view: D) -> (State, Response<Body>) {
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
