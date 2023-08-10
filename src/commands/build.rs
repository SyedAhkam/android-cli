use std::process::Command;

use clap::{
    Parser,
    ArgAction
};

use crate::utils::find_gradle;

#[derive(Parser, Debug)]
pub struct Build {
    /// Should build in release mode
    #[clap(short, long, default_value="false", action = ArgAction::SetTrue)]
    release: bool
}

pub fn handle(args: Build) {
    let gradle_path = find_gradle().expect("ERROR: Gradle not found on system");

    // Decide gradle subcommand to use
    let cmd = match args.release {
        true => "assembleRelease",
        false => "assembleDebug"
    };

    // Invoke gradle as child process
    println!("Invoking Gradle: {}", gradle_path);
    let status = Command::new(gradle_path)
        .arg(cmd)
        .status()
        .unwrap();

    match status.success() {
        true => println!("Success!"),
        false => println!("Failed while executing Gradle.")
    }
}
