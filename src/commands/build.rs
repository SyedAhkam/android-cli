use anyhow::Result;
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
pub struct Build {
    /// Should build in release mode
    #[clap(short, long, default_value="false", action = ArgAction::SetTrue)]
    release: bool,
}

pub fn handle(args: Build) -> Result<()> {
    let status = android_cli::trigger_build(args.release)?;

    match status.success() {
        true => println!("Success!"),
        false => println!("Failed while executing Gradle."),
    }

    Ok(())
}
