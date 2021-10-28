use clap::Parser;
use reign_task::Error;

use std::process::Command;

/// List all available tasks in a reign app
#[derive(Debug, Parser)]
pub struct Tasks {}

impl Tasks {
    pub fn run(&self) -> Result<(), Error> {
        run_task(vec!["tasks".into()])
    }
}

pub fn run_task(args: Vec<String>) -> Result<(), Error> {
    Command::new("cargo")
        .args(&["run", "-p", "xtask", "--"])
        .args(args)
        .status()
        .map_err(|_| Error::Cargo)?;

    Ok(())
}
