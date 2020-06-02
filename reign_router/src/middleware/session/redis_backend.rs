use crate::middleware::session::SessionBackend;
use bb8_redis::{bb8::Pool, redis::AsyncCommands, RedisConnectionManager};
use futures::{future::BoxFuture, FutureExt};
use log::error;

pub struct RedisBackend {
    ttl: usize,
    pool: Pool<RedisConnectionManager>,
}

impl RedisBackend {
    pub fn new(ttl: usize, pool: Pool<RedisConnectionManager>) -> Self {
        Self { ttl, pool }
    }

    pub fn pool(pool: Pool<RedisConnectionManager>) -> Self {
        Self::new(60 * 60 * 24 * 7, pool)
    }
}

impl SessionBackend for RedisBackend {
    fn persist_session<'a>(&'a self, id: &'a str, content: &'a [u8]) -> BoxFuture<'a, bool> {
        let content = Vec::from(content);
        let ttl = self.ttl;

        async move {
            if let Ok(mut conn) = self.pool.get().await {
                let conn = conn.as_mut().unwrap();

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
                let conn = conn.as_mut().unwrap();

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
                let conn = conn.as_mut().unwrap();

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
