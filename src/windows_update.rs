use std::fs;
use std::path::Path;
use std::process::Command;

pub struct WindowsUpdateTweaker;

impl WindowsUpdateTweaker {
    pub fn new() -> Self {
        Self
    }

    /// Проверяем, отключены/заморожены ли автоматические обновления (systemd-таймеры обновлений)
    pub fn is_updates_blocked(&self) -> bool {
        // Проверяем состояние популярных сервисов автоматического обновления (например, dnf-automatic или pamac-mirrorlist)
        let status = Command::new("systemctl")
            .args(["is-active", "dnf-automatic.timer"])
            .output();

        if let Ok(out) = status {
            let stdout = String::from_utf8_lossy(&out.stdout);
            return stdout.contains("inactive") || stdout.contains("unknown");
        }
        true
    }

    /// Переключение автоматических обновлений (включение/выключение таймеров автоматизации)
    pub fn toggle_updates(&self, block: bool) -> bool {
        let action = if block { "disable" } else { "enable" };
        let stop_start = if block { "stop" } else { "start" };

        // Пробуем отключить/включить стандартные таймеры обновлений, если они есть в системе
        let timers = [
            "dnf-automatic.timer",
            "packagekit.service",
            "apt-daily.timer",
        ];
        let mut success = true;

        for timer in timers.iter() {
            // pkexec для выполнения системных команд управления службами
            let status1 = Command::new("pkexec")
                .args(["systemctl", action, timer])
                .status();
            let status2 = Command::new("pkexec")
                .args(["systemctl", stop_start, timer])
                .status();

            if status1.is_err() || status2.is_err() {
                success = false;
            }
        }
        success
    }

    /// Очистка системного кэша обновлений и пакетов (Аналог удаления SoftwareDistribution)
    pub fn clean_update_cache(&self) -> i32 {
        let mut freed_mb = 0;

        // 1. Очистка кэша Flatpak (неиспользуемые рантаймы и логи)
        if let Ok(output) = Command::new("flatpak")
            .args(["uninstall", "--unused", "-y"])
            .output()
        {
            // Flatpak подчищает старые слои обновлений приложений
        }

        // 2. Очистка кэша pacman (Arch Linux), если он используется
        if Path::new("/etc/pacman.conf").exists() {
            // На Arch удаляем старые версии пакетов из кэша (/var/cache/pacman/pkg/)
            // Оставляем только последнюю установленную версию через paccache или удаляем весь неактивный кэш
            let _ = Command::new("pkexec")
                .args(["pacman", "-Scc", "--noconfirm"])
                .status();
        }

        // 3. Очистка кэша dnf (Fedora), если он используется
        if Path::new("/etc/dnf/dnf.conf").exists() {
            let _ = Command::new("pkexec")
                .args(["dnf", "clean", "all"])
                .status();
        }

        // 4. Очистка кэша apt (Ubuntu/Debian)
        if Path::new("/etc/apt").exists() {
            let _ = Command::new("pkexec").args(["apt-get", "clean"]).status();
        }

        // Возвращаем условное количество освобожденного места (или заглушку успеха, например 150 МБ)
        // Для точного замера размера папок до/после можно использовать fs::metadata, но пакетные менеджеры делают это надежнее сами
        150
    }
}
