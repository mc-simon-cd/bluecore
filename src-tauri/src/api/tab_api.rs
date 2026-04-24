// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

use std::sync::Mutex;
use tauri::State;
use crate::core::tab_manager::TabManager;

#[tauri::command]
pub async fn create_tab(url: String, tab_manager: State<'_, Mutex<TabManager>>) -> Result<String, String> {
    let mut manager = tab_manager.lock().map_err(|e| e.to_string())?;
    let id = manager.create_tab(url);
    Ok(id)
}

#[tauri::command]
pub async fn close_tab(id: String, tab_manager: State<'_, Mutex<TabManager>>) -> Result<bool, String> {
    let mut manager = tab_manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.close_tab(&id))
}
