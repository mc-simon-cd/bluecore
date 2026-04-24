// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;
use crate::core::ui_config_manager::{UIConfigManager, NTPSettings, BrowserTheme};

#[tauri::command]
pub async fn set_vertical_tabs(enabled: bool, ui_config: State<'_, Arc<Mutex<UIConfigManager>>>) -> Result<(), String> {
    let mut config = ui_config.lock().await;
    config.set_vertical_tabs(enabled);
    Ok(())
}

#[tauri::command]
pub async fn get_ntp_config(ui_config: State<'_, Arc<Mutex<UIConfigManager>>>) -> Result<NTPSettings, String> {
    let config = ui_config.lock().await;
    Ok(config.ntp_config.clone())
}

#[tauri::command]
pub async fn update_ntp_setting(settings: NTPSettings, ui_config: State<'_, Arc<Mutex<UIConfigManager>>>) -> Result<(), String> {
    let mut config = ui_config.lock().await;
    config.update_ntp_setting(settings);
    Ok(())
}

#[tauri::command]
pub async fn set_browser_theme(mode: BrowserTheme, ui_config: State<'_, Arc<Mutex<UIConfigManager>>>) -> Result<(), String> {
    let mut config = ui_config.lock().await;
    config.set_theme(mode);
    Ok(())
}
