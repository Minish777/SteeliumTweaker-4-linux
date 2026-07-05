use std::process::Command;

pub struct WindowsUpdateTweaker;

impl WindowsUpdateTweaker {
    pub fn new() -> Self {
        Self
    }

    pub fn toggle_updates(&self, block: bool) -> bool {
        let action = if block { "stop" } else { "start" };
        
        Command::new("systemctl")
            .args(&[action, "apt-daily"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    pub fn is_updates_blocked(&self) -> bool {
        // Проверяем статус apt-daily сервиса
        Command::new("systemctl")
            .args(&["is-active", "apt-daily"])
            .status()
            .map(|s| !s.success())
            .unwrap_or(false)
    }

    pub fn clean_update_cache(&self) -> u64 {
        let output = Command::new("bash")
            .arg("-c")
            .arg("apt-get clean && du -sh /var/cache/apt 2>/dev/null | awk '{print $1}'")
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                stdout.trim().len() as u64
            }
            Err(_) => 0,
        }
    }
}