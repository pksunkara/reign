#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub mod error;
mod task;
mod tasks;
#[cfg(feature = "templating")]
mod templating;

pub use error::Error;
pub use task::Task;
pub use tasks::Tasks;
#[cfg(feature = "templating")]
pub use templating::Template;

pub use oclif;
#[cfg(feature = "templating")]
pub use serde_json;

use std::{path::PathBuf, process::Command, str::from_utf8};

pub fn workspace_dir() -> Result<PathBuf, Error> {
    let out = Command::new("cargo")
        .args(&["locate-project", "--workspace", "--mesage-format", "plain"])
        .output()?;

    let mut path = PathBuf::from(from_utf8(&out.stdout).map_err(|_| Error::NoWorkspace)?);
    path.pop();

    Ok(path)
}
