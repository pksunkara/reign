use crate::models::{Article, Articles};
use diesel::prelude::*;
use gotham::handler::HandlerFuture;
use gotham::helpers::http::response::*;
use gotham::hyper::{Body, Response, StatusCode};
use gotham::state::State;
use mime;
use reign::prelude::*;
use std::pin::Pin;

pub fn list(state: State) -> Pin<Box<HandlerFuture>> {
    use futures::prelude::*;
    use gotham::{handler::IntoHandlerError, state::FromState};

    let repo = crate::Repo::borrow_from(&state).clone();

    async move {
        let articles = match repo
            .run(move |connection| Articles.load::<Article>(&connection))
            .await
        {
            Ok(rows) => rows,
            Err(e) => return Err((state, e.into_handler_error())),
        };

        Ok((state, render!(articles::list)))
    }
    .boxed()
}

pub fn show(state: State) -> Pin<Box<HandlerFuture>> {
    use futures::prelude::*;
    use gotham::{handler::IntoHandlerError, state::FromState};

    let id = crate::routes::IdExtractor::borrow_from(&state).id;
    let repo = crate::Repo::borrow_from(&state).clone();

    async move {
        let article = match repo
            .run(move |connection| Articles.find(id).first::<Article>(&connection))
            .await
        {
            Ok(rows) => rows,
            Err(e) => return Err((state, e.into_handler_error())),
        };

        Ok((state, render!(articles::show)))
    }
    .boxed()
}

pub fn create(state: State) -> (State, Response<Body>) {
    let response = create_response(&state, StatusCode::OK, mime::TEXT_PLAIN, "Article Create");

    (state, response)
}
