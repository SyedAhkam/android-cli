mod utils;

use guidon::{GitOptions, Guidon, TryNew};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

use utils::{find_adb, find_gradle};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const DEFAULT_TEMPLATE_REPO: &str = "https://github.com/SyedAhkam/android-cli-template";
const TEMPLATE_REV: &str = "master";

const DOTFILE_COMMENT: &str = "// DO NOT MODIFY; Generated by Android CLI for internal usage.\n";

#[derive(Debug, Serialize, Deserialize)]
struct DotAndroid {
    pub package_id: String,
    pub gen_at_version: String,
}

pub fn copy_template(dest: &Path, vars: BTreeMap<String, String>) {
    let git_options = GitOptions::builder()
        .repo(DEFAULT_TEMPLATE_REPO)
        .rev(TEMPLATE_REV)
        .build()
        .unwrap();

    let mut guidon = Guidon::try_new(git_options).unwrap();

    guidon.variables(vars);
    guidon.apply_template(dest).unwrap();
}

pub fn create_local_properties_file(root: &Path, sdk_path: &str) {
    let prop_file_path = PathBuf::new().join(root).join("local.properties");

    let content = format!("sdk.dir={}", sdk_path);
    std::fs::write(prop_file_path, content).expect("Unable to write local.properties file")
}

pub fn invoke_gradle_command(cmd: &str) -> Result<ExitStatus, Box<dyn std::error::Error>> {
    let gradle_path = find_gradle().expect("ERROR: Gradle not found on system");

    let mut run = Command::new(gradle_path);
    run.arg(cmd);

    println!(
        "Invoking Gradle: {} {}",
        &run.get_program().to_string_lossy(),
        &run.get_args().map(|arg| arg.to_string_lossy()).join(" ")
    );

    Ok(run.status()?)
}

pub fn invoke_adb_command(args: &[&str]) -> Result<ExitStatus, Box<dyn std::error::Error>> {
    let adb_path = find_adb().expect("ERROR: ADB not found on system");

    let mut run = Command::new(adb_path);
    run.args(args);

    println!(
        "Invoking ADB: {} {}",
        &run.get_program().to_string_lossy(),
        &run.get_args().map(|arg| arg.to_string_lossy()).join(" ")
    );

    Ok(run.status()?)
}

pub fn create_dot_file(dest: &Path, package_id: String) {
    // Construct the structure
    let dot_android = DotAndroid {
        package_id,
        gen_at_version: VERSION.to_owned(),
    };

    // Serialize into Ron
    let mut ron_contents =
        ron::ser::to_string_pretty(&dot_android, ron::ser::PrettyConfig::default()).unwrap();

    // Add a comment at top
    ron_contents.insert_str(0, DOTFILE_COMMENT);

    // Write to file
    let path = PathBuf::from(dest).join(".android");
    std::fs::write(path, ron_contents).expect("failed to write .android file");
}
