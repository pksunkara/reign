use crate::controllers::*;
use gotham::middleware::logger::RequestLogger;
use gotham::pipeline::{new_pipeline, single::single_pipeline};
use gotham::router::{builder::*, Router};
use gotham_derive::*;
use gotham_middleware_diesel::{DieselMiddleware, Repo};
use reign::log::Level;
use serde::Deserialize;

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct IdExtractor {
    pub id: i32,
}

pub fn router<T>(repo: Repo<T>) -> Router
where
    T: diesel::Connection,
{
    let (chain, pipelines) = single_pipeline(
        new_pipeline()
            .add(RequestLogger::new(Level::Info))
            .add(DieselMiddleware::new(repo))
            .build(),
    );

    build_router(chain, pipelines, |route| {
        route.scope("/articles", |route| {
            route.associate("/", |assoc| {
                assoc.get().to(articles::list);
                assoc.post().to(articles::create);
            });

            route.associate("/:id", |assoc| {
                assoc
                    .get()
                    .with_path_extractor::<IdExtractor>()
                    .to(articles::show);
            });
        });
    })
}
