#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc = include_str!("../README.md")]

use reign_plugin::{
    reign_router::{hyper::Method, Path, Router},
    Plugin,
};

mod handlers;
mod stream;

/// Plugin that serves a directory as static server at the given prefix
///
/// # Examples
///
/// ```no_run
/// use reign::Reign;
/// use reign_plugin_static::StaticPlugin;
///
/// #[tokio::main]
/// async fn main() {
///     Reign::build()
///         .add_plugin(StaticPlugin::new("assets").dir(&["src", "assets"]))
///         .serve("127.0.0.1:8000", |r| {})
///         .await
/// }
/// ```
#[derive(Default)]
pub struct StaticPlugin {
    prefix: String,
    dir: Vec<String>,
    cache: u32,
}

impl StaticPlugin {
    pub fn new<S>(prefix: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            prefix: prefix.into(),
            ..Default::default()
        }
    }

    /// Specify the directory from which to serve the static assets from
    ///
    /// **NOTE:** This should be relative to `CARGO_MANIFEST_DIR`
    pub fn dir(mut self, dir: &[&str]) -> Self {
        self.dir = dir.into_iter().map(|x| x.to_string()).collect();
        self
    }

    pub fn cache(mut self, cache: u32) -> Self {
        self.cache = cache;
        self
    }
}

impl Plugin for StaticPlugin {
    fn router(&self, f: Box<dyn FnOnce(&mut Router)>) -> Box<dyn FnOnce(&mut Router)> {
        let prefix = Path::new().path(&*self.prefix);
        let handle = handlers::to_dir(self.dir.clone(), self.cache);

        Box::new(|r| {
            r.scope(prefix).to(|r: &mut Router| {
                r.any(
                    &[Method::GET, Method::HEAD],
                    Path::new().param_opt_regex("path", ".+"),
                    handle,
                );
            });

            f(r);
        })
    }
}
