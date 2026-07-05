use std::rc::Rc;

pub struct QuickSettingsHandler {
    perf: Rc<crate::performance::PerformanceHandler>,
    tweaks: Rc<crate::system_tweaks::SystemTweaksHandler>,
    updates: Rc<crate::windows_update::WindowsUpdateTweaker>,
}

impl QuickSettingsHandler {
    pub fn new(
        perf: Rc<crate::performance::PerformanceHandler>,
        tweaks: Rc<crate::system_tweaks::SystemTweaksHandler>,
        updates: Rc<crate::windows_update::WindowsUpdateTweaker>,
    ) -> Self {
        Self {
            perf,
            tweaks,
            updates,
        }
    }

    pub fn apply_makuos_preset(&self) {
        // Применяем набор оптимизаций
        self.tweaks.toggle_telemetry(true);
        let _ = self.updates.toggle_updates(true);
        let _ = self.perf.optimize_memory();
        let _ = self.perf.disable_animations();
    }

    pub fn get_applicable_tweaks_count(&self) -> i32 {
        let mut count = 0;
        
        if !self.tweaks.check_telemetry_disabled() {
            count += 1;
        }
        if !self.updates.is_updates_blocked() {
            count += 1;
        }
        
        count
    }
}