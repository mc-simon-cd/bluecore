// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;
use crate::core::ui_config_manager::{UIConfigManager, ReaderSettings};
use serde::Serialize;

#[derive(Serialize)]
pub struct ReaderState {
    pub active: bool,
    pub settings: ReaderSettings,
}

#[tauri::command]
pub async fn toggle_reader_mode(tab_id: String, active: bool, ui_config: State<'_, Arc<Mutex<UIConfigManager>>>) -> Result<ReaderState, String> {
    let mut config = ui_config.lock().await;
    let is_active = config.set_reader_mode(tab_id, active).await?;
    Ok(ReaderState {
        active: is_active,
        settings: config.reader_settings.clone(),
    })
}

#[tauri::command]
pub async fn get_reader_settings(ui_config: State<'_, Arc<Mutex<UIConfigManager>>>) -> Result<ReaderSettings, String> {
    let config = ui_config.lock().await;
    Ok(config.reader_settings.clone())
}

#[tauri::command]
pub async fn speak_content(tab_id: String) -> Result<(), String> {
    println!("TTS: Speaking content for tab {}", tab_id);
    // Future: Integrate with OS-native TTS or Cloud API
    Ok(())
}
