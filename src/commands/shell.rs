use anyhow::{bail, Result};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Shell {}

pub fn handle(_args: Shell) -> Result<()> {
    // Try to run adb shell
    let query_status = android_cli::attach_shell()?;

    if !query_status.success() {
        bail!("failed to attach shell");
    }

    Ok(())
}