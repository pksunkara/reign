mod new;

use structopt::{
    clap::AppSettings::{ColoredHelp, VersionlessSubcommands},
    StructOpt,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "reign")]
#[structopt(global_settings = &[VersionlessSubcommands, ColoredHelp])]
struct Reign {
    #[structopt(subcommand)]
    cmd: ReignSubcommand,
}

#[derive(Debug, StructOpt)]
enum ReignSubcommand {
    New(new::New),
}

fn main() {
    let program = Reign::from_args();

    match program.cmd {
        ReignSubcommand::New(x) => x.run(),
    }
    .unwrap()
}
