#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_plugin/0.2.1")]
#![doc = include_str!("../README.md")]

pub use reign_router;

mod plugin;

pub use plugin::Plugin;
