use crate::connection::Database;

use reign_plugin::{reign_router::futures::FutureExt, Plugin};

use std::{future::Future, pin::Pin};

impl Plugin for Database {
    fn init<'a>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        async move {
            self.connect();
        }
        .boxed()
    }
}
