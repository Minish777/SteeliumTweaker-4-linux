use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct PersonalizationHandler {
    // Хранит состояние (включен ли твик)
    is_hyprland: bool,
}

impl PersonalizationHandler {
    pub fn new() -> Self {
        let is_hyprland = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok();
        Self { is_hyprland }
    }

    /// Аналог скрытия/показа скрытых файлов (в Linux обычно через конфиг глобального файл-менеджера,
    /// например, Nautilus, Dolphin или через переменные среды)
    pub fn toggle_hidden_files(&self, show: bool) {
        // Пример для Dolphin (KDE):
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
        path.push("dolphinrc");
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                let updated = if show {
                    content.replace("ShowHiddenFiles=false", "ShowHiddenFiles=true")
                } else {
                    content.replace("ShowHiddenFiles=true", "ShowHiddenFiles=false")
                };
                let _ = fs::write(&path, updated);
            }
        }
    }

    /// Твик для смены/перезапуска обоев через awww (бывший swww)
    pub fn toggle_wallpaper_daemon(&self, start: bool) {
        if start {
            let _ = Command::new("awww-daemon").spawn();
        } else {
            let _ = Command::new("killall").arg("awww-daemon").output();
        }
    }

    /// Пример твика размытия (Glassmorphic эффект), который часто используется в Hyprland
    pub fn toggle_glassmorphism(&self, enable: bool) {
        if !self.is_hyprland {
            return;
        }

        // В новых версиях Hyprland (0.55+) конфигурация пишется на Lua.
        // Здесь можно вызывать hyprctl для динамического переключения размытия окон:
        let value = if enable { "1" } else { "0" };
        let _ = Command::new("hyprctl")
            .args(&["keyword", "decoration:blur:enabled", value])
            .output();
    }
}
