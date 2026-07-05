use std::fs;
use std::path::Path;

pub struct WindowsComponentsTweaker;

impl WindowsComponentsTweaker {
    pub fn new() -> Self {
        Self
    }

    pub fn is_photo_viewer_enabled(&self) -> bool {
        true
    }

    pub fn is_game_dvr_disabled(&self) -> bool {
        true
    }

    pub fn enable_photo_viewer(&self) -> bool {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        if let Err(e) = fs::create_dir_all(format!("{}/.config/MangoHud", home)) {
            eprintln!("Не удалось создать директорию MangoHud: {}", e);
            return false;
        }
        true
    }

    pub fn disable_game_dvr(&self) -> bool {
        true
    }

    pub fn disable_hyperv(&self) {
        // В Linux оставляем пустым
    }
}
