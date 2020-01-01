use askama::Template;
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use mime;

pub mod parse;

pub trait Layout: Template {
    fn content(self, content: String) -> Self;
}

/// Renders an askama template with layout for gotham handler.
///
/// # Examples
///
/// ```
/// use reign::view::render;
///
/// pub fn handler(mut state: State) -> (State, Response<Body>) {
///     render(
///         state,
///         View {
///             name: "world".to_string()
///         },
///         Layout {
///             title: "Application".to_string(),
///             content: "".to_string(), // Will be overridden by template
///         },
///     )
/// }
/// ```
pub fn render<T: Template, L: Layout>(
    state: State,
    template: T,
    layout: L,
) -> (State, Response<Body>) {
    let response = match template.render() {
        Ok(content) => match layout.content(content).render() {
            Ok(content) => create_response(
                &state,
                StatusCode::OK,
                mime::TEXT_HTML_UTF_8,
                content.into_bytes(),
            ),
            Err(_) => create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR),
    };

    (state, response)
}
