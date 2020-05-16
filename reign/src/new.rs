use std::io::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct New {}

impl New {
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
