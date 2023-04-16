pub fn prompt_for_input(prompt: &str) -> String {
    dialoguer::Input::<String>::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .unwrap()
}
