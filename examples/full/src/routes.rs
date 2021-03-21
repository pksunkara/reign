use crate::controllers::*;

use reign::{
    log::Level,
    prelude::*,
    router::{
        middleware::{session::Session, ContentType, RequestLogger},
        Router,
    },
};
use reign_plugin_redis::RedisPlugin;
use reign_session_backend_redis::RedisBackend;

pub fn router(r: &mut Router) {
    r.pipe("common").add(RequestLogger::new(Level::Info));
    r.pipe("app").add(ContentType::default()).add(
        Session::<reign_plugin_auth::AuthUser, _>::new(RedisBackend::pool(RedisPlugin::get()))
            .secure(false),
    );

    r.scope("").through(&["common"]).to(|r| {
        r.get("", pages::home);

        r.scope("").through(&["app"]).to(|r| {
            r.get("login", users::login);
            r.get("login_submit", users::login_submit);
            r.get("logout", users::logout);

            r.get("register", users::register);
            r.post("register", users::register_submit);

            r.get("profile", users::profile);
        });

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
