use gotham::{
    helpers::http::response::create_empty_response,
    hyper::{header, Body, Response, StatusCode},
    state::State,
};
use std::borrow::Cow;

// TODO: Allow non-string locations like route names etc..
/// Creates a gotham response tuple that is a redirect with status code 303.
///
/// # Examples
///
/// ```
/// use reign::router::redirect;
/// use gotham::state::State;
/// use gotham::hyper::{Response, Body};
///
/// pub fn handler(mut state: State) -> (State, Response<Body>) {
///     redirect(state, "/redirect")
/// }
/// ```
pub fn redirect<L: Into<Cow<'static, str>>>(state: State, location: L) -> (State, Response<Body>) {
    let mut response = create_empty_response(&state, StatusCode::SEE_OTHER);

    response.headers_mut().insert(
        header::LOCATION,
        location.into().to_string().parse().unwrap(),
    );

    (state, response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::{
        hyper::{header::*, Body, Response, StatusCode},
        router::builder::*,
        state::State,
        test::TestServer,
    };

    #[test]
    fn test_redirect() {
        fn handler(state: State) -> (State, Response<Body>) {
            redirect(state, "/target")
        }

        let router = build_simple_router(|route| {
            route.get("/").to(handler);
        });

        let test_server = TestServer::new(router).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        let headers = response.headers();

        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert_eq!(headers.contains_key(LOCATION), true);
        assert_eq!(headers[LOCATION], "/target");
    }
}
