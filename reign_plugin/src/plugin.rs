use reign_router::{futures::FutureExt, Router};

use std::{future::Future, pin::Pin};

pub trait Plugin {
    fn init<'a>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        async move {}.boxed()
    }

    fn router(&self, f: Box<dyn FnOnce(&mut Router)>) -> Box<dyn FnOnce(&mut Router)> {
        f
    }
}
