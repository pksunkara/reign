use crate::boot::Reign;

use envy::{from_env, prefixed};
use once_cell::sync::OnceCell;
use serde::Deserialize;

use std::fmt::Debug;

pub(crate) const LOAD_ERR: &str = "Unable to load config from environment variables";
pub(crate) const STORE_ERR: &str = "Unable to store the loaded config";

pub trait Config: for<'de> Deserialize<'de> + Debug {
    fn get() -> &'static Self;

    fn cell() -> &'static OnceCell<Self>;
}

impl Reign {
    pub fn env<T>(self) -> Self
    where
        T: Config + 'static,
    {
        T::cell()
            .set(from_env::<T>().expect(LOAD_ERR))
            .expect(STORE_ERR);
        self
    }

    pub fn env_prefixed<T>(self, prefix: &str) -> Self
    where
        T: Config + 'static,
    {
        T::cell()
            .set(prefixed(prefix).from_env::<T>().expect(LOAD_ERR))
            .expect(STORE_ERR);
        self
    }
}
