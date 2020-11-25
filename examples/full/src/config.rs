use reign::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Config)]
pub struct App {
    pub database_url: String,
    pub redis_url: String,
}
