use envy::{from_env, prefixed};
use once_cell::sync::OnceCell;
use serde::Deserialize;

use std::fmt::Debug;

const LOAD_ERR: &str = "Unable to load config from environment variables";

pub trait Config: for<'de> Deserialize<'de> + Debug {
    fn get() -> &'static Self;

    fn cell() -> &'static OnceCell<Self>;
}

pub struct ConfigLoader {}

impl ConfigLoader {
    pub fn load<T>(&self) -> &Self
    where
        T: Config + 'static,
    {
        T::cell().set(from_env::<T>().unwrap()).expect(LOAD_ERR);
        self
    }

    pub fn load_prefixed<T>(&self, prefix: &str) -> &Self
    where
        T: Config + 'static,
    {
        T::cell()
            .set(prefixed(prefix).from_env::<T>().unwrap())
            .expect(LOAD_ERR);
        self
    }
}
