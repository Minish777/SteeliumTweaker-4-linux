use std::process::Command;

pub struct SatHandler;

impl SatHandler {
    pub fn new() -> Self {
        Self
    }

    /// Планирует выключение системы через указанное количество минут
    pub fn shutdown(&self, minutes: u64) -> bool {
        // В Linux shutdown принимает время в формате +минуты (например, +10, +60)
        let time_arg = format!("+{}", minutes);

        let status = Command::new("shutdown").arg(time_arg).status();

        match status {
            Ok(s) => s.success(),
            Err(_) => {
                // Если обычный shutdown не сработал (требует root-прав на некоторых старых конфигурациях),
                // пробуем вызвать через pkexec или systemctl (обычно logind позволяет пользователям слать shutdown +M)
                Command::new("pkexec")
                    .args(["shutdown", &format!("+{}", minutes)])
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
            }
        }
    }

    /// Отменяет ранее запланированное выключение системы
    pub fn cancel(&self) -> bool {
        let status = Command::new("shutdown").arg("-c").status();

        match status {
            Ok(s) => s.success(),
            Err(_) => Command::new("pkexec")
                .args(["shutdown", "-c"])
                .status()
                .map(|s| s.success())
                .unwrap_or(false),
        }
    }
}
