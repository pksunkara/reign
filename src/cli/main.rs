use clap::{
    AppSettings::{ColoredHelp, VersionlessSubcommands},
    Clap,
};

mod new;

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
}

fn main() {
    let program = Reign::parse();

    match program.cmd {
        ReignSubcommand::New(x) => x.run(),
    }
    .unwrap()
}
