#[cfg(feature = "templating")]
use handlebars::TemplateRenderError;
use oclif::{term::ERR_YELLOW, CliError};
use thiserror::Error;

use std::io;

pub(crate) const INTERNAL_ERR: &str =
    "Internal error on reign_task. Please create an issue on https://github.com/pksunkara/reign";

#[derive(Error, Debug)]
pub enum Error {
    #[error("unable to execute cargo process")]
    Cargo,
    #[error("need at least one task to be specified since {0} is a group of tasks")]
    NoArgs(String),
    #[error("there is no task named {0}")]
    NoTask(String),

    #[cfg(feature = "templating")]
    #[error(transparent)]
    Render(#[from] TemplateRenderError),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl CliError for Error {
    fn color(self) -> Self {
        match self {
            Self::NoArgs(name) => Self::NoArgs(format!("{}", ERR_YELLOW.apply_to(name))),
            Self::NoTask(name) => Self::NoTask(format!("{}", ERR_YELLOW.apply_to(name))),
            _ => self,
        }
    }
}
