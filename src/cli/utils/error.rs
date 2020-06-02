use crate::utils::term::{RED_BOLD, TERM_ERR, YELLOW};
use console::Term;
use handlebars::TemplateRenderError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Render(#[from] TemplateRenderError),
    #[error(transparent)]
    Io(#[from] io::Error),
}

impl Error {
    pub fn print_err(self) -> io::Result<()> {
        self.print(&TERM_ERR)
    }

    fn color(self) -> Self {
        match self {
            _ => self,
        }
    }

    pub fn print(self, term: &Term) -> io::Result<()> {
        term.write_str(&format!("{}: ", RED_BOLD.apply_to("error").to_string()))?;
        term.write_line(&self.color().to_string())?;
        term.flush()
    }
}
