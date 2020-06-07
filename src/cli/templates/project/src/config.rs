use reign::prelude::*;
use serde::Deserialize;

#[derive(Config, Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
}
