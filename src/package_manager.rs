use std::process::Command;

#[derive(Clone)]
pub struct LinuxPackageItem {
    pub id: String,
    pub name: String,
    pub category: String,
}

pub struct PackageManagerHandler;

impl PackageManagerHandler {
    pub fn new() -> Self {
        Self
    }

    /// Аналог CheckInstalledUWPAppsAsync
    /// Сканирует установленные пакеты (например, Flatpak — стандарт для современных Linux DE)
    pub fn get_installed_packages(&self) -> Vec<LinuxPackageItem> {
        let mut packages = Vec::new();

        // Пробуем вызвать flatpak list (если используется)
        if let Ok(output) = Command::new("flatpak")
            .args(&["list", "--columns=application,name"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    packages.push(LinuxPackageItem {
                        id: parts[0].to_string(),
                        name: parts[1..].join(" "),
                        category: "Flatpak".to_string(),
                    });
                }
            }
        }

        // Если список пуст, можем зашить пару дефолтных системных "bloatware" компонентов для примера
        if packages.is_empty() {
            packages.push(LinuxPackageItem {
                id: "org.gnome.Tour".to_string(),
                name: "GNOME Tour".to_string(),
                category: "System".to_string(),
            });
            packages.push(LinuxPackageItem {
                id: "tracker3".to_string(),
                name: "Tracker3 Indexer".to_string(),
                category: "Services".to_string(),
            });
        }

        packages
    }

    /// Выполняет реальное удаление пакета из системы
    pub fn uninstall_package(&self, package_id: &str) -> bool {
        println!("Выполняется удаление пакета: {}", package_id);

        let status = if package_id.contains("org.") {
            // Если это Flatpak
            Command::new("flatpak")
                .args(&["uninstall", "-y", package_id])
                .status()
        } else {
            // Если это системный пакет (например, для Arch Linux)
            Command::new("sudo")
                .args(&["pacman", "-Rns", "--noconfirm", package_id])
                .status()
        };

        status.map(|s| s.success()).unwrap_or(false)
    }
}
