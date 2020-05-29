#[cfg(feature = "router-gotham")]
use gotham_derive::{StateData, StaticResponseExtender};
#[cfg(feature = "router-gotham")]
use serde::Deserialize;
use std::collections::HashMap as Map;

#[derive(Debug)]
#[cfg_attr(
    feature = "router-gotham",
    derive(Deserialize, StateData, StaticResponseExtender)
)]
pub struct Query(Map<String, String>);

impl Query {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
}
