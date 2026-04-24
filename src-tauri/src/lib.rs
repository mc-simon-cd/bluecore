// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod core;
pub mod engines;
pub mod api;
pub mod modules;
pub mod extensions;
pub mod config;
pub mod security;
pub mod storage;
pub mod identity;

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::core::engine_manager::EngineManager;
use crate::core::tab_manager::TabManager;
use crate::core::ui_config_manager::UIConfigManager;
use crate::core::split_view_manager::SplitViewManager;
use crate::core::performance_manager::PerformanceManager;
use crate::core::security_manager::SecurityManager;
use crate::engines::tauri_engine::TauriEngine;
use crate::identity::SessionManager;

use crate::modules::{ModuleRegistry, privacy_shield::PrivacyShield};

pub struct BlueCoreState {
    pub engine_manager: Arc<Mutex<EngineManager>>,
    pub tab_manager: Arc<Mutex<TabManager>>,
    pub session_manager: Arc<Mutex<SessionManager>>,
    pub ui_config: Arc<Mutex<UIConfigManager>>,
    pub split_view_manager: Arc<Mutex<SplitViewManager>>,
    pub performance_manager: Arc<Mutex<PerformanceManager>>,
    pub security_manager: Arc<Mutex<SecurityManager>>,
}

pub fn run() {
    let engine_manager = EngineManager::new(Box::new(TauriEngine));
    let tab_manager = TabManager::new();
    let session_manager = SessionManager::new();
    let ui_config_manager = UIConfigManager::new();
    let split_view_manager = SplitViewManager::new();
    let performance_manager = PerformanceManager::new();
    let security_manager = SecurityManager::new();
    let module_registry = ModuleRegistry::new();

    // Register initial modules
    tauri::async_runtime::block_on(async {
        module_registry.register(Box::new(PrivacyShield::new())).await;
    });

    let state = BlueCoreState {
        engine_manager: Arc::new(Mutex::new(engine_manager)),
        tab_manager: Arc::new(Mutex::new(tab_manager)),
        session_manager: Arc::new(Mutex::new(session_manager)),
        ui_config: Arc::new(Mutex::new(ui_config_manager)),
        split_view_manager: Arc::new(Mutex::new(split_view_manager)),
        performance_manager: Arc::new(Mutex::new(performance_manager)),
        security_manager: Arc::new(Mutex::new(security_manager)),
    };

    tauri::Builder::default()
        .manage(state.engine_manager)
        .manage(state.tab_manager)
        .manage(state.session_manager)
        .manage(state.ui_config)
        .manage(state.split_view_manager)
        .manage(state.performance_manager)
        .manage(state.security_manager)
        .manage(module_registry)

        .invoke_handler(tauri::generate_handler![
            api::engine_api::switch_engine,
            api::tab_api::create_tab,
            api::tab_api::close_tab,
            api::identity_api::get_session,
            api::identity_api::login,
            api::identity_api::logout,
            api::modules::get_available_modules,
            api::modules::toggle_module,
            api::modules::execute_module_action,
            api::ui_api::set_vertical_tabs,
            api::ui_api::set_tab_sidebar_collapsed,
            api::ui_api::group_tabs_by_domain,
            api::ui_api::get_ntp_config,
            api::ui_api::update_ntp_setting,
            api::ui_api::set_ntp_background,
            api::ui_api::set_browser_theme,
            api::content_api::toggle_reader_mode,
            api::content_api::set_reader_preferences,
            api::content_api::get_reader_settings,
            api::content_api::start_tts_engine,
            api::content_api::speak_content,
            api::navigation_api::bc_handle_navigation,
            api::split_api::enable_split_view,
            api::split_api::update_split_focus,
            api::split_api::resize_split_panes,
            api::split_api::swap_split_panes,
            api::split_api::disable_split_view,
            api::split_api::get_split_view_state,
            api::performance_api::get_performance_stats,
            api::performance_api::set_memory_saver,
            api::performance_api::set_energy_saver,
            api::performance_api::freeze_tab,
            api::performance_api::unfreeze_tab,
            api::performance_api::add_memory_exception,
            api::performance_api::remove_memory_exception,
            api::performance_api::get_experimental_flags,
            api::performance_api::set_experimental_flag,
            api::security_api::run_safety_check,
            api::security_api::clear_browser_data,
            api::security_api::get_site_permissions,
            api::security_api::update_site_permissions,
            api::security_api::open_incognito_window,
            api::security_api::close_incognito_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
