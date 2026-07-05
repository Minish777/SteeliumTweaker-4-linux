pub struct ConfirmationDialogHandler {
    required_phrase: &'static str,
}

impl ConfirmationDialogHandler {
    pub fn new() -> Self {
        Self {
            required_phrase: "ILOVESTEELIUMTWEAKER",
        }
    }

    /// Проверяет, совпадает ли введенный текст с секретной фразой
    /// На основе этого Slint будет включать/выключать доступность кнопки OK (Primary Button)
    pub fn validate_input(&self, input: String) -> bool {
        input.trim() == self.required_phrase
    }

    /// Симуляция действия при успешном подтверждении (например, удаление компонента)
    pub fn execute_removal(&self, app_name: String) {
        println!(
            "Пользователь подтвердил операцию. Удаляем компонент: {}",
            app_name
        );

        // Здесь в будущем будет вызов пакетного менеджера (например, apt purge, pacman -R или flatpak uninstall)
    }
}
