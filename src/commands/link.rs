use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;

use crate::utils::{prompt_for_input, safe_name};

#[derive(Parser, Debug)]
pub struct Link {}

pub fn handle(_args: Link) -> Result<()> {
    let project_name = prompt_for_input("Enter project name", None)?;
    let package_id = prompt_for_input("Enter package identifier", None)
        .map(safe_name)
        .unwrap();
    let main_activity_name = prompt_for_input(
        "Main Activity name",
        Some(android_cli::DEFAULT_MAIN_ACTIVITY.into()),
    )?;

    android_cli::create_dot_android(
        &Path::new("."),
        project_name,
        package_id,
        Some(main_activity_name.into()),
    )
    .context("Failed to link project")?;

    println!("Successfully linked the project");

    Ok(())
}
