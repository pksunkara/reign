use crate::env::load_env_files;

use env_logger::{from_env, Env};
use reign_plugin::{
    reign_router::{serve, Router},
    Plugin,
};

use std::net::ToSocketAddrs;

#[derive(Default)]
pub struct Reign {
    pub(crate) plugins: Vec<Box<dyn Plugin>>,
}

impl Reign {
    pub fn build() -> Self {
        load_env_files();

        from_env(Env::default().default_filter_or("info"))
            .format_timestamp(None)
            .init();

        Self::default()
    }

    pub async fn serve<A, R>(self, addr: A, f: R)
    where
        A: ToSocketAddrs + Send + 'static,
        R: FnOnce(&mut Router) + 'static,
    {
        let mut router_fn: Box<dyn FnOnce(&mut Router)> = Box::new(f);

        for plugin in self.plugins {
            // Initialize the plugin
            plugin.init().await;

            router_fn = plugin.router(router_fn);
        }

        serve(addr, router_fn).await.unwrap()
    }
}
