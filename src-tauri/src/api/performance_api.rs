// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;
use crate::core::performance_manager::{PerformanceManager, PerformanceStats, ExperimentalFlag};

#[tauri::command]
pub async fn get_performance_stats(
    perf_manager: State<'_, Arc<Mutex<PerformanceManager>>>,
) -> Result<PerformanceStats, String> {
    let manager = perf_manager.lock().await;
    Ok(manager.get_stats())
}

#[tauri::command]
pub async fn set_memory_saver(
    enabled: bool,
    perf_manager: State<'_, Arc<Mutex<PerformanceManager>>>,
) -> Result<(), String> {
    let mut manager = perf_manager.lock().await;
    manager.set_memory_saver(enabled);
    Ok(())
}

#[tauri::command]
pub async fn set_energy_saver(
    enabled: bool,
    perf_manager: State<'_, Arc<Mutex<PerformanceManager>>>,
) -> Result<(), String> {
    let mut manager = perf_manager.lock().await;
    manager.set_energy_saver(enabled);
    Ok(())
}

#[tauri::command]
pub async fn freeze_tab(
    tab_id: String,
    perf_manager: State<'_, Arc<Mutex<PerformanceManager>>>,
) -> Result<bool, String> {
    let mut manager = perf_manager.lock().await;
    Ok(manager.freeze_tab(tab_id))
}

#[tauri::command]
pub async fn unfreeze_tab(
    tab_id: String,
    perf_manager: State<'_, Arc<Mutex<PerformanceManager>>>,
) -> Result<(), String> {
    let mut manager = perf_manager.lock().await;
    manager.unfreeze_tab(&tab_id);
    Ok(())
}

#[tauri::command]
pub async fn add_memory_exception(
    origin: String,
    perf_manager: State<'_, Arc<Mutex<PerformanceManager>>>,
) -> Result<(), String> {
    let mut manager = perf_manager.lock().await;
    manager.add_memory_exception(origin);
    Ok(())
}

#[tauri::command]
pub async fn remove_memory_exception(
    origin: String,
    perf_manager: State<'_, Arc<Mutex<PerformanceManager>>>,
) -> Result<(), String> {
    let mut manager = perf_manager.lock().await;
    manager.remove_memory_exception(&origin);
    Ok(())
}

#[tauri::command]
pub async fn get_experimental_flags(
    perf_manager: State<'_, Arc<Mutex<PerformanceManager>>>,
) -> Result<Vec<ExperimentalFlag>, String> {
    let manager = perf_manager.lock().await;
    Ok(manager.get_experimental_flags().clone())
}

#[tauri::command]
pub async fn set_experimental_flag(
    flag_id: String,
    enabled: bool,
    perf_manager: State<'_, Arc<Mutex<PerformanceManager>>>,
) -> Result<(), String> {
    let mut manager = perf_manager.lock().await;
    manager.set_experimental_flag(&flag_id, enabled)
}
