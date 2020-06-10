use crate::controllers::*;
use reign::{
    log::Level,
    prelude::*,
    router::{handlers::to_dir, middleware::RequestLogger, Router},
};

pub fn router(r: &mut Router) {
    r.pipe("common").add(RequestLogger::new(Level::Info));

    r.scope("assets", |r| {
        r.get(p!(path*), to_dir(&["src", "assets"], None));
    });

    r.scope_through("", &["common"], |r| {
        r.get("", pages::home);
    });
}
