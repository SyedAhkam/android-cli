use anyhow::{anyhow, bail, Result};
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
pub struct Run {
    /// Should build in release mode
    #[clap(short, long, default_value="false", action = ArgAction::SetTrue)]
    release: bool,
}

pub fn handle(args: Run) -> Result<()> {
    // Trigger a build
    let build_status = android_cli::trigger_build(args.release)?;
    if !build_status.success() {
        bail!("failed to build project");
    }

    // Request an install to device
    let install_status = android_cli::install_apk(args.release)?;
    if !install_status.success() {
        bail!("failed to install APK");
    }

    // Fetch and deserialize .android
    let dot_android = android_cli::get_dot_android()
        .ok_or_else(|| anyhow!(".android not found, can't launch activity"))?;

    // Try to launch MainActivity using ADB
    let launch_status =
        android_cli::launch_activity(dot_android.package_id, dot_android.main_activity_name)?;
    if !launch_status.success() {
        bail!("failed to launch activity");
    }

    Ok(())
}
