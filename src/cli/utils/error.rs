use crate::utils::term::{RED_BOLD, TERM_ERR};

use console::Term;
use handlebars::TemplateRenderError;
use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Render(#[from] TemplateRenderError),
    #[error(transparent)]
    Notify(#[from] notify::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}

impl Error {
    pub fn print_err(self) -> io::Result<()> {
        self.print(&TERM_ERR)
    }

    fn color(self) -> Self {
        self
        // match self {
        //     _ => self,
        // }
    }

    pub fn print(self, term: &Term) -> io::Result<()> {
        term.write_str(&format!("{}: ", RED_BOLD.apply_to("error").to_string()))?;
        term.write_line(&self.color().to_string())?;
        term.flush()
    }
}
