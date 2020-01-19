use crate::models::{Article, Articles};
use crate::views::articles::{List, Show};
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
            .run(move |connection| Articles.load::<Article>(&connection))
            .then(|result| match result {
                Ok(articles) => future::ok({
                    render!(List {
                        _slots: Slots::default(),
                        articles,
                    })
                }),
                Err(e) => future::err((state, e.into_handler_error())),
            })
    });

    Box::new(f)
}

pub fn show(state: State) -> Box<HandlerFuture> {
    use futures::{future, Future};
    use gotham::{handler::IntoHandlerError, state::FromState};

    // let path = crate::routes::IdExtractor::borrow_from(&state);

    let f = future::ok((state, ())).and_then(move |(state, _)| {
        crate::Repo::borrow_from(&state)
            .run(move |connection| Articles.find(1).first::<Article>(&connection))
            .then(|result| match result {
                Ok(article) => future::ok({
                    println!("{:?}", article);
                    render!(Show {
                        _slots: Slots::default(),
                        article,
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
