use crate::controllers::*;
use gotham::middleware::logger::RequestLogger;
use gotham::router::Router;
use gotham_derive::*;
use gotham_middleware_diesel::{DieselMiddleware, Repo};
use reign::{log::Level, prelude::*, router::middleware::ContentType};
use serde::Deserialize;

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct IdExtractor {
    pub id: i32,
}

pub fn router<T>(repo: Repo<T>) -> Router
where
    T: diesel::Connection,
{
    pipelines!(
        common: [
            RequestLogger::new(Level::Info),
        ],
        app: [
            ContentType::default(),
            DieselMiddleware::new(repo),
        ],
    );

    use ::gotham::router::builder::{DefineSingleRoute, DrawRoutes};

    ::gotham::router::builder::build_simple_router(|route| {
        scope!("/", [common, app], {
            scope!("/articles", {
                get!("/", articles::list);
                post!("/", articles::create);
            });
        });
    })

    //         route.associate("/:id", |assoc| {
    //             assoc
    //                 .get()
    //                 .with_path_extractor::<IdExtractor>()
    //                 .to(articles::show);
    //         });
}
