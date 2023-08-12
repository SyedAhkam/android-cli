use anyhow::{bail, Context, Result, anyhow};
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
pub struct Run {
    /// Should build in release mode
    #[clap(short, long, default_value="false", action = ArgAction::SetTrue)]
    release: bool,
}

pub fn handle(args: Run) -> Result<()> {
    // Decide gradle subcommand to use
    let build_cmd = match args.release {
        true => "assembleRelease",
        false => "assembleDebug",
    };

    // Trigger a build
    let build_status =
        android_cli::invoke_gradle_command(build_cmd).context("failed to invoke gradle command")?;

    if !build_status.success() {
        bail!("failed to build project");
    }

    // Fetch and deserialize .android
    let dot_android = android_cli::get_dot_android().ok_or_else(|| anyhow!(".android not found, can't launch activity"))?;

    // Try to launch MainActivity using ADB
    let run_status = android_cli::invoke_adb_command(&[
        "shell",
        "am",
        "start",
        "-n",
        &format!("{}/.{}", dot_android.package_id, dot_android.main_activity_name)
    ])
        .context("failed to invoke adb command")?;

    if !run_status.success() {
        bail!("failed to run the APK");
    }

    Ok(())
}
