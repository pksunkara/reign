#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_plugin_auth/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

use reign_plugin::{
    reign_router::{Request, Router},
    Plugin,
};
use serde::{Deserialize, Serialize};

/// Plugin that serves a directory as static server at the given prefix
///
/// # Examples
///
/// ```no_run
/// use reign::Reign;
/// use reign_plugin_auth::AuthPlugin;
///
/// #[tokio::main]
/// async fn main() {
///     Reign::build()
///         .add_plugin(AuthPlugin::new("assets").dir(&["src", "assets"]))
///         .serve("127.0.0.1:8000", |r| {})
///         .await
/// }
/// ```
#[derive(Default)]
pub struct AuthPlugin {}

impl AuthPlugin {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Plugin for AuthPlugin {
    fn router(&self, f: Box<dyn FnOnce(&mut Router)>) -> Box<dyn FnOnce(&mut Router)> {
        Box::new(|r| {
            r.pipe("auth");

            f(r);
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: i32,
}

pub fn login<U, P>(req: &mut Request, username: U, password: P)
where
    U: AsRef<str>,
    P: AsRef<str>,
{
    req.save_session(AuthUser { id: 1 });
}

pub fn logout(req: &mut Request) {
    req.delete_session::<AuthUser>();
}

pub fn user(req: &mut Request) -> Option<i32> {
    req.session::<AuthUser>().map(|u| u.id)
}
