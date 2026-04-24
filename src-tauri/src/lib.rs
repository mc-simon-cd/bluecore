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

use std::sync::Mutex;
use crate::core::engine_manager::EngineManager;
use crate::core::tab_manager::TabManager;
use crate::engines::tauri_engine::TauriEngine;
use crate::identity::SessionManager;

use crate::modules::{ModuleRegistry, privacy_shield::PrivacyShield};

pub fn run() {
    let engine_manager = EngineManager::new(Box::new(TauriEngine));
    let tab_manager = TabManager::new();
    let session_manager = SessionManager::new();
    let module_registry = ModuleRegistry::new();

    // Register initial modules
    tauri::async_runtime::block_on(async {
        module_registry.register(Box::new(PrivacyShield::new())).await;
    });

    tauri::Builder::default()
        .manage(Mutex::new(engine_manager))
        .manage(Mutex::new(tab_manager))
        .manage(Mutex::new(session_manager))
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
