use std::fs;
use std::path::Path;
use std::process::Command;

pub struct SettingsAboutHandler {
    pub version: String,
    pub authors: String,
    pub github_url: String,
}

impl SettingsAboutHandler {
    pub fn new() -> Self {
        let build_number = Self::get_build_number();
        Self {
            version: format!("5.7.3 ({})", build_number),
            authors: "Minish777".to_string(),
            github_url: "https://github.com/Minish777".to_string(),
        }
    }

    /// Аналог GetBuildNumberFromResources
    fn get_build_number() -> String {
        // В Rust проще всего внедрить файл сборки прямо в бинарник на этапе компиляции
        // Если файла нет, вернется "N/A"
        let bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/BuildNumber.txt"));
        String::from_utf8_lossy(bytes).trim().to_string()
    }

    /// Открытие GitHub репозитория в браузере
    pub fn open_github(&self) {
        #[cfg(target_os = "windows")]
        let _ = Command::new("cmd")
            .args(["/c", "start", &self.github_url])
            .spawn();

        #[cfg(target_os = "linux")]
        let _ = Command::new("xdg-open").arg(&self.github_url).spawn();
    }

    /// Проверка статуса алиасов Win+R (App Paths)
    pub fn check_winr_alias_status(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            // Используем утилиту reg для проверки существования ключа
            if let Ok(output) = Command::new("reg")
                .args([
                    "query",
                    "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\makut.exe",
                ])
                .output()
            {
                return output.status.success();
            }
        }
        false
    }

    /// Установка/Удаление алиасов для диалогового окна Выполнить (Win+R)
    pub fn toggle_winr_alias(&self, enable: bool) {
        #[cfg(target_os = "windows")]
        {
            let aliases = ["makut.exe", "maku.exe", "mt.exe"];
            // Получаем путь к текущему запущенному .exe файлу
            if let Ok(current_exe) = std::env::current_exe() {
                if let Some(exe_str) = current_exe.to_str() {
                    let exe_dir = current_exe
                        .parent()
                        .unwrap_or(Path::new(""))
                        .to_str()
                        .unwrap_or("");

                    for alias in aliases.iter() {
                        let key_path = format!(
                            "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\{}",
                            alias
                        );
                        if enable {
                            let _ = Command::new("reg")
                                .args([
                                    "add", &key_path, "/f", "/ve", "/t", "REG_SZ", "/d", exe_str,
                                ])
                                .status();
                            let _ = Command::new("reg")
                                .args([
                                    "add", &key_path, "/v", "Path", "/f", "/t", "REG_SZ", "/d",
                                    exe_dir,
                                ])
                                .status();
                        } else {
                            let _ = Command::new("reg")
                                .args(["delete", &key_path, "/f"])
                                .status();
                        }
                    }
                }
            }
        }
    }
}
Ы
