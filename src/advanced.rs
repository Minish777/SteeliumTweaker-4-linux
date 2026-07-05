use std::process::Command;

pub struct AdvancedHandler;

impl AdvancedHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn control_service(&self, name: &str, action: &str) -> bool {
        let valid_actions = ["start", "stop", "restart", "enable", "disable"];
        if !valid_actions.contains(&action) {
            return false;
        }

        let status = Command::new("systemctl")
            .arg(action)
            .arg(name)
            .status();

        match status {
            Ok(s) => s.success(),
            Err(_) => false,
        }
    }

    pub fn disable_bluetooth(&self) -> bool {
        Command::new("systemctl")
            .args(&["disable", "bluetooth"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    pub fn disable_wifi(&self) -> bool {
        let _ = Command::new("nmcli")
            .args(&["radio", "wifi", "off"])
            .status();
        true
    }

    pub fn enable_firewall(&self) -> bool {
        Command::new("systemctl")
            .args(&["enable", "ufw"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}
