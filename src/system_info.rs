use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};
use sysinfo::{Component, Disks, System};

pub struct GpuInfo {
    pub name: String,
}

pub struct SystemInfoHandler {
    sys: System,
}

impl SystemInfoHandler {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    /// Собирает базовые характеристики процессора
    pub fn get_cpu_info(&mut self) -> (String, usize, String) {
        // В sysinfo 0.30 вместо refresh_cpu_all используется refresh_cpu_usage
        self.sys.refresh_cpu_usage();

        let processor_name = self
            .sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Unknown Linux CPU".to_string());

        let cores = self.sys.physical_core_count().unwrap_or(0);

        let max_freq = self
            .sys
            .cpus()
            .iter()
            .map(|c| c.frequency())
            .max()
            .unwrap_or(0);
        let freq_ghz = format!("{:.2} GHz", max_freq as f64 / 1000.0);

        (processor_name, cores, freq_ghz)
    }

    /// Получает информацию об общей оперативной памяти
    pub fn get_ram_info(&mut self) -> String {
        self.sys.refresh_memory();
        // В sysinfo 0.30 total_memory() возвращает БАЙТЫ, а не КИЛОБАЙТЫ
        let total_bytes = self.sys.total_memory();
        let total_gb = total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        format!("{:.0} GB / DDR or LPDDR", total_gb)
    }

    /// Скан накопителей (аналог LoadStorageList / StorageHelper)
    pub fn get_storage_list(&self) -> Vec<(String, String)> {
        let disks = Disks::new_with_refreshed_list();
        disks
            .iter()
            .map(|disk| {
                let name = disk.name().to_string_lossy().into_owned();
                let display_name = if name.is_empty() {
                    "System Disk".to_string()
                } else {
                    name
                };
                let size_gb = disk.total_space() as f64 / (1024.0 * 1024.0 * 1024.0);
                (display_name, format!("{:.1} GB", size_gb))
            })
            .collect()
    }

    /// Получение информации о GPU в Linux (партисипативный парсинг через lspci)
    pub fn get_gpu_list(&self) -> Vec<GpuInfo> {
        let mut gpus = Vec::new();
        if let Ok(output) = std::process::Command::new("sh")
            .args(&["-c", "lspci | grep -E 'VGA|3D'"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Some(pos) = line.find("controller:") {
                    let name = line[pos + 11..].trim().to_string();
                    gpus.push(GpuInfo { name });
                }
            }
        }
        if gpus.is_empty() {
            gpus.push(GpuInfo {
                name: "Standard Linux Video Device".to_string(),
            });
        }
        gpus
    }

    /// Нативный математический бенчмарк (аналог RunBenchmarkAsync)
    pub fn run_benchmark(&self, multithreaded: bool) -> f64 {
        let duration = Duration::from_secs(10);
        let start_time = Instant::now();

        if multithreaded {
            // Многопоточный просчет с помощью rayon
            let num_threads = num_cpus::get();
            let total_ops: i64 = (0..num_threads)
                .into_par_iter()
                .map(|i| {
                    let mut a: f64 = 1.000001 + (i as f64) * 0.00001;
                    let mut b: f64 = 1.000002 + (i as f64) * 0.00002;
                    let mut local_ops = 0;

                    while start_time.elapsed() < duration {
                        for _ in 0..50_000 {
                            a = a.sin() * b.cos() + (a + b).abs().sqrt();
                            b = a * 0.999999 + b * 0.000001;
                            local_ops += 2;
                        }
                    }
                    local_ops
                })
                .sum();

            (total_ops as f64 / duration.as_secs_f64()) / 100000.0
        } else {
            // Однопоточный просчет
            let mut a: f64 = 1.000001;
            let mut b: f64 = 1.000002;
            let mut ops = 0;

            while start_time.elapsed() < duration {
                for _ in 0..50_000 {
                    a = a.sin() * b.cos() + (a + b).abs().sqrt();
                    b = a * 0.999999 + b * 0.000001;
                    ops += 2;
                }
            }
            (ops as f64 / duration.as_secs_f64()) / 100000.0
        }
    }

    /// Сохранение информации в текстовый файл (аналог SaveDataToTxt)
    pub fn save_report_to_txt(&mut self, path: &str) -> std::io::Error {
        let (cpu_name, cores, freq) = self.get_cpu_info();
        let ram = self.get_ram_info();

        let mut file = match File::create(path) {
            Ok(f) => f,
            Err(e) => return e,
        };

        let mut report = String::new();
        report.push_str("SteeliumTweaker Hardware Report // Linux Native\n\n");
        report.push_str(&format!(
            "=== PROCESSOR ===\nName: {}\nCores: {}\nMax Frequency: {}\n\n",
            cpu_name, cores, freq
        ));
        report.push_str(&format!("=== RAM ===\nTotal Memory: {}\n\n", ram));

        report.push_str("=== STORAGE DEVICES ===\n");
        for (name, size) in self.get_storage_list() {
            report.push_str(&format!("- {}: {}\n", name, size));
        }

        if let Err(e) = file.write_all(report.as_bytes()) {
            return e;
        }

        std::io::Error::new(std::io::ErrorKind::Other, "Success")
    }
}
