use crate::term::{RED_BOLD, TERM_ERR, TERM_OUT, YELLOW};

use console::Term;
#[cfg(feature = "templating")]
use handlebars::TemplateRenderError;
use thiserror::Error;

use std::{io, process::exit};

pub(crate) const INTERNAL_ERR: &str =
    "Internal error on reign_task. Please create an issue on https://github.com/pksunkara/reign";

#[derive(Error, Debug)]
pub enum Error {
    #[error("unable to execute cargo process")]
    Cargo,
    #[error("unable to find cargo workspace dir")]
    NoWorkspace,
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

impl Error {
    fn color(self) -> Self {
        match self {
            Self::NoArgs(name) => Self::NoArgs(format!("{}", YELLOW.apply_to(name))),
            Self::NoTask(name) => Self::NoTask(format!("{}", YELLOW.apply_to(name))),
            _ => self,
        }
    }

    pub(crate) fn print(self, term: &Term) -> io::Result<()> {
        term.write_str(&format!("{}: ", RED_BOLD.apply_to("error").to_string()))?;
        term.write_line(&self.color().to_string())?;
        term.flush()
    }
}

pub fn finish(err: Option<Error>) {
    let code = if let Some(e) = err {
        e.print(&TERM_ERR).unwrap();
        1
    } else {
        0
    };

    TERM_ERR.flush().unwrap();
    TERM_OUT.flush().unwrap();

    exit(code);
}
