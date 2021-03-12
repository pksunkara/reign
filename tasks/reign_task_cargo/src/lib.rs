#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_task_cargo/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

use reign_task::{Error, Task};

use std::process::Command;

pub struct Cargo {
    name: String,
    short_about: String,
    args: Vec<String>,
}

impl Cargo {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            short_about: "".into(),
            args: vec![],
        }
    }

    pub fn about<S>(mut self, about: S) -> Self
    where
        S: Into<String>,
    {
        self.short_about = about.into();
        self
    }

    pub fn args<I, T>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        self.args = args.into_iter().map(|x| x.as_ref().into()).collect();
        self
    }
}

impl Task for Cargo {
    fn command(&self) -> String {
        self.name.clone()
    }

    fn short_about(&self) -> String {
        self.short_about.clone()
    }

    fn run(&self, args: Vec<String>) -> Result<(), Error> {
        Command::new("cargo")
            .args(&self.args)
            .args(&args)
            .status()
            .map_err(|_| Error::Cargo)?;

        Ok(())
    }
}
