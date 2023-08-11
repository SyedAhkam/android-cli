use std::path::PathBuf;

use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
pub struct Install {
    /// Should install release APK
    #[clap(short, long, default_value="false", action = ArgAction::SetTrue)]
    release: bool,
}

pub fn handle(args: Install) {
    let output_dir = PathBuf::from("app/build/outputs/apk");

    let apk_path = match args.release {
        true => output_dir.join("release/app-release.apk"),
        false => output_dir.join("debug/app-debug.apk")
    };

    let status = android_cli::invoke_adb_command(&[
        "install",
        apk_path.to_str().unwrap()
    ])
    .unwrap();

    match status.success() {
        true => println!("Successfully installed APK."),
        false => eprintln!("Failed to install APK"),
    };
}
