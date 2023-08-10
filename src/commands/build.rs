use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
pub struct Build {
    /// Should build in release mode
    #[clap(short, long, default_value="false", action = ArgAction::SetTrue)]
    release: bool,
}

pub fn handle(args: Build) {
    // Decide gradle subcommand to use
    let cmd = match args.release {
        true => "assembleRelease",
        false => "assembleDebug",
    };

    // Invoke gradle as child process
    let status = android_cli::invoke_gradle_command(cmd).unwrap();

    match status.success() {
        true => println!("Success!"),
        false => println!("Failed while executing Gradle."),
    }
}
