// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
pub async fn set_reader_preferences(font_size: u8, theme_style: String, ui_config: State<'_, Arc<Mutex<UIConfigManager>>>) -> Result<(), String> {
    let mut config = ui_config.lock().await;
    config.reader_settings.font_size = font_size;
    config.reader_settings.theme = theme_style;
    Ok(())
}

#[tauri::command]
pub async fn get_reader_settings(ui_config: State<'_, Arc<Mutex<UIConfigManager>>>) -> Result<ReaderSettings, String> {
    let config = ui_config.lock().await;
    Ok(config.reader_settings.clone())
}

#[tauri::command]
pub async fn start_tts_engine(tab_id: String) -> Result<(), String> {
    println!("TTS: Starting high-quality speech synthesis for tab {}", tab_id);
    // Future: Integrate with OS TTS (Speech-dispatcher on Linux, SAPI on Windows)
    Ok(())
}

// Keep backward compat alias
#[tauri::command]
pub async fn speak_content(tab_id: String) -> Result<(), String> {
    start_tts_engine(tab_id).await
}
