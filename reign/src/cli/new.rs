use clap::Clap;

#[derive(Debug, Clap)]
pub struct New {}

impl New {
    pub fn run(&self) -> Result<(), std::io::Error> {
        Ok(())
    }
}
