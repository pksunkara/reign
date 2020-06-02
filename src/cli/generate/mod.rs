use crate::utils::Result;
use clap::Clap;

mod model;

#[derive(Debug, Clap)]
pub enum Generate {
    Model(model::Model),
}

impl Generate {
    pub fn run(&self) -> Result {
        match self {
            Generate::Model(x) => x.run(),
        }
    }
}
