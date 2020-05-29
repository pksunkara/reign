use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ROUTE_NUM: Mutex<Id> = Mutex::new(Id::new());
    static ref STRUCT_NUM: Mutex<Id> = Mutex::new(Id::new());
}

mod action;
mod pipeline;
#[allow(clippy::module_inception)]
mod router;
mod scope;
mod to;

mod path;
mod ty;

pub use action::action;
pub use pipeline::{pipeline, Pipeline};
pub use router::router;
pub use scope::{scope, Scope};
pub use to::{get, to, To};

struct Id(u32);

impl Id {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn get(&mut self) -> u32 {
        let ret = self.0;
        self.0 += 1;
        ret
    }
}
