use std::fs;
use std::process::Command;

pub struct PerformanceHandler {}
impl PerformanceHandler {
    pub fn new() -> Self {
        Self {}
    }
    pub fn toggle_gamemode(&self, enable: bool) { /* код */
    }
    pub fn toggle_gfx_env(&self, enable: bool) { /* код */
    }

    /// Аналог GetActiveScheme и ApplyThrottle из C#
    /// Управляет максимальной частотой и лимитами процессора (в процентах)
    pub fn apply_cpu_throttle(&self, percent: u32) -> Result<(), std::io::Error> {
        if percent < 10 || percent > 100 {
            return Ok(());
        }

        // 1. Способ через современный power-profiles-daemon (если доступен)
        if percent <= 50 {
            let _ = Command::new("powerprofilesctl")
                .arg("set")
                .arg("power-saver")
                .output();
        } else if percent <= 85 {
            let _ = Command::new("powerprofilesctl")
                .arg("set")
                .arg("balanced")
                .output();
        } else {
            let _ = Command::new("powerprofilesctl")
                .arg("set")
                .arg("performance")
                .output();
        }

        // 2. Альтернативный прямой способ (требует root/sudo прав или настроенного polkit)
        // Меняем max_perf_pct для intel_pstate (если процессор Intel Xeon)
        let intel_pstate_path = "/sys/devices/system/cpu/intel_pstate/max_perf_pct";
        if std::path::Path::new(intel_pstate_path).exists() {
            // В продакшене запись потребует повышенных прав (pkexec или запуск через sudo)
            let _ = fs::write(intel_pstate_path, format!("{}", percent));
        }

        Ok(())
    }

    /// Возвращает текущий профиль или приблизительный процент троттлинга
    pub fn get_current_throttle(&self) -> u32 {
        let intel_pstate_path = "/sys/devices/system/cpu/intel_pstate/max_perf_pct";
        if let Ok(content) = fs::read_to_string(intel_pstate_path) {
            if let Ok(val) = content.trim().parse::<u32>() {
                return val;
            }
        }
        100 // По умолчанию без ограничений
    }
}
