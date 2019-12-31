use crate::controllers::*;
use gotham::middleware::logger::RequestLogger;
use gotham::pipeline::{new_pipeline, single::single_pipeline};
use gotham::router::{builder::*, Router};
use gotham_middleware_diesel::{DieselMiddleware, Repo};
use reign::log::Level;

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
        });
    })
}
