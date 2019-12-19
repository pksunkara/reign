use gotham::helpers::http::response::*;
use gotham::state::State;
use hyper::{Body, Response};

// TODO: Allow non-string locations like route names etc..
/// Creates a gotham response tuple that is a temporary redirect
///
/// # Examples
///
/// ```
/// use gotham::state::State;
/// use hyper::{Body, Response};
/// use reign::router::redirect;
///
/// pub fn handler(mut state: state) -> (State, Response<Body>) {
///     redirect(state, "/redirect")
/// }
/// ```
pub fn redirect(state: State, location: &'static str) -> (State, Response<Body>) {
    let response = create_temporary_redirect(&state, location);
    (state, response)
}
