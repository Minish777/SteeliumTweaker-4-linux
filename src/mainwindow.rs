use serde::{Deserialize, Serialize};
use slint::{ComponentHandle, Model, SharedString, VecModel};
use std::collections::{HashMap, HashSet};
use std::process::Command;

// Вшиваем файлы локализации и сборки (как ресурсы в C#)
const LOCALIZATION_RU: &str = include_str!("../loc/ru.json");

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TweakSuggestion {
    pub id: String,
    pub display_name: String,
    pub category_key: String,
    pub internal_category_tag: String,
}

pub struct MainWindowHandler {
    search_library: Vec<TweakSuggestion>,
}

impl MainWindowHandler {
    pub fn new() -> Self {
        let mut handler = Self {
            search_library: Vec::new(),
        };
        handler.initialize_search();
        handler
    }

    // Аналог GetAllTweaksForSearch из C#
    fn initialize_search(&mut self) {
        let v: serde_json::Value = match serde_json::from_str(LOCALIZATION_RU) {
            Ok(json) => json,
            Err(_) => return,
        };

        let categories = match v.get("categories") {
            Some(c) => c,
            None => return,
        };

        let cat_names: HashMap<String, String> = categories
            .get("base")
            .and_then(|b| b.get("catname"))
            .and_then(|c| serde_json::from_value(c.clone()).ok())
            .unwrap_or_default();

        let ignored_keys: HashSet<&str> = [
            "label", "choose", "showall", "info1", "info2", "info3", "chk", "comp", "b", "mode1",
            "mode2", "mode3", "tpmy", "tpmn", "tooltip", "title", "info",
        ]
        .iter()
        .cloned()
        .collect();

        if let Some(obj) = categories.as_object() {
            for (cat_name, cat_val) in obj {
                if ["base", "pmgr", "perfor", "settings", "ab", "quick"]
                    .contains(&cat_name.as_str())
                {
                    continue;
                }
                if let Some(main) = cat_val.get("main").and_then(|m| m.as_object()) {
                    let display_cat = cat_names
                        .get(cat_name)
                        .cloned()
                        .unwrap_or_else(|| cat_name.clone());

                    for (tweak_key, tweak_val) in main {
                        if ignored_keys.contains(tweak_key.as_str())
                            || tweak_key.starts_with("tooltip")
                        {
                            continue;
                        }
                        if let Some(display_value) = tweak_val.as_str() {
                            if display_value.len() > 90 {
                                continue;
                            }

                            self.search_library.push(TweakSuggestion {
                                id: tweak_key.clone(),
                                display_name: display_value.to_string(),
                                category_key: display_cat.clone(),
                                internal_category_tag: cat_name.clone(),
                            });
                        }
                    }
                }
            }
        }
    }

    // Аналог AutoSuggestBox_TextChanged
    pub fn handle_search_changed(&self, query: String) -> Vec<String> {
        if query.trim().is_empty() {
            return Vec::new();
        }

        let query_lower = query.to_lowercase();
        self.search_library
            .iter()
            .filter(|t| t.display_name.to_lowercase().contains(&query_lower))
            .take(10)
            .map(|t| t.display_name.clone())
            .collect()
    }

    // Аналог rexplorer_Click (Перезапуск графического окружения/панели в Linux)
    pub fn restart_linux_panel(&self) {
        // Квази-аналог taskkill + explorer.exe. Убиваем и запускаем waybar (или твой бар/wms)
        let _ = Command::new("killall").arg("waybar").output();

        // Запускаем в фоне, чтобы не вешать наш твикер
        std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_millis(500));
            let _ = Command::new("waybar").spawn();
        });
    }
}
