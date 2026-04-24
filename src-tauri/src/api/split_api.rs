// Copyright (c) 2026 Simon Project
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;
use crate::core::split_view_manager::{SplitViewManager, SplitViewState, SplitPane};

#[tauri::command]
pub async fn enable_split_view(
    tab_id: String,
    url: String,
    left_url: String,
    split_manager: State<'_, Arc<Mutex<SplitViewManager>>>,
) -> Result<SplitViewState, String> {
    let mut manager = split_manager.lock().await;
    manager.enable_split_view(tab_id, left_url, url)
}

#[tauri::command]
pub async fn update_split_focus(
    tab_id: String,
    active_pane: String,
    split_manager: State<'_, Arc<Mutex<SplitViewManager>>>,
) -> Result<(), String> {
    let pane = SplitPane::from_str(&active_pane)?;
    let mut manager = split_manager.lock().await;
    manager.set_focus(&tab_id, pane)
}

#[tauri::command]
pub async fn resize_split_panes(
    tab_id: String,
    ratio: f32,
    split_manager: State<'_, Arc<Mutex<SplitViewManager>>>,
) -> Result<(), String> {
    let mut manager = split_manager.lock().await;
    manager.resize_panes(&tab_id, ratio)
}

#[tauri::command]
pub async fn swap_split_panes(
    tab_id: String,
    split_manager: State<'_, Arc<Mutex<SplitViewManager>>>,
) -> Result<(), String> {
    let mut manager = split_manager.lock().await;
    manager.swap_panes(&tab_id)
}

#[tauri::command]
pub async fn disable_split_view(
    tab_id: String,
    split_manager: State<'_, Arc<Mutex<SplitViewManager>>>,
) -> Result<(), String> {
    let mut manager = split_manager.lock().await;
    manager.disable_split_view(&tab_id);
    Ok(())
}

#[tauri::command]
pub async fn get_split_view_state(
    tab_id: String,
    split_manager: State<'_, Arc<Mutex<SplitViewManager>>>,
) -> Result<Option<SplitViewState>, String> {
    let manager = split_manager.lock().await;
    Ok(manager.get_split_view(&tab_id).cloned())
}
