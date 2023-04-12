use clap::Parser;

/// Create, build, and release Android apps faster without Android Studio
#[derive(Parser, Debug)]
#[clap(name = "android")]
#[clap(author = "Syed Ahkam <smahkam57@gmail.com>")]
#[clap(arg_required_else_help = true)]
struct Cli {}

fn main() {
    let _args = Cli::parse();

    println!("Hello, world!");
}
