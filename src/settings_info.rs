pub struct SettingsAboutHandler {
    pub version: String,
    pub authors: String,
}

impl SettingsAboutHandler {
    pub fn new() -> Self {
        Self {
            version: "4.0.0".to_string(),
            authors: "Minish777 Team".to_string(),
        }
    }

    pub fn check_winr_alias_status(&self) -> bool {
        false
    }
}
