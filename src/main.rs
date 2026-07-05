mod advanced;
mod app_blocker;
mod confirmation_dialog;
mod mainwindow;
mod package_manager;
mod performance;
mod personalization;
mod process_manager;
mod quick_settings;
mod sat_handler;
mod settings_info;
mod system_info;
mod system_tweaks;
mod windows_components;
mod windows_update;

use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

// Импорты ваших структур
use advanced::AdvancedHandler as AdvancedTweaker;
use app_blocker::AppBlockerHandler;
use package_manager::PackageManagerHandler;
use performance::PerformanceHandler as PerformanceTweaker;
use process_manager::ProcessManager;
use quick_settings::QuickSettingsHandler;
use sat_handler::SatHandler;
use settings_info::SettingsAboutHandler;
use system_info::SystemInfoHandler;
use system_tweaks::SystemTweaksHandler;
use windows_components::WindowsComponentsTweaker;
use windows_update::WindowsUpdateTweaker;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // =========================================================================
    // ИНИЦИАЛИЗАЦИЯ ДВИЖКОВ БЭКЕНДА
    // =========================================================================
    let perf_tweaker = Rc::new(PerformanceTweaker::new());
    let adv_tweaker = Rc::new(AdvancedTweaker::new());
    let process_manager = Rc::new(Mutex::new(ProcessManager::new()));
    let package_manager = Rc::new(PackageManagerHandler::new());
    let app_blocker = Arc::new(Mutex::new(AppBlockerHandler::new()));
    let mut system_info = SystemInfoHandler::new();
    let settings_about = Rc::new(std::cell::RefCell::new(SettingsAboutHandler::new()));
    let sat_handler = Rc::new(SatHandler::new());
    let system_tweaks = Rc::new(SystemTweaksHandler::new());
    let win_components = Rc::new(WindowsComponentsTweaker::new());
    let win_update = Rc::new(WindowsUpdateTweaker::new());

    let quick_settings = Rc::new(QuickSettingsHandler::new(
        perf_tweaker.clone(),
        system_tweaks.clone(),
        win_update.clone(),
    ));

    // Загрузка данных системы
    let (cpu_name, cores, freq) = system_info.get_cpu_info();
    ui.set_cpu_info(SharedString::from(cpu_name));
    ui.set_cpu_cores(cores as i32);
    ui.set_cpu_freq(SharedString::from(freq));
    ui.set_ram_summary(SharedString::from(system_info.get_ram_info()));

    // Настройки интерфейса
    ui.set_about_version(SharedString::from(&settings_about.borrow().version));
    ui.set_about_credits(SharedString::from(format!(
        "{}\n{}\nCommunity Translators & Contributors",
        settings_about.borrow().version,
        settings_about.borrow().authors
    )));
    ui.set_winr_alias_active(settings_about.borrow().check_winr_alias_status());

    // Начальные состояния триггеров
    ui.set_has_battery(system_tweaks.has_battery());
    ui.set_telemetry_disabled(system_tweaks.check_telemetry_disabled());
    ui.set_photo_viewer_applied(win_components.is_photo_viewer_enabled());
    ui.set_game_dvr_disabled(win_components.is_game_dvr_disabled());
    ui.set_updates_blocked(win_update.is_updates_blocked());
    ui.set_visible_tweaks_count(quick_settings.get_applicable_tweaks_count() as i32);

    // =========================================================================
    // QUICK SETTINGS
    // =========================================================================
    let qs_makuos = quick_settings.clone();
    let ui_weak_qs = ui.as_weak();
    ui.on_apply_makuos_preset(move || {
        qs_makuos.apply_makuos_preset();
        if let Some(ui) = ui_weak_qs.upgrade() {
            ui.set_telemetry_disabled(true);
            ui.set_updates_blocked(true);
            ui.set_visible_tweaks_count(0);
        }
        println!("Комплексный Linux-пресет оптимизации MakuOS успешно применен.");
    });

    let qs_hide = quick_settings.clone();
    ui.on_hide_applied_tweaks(move || qs_hide.get_applicable_tweaks_count() as i32);

    // =========================================================================
    // PERFORMANCE & ADVANCED
    // =========================================================================
    let pt_gamemode = perf_tweaker.clone();
    ui.on_toggle_gamemode(move |enable| {
        pt_gamemode.toggle_gamemode(enable);
    });

    let pt_gfx = perf_tweaker.clone();
    ui.on_toggle_gfx_optimization(move |enable| {
        pt_gfx.toggle_gfx_env(enable);
    });

    let at_service = adv_tweaker.clone();
    ui.on_manage_service(move |service_name, action| {
        let _ = at_service.control_service(&service_name, &action);
    });

    // =========================================================================
    // КОНФИДЕНЦИАЛЬНОСТЬ (SysAndRec)
    // =========================================================================
    let st_telemetry = system_tweaks.clone();
    ui.on_toggle_system_telemetry(move |disable| {
        st_telemetry.toggle_telemetry(disable);
    });

    let st_battery = system_tweaks.clone();
    ui.on_generate_battery_report(move || {
        st_battery.generate_battery_report();
    });

    // =========================================================================
    // КОМПОНЕНТЫ СИСТЕМЫ
    // =========================================================================
    let wc_pv = win_components.clone();
    let ui_weak_pv = ui.as_weak();
    ui.on_enable_classic_photo_viewer(move || {
        let success = wc_pv.enable_photo_viewer();
        if success {
            if let Some(ui) = ui_weak_pv.upgrade() {
                ui.set_photo_viewer_applied(true);
            }
        }
    });

    let wc_dvr = win_components.clone();
    let ui_weak_dvr = ui.as_weak();
    ui.on_toggle_game_dvr(move |disable| {
        if disable {
            if wc_dvr.disable_game_dvr() {
                if let Some(ui) = ui_weak_dvr.upgrade() {
                    ui.set_game_dvr_disabled(true);
                }
            }
        }
    });

    let wc_hyperv = win_components.clone();
    ui.on_disable_hyperv(move || {
        wc_hyperv.disable_hyperv();
    });

    // =========================================================================
    // ОБНОВЛЕНИЯ
    // =========================================================================
    let wu_toggle = win_update.clone();
    ui.on_toggle_system_updates(move |block| {
        let _ = wu_toggle.toggle_updates(block);
    });

    let wu_clean = win_update.clone();
    let ui_weak_wu = ui.as_weak();
    ui.on_clean_update_cache(move || {
        let _freed_mb = wu_clean.clean_update_cache();
        if let Some(ui) = ui_weak_wu.upgrade() {
            ui.set_update_cache_cleaned(true);
        }
    });

    // =========================================================================
    // PROCESS MANAGER
    // =========================================================================
    let pm_refresh = process_manager.clone();
    let ui_weak = ui.as_weak();
    ui.on_refresh_processes(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut pm = pm_refresh.lock().unwrap();
            pm.refresh();

            let slint_processes: Vec<ProcessItem> = pm
                .get_process_list()
                .iter()
                .map(|p| ProcessItem {
                    pid: p.pid as i32,
                    name: SharedString::from(&p.name),
                    cpu: SharedString::from(format!("{:.1}%", p.cpu_usage)),
                    memory: SharedString::from(format!("{} MB", p.memory_bytes / 1024 / 1024)),
                })
                .collect();

            let model = Rc::new(VecModel::from(slint_processes));
            ui.set_process_list(ModelRc::from(model));
        }
    });

    let pm_kill = process_manager.clone();
    ui.on_kill_process(move |pid| {
        let mut pm = pm_kill.lock().unwrap();
        pm.kill_process(pid as u32);
    });

    // =========================================================================
    // APP BLOCKER
    // =========================================================================
    let ab_add = app_blocker.clone();
    ui.on_add_blocked_app(move |_process_name| {
        let mut ab = ab_add.lock().unwrap();
        let _ = ab.toggle_yandex_blocking(true);
    });

    let ab_remove = app_blocker.clone();
    ui.on_remove_blocked_app(move |_process_name| {
        let mut ab = ab_remove.lock().unwrap();
        let _ = ab.toggle_yandex_blocking(false);
    });

    // =========================================================================
    // PACKAGE MANAGER
    // =========================================================================
    let pm_list = package_manager.clone();
    let ui_weak_pm = ui.as_weak();
    ui.on_load_flatpaks(move || {
        if let Some(ui) = ui_weak_pm.upgrade() {
            let packages = pm_list.get_installed_packages();
            let slint_packages: Vec<FlatpakItem> = packages
                .into_iter()
                .map(|p| FlatpakItem {
                    id: SharedString::from(p.id),
                    name: SharedString::from(p.name),
                    version: SharedString::from(p.category),
                })
                .collect();
            let model = Rc::new(VecModel::from(slint_packages));
            ui.set_package_list(ModelRc::from(model));
        }
    });

    ui.run()
}
