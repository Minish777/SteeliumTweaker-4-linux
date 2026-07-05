use std::collections::HashSet;
use sysinfo::{Pid, System};

#[derive(Clone)]
pub struct LinuxProcessItem {
    pub pid: u32,
    pub name: String,
    pub cmdline: String,
    pub memory_bytes: u64,
    pub memory_formatted: String,
    pub cpu_usage: f32,
}

pub struct ProcessManager {
    sys: System,
    hardcoded_exclusions: HashSet<&'static str>,
}

impl ProcessManager {
    pub fn new() -> Self {
        let sys = System::new_all();
        let exclusions = [
            "systemd",
            "kthreadd",
            "init",
            "dbus-daemon",
            "wayland",
            "hyprland",
            "sway",
            "weston",
            "Xorg",
            "waybar",
            "pipewire",
            "wireplumber",
            "bash",
            "zsh",
            "fish",
            "makutweaker",
        ]
        .into();
        Self {
            sys,
            hardcoded_exclusions: exclusions,
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    pub fn get_process_list(&mut self) -> Vec<LinuxProcessItem> {
        let mut result = Vec::new();
        for (pid, process) in self.sys.processes() {
            let name_str = process.name().to_string();
            if self.hardcoded_exclusions.contains(name_str.as_str()) {
                continue;
            }

            let memory_bytes = process.memory();
            let cmdline = process
                .cmd()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            result.push(LinuxProcessItem {
                pid: pid.as_u32(),
                name: name_str,
                cmdline,
                memory_bytes,
                memory_formatted: format!("{} MB", memory_bytes / 1024 / 1024),
                cpu_usage: process.cpu_usage(),
            });
        }
        result.sort_by(|a, b| b.memory_bytes.cmp(&a.memory_bytes));
        result
    }

    pub fn kill_process(&mut self, pid: u32) -> bool {
        if let Some(process) = self.sys.process(Pid::from(pid as usize)) {
            process.kill()
        } else {
            false
        }
    }
}
