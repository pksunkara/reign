use gotham::helpers::http::response::*;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use mime;
use reign::prelude::*;

views!(articles);

pub fn list(state: State) -> (State, Response<Body>) {
    let response = create_response(&state, StatusCode::OK, mime::TEXT_PLAIN, "Article List");

    (state, response)
}

pub fn create(state: State) -> (State, Response<Body>) {
    let response = create_response(&state, StatusCode::OK, mime::TEXT_PLAIN, "Article Create");

    (state, response)
}
