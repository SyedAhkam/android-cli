use clap::{Parser, Subcommand};

mod commands;
mod utils;

#[derive(Parser, Debug)]
#[clap(name = "Android CLI")]
#[clap(author, version, about)]
#[clap(author = "Syed Ahkam <smahkam57@gmail.com>")]
#[clap(arg_required_else_help = true)]
struct Cli {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Create(commands::create::Create),
    Build(commands::build::Build),
    Install(commands::install::Install),
    Run(commands::run::Run),
    Launch(commands::launch::Launch),
    Devices(commands::devices::Devices)
}

fn main() {
    // Initialize the logger with the log level info
    env_logger::init();

    let args = Cli::parse();

    let result = match args.command {
        SubCommand::Create(args) => commands::create::handle(args),
        SubCommand::Build(args) => commands::build::handle(args),
        SubCommand::Install(args) => commands::install::handle(args),
        SubCommand::Run(args) => commands::run::handle(args),
        SubCommand::Launch(args) => commands::launch::handle(args),
        SubCommand::Devices(args) => commands::devices::handle(args)
    };

    if result.is_err() {
        eprintln!("ERROR: {:?}", result.unwrap_err());
    }
}
