use clap::Parser;
use reign_task::oclif::finish;

// mod server;
mod new;
mod tasks;

#[derive(Debug, Parser)]
#[clap(name = "reign", version)]
struct Reign {
    #[clap(subcommand)]
    cmd: ReignSubcommand,
}

#[derive(Debug, Parser)]
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
