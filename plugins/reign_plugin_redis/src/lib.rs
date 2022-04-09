#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc = include_str!("../README.md")]

use bb8_redis::{bb8::Pool, RedisConnectionManager};
use once_cell::sync::OnceCell;
use reign_plugin::{reign_router::futures::FutureExt, Plugin};

use std::{future::Future, pin::Pin};

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
    fn init<'a>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        async move {
            let manager =
                RedisConnectionManager::new(&*self.url).expect("Bad redis connection URL");

            let pool = Pool::builder()
                .build(manager)
                .await
                .expect("Unable to connect to redis");

            REDIS
                .set(pool)
                .expect("Unable to store the redis connection");
        }
        .boxed()
    }
}
