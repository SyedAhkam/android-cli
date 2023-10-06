#![allow(dead_code)]

use anyhow::{anyhow, Context, Result};
use which::which;

use std::path::Path;

pub fn prompt_for_input(prompt: &str, default: Option<String>) -> Result<String> {
    let theme = dialoguer::theme::ColorfulTheme::default();
    let mut builder = dialoguer::Input::<String>::with_theme(&theme);

    if let Some(default_value) = default {
        builder.default(default_value);
    }

    Ok(builder
        .with_prompt(prompt)
        .interact_text()
        .context("failed to prompt user")?)
}

// FIXME: this is a hack, we should use a proper parser
pub fn parse_package_id(package_id: String) -> Result<(String, String, String)> {
    let mut parts = package_id.split('.');
    let domain = parts
        .next()
        .ok_or_else(|| anyhow!("domain part missing in package"))?;

    let org = parts
        .next()
        .ok_or_else(|| anyhow!("org part missing in package"))?;

    let name = parts
        .next()
        .ok_or_else(|| anyhow!("name part missing in package"))?;

    anyhow::Ok((domain.to_owned(), org.to_owned(), name.to_owned()))
}

pub fn safe_name(name: String) -> String {
    name.to_lowercase().replace(" ", "_")
}

pub fn find_gradle() -> Option<String> {
    let curr_dir_gradle = if cfg!(windows) {
        "./gradlew.bat"
    } else {
        "./gradlew"
    };
    
    if Path::new(curr_dir_gradle).exists() {
        return Some(curr_dir_gradle.to_owned());
    }

    if which("gradle").is_ok() {
        return Some("gradle".to_owned());
    }

    None
}

pub fn find_adb() -> Option<String> {
    if which("adb").is_ok() {
        return Some("adb".to_owned());
    }

    None
}
