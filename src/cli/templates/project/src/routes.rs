use crate::controllers::*;
use reign::{
    prelude::*,
    router::{handlers::to_dir, log::Level, middleware::RequestLogger, Pipe, Router},
};

pub fn router(r: &mut Router) {
    r.pipe(Pipe::new("common").add(RequestLogger::new(Level::Info)));

    r.scope("assets", |r| {
        r.get(p!(path*), to_dir("src/assets", None));
    });

    r.scope_through("", &["common"], |r| {
        r.get("", pages::home);
    });
}
