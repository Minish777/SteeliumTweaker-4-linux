use crate::performance::PerformanceHandler as PerformanceTweaker;
use crate::system_tweaks::SystemTweaksHandler;
use crate::windows_update::WindowsUpdateTweaker;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::rc::Rc;

pub struct QuickSettingsHandler {
    perf: Rc<PerformanceTweaker>,
    sys: Rc<SystemTweaksHandler>,
    updates: Rc<WindowsUpdateTweaker>,
}

impl QuickSettingsHandler {
    pub fn new(
        perf: Rc<PerformanceTweaker>,
        sys: Rc<SystemTweaksHandler>,
        updates: Rc<WindowsUpdateTweaker>,
    ) -> Self {
        Self { perf, sys, updates }
    }

    /// Возвращает количество быстрых настроек, которые еще можно применить
    pub fn get_applicable_tweaks_count(&self) -> usize {
        let mut count = 0;

        // Проверяем, отключена ли телеметрия
        if !self.sys.check_telemetry_disabled() {
            count += 1;
        }

        // Проверяем, включен ли показ скрытых файлов в GNOME/Nautilus (если он есть)
        let output = Command::new("gsettings")
            .args(["get", "org.gnome.nautilus.preferences", "show-hidden-files"])
            .output();
        if let Ok(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if stdout.contains("false") {
                count += 1;
            }
        }

        count
    }

    /// Применить комплексный пресет MakuOS
    pub fn apply_makuos_preset(&self) {
        // 1. Отключаем системную телеметрию Linux через наш хэндлер
        self.sys.toggle_telemetry(true);

        // 2. Блокируем нежелательные обновления/сервисы
        let _ = self.updates.toggle_updates(true);

        // 3. Включаем отображение скрытых файлов в Nautilus
        let _ = Command::new("gsettings")
            .args([
                "set",
                "org.gnome.nautilus.preferences",
                "show-hidden-files",
                "true",
            ])
            .status();

        // 4. Отключаем сбор статистики и трекеры GNOME/Ubuntu
        let _ = Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.privacy",
                "report-technical-problems",
                "false",
            ])
            .status();
        let _ = Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.privacy",
                "send-software-usage-stats",
                "false",
            ])
            .status();

        // 5. Твик отзывчивости интерфейса
        let _ = Command::new("gsettings")
            .args([
                "set",
                "org.gnome.desktop.interface",
                "enable-animations",
                "true",
            ])
            .status();

        // 6. Низкоуровневые твики ядра Linux для отзывчивости под нагрузкой (уменьшаем Swappiness)
        let _ = Command::new("pkexec")
            .args(["sysctl", "vm.swappiness=10"])
            .status();
    }
}
