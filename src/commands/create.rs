use std::{collections::BTreeMap, path::PathBuf};

use clap::Parser;

use crate::utils::prompt_for_input;

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

fn get_vars(args: &Create) -> BTreeMap<String, String> {
    let mut map = BTreeMap::<String, String>::new();

    // FIXME: this is a hack, we should use a proper parser
    let get_package_id = |package_id: String| {
        let mut parts = package_id.split('.');
        let domain = parts.next().unwrap();
        let org = parts.next().unwrap();
        let name = parts.next().unwrap();

        (domain.to_owned(), org.to_owned(), name.to_owned())
    };

    // Metadata
    map.insert(
        "project_name".into(),
        args.project_name.as_ref().unwrap().to_owned(),
    );
    map.insert("app_name".into(), args.name.as_ref().unwrap().to_owned());

    // Package identifiers
    let (domain, org, name) = get_package_id(args.package_id.as_ref().unwrap().to_owned());
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

    map
}

fn post_create(args: Create) {
    android_cli::create_local_properties_file(args.dest.unwrap().as_path(), &args.sdk_path.unwrap())
}

fn ensure_valid_args(args: Create) -> Create {
    let dest = args
        .dest
        .unwrap_or_else(|| prompt_for_input("Enter destination path").into());

    let dest_folder_name = dest.file_name().unwrap().to_str().unwrap().to_owned(); // FIXME: this could fail on non-unicode paths

    let sdk_path = args
        .sdk_path
        .unwrap_or_else(|| std::env::var("ANDROID_SDK_ROOT").expect("ANDROID_SDK_ROOT not set"));

    let project_name = args
        .project_name
        .unwrap_or_else(|| dest_folder_name)
        .to_lowercase()
        .replace(" ", "_"); // Perform some cleanup

    let app_name = args.name.unwrap_or_else(|| project_name.clone());

    let package_id = args.package_id.unwrap_or_else(|| {
        prompt_for_input("Enter package identifier [example: com.example.demo]")
            .to_lowercase()
            .replace(" ", "_")
    });

    Create {
        dest: Some(dest),
        sdk_path: Some(sdk_path),
        project_name: Some(project_name),
        name: Some(app_name),
        package_id: Some(package_id),
        ..args
    }
}

pub fn handle(args: Create) {
    let args = ensure_valid_args(args);

    // Prepare variables to substitute
    let vars = get_vars(&args);

    // Copy template
    let dest = args.dest.as_ref().unwrap();
    android_cli::copy_template(dest, vars);

    // Perform post init tasks
    post_create(args);

    println!("Project created successfully");
}
