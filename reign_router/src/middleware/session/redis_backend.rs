use crate::middleware::session::SessionBackend;
use bb8_redis::{redis::AsyncCommands, RedisPool};
use futures::{future::BoxFuture, FutureExt};
use log::error;

pub struct RedisBackend {
    ttl: usize,
    pool: RedisPool,
}

impl RedisBackend {
    pub fn new(ttl: usize, pool: RedisPool) -> Self {
        Self { ttl, pool }
    }

    pub fn pool(pool: RedisPool) -> Self {
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

                if let Err(_) = conn.set_ex::<_, _, String>(id, content, ttl).await {
                    error!("Failed to run redis command");
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

                if let Ok(value) = conn.get(id).await {
                    return Some(value);
                } else {
                    error!("Failed to run redis command");
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

                if let Err(_) = conn.del::<_, String>(id).await {
                    error!("Failed to run redis command");
                }
            } else {
                error!("Failed to get redis connection from pool");
            }
        }
        .boxed()
    }
}
