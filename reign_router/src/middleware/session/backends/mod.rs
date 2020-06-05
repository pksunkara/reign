//! Contains common sesssion backends

#[cfg(feature = "session-redis")]
mod redis_backend;

#[cfg(feature = "session-redis")]
pub use redis_backend::RedisBackend;
