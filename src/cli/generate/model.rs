use crate::utils::Result;

use clap::Clap;

#[derive(Debug, Clap)]
pub struct Model {}

impl Model {
    pub fn run(&self) -> Result {
        Ok(())
    }
}
