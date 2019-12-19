use askama::Template;
use gotham::helpers::http::response::{create_empty_response, create_response};
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use mime;

pub trait Layout: Template {
    fn content(self, content: String) -> Self;
}

// TODO: Convert to macro 2.0
// TODO: Capture local variables unhygienically and send them to templates
/// Shorthand notation for rendering a template in a controller action.
///
/// # Examples
///
/// Render the given template using the default application layout (`src/views/layouts/application.html`)
///
/// ```
/// use reign::view::render;
///
/// pub fn handler(mut state: State) -> (State, Response<Body>) {
///     render!(state, ViewIndex {})
/// }
/// ```
///
/// Render the given template using a different layout (`src/views/layouts/different.html`)
///
/// ```
/// use reign::view::render;
///
/// pub fn handler(mut state: State) -> (State, Response<Body>) {
///     render!(state, ViewIndex {}, LayoutDifferent)
/// }
/// ```
///
/// Render the given template using a non-standard layout
///
/// ```
/// use reign::view::render;
///
/// pub fn handler(mut state: State) -> (State, Response<Body>) {
///     render!(
///         state,
///         ViewIndex {},
///         LayoutDifferent {
///             title: "Application".to_string(),
///             content: "".to_string(),
///         },
///     )
/// }
/// ```
#[macro_export]
macro_rules! render {
    ($state:ident, $template:expr, $layout:ident { $($f:ident : $e:expr),* $(,)? } $(,)?) => {
        ::reign::view::render($state, $template, crate::layouts::$layout {
            $(
                $f: $e,
            )*
        })
    };
    ($state:ident, $template:expr, $layout:ident $(,)?) => {
        ::reign::view::render($state, $template, crate::layouts::$layout::default())
    };
    ($state:ident, $template:expr $(,)?) => {
        render!($state, $template, LayoutApplication)
    };
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
///             content: "".to_string(),
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
