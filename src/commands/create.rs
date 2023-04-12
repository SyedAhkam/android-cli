use clap::Parser;

#[derive(Parser, Debug)]
pub struct Create {
    #[clap(short, long)]
    name: String,
}

pub fn handle(args: Create) {
    println!("Create command: {}", args.name)
}
