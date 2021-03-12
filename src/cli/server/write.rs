use crate::INTERNAL_ERR;

use reign_view::common::Manifest;
use serde_json::to_string;

use std::{fs::write, io::Result, path::Path, process::Command};

fn fmt(path: &Path) -> Result<()> {
    Command::new("rustfmt").args(&[path]).status()?;
    Ok(())
}

pub fn write_file(path: &Path, file: String) -> Result<()> {
    write(path, file)?;
    fmt(path)?;
    Ok(())
}

pub fn write_manifest(views: &Path, manifest: &Manifest) -> Result<()> {
    write(
        views.join("_manifest.json"),
        to_string(manifest).expect(INTERNAL_ERR),
    )
}
