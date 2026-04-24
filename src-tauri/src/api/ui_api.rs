// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
pub async fn set_tab_sidebar_collapsed(collapsed: bool, ui_config: State<'_, Arc<Mutex<UIConfigManager>>>) -> Result<(), String> {
    let mut config = ui_config.lock().await;
    config.set_sidebar_collapsed(collapsed);
    Ok(())
}

#[tauri::command]
pub async fn group_tabs_by_domain(tab_domain_map: Vec<(String, String)>, ui_config: State<'_, Arc<Mutex<UIConfigManager>>>) -> Result<(), String> {
    let mut config = ui_config.lock().await;
    config.group_tabs_by_domain(tab_domain_map);
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
pub async fn set_ntp_background(source_path: String, ui_config: State<'_, Arc<Mutex<UIConfigManager>>>) -> Result<(), String> {
    let mut config = ui_config.lock().await;
    config.ntp_config.background_url = if source_path.is_empty() { None } else { Some(source_path) };
    Ok(())
}

#[tauri::command]
pub async fn set_browser_theme(mode: BrowserTheme, ui_config: State<'_, Arc<Mutex<UIConfigManager>>>) -> Result<(), String> {
    let mut config = ui_config.lock().await;
    config.set_theme(mode);
    Ok(())
}
