use anyhow::{bail, Result};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Devices {}

pub fn handle(_args: Devices) -> Result<()> {
    // Try to run adb devices
    let query_status = android_cli::query_devices()?;

    if !query_status.success() {
        bail!("failed to query devices");
    }

    Ok(())
}
