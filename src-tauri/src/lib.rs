// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

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
use crate::engines::tauri_engine::TauriEngine;
use crate::identity::SessionManager;

use crate::modules::{ModuleRegistry, privacy_shield::PrivacyShield};

pub struct BlueCoreState {
    pub engine_manager: Arc<Mutex<EngineManager>>,
    pub tab_manager: Arc<Mutex<TabManager>>,
    pub session_manager: Arc<Mutex<SessionManager>>,
    pub ui_config: Arc<Mutex<UIConfigManager>>,
}

pub fn run() {
    let engine_manager = EngineManager::new(Box::new(TauriEngine));
    let tab_manager = TabManager::new();
    let session_manager = SessionManager::new();
    let ui_config_manager = UIConfigManager::new();
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
    };

    tauri::Builder::default()
        .manage(state.engine_manager)
        .manage(state.tab_manager)
        .manage(state.session_manager)
        .manage(state.ui_config)
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
            api::ui_api::get_ntp_config,
            api::ui_api::update_ntp_setting,
            api::ui_api::set_browser_theme,
            api::content_api::toggle_reader_mode,
            api::content_api::get_reader_settings,
            api::content_api::speak_content,
            api::navigation_api::bc_handle_navigation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
