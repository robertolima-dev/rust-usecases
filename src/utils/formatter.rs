pub fn generate_username_from_email(email: &str) -> String {
    let prefix = email.split('@').next().unwrap_or("");

    prefix
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}
