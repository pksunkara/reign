use clap::{
    AppSettings::{ColoredHelp, DisableVersionForSubcommands},
    Clap,
};
use reign_task::oclif::finish;

// mod server;
mod new;
mod tasks;

#[derive(Debug, Clap)]
#[clap(name = "reign", version)]
#[clap(global_setting(DisableVersionForSubcommands))]
#[clap(global_setting(ColoredHelp))]
struct Reign {
    #[clap(subcommand)]
    cmd: ReignSubcommand,
}

#[derive(Debug, Clap)]
enum ReignSubcommand {
    New(new::New),
    // #[clap(alias = "s")]
    // Server(server::Server),
    Tasks(tasks::Tasks),
    #[clap(external_subcommand)]
    Other(Vec<String>),
}

fn main() {
    let program = Reign::parse();

    let result = match program.cmd {
        ReignSubcommand::New(x) => x.run(),
        // ReignSubcommand::Server(x) => x.run(),
        ReignSubcommand::Tasks(x) => x.run(),
        ReignSubcommand::Other(x) => tasks::run_task(x),
    };

    finish(result);
}
