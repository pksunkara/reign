use crate::boot::Reign;

use reign_router::Router;

pub trait Plugin {
    fn init(&self) {}

    fn router(&self, f: Box<dyn FnOnce(&mut Router)>) -> Box<dyn FnOnce(&mut Router)> {
        f
    }
}

impl Reign {
    pub fn add_plugin<P>(mut self, plugin: P) -> Self
    where
        P: Plugin + 'static,
    {
        // TODO: plugin: Maybe use spawn_blocking to allow async
        // Initialize the plugin
        plugin.init();

        self.plugins.push(Box::new(plugin));
        self
    }
}
