use once_cell::sync::OnceCell;
use serde::Deserialize;

pub static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn config() -> &'static Config {
    CONFIG.get().unwrap()
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
}
