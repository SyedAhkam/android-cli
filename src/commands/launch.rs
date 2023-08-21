use clap::Parser;
use anyhow::{Result, bail, anyhow};

#[derive(Parser, Debug)]
pub struct Launch {}

pub fn handle(_args: Launch) -> Result<()> {
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
