use gotham::helpers::http::response::*;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use mime;
use reign::prelude::*;
use reign::view::Slots;

pub fn list(state: State) -> (State, Response<Body>) {
    render!(articles::List {
        _slots: Slots::default(),
    })
}

pub fn create(state: State) -> (State, Response<Body>) {
    let response = create_response(&state, StatusCode::OK, mime::TEXT_PLAIN, "Article Create");

    (state, response)
}
