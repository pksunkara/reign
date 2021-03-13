use console::{Style, Term};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TERM_ERR: Term = Term::stderr();
    pub static ref TERM_OUT: Term = Term::stdout();
    pub static ref YELLOW: Style = Style::new().for_stderr().yellow();
    pub static ref YELLOW_BOLD: Style = Style::new().for_stderr().yellow().bold();
    pub static ref RED_BOLD: Style = Style::new().for_stderr().red().bold();
    pub static ref GREEN_BOLD: Style = Style::new().for_stderr().green().bold();
}
