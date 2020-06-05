use reign_router::{hyper::Body, Response};

#[test]
fn test_invalid_status_code() {
    let data = (999 as u16, mime::TEXT_PLAIN, Body::empty());

    let response = data.respond();

    assert!(response.is_err());
}
