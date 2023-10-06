use anyhow::{Context, Result};
use clap::Parser;

use std::{collections::BTreeMap, path::PathBuf};

use crate::utils::{parse_package_id, prompt_for_input};

const DEFAULT_COMPILE_SDK_VERSION: &str = "33";
const DEFAULT_TARGET_SDK_VERSION: &str = "33";
const DEFAULT_MIN_SDK_VERSION: &str = "24";

#[derive(Parser, Debug)]
pub struct Create {
    #[clap(value_parser)]
    dest: Option<PathBuf>,

    /// The path to the Android SDK [default: $ANDROID_SDK_ROOT]
    #[clap(long)]
    sdk_path: Option<String>,

    /// Project name [default: dest dir path]
    #[clap(long)]
    project_name: Option<String>,

    /// Application name, defaults to the project name
    #[clap(short, long)]
    name: Option<String>,

    /// Package identifier [example: com.example.demo]
    #[clap(long)]
    package_id: Option<String>,

    /// SDK version the app is compiled against
    #[clap(long, default_value = DEFAULT_COMPILE_SDK_VERSION)]
    compile_sdk_version: u32,

    /// SDK version the app is targeting
    #[clap(long, default_value = DEFAULT_TARGET_SDK_VERSION)]
    target_sdk_version: u32,

    /// Minimum SDK version that the app supports
    #[clap(long, default_value = DEFAULT_MIN_SDK_VERSION)]
    min_sdk_version: u32,
}

fn get_vars(args: &Create) -> Result<BTreeMap<String, String>> {
    let mut map = BTreeMap::<String, String>::new();

    // Metadata
    map.insert(
        "project_name".into(),
        args.project_name.as_ref().unwrap().to_owned(),
    );
    map.insert("app_name".into(), args.name.as_ref().unwrap().to_owned());

    // Package identifiers
    let (domain, org, name) = parse_package_id(args.package_id.as_ref().unwrap().to_owned())
        .context("failed to parse package id")?;
    map.insert("package_id_domain".into(), domain);
    map.insert("package_id_org".into(), org);
    map.insert("package_id_name".into(), name);

    // Version numbers
    map.insert(
        "compile_sdk_version".into(),
        args.compile_sdk_version.to_string(),
    );
    map.insert(
        "target_sdk_version".into(),
        args.target_sdk_version.to_string(),
    );
    map.insert("min_sdk_version".into(), args.min_sdk_version.to_string());

    Ok(map)
}

fn post_create(args: Create) -> Result<()> {
    let dest = args.dest.clone().unwrap();

    android_cli::create_local_properties_file(&dest, &args.sdk_path.unwrap())?;
    android_cli::create_dot_android(
        &dest,
        args.project_name.unwrap(),
        args.package_id.unwrap(),
        None,
    )?;

    Ok(())
}

fn ensure_valid_args(args: Create) -> Result<Create> {
    let safe_name = |name: String| name.to_lowercase().replace(" ", "_");

    let dest = args.dest.unwrap_or_else(|| {
        prompt_for_input("Enter destination path", Some(".".into()))
            .unwrap()
            .into()
    });

    // Ensure dest exists
    if !dest.exists() {
        std::fs::create_dir_all(&dest)?;
    }

    // Normalize or Canonicalize the path
    let dest = dest.canonicalize()?;

    let dest_folder_name = dest
        .file_name()
        .unwrap()
        .to_str()
        .map(|name| safe_name(name.into()))
        .unwrap()
        .to_owned();

    let sdk_path = args.sdk_path.unwrap_or_else(|| {
        std::env::var("ANDROID_SDK_ROOT")
            .context("ANDROID_SDK_ROOT not set")
            .unwrap()
    });

    let project_name = args.project_name.map(safe_name).unwrap_or_else(|| {
        prompt_for_input("Enter project name", Some(safe_name(dest_folder_name))).unwrap()
    });

    let app_name = args.name.unwrap_or_else(|| project_name.clone());

    let package_id = args.package_id.unwrap_or_else(|| {
        prompt_for_input(
            "Enter package identifier [suggested to customize]",
            Some(format!("com.example.{}", project_name)),
        )
        .map(safe_name)
        .unwrap()
    });

    Ok(Create {
        dest: Some(dest),
        sdk_path: Some(sdk_path),
        project_name: Some(project_name),
        name: Some(app_name),
        package_id: Some(package_id),
        ..args
    })
}

pub fn handle(args: Create) -> Result<()> {
    let args = ensure_valid_args(args)?;

    // Prepare variables to substitute
    let vars = get_vars(&args)?;

    // Copy template
    let dest = args.dest.as_ref().unwrap();
    android_cli::copy_template(dest, vars)?;

    // Perform post init tasks
    post_create(args)?;

    println!("Project created successfully");

    Ok(())
}
