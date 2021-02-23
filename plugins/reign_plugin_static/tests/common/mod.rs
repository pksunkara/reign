use reign::router::{service as router_service, Router, Service};
use reign_plugin::Plugin;
use reign_plugin_static::StaticPlugin;

pub fn service(cache: u32) -> Service {
    let mut router_fn: Box<dyn FnOnce(&mut Router)> = Box::new(|_r| {});

    router_fn = StaticPlugin::new("static")
        .dir(&["tests", "fixture"])
        .cache(cache)
        .router(router_fn);

    router_service(router_fn)
}
