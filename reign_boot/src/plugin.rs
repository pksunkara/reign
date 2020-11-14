use crate::boot::Reign;

use reign_plugin::Plugin;

impl Reign {
    pub fn add_plugin<P>(mut self, plugin: P) -> Self
    where
        P: Plugin + 'static,
    {
        // TODO: plugin: Maybe use tokio::block_in_place to allow async
        // Initialize the plugin
        plugin.init();

        self.plugins.push(Box::new(plugin));
        self
    }
}
