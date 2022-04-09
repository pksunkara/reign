#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc = include_str!("../README.md")]

use bb8_redis::{bb8::Pool, redis::AsyncCommands, RedisConnectionManager};
use log::error;
use reign_router::{
    futures::{future::BoxFuture, FutureExt},
    middleware::session::SessionBackend,
};

/// Redis backend for session data
pub struct RedisBackend {
    ttl: usize,
    pool: Pool<RedisConnectionManager>,
}

impl RedisBackend {
    pub fn new(ttl: usize, pool: &Pool<RedisConnectionManager>) -> Self {
        Self {
            ttl,
            pool: pool.clone(),
        }
    }

    pub fn pool(pool: &Pool<RedisConnectionManager>) -> Self {
        Self::new(60 * 60 * 24 * 7, pool)
    }
}

impl SessionBackend for RedisBackend {
    fn persist_session<'a>(&'a self, id: &'a str, content: &'a [u8]) -> BoxFuture<'a, bool> {
        let content = Vec::from(content);
        let ttl = self.ttl;

        async move {
            if let Ok(mut conn) = self.pool.get().await {
                if let Err(e) = conn.set_ex::<_, _, String>(id, content, ttl).await {
                    error!("Failed to run redis command, {}", e);
                } else {
                    return true;
                }
            } else {
                error!("Failed to get redis connection from pool");
            }

            false
        }
        .boxed()
    }

    fn read_session<'a>(&'a self, id: &'a str) -> BoxFuture<'a, Option<Vec<u8>>> {
        async move {
            if let Ok(mut conn) = self.pool.get().await {
                match conn.get(id).await {
                    Ok(value) => return Some(value),
                    Err(e) => error!("Failed to run redis command, {}", e),
                }
            } else {
                error!("Failed to get redis connection from pool");
            }

            None
        }
        .boxed()
    }

    fn drop_session<'a>(&'a self, id: &'a str) -> BoxFuture<'a, ()> {
        async move {
            if let Ok(mut conn) = self.pool.get().await {
                if let Err(e) = conn.del::<_, String>(id).await {
                    error!("Failed to run redis command, {}", e);
                }
            } else {
                error!("Failed to get redis connection from pool");
            }
        }
        .boxed()
    }
}
