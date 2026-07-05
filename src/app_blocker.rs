use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

pub struct AppBlockerHandler {
    hosts_path: &'static str,
}

impl AppBlockerHandler {
    pub fn new() -> Self {
        Self {
            hosts_path: "/etc/hosts",
        }
    }

    /// Аналог BlockYandex_Click: блокирует домены телеметрии/сервисов Яндекса через /etc/hosts
    pub fn toggle_yandex_blocking(&self, block: bool) -> Result<(), std::io::Error> {
        let yandex_hosts = vec![
            "127.0.0.1 yandex.ru",
            "127.0.0.1 www.yandex.ru",
            "127.0.0.1 metrika.yandex.ru",
            "127.0.0.1 mc.yandex.ru",
            "127.0.0.1 passport.yandex.ru",
        ];

        // Читаем текущий /etc/hosts
        let file = File::open(self.hosts_path)?;
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

        if block {
            // Добавляем хосты, если их еще нет
            for host in yandex_hosts {
                if !lines.iter().any(|line| line.contains(host)) {
                    lines.push(host.to_string());
                }
            }
        } else {
            // Удаляем хосты Яндекса из списка
            lines.retain(|line| !yandex_hosts.iter().any(|&y_host| line.contains(y_host)));
        }

        // Записываем обратно (требуются права root/sudo, поэтому приложение должно быть запущено должным образом и[...]
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.hosts_path)?;
        for line in lines {
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }

    /// Применяет пользовательский список блокировки процессов (Аналог ApplyDisallowRun)
    /// В Linux один из лучших способов забанить запуск для юзера — создать заглушку в начале PATH
    pub fn apply_custom_process_block(&self, raw_input: String) -> Result<(), std::io::Error> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let block_dir = std::path::Path::new(&home).join(".local/bin");

        // Разделяем строку по запятым или переносам строк (как в TextBox)
        let bad_apps: Vec<&str> = raw_input
            .split(|c| c == ',' || c == '\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for app in bad_apps {
            let app_path = block_dir.join(app);
            // Создаем пустой неисполняемый файл-заглушку или скрипт, выдающий ошибку
            let mut file = File::create(app_path)?;
            writeln!(
                file,
                "#!/bin/sh\necho 'This application has been blocked by SteeliumTweaker!'\nexit 1"
            )?;

            // Делаем исполняемым
            Command::new("chmod")
                .args(&["+x", block_dir.join(app).to_str().unwrap()])
                .status()?;
        }

        Ok(())
    }
}
