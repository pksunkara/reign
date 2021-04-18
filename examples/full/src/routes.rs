use crate::controllers::*;

use reign::{
    log::Level,
    prelude::*,
    router::{
        middleware::{ContentType, RequestLogger},
        path as p, Router,
    },
};

pub fn router(r: &mut Router) {
    r.pipe("common").add(RequestLogger::new(Level::Info));
    r.pipe("app").add(ContentType::default());

    r.scope("").through(&["common", "app"]).to(|r| {
        r.get("", pages::home);

        r.scope("articles").to(|r| {
            r.get("", articles::list);
            r.get("new", articles::new);
            r.post("", articles::create);

            r.scope(p!(id)).to(|r| {
                r.get("", articles::show);
            });
        });
    });
}
