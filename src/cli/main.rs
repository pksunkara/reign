use clap::{
    AppSettings::{ColoredHelp, VersionlessSubcommands},
    Clap,
};

use std::process::exit;

mod generate;
mod new;
mod server;

mod templates;
mod utils;

use utils::term::{TERM_ERR, TERM_OUT};

#[derive(Debug, Clap)]
#[clap(name = "reign")]
#[clap(global_setting(VersionlessSubcommands))]
#[clap(global_setting(ColoredHelp))]
struct Reign {
    #[clap(subcommand)]
    cmd: ReignSubcommand,
}

#[derive(Debug, Clap)]
enum ReignSubcommand {
    New(new::New),
    #[clap(alias = "s")]
    Server(server::Server),
    #[clap(alias = "g")]
    Generate(generate::Generate),
}

fn main() {
    let program = Reign::parse();

    // TODO: cli: tasks with feature
    let err = match program.cmd {
        ReignSubcommand::New(x) => x.run(),
        ReignSubcommand::Server(x) => x.run(),
        ReignSubcommand::Generate(x) => x.run(),
    }
    .err();

    let code = if let Some(e) = err {
        e.print_err().unwrap();
        1
    } else {
        0
    };

    TERM_ERR.flush().unwrap();
    TERM_OUT.flush().unwrap();

    exit(code)
}
