pub struct SettingsAboutHandler {
    pub version: String,
    pub authors: String,
}

impl SettingsAboutHandler {
    pub fn new() -> Self {
        Self {
            version: "v4.0.0-linux".to_string(),
            authors: "Maku & Contributors".to_string(),
        }
    }

    /// Проверка статуса бинда / быстрого запуска (заглушка для Linux)
    pub fn check_winr_alias_status(&self) -> bool {
        false
    }

    /// Переключение псевдонима (заглушка)
    pub fn toggle_winr_alias(&self, _enable: bool) {
        println!("Переключение псевдонима быстрого запуска: {}", _enable);
    }
}
