#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_plugin_redis/0.0.0")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

use bb8_redis::{bb8::Pool, RedisConnectionManager, RedisPool};
use reign_boot::{once_cell::sync::OnceCell, Plugin};

static REDIS: OnceCell<RedisPool> = OnceCell::new();

pub struct RedisPlugin {
    url: String,
}

impl RedisPlugin {
    pub fn new<S: Into<String>>(url: S) -> Self {
        Self { url: url.into() }
    }

    pub fn get() -> &'static RedisPool {
        REDIS
            .get()
            .expect("Redis must be connected before using it")
    }
}

impl Plugin for RedisPlugin {
    fn init(&self) {
        let manager = RedisConnectionManager::new(&*self.url).expect("Bad redis connection URL");
        let pool = RedisPool::new(Pool::builder().build_unchecked(manager));

        if REDIS.set(pool).is_err() {
            panic!("Unable to store the redis connection");
        }
    }
}
