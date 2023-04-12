use clap::{Parser, Subcommand};

mod commands;

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
}

fn main() {
    let args = Cli::parse();

    match args.command {
        SubCommand::Create(args) => commands::create::handle(args),
    }
}
