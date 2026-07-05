use std::process::Command;
use sysinfo::{System, SystemExt, ProcessExt};

pub struct SystemInfoHandler {
    sys: System,
}

impl SystemInfoHandler {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }

    pub fn get_cpu_info(&self) -> (String, u32, String) {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_name = sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand())
            .unwrap_or("Unknown CPU")
            .to_string();

        let cores = num_cpus::get() as u32;
        let freq = format!("{}MHz", sys.cpus().first().map(|c| c.frequency()).unwrap_or(0));

        (cpu_name, cores, freq)
    }

    pub fn get_ram_info(&self) -> String {
        let mut sys = System::new_all();
        sys.refresh_all();

        let total_ram = sys.total_memory() / 1024 / 1024; // в ГБ
        let used_ram = sys.used_memory() / 1024 / 1024;

        format!("ОЗУ: {} МБ / {} МБ", used_ram, total_ram)
    }

    pub fn get_disk_info(&self) -> String {
        let output = Command::new("df")
            .arg("-h")
            .arg("/")
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                stdout.lines().last().unwrap_or("").to_string()
            }
            Err(_) => "Невозможно получить информацию о диске".to_string(),
        }
    }

    pub fn get_uptime(&self) -> String {
        let mut sys = System::new_all();
        sys.refresh_all();

        let uptime_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let days = uptime_secs / 86400;
        let hours = (uptime_secs % 86400) / 3600;
        let minutes = (uptime_secs % 3600) / 60;

        format!("{}д {}ч {}м", days, hours, minutes)
    }
}