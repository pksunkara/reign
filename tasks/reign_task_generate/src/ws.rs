use anyhow::Context;
use reign_task::Error;

use std::{path::PathBuf, process::Command, str::from_utf8};

pub(crate) fn ws_dir() -> Result<PathBuf, Error> {
    let out = Command::new("cargo")
        .args(&["locate-project", "--workspace", "--mesage-format", "plain"])
        .output()?;

    let mut path =
        PathBuf::from(from_utf8(&out.stdout).context("Failed to find the workspace dir")?);
    path.pop();

    Ok(path)
}
