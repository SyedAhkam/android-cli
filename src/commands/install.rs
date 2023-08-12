use anyhow::Result;
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
pub struct Install {
    /// Should install release APK
    #[clap(short, long, default_value="false", action = ArgAction::SetTrue)]
    release: bool,
}

pub fn handle(args: Install) -> Result<()> {
    let status = android_cli::install_apk(args.release)?;

    match status.success() {
        true => println!("Successfully installed APK."),
        false => eprintln!("Failed to install APK"),
    };

    Ok(())
}
