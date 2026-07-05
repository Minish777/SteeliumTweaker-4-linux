use std::process::Command;

pub struct SystemTweaksHandler;

impl SystemTweaksHandler {
    pub fn new() -> Self {
        Self
    }

    /// Проверяет, запущен ли твикер на устройстве с батареей (ноутбук/планшет)
    /// Аналог C# метода HasBattery() через консольный вызов WMIC / PowerShell
    pub fn has_battery(&self) -> bool {
        if let Ok(output) = Command::new("powershell")
            .args(["-Command", "Get-CimInstance -ClassName Win32_Battery"])
            .output()
        {
            // Если вывод не пустой, значит устройство управления питанием (батарея) найдено
            return !output.stdout.is_empty();
        }
        false
    }

    /// Генерация отчета о состоянии батареи (Аналог вызова powercfg /batteryreport)
    pub fn generate_battery_report(&self) -> bool {
        if let Ok(home) = std::env::var("USERPROFILE") {
            let report_path = format!(r"{}\Desktop\battery-report.html", home);
            if let Ok(status) = Command::new("powercfg")
                .args(["/batteryreport", "/output", &report_path])
                .status()
            {
                return status.success();
            }
        }
        false
    }

    /// Проверка: отключена ли телеметрия в данный момент
    pub fn check_telemetry_disabled(&self) -> bool {
        if let Ok(output) = Command::new("reg")
            .args([
                "query",
                r"HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                "/v",
                "AllowTelemetry",
            ])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Если значение равно 0, значит телеметрия уже успешно заблокирована
            return stdout.contains("0x0") || stdout.contains("0");
        }
        false
    }

    /// Включение или отключение телеметрии и слежки Windows (Аналог telemetry_Toggled)
    pub fn toggle_telemetry(&self, disable: bool) {
        let val = if disable { "0" } else { "1" };
        let no_gen_ticket = if disable { "1" } else { "0" };
        let disable_engine = if disable { "1" } else { "0" };

        // 1. Политики сбора данных (DataCollection)
        let _ = Command::new("reg")
            .args([
                "add",
                r"HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                "/v",
                "AllowTelemetry",
                "/t",
                "REG_DWORD",
                "/d",
                val,
                "/f",
            ])
            .status();
        let _ = Command::new("reg")
            .args([
                "add",
                r"HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                "/v",
                "MaxTelemetryAllowed",
                "/t",
                "REG_DWORD",
                "/d",
                val,
                "/f",
            ])
            .status();
        let _ = Command::new("reg")
            .args([
                "add",
                r"HKLM\SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                "/v",
                "DoNotShowFeedbackNotifications",
                "/t",
                "REG_DWORD",
                "/d",
                if disable { "1" } else { "0" },
                "/f",
            ])
            .status();

        // 2. Защита платформы программного обеспечения (Software Protection Platform)
        let _ = Command::new("reg")
            .args(["add", r"HKLM\SOFTWARE\Policies\Microsoft\Windows NT\CurrentVersion\Software Protection Platform", "/v", "NoGenTicket", "/t", "REG_DWORD", "/d", no_gen_ticket, "/f"])
            .status();

        // 3. Компоненты совместимости приложений (AppCompat)
        let appcompat_path = r"HKLM\SOFTWARE\Policies\Microsoft\Windows\AppCompat";
        let _ = Command::new("reg")
            .args([
                "add",
                appcompat_path,
                "/v",
                "AITEnable",
                "/t",
                "REG_DWORD",
                "/d",
                val,
                "/f",
            ])
            .status();
        let _ = Command::new("reg")
            .args([
                "add",
                appcompat_path,
                "/v",
                "AllowTelemetry",
                "/t",
                "REG_DWORD",
                "/d",
                val,
                "/f",
            ])
            .status();
        let _ = Command::new("reg")
            .args([
                "add",
                appcompat_path,
                "/v",
                "DisableEngine",
                "/t",
                "REG_DWORD",
                "/d",
                disable_engine,
                "/f",
            ])
            .status();
        let _ = Command::new("reg")
            .args([
                "add",
                appcompat_path,
                "/v",
                "DisableInventory",
                "/t",
                "REG_DWORD",
                "/d",
                disable_engine,
                "/f",
            ])
            .status();
        let _ = Command::new("reg")
            .args([
                "add",
                appcompat_path,
                "/v",
                "DisablePCA",
                "/t",
                "REG_DWORD",
                "/d",
                disable_engine,
                "/f",
            ])
            .status();
        let _ = Command::new("reg")
            .args([
                "add",
                appcompat_path,
                "/v",
                "DisableUAR",
                "/t",
                "REG_DWORD",
                "/d",
                disable_engine,
                "/f",
            ])
            .status();
    }
}
