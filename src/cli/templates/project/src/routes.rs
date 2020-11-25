use crate::controllers::*;

use reign::{
    log::Level,
    prelude::*,
    router::{
        middleware::{ContentType, RequestLogger},
        Router,
    },
};

pub fn router(r: &mut Router) {
    r.pipe("common").add(RequestLogger::new(Level::Info));
    r.pipe("app").add(ContentType::default());

    r.scope("").through(&["common", "app"]).to(|r| {
        r.get("", pages::home);
    });
}
