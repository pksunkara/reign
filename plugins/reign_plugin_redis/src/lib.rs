#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_plugin_redis/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

use bb8_redis::{bb8::Pool, RedisConnectionManager};
use once_cell::sync::OnceCell;
use reign_plugin::Plugin;

static REDIS: OnceCell<Pool<RedisConnectionManager>> = OnceCell::new();

pub struct RedisPlugin {
    url: String,
}

impl RedisPlugin {
    pub fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        Self { url: url.into() }
    }

    pub fn get() -> &'static Pool<RedisConnectionManager> {
        REDIS
            .get()
            .expect("Redis must be connected before using it")
    }
}

impl Plugin for RedisPlugin {
    fn init(&self) {
        let manager = RedisConnectionManager::new(&*self.url).expect("Bad redis connection URL");
        let pool = Pool::builder().build_unchecked(manager);

        REDIS
            .set(pool)
            .expect("Unable to store the redis connection");
    }
}
