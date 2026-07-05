use std::fs;
use std::path::Path;
use std::process::Command;

pub struct SystemTweaksHandler;

impl SystemTweaksHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn toggle_telemetry(&self, disable: bool) {
        let services = vec![
            "systemd-journal-gateway",
            "systemd-journal-remote",
            "apport",
            "ubuntu-report",
        ];

        for service in services {
            let action = if disable { "disable" } else { "enable" };
            let _ = Command::new("systemctl")
                .args(&[action, service])
                .status();
        }

        // Отключаем данные о использовании в GNOME
        if disable {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let dconf_setting = "org.gnome.shell/disable-user-extensions-setting";
            let _ = Command::new("dconf")
                .args(&["write", dconf_setting, "true"])
                .status();
        }
    }

    pub fn has_battery(&self) -> bool {
        Path::new("/sys/class/power_supply/BAT0").exists()
            || Path::new("/sys/class/power_supply/BAT1").exists()
    }

    pub fn check_telemetry_disabled(&self) -> bool {
        !Path::new("/var/log/ubuntu-report-stats").exists()
    }

    pub fn generate_battery_report(&self) {
        if self.has_battery() {
            let _ = Command::new("bash")
                .arg("-c")
                .arg("cat /sys/class/power_supply/BAT0/energy_full_design /sys/class/power_supply/BAT0/energy_full 2>/dev/null")
                .status();
        }
    }
}