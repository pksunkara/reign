use gotham::helpers::http::response::*;
use gotham::state::State;
use hyper::{Body, Response};

// TODO: Allow non-string locations like route names etc..
/// Creates a gotham response tuple that is a temporary redirect.
///
/// # Examples
///
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::{router::builder::*, state::State, test::TestServer};
    use hyper::{header::*, Body, Response, StatusCode};

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

        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
        assert_eq!(headers.contains_key(LOCATION), true);
        assert_eq!(headers[LOCATION], "/target");
    }
}
