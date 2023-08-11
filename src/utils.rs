use which::which;

pub fn prompt_for_input(prompt: &str) -> String {
    dialoguer::Input::<String>::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .unwrap()
}

pub fn find_gradle() -> Option<String> {
    if std::path::Path::new("./gradlew").exists() {
        return Some("./gradlew".to_owned());
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
