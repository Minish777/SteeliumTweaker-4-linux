pub struct WindowsComponentsTweaker;

impl WindowsComponentsTweaker {
    pub fn new() -> Self {
        Self
    }

    pub fn enable_photo_viewer(&self) -> bool {
        // На Linux это не применимо, но реализуем для совместимости
        true
    }

    pub fn is_photo_viewer_enabled(&self) -> bool {
        true
    }

    pub fn disable_game_dvr(&self) -> bool {
        // На Linux это не требуется
        true
    }

    pub fn is_game_dvr_disabled(&self) -> bool {
        true
    }

    pub fn disable_hyperv(&self) -> bool {
        // Отключаем виртуализацию если нужно
        false
    }
}