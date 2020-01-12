use crate::models::articles::Article;
use crate::schema::articles::dsl::articles;
use diesel::prelude::*;
use gotham::handler::HandlerFuture;
use gotham::helpers::http::response::*;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use mime;
use reign::prelude::*;
use reign::view::Slots;

pub fn list(state: State) -> Box<HandlerFuture> {
    use futures::{future, Future};
    use gotham::{handler::IntoHandlerError, state::FromState};

    let f = future::ok((state, ())).and_then(move |(state, _)| {
        crate::Repo::borrow_from(&state)
            .run(move |connection| articles.load::<Article>(&connection))
            .then(|result| match result {
                Ok(data) => future::ok({
                    render!(articles::List {
                        _slots: Slots::default(),
                        articles: data,
                    })
                }),
                Err(e) => future::err((state, e.into_handler_error())),
            })
    });

    Box::new(f)
}

pub fn create(state: State) -> (State, Response<Body>) {
    let response = create_response(&state, StatusCode::OK, mime::TEXT_PLAIN, "Article Create");

    (state, response)
}
