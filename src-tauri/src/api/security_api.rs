// Copyright (c) 2026 Simon Project
// Licensed under the Apache License, Version 2.0
// Part of the Simon Project BlueCore Browser

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;
use crate::core::security_manager::{SecurityManager, TimeRange, SitePermissions, SafetyCheckReport};

#[tauri::command]
pub async fn run_safety_check(
    security_manager: State<'_, Arc<Mutex<SecurityManager>>>,
) -> Result<SafetyCheckReport, String> {
    let manager = security_manager.lock().await;
    Ok(manager.run_safety_check())
}

#[tauri::command]
pub async fn clear_browser_data(
    time_range: TimeRange,
    security_manager: State<'_, Arc<Mutex<SecurityManager>>>,
) -> Result<String, String> {
    let mut manager = security_manager.lock().await;
    manager.clear_browser_data(time_range)
}

#[tauri::command]
pub async fn get_site_permissions(
    origin: String,
    security_manager: State<'_, Arc<Mutex<SecurityManager>>>,
) -> Result<Option<SitePermissions>, String> {
    let manager = security_manager.lock().await;
    Ok(manager.get_site_permissions(&origin).cloned())
}

#[tauri::command]
pub async fn update_site_permissions(
    permissions: SitePermissions,
    security_manager: State<'_, Arc<Mutex<SecurityManager>>>,
) -> Result<(), String> {
    let mut manager = security_manager.lock().await;
    manager.update_site_permissions(permissions);
    Ok(())
}

#[tauri::command]
pub async fn open_incognito_window(
    security_manager: State<'_, Arc<Mutex<SecurityManager>>>,
) -> Result<(), String> {
    let mut manager = security_manager.lock().await;
    manager.set_incognito_mode(true);
    println!("Incognito session started – SessionManager switched to Non-Persistent mode.");
    Ok(())
}

#[tauri::command]
pub async fn close_incognito_window(
    security_manager: State<'_, Arc<Mutex<SecurityManager>>>,
) -> Result<(), String> {
    let mut manager = security_manager.lock().await;
    manager.set_incognito_mode(false);
    Ok(())
}
